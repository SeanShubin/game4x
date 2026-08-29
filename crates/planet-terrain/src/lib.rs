//! One continuous field over the sphere, and the biome that falls out of it.
//!
//! `docs/notes/planet-appearance.md`: *generate a continuous field over the sphere and let
//! both the picture and the model read it, rather than generating a picture and a model
//! separately and hoping they agree.* The renderer reads it per point; the model reads it
//! once per territory, at that territory's seed. They cannot disagree, because one is
//! derived from the other rather than authored beside it.
//!
//! # It does not know the tessellation exists
//!
//! Nothing here takes a territory, a neighbour list or a cell boundary. [`sample`] takes a
//! direction and a seed and nothing else, which is what makes `spec/planet.md`'s *the
//! terrain of the realistic drawing is continuous ... it runs across boundaries* true by
//! construction rather than by care: there is no boundary in scope to run up against.
//!
//! The one regularity that does survive is latitude, and it is deliberate. A fixed axis
//! makes the same two territories polar on every planet - both five-neighbour cells - and
//! `docs/notes/planet-appearance.md` records Sean's decision that this is acceptable,
//! because real planets have ice caps and a player who learns where the cold is has learned
//! something true about the world rather than something about the tessellation.
//!
//! # Axes
//!
//! Four, three of them noise and one of them free:
//!
//! | Axis | Where it comes from |
//! | --- | --- |
//! | Elevation | Broad fractal noise for continents, with ridges added inland |
//! | Moisture | Fractal noise on an unrelated offset |
//! | Drainage | Fractal noise, and it only does work where moisture is already high |
//! | Temperature | Latitude, less a lapse rate for height. No noise layer at all |
//!
//! Drainage is Dwarf Fortress's insight rather than an invention: high rain and low
//! drainage is swamp, high rain and high drainage is forest, and it does not subdivide a
//! desert at all. Temperature is free on a sphere with a fixed axis - one cosine - and it
//! is the single strongest cue that a picture of a ball is a picture of a *planet*.
//!
//! # Reproducibility
//!
//! Floating point lives here, above the game logic, which
//! [`docs/architecture.md`](../../../docs/architecture.md) rule 3 allows. The biome that
//! comes out is an integer fact and that is what the model stores - the same arrangement
//! adjacency already has, where the geometry is computed here and handed in.
//!
//! Only addition, subtraction and multiplication are used, over `f64`, with no
//! transcendental functions and no library random number generator. So the same seed gives
//! the same world on every machine that agrees about IEEE arithmetic, which is what
//! `spec/invariants.md`'s *a game state is exactly the result of applying every transition
//! in order* needs in order to survive a replay on somebody else's computer.

use game_model::Biome;
use sphere_tessellation::Solid;
use sphere_tessellation::vec3::Vec3;

/// What the field says about one point on the sphere.
///
/// Every value is nominally in `0.0..=1.0`. Nothing depends on that being exact, and the
/// noise is not clamped to it, because a clamp would flatten the extremes into plateaus -
/// which is exactly where mountains and ocean trenches live.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sample {
    pub elevation: f64,
    pub moisture: f64,
    pub drainage: f64,
    pub temperature: f64,
}

/// Sea level, as an elevation. Below this is ocean.
///
/// Half, because [`spread`] centres the field on half - so this says *below the median is
/// water* rather than picking a number and hoping. A little over half the sphere comes out
/// wet once the ridges have lifted the land, which is what a planet looks like.
///
/// A constant here rather than a tuning figure in a release, because nothing has asked to
/// tune it; if a release ever does, it moves there.
const SEA_LEVEL: f64 = 0.5;
/// Above this is bare rock, whatever the moisture says.
///
/// Above one, which looks wrong until you notice that ridges are *added* to land: an
/// elevation is only bounded by one before they go on. Measured over four thousand points,
/// this puts bare rock on about a sixth of the land and nowhere else.
const TREE_LINE: f64 = 1.05;
/// Below this temperature the ground is ice, whatever else is true of it.
const FROZEN: f64 = 0.34;
/// Moisture below this is desert, above the second is jungle.
const ARID: f64 = 0.34;
const WET: f64 = 0.62;

