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
//! **Sean, 2026-09-29**: *we don't need a command that can be expressed as a row.* `set-biome`
//! writes one row and `add-ark-orbit` writes one, so a player could have typed the row. **This
//! writes ninety-six at `tiny-12` and eight hundred and sixteen at `huge-92`**, and the adjacency
//! among them comes out of `sphere_tessellation` - not tedious to write by hand but not possible
//! to write correctly. `C-176` is the promotion that removes the others.
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
                    ("is", biome.name().to_string()),
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
                ("of", "deposit".into()),
                ("for", "extractor".into()),
                ("what", "resource".into()),
                ("per", "place".into()),
                ("quantity", "1".into()),
            ],
        ),
        row(
            "capacity",
            &[
                ("of", "place".into()),
                ("for", "bin".into()),
                ("what", "resource".into()),
                ("per", "place".into()),
                ("quantity", "2".into()),
            ],
        ),
        row(
            "capacity",
            &[
                ("of", "bin".into()),
                ("for", "resource".into()),
                ("what", "resource".into()),
                ("per", "place".into()),
                ("quantity", "10".into()),
            ],
        ),
        row(
            "provides",
            &[
                ("kind", "place".into()),
                ("what", "berth".into()),
                ("quantity", "6".into()),
            ],
        ),
        row(
            "consumes",
            &[
                ("kind", "pioneer".into()),
                ("what", "berth".into()),
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
            ("is", biome.name().to_string()),
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

    /// **The same size gives the same planet**, which is what a seed is for.
    #[test]
    fn a_size_gives_the_same_planet_twice() {
        assert_eq!(planet(PlanetSize::Tiny), planet(PlanetSize::Tiny));
    }
}
