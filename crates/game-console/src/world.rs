//! The design phase, as rows.
//!
//! # Two paths, and this is the first
//!
//! **Sean, 2026-09-29**: *build rows before `start`, fire rules after.* A command before `{start}`
//! writes rows into a world being built; a command after it fires a rule against a world the
//! engine holds. **They were both `Transition`s and they are not the same operation** - one states
//! what is there and the other says what may be done about it.
//!
//! **`{start}` is where the two meet.** It hands what was built to `Game::of`, together with the
//! foundation the engine compiles in, and after that nothing writes a row except a rule.
//!
//! # Why `create-planet` survived the cut and the rest did not
//!
//! **`spec/console.md`, since `P-591`**: *a command that can be expressed as a row is not needed.*
//! `set biome` wrote one row and is gone - `create planet` already gives every territory the biome
//! the terrain field computes. **This writes a hundred and fourteen at `tiny-12` and eight hundred
//! and sixteen at `huge-92`**, and the adjacency among them comes out of `sphere_tessellation` -
//! not tedious to write by hand but not possible to write correctly.
//!
//! **`add <unit> orbit` writes one row and stays**, which is the same rule's second half: *it
//! goes when the notation can say what it said, not before*, and nothing else puts the release's
//! ark anywhere. **It was out of `spec/console.md` for a few hours** - `P-591` applied the first
//! half of the bullet and not the second - and `P-592` put it back. `C-180`.
//!
//! # The numbering is stated here because two modules rely on it
//!
//! **A territory's places are found by arithmetic rather than by searching**, so `binding` can
//! turn `territory:3` into a place without holding a world. That only works while the builder and
//! the reader agree, so they share [`surface_of`] and [`orbit_of`] rather than each knowing the
//! rule - and `the_places_of_a_territory_are_where_the_builder_put_them` is what says they do.

use game_model::notation::Row;
use planet_model::{Biome, PlanetSize};

/// The world seed, so the same size always gives the same planet.
///
/// **`spec/console.md` asks for a seed the player can give** - *the same seed and the same policy
/// give the same planet* - and `{generate-planet size policy seed}` is where that lands. It is
/// specified and not built, so this is the one seed there is until it is.
const WORLD_SEED: u64 = 1;

/// What a name is, in the form the engine reads.
///
/// **The foundation names nothing twice**, so a reference is an id: `{deposit what:31}` is a
/// deposit of food, and `{terrain is:2}` is ice. This resolves the handful of names these rows
/// carry - a relation, a supply and a biome - out of the foundation itself, so a relation
/// renumbered in `spec/data/schema.4x` does not need finding here.
///
/// **It generates rather than converts, which is why `friendly-notation` is not used.** `Names`
/// turns a friendly row into a foundation one and is right for a file somebody wrote; these rows
/// have no author to have written them in the other form.
fn id_of(sort: &str, name: &str) -> String {
    game_model::foundation::rows()
        .iter()
        .find(|it| it.relation == sort && it.value("name") == Some(name))
        .and_then(|it| it.value("id").map(str::to_string))
        .unwrap_or_else(|| panic!("`{sort}` declares no `{name}`, and this row needs its id"))
}

/// One row, from a relation and its cells.
fn row(relation: &str, cells: &[(&str, String)]) -> Row {
    Row {
        relation: relation.to_string(),
        values: cells
            .iter()
            .map(|(key, value)| (key.to_string(), value.clone()))
            .collect(),
    }
}

/// The surface of a territory.
///
/// **Places are numbered two to a territory, surface then orbit**, so a territory's places are an
/// arithmetic step from its id rather than something to look up. `spec/orbit.md` gives every
/// territory exactly one orbit, which is what makes a fixed stride right rather than convenient.
pub fn surface_of(territory: u32) -> u32 {
    territory * 2 - 1
}

/// The orbit above a territory.
pub fn orbit_of(territory: u32) -> u32 {
    territory * 2
}