/// How much colder the air is at the top of the range than at sea level.
///
/// A lapse rate, in temperature units per unit of elevation above sea level. Without it the
/// only cold is polar, and a mountain on the equator is as warm as the beach below it.
const LAPSE: f64 = 0.55;

/// How many octaves of noise are summed. Enough for a coastline to have inlets; more is
/// detail no territory-sized sample can see.
const OCTAVES: u32 = 6;

/// How much the ridges lift the ground they are added to.
const RELIEF: f64 = 0.42;

/// How far the field is stretched away from its middle before anything reads it.
///
/// Summing octaves narrows a field: each one is an independent average, so the total piles
/// up around the middle and the tails thin out. Measured over four thousand points, raw
/// fractal noise ran from 0.37 at the tenth percentile to 0.67 at the ninetieth - which
/// would leave a planet with almost no desert and almost no swamp, because both live in a
/// tail. This puts the tails back.
const SPREAD: f64 = 2.4;

/// Unrelated offsets, so the three noise fields are three fields rather than one field
/// read three times. Any constants would do; these are splitmix64's, already in the file.
const MOISTURE_OFFSET: u64 = 0x9E37_79B9_7F4A_7C15;
const DRAINAGE_OFFSET: u64 = 0xBF58_476D_1CE4_E5B9;
const RIDGE_OFFSET: u64 = 0x94D0_49BB_1331_11EB;

/// The field at one point. `direction` need not be normalized.
pub fn sample(direction: Vec3, seed: u64) -> Sample {
    let at = direction.normalized();

    // Continents first, at a frequency low enough that a landmass spans several
    // territories. `docs/notes/planet-appearance.md`: noise alone does not make continents,
    // so the shape of the land and the roughness of it are two different layers.
    let continents = spread(fractal(at, seed, 1.4));

    // Ridges are added only to ground that is already land, by a weight that grows from
    // nothing at the shoreline. Adding them everywhere would push the sea floor up into
    // the air; adding them with a hard cutoff at sea level would put a cliff along every
    // coast. Growing the weight smoothly does neither, and keeps the field continuous,
    // which is what `spec/planet.md` asks of the terrain.
    let inland = smooth(((continents - SEA_LEVEL) / (1.0 - SEA_LEVEL)).clamp(0.0, 1.0));
    let elevation = continents + ridged(at, seed ^ RIDGE_OFFSET, 3.3) * RELIEF * inland;

    let moisture = spread(fractal(at, seed ^ MOISTURE_OFFSET, 2.3));
    let drainage = spread(fractal(at, seed ^ DRAINAGE_OFFSET, 3.1));

    // Latitude, free. `+z` is north; `spec/planet.md` fixes the axis and puts the poles at
    // the centres of two pentagons, so this is a real axis rather than a second name for
    // elevation.
    let latitude = at.z.abs();
    let by_latitude = 1.0 - latitude * latitude;
    let above_sea = (elevation - SEA_LEVEL).max(0.0);
    let temperature = by_latitude - above_sea * LAPSE;

    Sample {
        elevation,
        moisture,
        drainage,
        temperature,
    }
}

/// The biome a sample crosses to.
///
/// Read the order as the questions a place answers about itself, most decisive first: is it
/// underwater, is it frozen, is it above the trees, is it dry, and only then how the wet
/// drains away. `docs/notes/planet-appearance.md` is firm that biomes should fall out of
/// where fields cross rather than be enumerated and placed, and this is that crossing
/// written down.
pub fn biome_of(sample: &Sample) -> Biome {
    if sample.elevation < SEA_LEVEL {
        // Ice caps float. A frozen sea is still not land, but it does not look like water.
        return if sample.temperature < FROZEN {
            Biome::Ice
        } else {
            Biome::Ocean
        };
    }
    on_land(sample)
}