/// How many territories a size has, and the adjacency among them.
fn shape(size: PlanetSize) -> (usize, Vec<Vec<usize>>, Vec<Biome>) {
    let seeds = sphere_tessellation::icosahedral::canonical_seeds(size.territory_count())
        .expect("every planet size is a Goldberg count; planet-render asserts it");
    let touching = sphere_tessellation::adjacency(&seeds);
    let near: Vec<Vec<usize>> = touching
        .iter()
        .map(|list| list.iter().map(|at| *at as usize).collect())
        .collect();
    // The terrain proposes and `biomes_of` disposes. Two of `spec/planet.md`'s statements are
    // about the arrangement rather than about any one point - a biome is what covers most of a
    // territory's ground, and oceans never isolate land from land - so the solid and the
    // adjacency are both in scope here, and neither is anything the field itself knows about.
    let solid = sphere_tessellation::solid(&seeds, &touching);
    let biomes = planet_terrain::biomes_of(&solid, &near, WORLD_SEED);
    (size.territory_count(), near, biomes)
}

/// Every row a planet of this size is made of.
///
/// **The planet, its territories, their places, the adjacency and the terrain.** Nothing else -
/// what a territory holds arrives later, by the commands that state it or by a rule that makes it.
pub fn planet(size: PlanetSize) -> Vec<Row> {
    let (count, near, biomes) = shape(size);
    let mut rows = Vec::new();

    // **One number for the whole world** - `spec/planet.md`, `P-588`: the planet carries the
    // star's density itself, and every orbit above it draws on that one row.
    rows.push(row(
        "planet",
        &[("id", "1".into()), ("energy", STAR_DENSITY.to_string())],
    ));

    for at in 0..count {
        let territory = at as u32 + 1;
        rows.push(row("territory", &[("id", territory.to_string())]));
        for (layer, id) in [
            ("surface", surface_of(territory)),
            ("orbit", orbit_of(territory)),
        ] {
            rows.push(row(
                "place",
                &[
                    ("id", id.to_string()),
                    ("of", territory.to_string()),
                    ("layer", layer.to_string()),
                ],
            ));
        }
        if let Some(biome) = biomes.get(at) {
            rows.push(row(
                "terrain",
                &[
                    ("of", territory.to_string()),
                    ("is", id_of("biome", biome.name())),
                ],
            ));
        }
    }

    // **Both directions, because an adjacency row says one.** `scenario/main.4x` states the pair,
    // and `move` requires a row from where it is to where it is going.
    let mut id = 0;
    for (at, near) in near.iter().enumerate() {
        for other in near {
            id += 1;
            rows.push(row(
                "adjacency",
                &[
                    ("id", id.to_string()),
                    ("from", (at as u32 + 1).to_string()),
                    ("to", (*other as u32 + 1).to_string()),
                ],
            ));
        }
    }

    rows.extend(limits());
    rows
}

/// What the star gives, per turn, to every orbit.
///
/// **Three, which is what `scenario/main.4x` states and what the test Sean read says** -
/// `an-ark-gathers-what-the-sun-gives` carries `{planet energy:3 id:1}`. It is a number the
/// specification does not fix, so it is here rather than derived, and a seed that chose it would
/// be `{generate-planet}`'s business.
const STAR_DENSITY: u32 = 3;

/// The capacities every world has, which nothing states per territory.
///
/// **`{capacity ...}` is keyed by what holds what and not by where**, so these are one row each
/// for the whole world. `scenario/main.4x` states the same five.
fn limits() -> Vec<Row> {
    vec![
        row(
            "capacity",
            &[
                ("of", id_of("relation", "deposit")),
                ("for", id_of("relation", "extractor")),
                ("what", id_of("relation", "resource")),
                ("per", id_of("relation", "place")),
                ("quantity", "1".into()),
            ],
        ),
        row(
            "capacity",
            &[
                ("of", id_of("relation", "place")),
                ("for", id_of("relation", "bin")),
                ("what", id_of("relation", "resource")),
                ("per", id_of("relation", "place")),
                ("quantity", "2".into()),
            ],
        ),
        row(
            "capacity",
            &[
                ("of", id_of("relation", "bin")),
                ("for", id_of("relation", "resource")),
                ("what", id_of("relation", "resource")),
                ("per", id_of("relation", "place")),
                ("quantity", "10".into()),
            ],
        ),
        row(
            "provides",
            &[
                ("kind", id_of("relation", "place")),
                ("what", id_of("supply", "berth")),
                ("quantity", "6".into()),
            ],
        ),
        row(
            "consumes",
            &[
                ("kind", id_of("relation", "pioneer")),
                ("what", id_of("supply", "berth")),
                ("quantity", "1".into()),
            ],
        ),
    ]
}