/// The biome of ground, asked as though it were above sea level whatever its elevation.
///
/// Separate from [`biome_of`] because a territory whose sample says ocean may still have to
/// be land: `spec/planet.md` forbids two ocean territories from being adjacent, so some
/// water has to become the ground it would otherwise have been. This is what it becomes,
/// and it is the same crossing rather than a second opinion.
fn on_land(sample: &Sample) -> Biome {
    if sample.temperature < FROZEN {
        return Biome::Ice;
    }
    if sample.elevation > TREE_LINE {
        return Biome::Mountain;
    }
    if sample.moisture < ARID {
        return Biome::Desert;
    }
    if sample.moisture > WET {
        // Drainage does no work in a desert, which is the point of having it as an axis:
        // it only separates ground that is already wet. With six biomes there is one
        // answer either side, and the wet-and-badly-drained half is not yet a biome the
        // rules can tell apart - so both are jungle, and `docs/notes/biomes.md` records
        // that a planet may draw a swamp that resolves to one.
        return Biome::Jungle;
    }
    Biome::Grassland
}

/// Which world every planet is.
///
/// One world per size, the same one every time: `spec/planet.md` offers no way to ask for
/// a different one, and there is no seed in the command language.
///
/// It lives here rather than in either reader because **both** must use it. The model
/// takes a biome per territory from this field and the realistic drawing paints the same
/// field; if they were seeded separately, a territory could be ice in the model while the
/// picture drew jungle over it - which is the one thing `spec/planet.md` forbids when it
/// says a territory's biome *is what the terrain gives it*, and which the release checks
/// by eye in its fourth capability.
pub const WORLD_SEED: u64 = 20260828;

/// Where the water stops, as an elevation.
///
/// Exposed because a drawing needs it for two things the model never asks about: how deep
/// water is, and where to stop raising the ground. Both are questions about this constant
/// rather than about a biome, and a second copy of it in the renderer would be a number
/// that could drift.
pub fn sea_level() -> f64 {
    SEA_LEVEL
}

/// Fine-grained noise, for a drawing that needs detail below the scale of a biome.
///
/// One octave rather than six, at a high frequency: this is not another axis of the world,
/// it is the texture of ground. `docs/notes/planet-appearance.md` calls micro-variation *a
/// second, cheaper layer*, and cheap is the point - it is called several times per vertex.
///
/// `strand` picks an independent one, so a caller wanting four uncorrelated grains asks for
/// four strands rather than four offsets it had to invent.
///
/// Returns roughly `-1.0..=1.0`.
pub fn grain(at: Vec3, seed: u64, strand: u32) -> f64 {
    const GRAIN_FREQUENCY: f64 = 42.0;
    let twist = seed ^ (0xA076_1D64_78BD_642F_u64.wrapping_mul(strand as u64 + 1));
    value_noise(at.normalized().scaled(GRAIN_FREQUENCY), twist) * 2.0 - 1.0
}

/// The biome at one point, which is the whole of what the model reads.
pub fn biome_at(direction: Vec3, seed: u64) -> Biome {
    biome_of(&sample(direction, seed))
}

/// How many points across each fan triangle a territory's ground is sampled at.
///
/// `releases/first-release.md` vets a biome by *no other biome covers more of that ground
/// in the realistic drawing* - a plurality over an area, not the answer at one point. A
/// territory is wide enough to hold several biomes, so asking only at its seed would give a
/// point's answer to a question about a region, and be wrong whenever the seed sits in a
/// minority patch.
const GROUND_SAMPLES: usize = 7;

/// The biome of each territory, in id order.
///
/// One discrete answer per territory, which is what claiming and working one needs, and it
/// is the answer that covers most of that territory's ground.
///
/// # Why this needs the solid and the adjacency, and [`sample`] does not
///
/// The field knows nothing about the tessellation, and that is the whole point of it. But
/// `spec/planet.md` states things about *territories* that no field can answer:
///
/// - a biome is what covers the territory's ground, so the ground has to be walked
/// - *no territory can be claimed whose biome is ocean*
/// - *oceans never isolate land from land; every territory that is not ocean can be
///   reached from every other without crossing one*
///
/// That last one is a property of the whole arrangement rather than of any pair, so it
/// cannot be enforced while walking. **A candidate is produced and then tested**: every
/// territory takes the biome covering most of it, and if that leaves land in more than one
/// piece, the fewest oceans are given back to land until it is joined again.
///
/// It used to forbid two oceans touching, which is a *sufficient* condition for the same
/// thing and a much stronger one - it costs every coastline, every sea and every island
/// chain, because water could never pool. Adjacent oceans are legal now.
pub fn biomes_of(solid: &Solid, adjacency: &[Vec<usize>], seed: u64) -> Vec<Biome> {
    let ground: Vec<Vec<Sample>> = (0..solid.cells.len())
        .map(|at| ground_of(solid, at, seed))
        .collect();

    let mut biomes: Vec<Biome> = ground.iter().map(|samples| covering(samples)).collect();
    join_the_land(&mut biomes, &ground, adjacency);
    biomes
}

/// Samples spread over one territory's ground.
///
/// The same barycentric walk the realistic drawing uses, at a far lower density: this is
/// counting what covers the ground, not drawing it.
fn ground_of(solid: &Solid, cell: usize, seed: u64) -> Vec<Sample> {
    let corners: Vec<Vec3> = solid.cells[cell]
        .iter()
        .map(|&corner| solid.corners[corner as usize].vector())
        .collect();
    let centre = corners
        .iter()
        .fold(Vec3::ZERO, |total, &corner| total.add(corner))
        .normalized();

    let n = GROUND_SAMPLES;
    let mut samples = Vec::new();
    for step in 0..corners.len() {
        let from = corners[step];
        let to = corners[(step + 1) % corners.len()];
        for row in 0..=n {
            for column in 0..=row {
                let at = centre
                    .scaled((n - row) as f64)
                    .add(from.scaled((row - column) as f64))
                    .add(to.scaled(column as f64))
                    .normalized();
                samples.push(sample(at, seed));
            }
        }
    }
    samples
}

/// The biome covering most of a patch of ground.
///
/// Ties break by the order [`Biome::ALL`] lists them, because two biomes can genuinely
/// cover the same number of samples and iteration order must never decide a world.
fn covering(samples: &[Sample]) -> Biome {
    most_common(samples.iter().map(biome_of))
}

/// The land biome covering most of a patch, for ground that has to stop being water.
fn covering_land(samples: &[Sample]) -> Biome {
    most_common(samples.iter().map(on_land))
}

fn most_common(biomes: impl Iterator<Item = Biome>) -> Biome {
    let mut tally = [0usize; Biome::ALL.len()];
    for biome in biomes {
        tally[Biome::ALL.iter().position(|kind| *kind == biome).unwrap()] += 1;
    }
    let mut best = 0;
    for at in 1..tally.len() {
        if tally[at] > tally[best] {
            best = at;
        }
    }
    Biome::ALL[best]
}

/// Gives back the fewest oceans that leave the land in one piece.
///
/// `spec/planet.md`: *oceans never isolate land from land.* An island is what is forbidden,
/// not a sea - so this looks at the whole arrangement, and only where it is actually broken.
///
/// Each round drains the one ocean that touches the most separate pieces of land, because
/// that is the one join that does the most work. Ties break by how shallow it is and then by
/// id. It always terminates: draining every ocean joins everything trivially, and each round
/// drains one.
fn join_the_land(biomes: &mut [Biome], ground: &[Vec<Sample>], adjacency: &[Vec<usize>]) {
    loop {
        let pieces = pieces_of_land(biomes, adjacency);
        if pieces.len() <= 1 {
            return;
        }
        let mut best: Option<(usize, usize)> = None;
        for at in 0..biomes.len() {
            if biomes[at] != Biome::Ocean {
                continue;
            }
            let mut touched: Vec<usize> = adjacency[at]
                .iter()
                .filter_map(|near| pieces.iter().position(|piece| piece.contains(near)))
                .collect();
            touched.sort_unstable();
            touched.dedup();
            let joins = touched.len();
            if best.is_none_or(|(most, _)| joins > most) {
                best = Some((joins, at));
            }
        }
        // No ocean touches two pieces, so drain the shallowest one and try again. This
        // happens when pieces are separated by more than one ocean wide.
        let drain = match best {
            Some((joins, at)) if joins > 1 => at,
            _ => match (0..biomes.len())
                .filter(|at| biomes[*at] == Biome::Ocean)
                .min_by(|a, b| {
                    deepest(&ground[*b])
                        .total_cmp(&deepest(&ground[*a]))
                        .then(a.cmp(b))
                }) {
                Some(at) => at,
                None => return,
            },
        };
        biomes[drain] = covering_land(&ground[drain]);
    }
}