/// A territory's biome, which is a row about the ground rather than about the territory.
pub fn terrain(territory: u32, biome: Biome) -> Vec<Row> {
    vec![row(
        "terrain",
        &[
            ("of", territory.to_string()),
            ("is", id_of("biome", biome.name())),
        ],
    )]
}

/// A deposit on a territory's surface.
///
/// **The extractor count has nowhere to go and that is `C-176` rather than a gap here.**
/// `{capacity of:deposit for:extractor ...}` is one row for the whole world, so a count stated per
/// territory cannot be held - the command that states it is one the promotion removes, because a
/// deposit is a row a player could write and this argument is the part that never was.
pub fn deposit(territory: u32, resource: &str, density: u32) -> Vec<Row> {
    vec![row(
        "deposit",
        &[
            ("where", surface_of(territory).to_string()),
            // **Already an id, resolved where a word that names nothing is a misreading.**
            // `crate::binding` does it, so `{set-resource ... resource:gold}` is refused with
            // what could have been written - where this would have panicked, which is no way
            // to answer a player.
            ("what", resource.to_string()),
            ("density", density.to_string()),
            ("quantity", "1".into()),
        ],
    )]
}

/// A unit placed in the orbit above a territory, before play begins.
pub fn in_orbit(territory: u32, kind: &str) -> Vec<Row> {
    let mut cells = vec![
        ("where", orbit_of(territory).to_string()),
        ("moving", "1".to_string()),
        ("quantity", "1".to_string()),
    ];
    // **An ark carries a second trait and a pioneer does not**, which `{carries}` says and this
    // has to obey: an `add` names every column of the kind it makes.
    if kind == "ark" {
        cells.push(("gathering", "1".to_string()));
    }
    vec![row(kind, &cells)]
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};

    use super::*;

    /// **The builder and the reader agree about where a territory's places are.**
    ///
    /// **This is why the stride is one function rather than two that match.** `binding` turns
    /// `territory:3` into a place by arithmetic, so a world numbered any other way would send
    /// every command to the wrong place and nothing would say so - the rows would exist and the
    /// ids would resolve.
    #[test]
    fn the_places_of_a_territory_are_where_the_builder_put_them() {
        let rows = planet(PlanetSize::Tiny);
        let mut checked = 0;
        for territory in 1..=12u32 {
            for (layer, id) in [
                ("surface", surface_of(territory)),
                ("orbit", orbit_of(territory)),
            ] {
                let found = rows.iter().any(|it| {
                    it.relation == "place"
                        && it.value("id") == Some(id.to_string().as_str())
                        && it.value("of") == Some(territory.to_string().as_str())
                        && it.value("layer") == Some(layer)
                });
                assert!(found, "no {layer} numbered {id} for territory {territory}");
                checked += 1;
            }
        }
        assert_eq!(checked, 24, "twelve territories, two places each");
    }

    /// **A planet is the rows nobody could write**, and the count is what says so.
    ///
    /// **Twelve territories, twenty-four places, sixty directed adjacency rows** - every face of a
    /// dodecahedron has five neighbours - one planet, twelve terrains and five limits.
    #[test]
    fn a_tiny_planet_is_a_hundred_and_fourteen_rows() {
        let rows = planet(PlanetSize::Tiny);
        let how_many = |relation: &str| rows.iter().filter(|it| it.relation == relation).count();
        assert_eq!(how_many("planet"), 1);
        assert_eq!(how_many("territory"), 12);
        assert_eq!(how_many("place"), 24);
        assert_eq!(how_many("terrain"), 12);
        assert_eq!(how_many("adjacency"), 60, "twelve faces of five neighbours");
        assert_eq!(rows.len(), 114);
    }

    /// **Every id a planet writes is its own**, which the engine refuses if it is not.
    #[test]
    fn no_two_rows_of_one_relation_share_an_id() {
        for size in PlanetSize::ALL {
            let rows = planet(size);
            let mut seen: BTreeSet<(String, String)> = BTreeSet::new();
            for row in &rows {
                let Some(id) = row.value("id") else { continue };
                assert!(
                    seen.insert((row.relation.clone(), id.to_string())),
                    "{}: two `{}` rows have id {id}",
                    size.name(),
                    row.relation
                );
            }
        }
    }

    /// **Every planet the console can make is a world the engine can hold.**
    ///
    /// # The defect this was written for
    ///
    /// **A planet of twenty-six territories or more would not load**, and no world in the tree
    /// had ever had that many. `{place id:51 of:26 layer:surface}` is the surface of territory
    /// 26; relation 26 is `unit`, which is a family, and `schema::reified` expanded the row into
    /// one per member - so the world held four places numbered 51 and `Game::of` refused it,
    /// correctly, for a duplicate this made.
    ///
    /// **A territory id and a relation id are different id spaces** and a value alone cannot say
    /// which it is. The fix is in `schema.rs`: a value names a family only where the column
    /// points at that family, or at `relation`.
    ///
    /// **Every size rather than the one that failed.** `tiny-12` passed throughout, which is
    /// what a check on one example would have gone on saying.
    #[test]
    fn every_size_makes_a_world_the_engine_will_hold() {
        let mut checked = 0;
        for size in PlanetSize::ALL {
            let mut rows = game_model::foundation::rows();
            rows.extend(planet(size));
            game_model::engine::Game::of(rows)
                .unwrap_or_else(|why| panic!("{} does not hold: {why}", size.name()));
            checked += 1;
        }
        assert_eq!(checked, 5, "every planet size");
    }

    /// **The same size gives the same planet**, which is what a seed is for.
    ///
    /// **A history is a save file only while this holds** - the terrain is recomputed from
    /// `{create-planet size:...}` rather than recorded, so a replay that built a different world
    /// would replay to a different game.
    #[test]
    fn a_size_gives_the_same_planet_twice() {
        assert_eq!(planet(PlanetSize::Tiny), planet(PlanetSize::Tiny));
    }

    /// The adjacency, as territory ids, read back out of the rows.
    fn touching(rows: &[Row]) -> BTreeMap<u32, Vec<u32>> {
        let mut out: BTreeMap<u32, Vec<u32>> = BTreeMap::new();
        for it in rows.iter().filter(|it| it.relation == "adjacency") {
            let (Some(from), Some(to)) = (it.value("from"), it.value("to")) else {
                continue;
            };
            if let (Ok(from), Ok(to)) = (from.parse(), to.parse()) {
                out.entry(from).or_default().push(to);
            }
        }
        out
    }

    /// **A planet is not one biome painted over twelve faces.**
    ///
    /// `spec/planet.md` asks that nothing in the terrain reveal how the sphere was divided, and a
    /// world with a single biome would reveal nothing because it would say nothing.
    #[test]
    fn a_planet_has_more_than_one_kind_of_ground() {
        let rows = planet(PlanetSize::Huge);
        let kinds: BTreeSet<&str> = rows
            .iter()
            .filter(|it| it.relation == "terrain")
            .filter_map(|it| it.value("is"))
            .collect();
        assert!(kinds.len() > 2, "ninety-two territories and only {kinds:?}");
    }

    /// **Adjacency reads the same from both ends**, or moving somewhere would not let you move
    /// back.
    #[test]
    fn adjacency_agrees_with_itself() {
        let rows = planet(PlanetSize::Tiny);
        let near = touching(&rows);
        let mut checked = 0;
        for (here, there) in &near {
            for other in there {
                assert!(
                    near.get(other).is_some_and(|back| back.contains(here)),
                    "{here} lists {other} and {other} does not list {here}"
                );
                checked += 1;
            }
        }
        assert_eq!(checked, 60, "twelve faces of five neighbours, both ways");
    }

    /// **A dodecahedron: every face touches exactly five others.**
    #[test]
    fn every_territory_of_the_smallest_planet_touches_five() {
        let rows = planet(PlanetSize::Tiny);
        let near = touching(&rows);
        assert_eq!(near.len(), 12);
        for (here, there) in &near {
            assert_eq!(there.len(), 5, "territory {here} has {}", there.len());
        }
    }
}