/// How low a territory's ground goes, for choosing which water to keep.
fn deepest(samples: &[Sample]) -> f64 {
    samples
        .iter()
        .map(|at| at.elevation)
        .fold(f64::INFINITY, f64::min)
}

/// The connected pieces of land, as sets of territory indices.
fn pieces_of_land(biomes: &[Biome], adjacency: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let mut seen = vec![false; biomes.len()];
    let mut pieces = Vec::new();
    for start in 0..biomes.len() {
        if seen[start] || biomes[start] == Biome::Ocean {
            continue;
        }
        let mut piece = Vec::new();
        let mut queue = vec![start];
        seen[start] = true;
        while let Some(at) = queue.pop() {
            piece.push(at);
            for near in &adjacency[at] {
                if !seen[*near] && biomes[*near] != Biome::Ocean {
                    seen[*near] = true;
                    queue.push(*near);
                }
            }
        }
        piece.sort_unstable();
        pieces.push(piece);
    }
    pieces
}

// ---------------------------------------------------------------------------
// The noise underneath. Value noise on an integer lattice, hashed rather than
// tabled, so there is no permutation array to carry and no state to thread.
// ---------------------------------------------------------------------------

/// Sums octaves of value noise, each half the amplitude and twice the frequency of the
/// last, and rescales the total back to `0.0..=1.0`.
fn fractal(at: Vec3, seed: u64, frequency: f64) -> f64 {
    let mut total = 0.0;
    let mut amplitude = 1.0;
    let mut scale = frequency;
    let mut sum = 0.0;
    for octave in 0..OCTAVES {
        total += amplitude * value_noise(at.scaled(scale), seed ^ (octave as u64) << 32);
        sum += amplitude;
        amplitude *= 0.5;
        scale *= 2.0;
    }
    total / sum
}

/// Stretches a field away from its middle, and holds it inside `0.0..=1.0`.
///
/// The ends do saturate, and that is wanted rather than tolerated: a flat abyssal plain
/// and a flat ice cap are what the extremes of a real field look like. The interesting
/// structure is in the middle, where nothing is clipped.
fn spread(value: f64) -> f64 {
    (0.5 + (value - 0.5) * SPREAD).clamp(0.0, 1.0)
}

/// Fractal noise folded about its midpoint, so the peaks become creases.
///
/// `1 - |2n - 1|` is the fold. Squaring afterwards pulls the low ground down and leaves the
/// ridges where they are, which is what makes an ocean floor flat and a range sharp.
fn ridged(at: Vec3, seed: u64, frequency: f64) -> f64 {
    let folded = 1.0 - (2.0 * fractal(at, seed, frequency) - 1.0).abs();
    folded * folded
}

/// Value noise at a point in space, smoothly interpolated across the unit lattice.
fn value_noise(at: Vec3, seed: u64) -> f64 {
    let (x0, fx) = split(at.x);
    let (y0, fy) = split(at.y);
    let (z0, fz) = split(at.z);
    let (sx, sy, sz) = (smooth(fx), smooth(fy), smooth(fz));

    let mut total = 0.0;
    for corner in 0..8u32 {
        let (dx, dy, dz) = (corner & 1, (corner >> 1) & 1, (corner >> 2) & 1);
        let weight = blend(sx, dx) * blend(sy, dy) * blend(sz, dz);
        total += weight * lattice(x0 + dx as i64, y0 + dy as i64, z0 + dz as i64, seed);
    }
    total
}

/// The integer cell a coordinate falls in, and how far through it the coordinate is.
fn split(value: f64) -> (i64, f64) {
    let cell = value.floor();
    (cell as i64, value - cell)
}

/// Smoothstep. `3t^2 - 2t^3` has a zero first derivative at both ends, so neighbouring
/// cells meet without a crease - which is the whole reason the terrain reads as continuous.
fn smooth(t: f64) -> f64 {
    t * t * (3.0 - 2.0 * t)
}

fn blend(weight: f64, corner: u32) -> f64 {
    if corner == 0 { 1.0 - weight } else { weight }
}

/// A fixed pseudo-random value in `0.0..=1.0` for one lattice point.
///
/// Hashed rather than looked up, so the field is infinite and stateless: two calls for the
/// same point anywhere in the program agree, with nothing carried between them. The mixing
/// is splitmix64's, chosen because `sphere-tessellation`'s own generator already uses it
/// and it is written out rather than pulled in.
fn lattice(x: i64, y: i64, z: i64, seed: u64) -> f64 {
    let mut hash = seed;
    for coordinate in [x, y, z] {
        hash ^= (coordinate as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15);
        hash = mix(hash);
    }
    // 53 bits is every bit an f64 mantissa can hold, so this is uniform rather than
    // uniform-with-gaps.
    (hash >> 11) as f64 / ((1u64 << 53) as f64)
}

fn mix(mut hash: u64) -> u64 {
    hash = (hash ^ (hash >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    hash = (hash ^ (hash >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    hash ^ (hash >> 31)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sphere_tessellation::icosahedral::canonical_seeds;

    const SEED: u64 = 20260828;

    /// Points spread evenly over the sphere, for asking questions about the whole field.
    /// A deterministic spiral rather than a random sample, so a failure is reproducible.
    fn over_the_sphere(count: usize) -> Vec<Vec3> {
        let mut points = Vec::with_capacity(count);
        for at in 0..count {
            // Fibonacci-ish: z walks the axis evenly, the angle turns by a fixed step. The
            // trigonometry is fine here - this is a test fixture, not the field.
            let z = 1.0 - 2.0 * (at as f64 + 0.5) / count as f64;
            let radius = (1.0 - z * z).max(0.0).sqrt();
            let angle = at as f64 * 2.399_963_229_728_653;
            points.push(Vec3::new(radius * angle.cos(), radius * angle.sin(), z));
        }
        points
    }

    /// `spec/planet.md`: the terrain is continuous.
    ///
    /// Continuity is what makes it look like ground rather than like a mosaic, and it is
    /// the property a lattice with a smoothstep buys. Two points a thousandth of a radian
    /// apart must not differ much; the bound is loose because a ridge is legitimately
    /// steep, and it still catches a discontinuity, which would be order 1.
    #[test]
    fn the_field_is_continuous() {
        let mut worst: f64 = 0.0;
        for at in over_the_sphere(600) {
            let near = at.add(at.any_perpendicular().scaled(0.001)).normalized();
            let here = sample(at, SEED);
            let there = sample(near, SEED);
            worst = worst.max((here.elevation - there.elevation).abs());
            worst = worst.max((here.moisture - there.moisture).abs());
        }
        assert!(
            worst < 0.05,
            "the field jumps by {worst} over a thousandth of a radian"
        );
    }

    /// The same seed gives the same world, on any machine that agrees about arithmetic.
    ///
    /// This is what lets a history be replayed somewhere else: the biomes are not stored in
    /// the history, they are recomputed from `create planet <size>`.
    #[test]
    fn the_same_seed_gives_the_same_field() {
        for at in over_the_sphere(200) {
            assert_eq!(sample(at, SEED), sample(at, SEED));
            assert_eq!(biome_at(at, SEED), biome_at(at, SEED));
        }
    }

    /// A different seed gives a different world, or the seed is not doing anything.
    #[test]
    fn a_different_seed_gives_a_different_field() {
        let points = over_the_sphere(200);
        let differ = points
            .iter()
            .filter(|at| biome_at(**at, SEED) != biome_at(**at, SEED + 1))
            .count();
        assert!(
            differ > 40,
            "only {differ} of 200 points changed with the seed"
        );
    }

    /// `spec/planet.md`: the terrain varies within a single territory.
    ///
    /// Sampled at the seed, a territory gets one biome - but the ground it covers must not
    /// be uniform, or the realistic drawing would be a coloured-in tessellation, which is
    /// the practical drawing with extra steps.
    #[test]
    fn the_terrain_varies_inside_one_territory() {
        let seeds = canonical_seeds(12).unwrap();
        // An equatorial territory, not a polar one. A pole is entirely ice by design -
        // that is the ice cap the note argues for - so it is the one place on the planet
        // where uniform ground is the correct answer and proves nothing.
        let centre = *seeds
            .iter()
            .min_by(|a, b| a.z.abs().partial_cmp(&b.z.abs()).unwrap())
            .unwrap();
        let across = centre.any_perpendicular().normalized();
        // A twelve-territory planet has territories about 1.1 radians across, so a third
        // of a radian is comfortably inside one.
        let mut seen = std::collections::BTreeSet::new();
        for step in 0..40 {
            let angle = step as f64 * 0.008;
            let at = centre.add(across.scaled(angle)).normalized();
            seen.insert(biome_at(at, SEED).name());
        }
        assert!(
            seen.len() > 1,
            "one territory's ground is entirely {:?}",
            seen
        );
    }

    /// The poles are cold and the equator is not. This is the cue that a ball is a planet,
    /// and it is the one regularity the terrain is allowed to have.
    #[test]
    fn it_is_cold_at_the_poles_and_warm_at_the_equator() {
        let north = sample(Vec3::new(0.0, 0.0, 1.0), SEED);
        let south = sample(Vec3::new(0.0, 0.0, -1.0), SEED);
        assert!(north.temperature < FROZEN, "{}", north.temperature);
        assert!(south.temperature < FROZEN, "{}", south.temperature);

        // Around the equator, at sea level, it must be warm nearly everywhere - a mountain
        // is allowed to be cold, which is what the lapse rate is for.
        let warm = (0..36)
            .map(|step| {
                let angle = step as f64 * std::f64::consts::TAU / 36.0;
                sample(Vec3::new(angle.cos(), angle.sin(), 0.0), SEED)
            })
            .filter(|at| at.temperature > FROZEN)
            .count();
        assert!(
            warm >= 30,
            "only {warm} of 36 equatorial points are above tundra"
        );
    }

    /// Every biome the model can hold should actually occur, or the list is longer than the
    /// crossing of the fields justifies - which is exactly what the note warns against.
    #[test]
    fn the_biomes_that_exist_are_the_biomes_that_occur() {
        let mut seen = std::collections::BTreeSet::new();
        for at in over_the_sphere(4000) {
            seen.insert(biome_at(at, SEED));
        }
        let missing: Vec<&str> = Biome::ALL
            .into_iter()
            .filter(|biome| !seen.contains(biome))
            .map(Biome::name)
            .collect();
        assert!(missing.is_empty(), "no point on the planet is {missing:?}");
    }

    /// More of the sphere is water than land, which is what a planet looks like.
    #[test]
    fn most_of_the_planet_is_ocean() {
        let points = over_the_sphere(4000);
        let water = points
            .iter()
            .filter(|at| sample(**at, SEED).elevation < SEA_LEVEL)
            .count();
        let fraction = water as f64 / points.len() as f64;
        assert!(
            (0.45..0.85).contains(&fraction),
            "{:.0}% of the planet is water",
            fraction * 100.0
        );
    }

    /// A planet of this size: its solid, and its adjacency as indices.
    fn planet(count: usize) -> (Solid, Vec<Vec<usize>>) {
        let seeds = canonical_seeds(count).unwrap();
        let neighbours = sphere_tessellation::adjacency(&seeds);
        let near = neighbours
            .iter()
            .map(|list| list.iter().map(|at| *at as usize).collect())
            .collect();
        (sphere_tessellation::solid(&seeds, &neighbours), near)
    }

    /// One biome per territory, in id order, and every one of them a biome.
    #[test]
    fn every_territory_gets_a_biome() {
        for count in [12, 32, 42, 72, 92] {
            let (solid, near) = planet(count);
            assert_eq!(biomes_of(&solid, &near, SEED).len(), count);
        }
    }

    /// `spec/planet.md`: *oceans never isolate land from land. Every territory that is not
    /// ocean can be reached from every other without crossing one.*
    ///
    /// An island is what is forbidden, not a sea. Checked on every planet size and on
    /// several worlds, because it is a property of the whole arrangement rather than of one
    /// lucky seed.
    #[test]
    fn land_is_never_cut_in_two_by_water() {
        for count in [12, 32, 42, 72, 92] {
            let (solid, near) = planet(count);
            for world in 0..8u64 {
                let biomes = biomes_of(&solid, &near, SEED + world);
                let pieces = pieces_of_land(&biomes, &near);
                assert!(
                    pieces.len() <= 1,
                    "on a {count}-territory world {world}, land is in {} pieces: {pieces:?}",
                    pieces.len()
                );
            }
        }
    }

    /// The rule that replaced *no two oceans are adjacent* has to actually allow what that
    /// one forbade, or nothing was gained and the old rule is still in force by accident.
    ///
    /// Adjacent oceans are what make a sea rather than a scattering of lakes, and they are
    /// the geography the stricter rule cost.
    #[test]
    fn two_oceans_may_touch() {
        let mut touching = 0;
        for count in [42, 72, 92] {
            let (solid, near) = planet(count);
            for world in 0..8u64 {
                let biomes = biomes_of(&solid, &near, SEED + world);
                for (at, neighbours) in near.iter().enumerate() {
                    if biomes[at] != Biome::Ocean {
                        continue;
                    }
                    touching += neighbours
                        .iter()
                        .filter(|beside| biomes[**beside] == Biome::Ocean)
                        .count();
                }
            }
        }
        assert!(
            touching > 0,
            "no two oceans touch anywhere; water still cannot pool"
        );
    }

    /// Water still happens. A rule that quietly produced no ocean would satisfy every
    /// constraint above and describe a world with no sea in it.
    #[test]
    fn a_planet_still_has_some_ocean() {
        let (solid, near) = planet(92);
        let biomes = biomes_of(&solid, &near, SEED);
        let ocean = biomes.iter().filter(|b| **b == Biome::Ocean).count();
        assert!(ocean > 4, "only {ocean} of 92 territories are ocean");
    }

    /// `releases/first-release.md`: a territory's biome is the one where *no other biome
    /// covers more of that ground*.
    ///
    /// A plurality over an area, not the answer at one point - so it is checked by counting
    /// the ground, which is the same question the release asks of the picture.
    #[test]
    fn a_territory_takes_the_biome_that_covers_most_of_it() {
        let (solid, near) = planet(92);
        let biomes = biomes_of(&solid, &near, SEED);
        for (at, biome) in biomes.iter().enumerate() {
            // Ocean is the one that may be overruled, because land has to stay joined.
            if *biome == Biome::Ocean {
                continue;
            }
            let ground = ground_of(&solid, at, SEED);
            let mine = ground.iter().filter(|s| biome_of(s) == *biome).count();
            for other in Biome::ALL {
                if other == Biome::Ocean {
                    continue;
                }
                let theirs = ground.iter().filter(|s| biome_of(s) == other).count();
                assert!(
                    theirs <= mine,
                    "territory {at} is {} but {} covers more of it",
                    biome.name(),
                    other.name()
                );
            }
        }
    }

    /// A territory that had to stop being water becomes the land its own ground would have
    /// been, not a filler value - so what it becomes still comes from the terrain under it.
    #[test]
    fn a_territory_drained_to_join_the_land_becomes_its_own_ground() {
        let (solid, near) = planet(92);
        let biomes = biomes_of(&solid, &near, SEED);
        for (at, biome) in biomes.iter().enumerate() {
            let ground = ground_of(&solid, at, SEED);
            if covering(&ground) == Biome::Ocean && *biome != Biome::Ocean {
                assert_eq!(*biome, covering_land(&ground));
            }
        }
    }
}
