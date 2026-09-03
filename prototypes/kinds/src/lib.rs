//! The kinds and recipes of the first release, as Rust data.
//!
//! **The question.** What do the inputs to the gameplay logic actually look like? The
//! release says it in tables, which is the right form for deciding it and the wrong one for
//! finding out whether it holds together. This is the same content in a form that compiles,
//! so the shape can be read before it is built into the model.
//!
//! It does not play. No turn, no board, no rule, no state. Only what a thing is and what
//! turns into what.
//!
//! # Written here, checked against there
//!
//! `tests/against_the_release.rs` renders this data back into the release's tables and
//! compares them with `releases/first-release.md` on disk, cell by cell.
//!
//! **That comparison is the only test here that is worth anything, and it took a red gate to
//! see it.** Seven other tests once passed against data that had stopped matching hours
//! earlier - one asserted eighteen recipes while the release had sixteen, another that
//! `revert` names a place while `revert` no longer existed. They read this crate and checked
//! it against numbers written in the same crate, so they were self-consistent and empty. A
//! test that reads one artifact can only tell you it has not changed.

pub mod catalog;
pub mod release;

// ---------------------------------------------------------------------------------------
// Kinds
// ---------------------------------------------------------------------------------------

/// The twelve kinds the release declares.
///
/// **Ten until `P-192`.** The recipes' `Kind` column had held `territory` in four rows all
/// along, and the Kinds table did not list it - so the release named a kind it had not
/// declared, and this crate carried a `Noun::Territory` beside `Noun::Of(Kind)` to render
/// it. That escape hatch is what let the two halves disagree in a crate built to stop them,
/// and it is gone: a territory is a kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Citizen,
    Garrison,
    Extractor,
    Yard,
    Ark,
    Pioneer,
    Food,
    Metal,
    Energy,
    Labor,
    Territory,
    Orbit,
}

impl Kind {
    pub fn name(self) -> &'static str {
        match self {
            Kind::Citizen => "citizen",
            Kind::Garrison => "garrison",
            Kind::Extractor => "extractor",
            Kind::Yard => "yard",
            Kind::Ark => "ark",
            Kind::Pioneer => "pioneer",
            Kind::Food => "food",
            Kind::Metal => "metal",
            Kind::Energy => "energy",
            Kind::Labor => "labor",
            Kind::Territory => "territory",
            Kind::Orbit => "orbit",
        }
    }

    pub fn what_it_is(self) -> &'static str {
        match self {
            Kind::Citizen => "a person: provides labor, eats, and grows on surplus",
            Kind::Garrison => "what holds a territory; a territory has at most one",
            Kind::Extractor => "built for one resource, and worked to produce it",
            Kind::Yard => "where an Ark is produced",
            Kind::Ark => "carries a landing, and can invade from orbit",
            Kind::Pioneer => "founds a territory",
            Kind::Food => "eaten by citizens; expires",
            Kind::Metal => "what things are built from; conserved",
            Kind::Energy => "what moves things; neither conserved nor expiring",
            Kind::Labor => "what working a machine takes; a citizen provides it each turn",
            Kind::Territory => concat!(
                "a place things are in, which has a biome, a force of nature, ",
                "and a density and a total capacity per resource"
            ),
            Kind::Orbit => "a place above one territory, which holds units and nothing else",
        }
    }

    /// How many of this kind a territory has total capacity for, as the release writes it.
    ///
    /// `None` for the two kinds that are places rather than contents. A territory has no
    /// capacity for territories, and the table says so by not having a row - which is a
    /// fact worth a type rather than a blank string that would render an empty row.
    pub fn total_capacity(self) -> Option<&'static str> {
        Some(match self {
            Kind::Citizen => "8",
            Kind::Garrison => "1",
            Kind::Extractor => "what the *Territory resources* table gives, per resource",
            Kind::Yard => "1",
            Kind::Ark => "2",
            Kind::Pioneer => "2",
            Kind::Labor => "8",
            Kind::Food => "20",
            Kind::Metal => "20",
            Kind::Energy => "20",
            Kind::Territory | Kind::Orbit => return None,
        })
    }
}

/// In the order the Kinds table lists them.
pub const KINDS: [Kind; 12] = [
    Kind::Citizen,
    Kind::Garrison,
    Kind::Extractor,
    Kind::Yard,
    Kind::Ark,
    Kind::Pioneer,
    Kind::Food,
    Kind::Metal,
    Kind::Energy,
    Kind::Labor,
    Kind::Territory,
    Kind::Orbit,
];

/// In the order the total-capacity table lists them, which is not the same order.
///
/// **Ten, not twelve.** A territory has no capacity for territories or for orbits, and the
/// release says so by not giving them a row. [`Kind::total_capacity`] returns `None` for
/// both, so adding one here fails loudly rather than rendering a blank.
pub const CAPACITY_ORDER: [Kind; 10] = [
    Kind::Citizen,
    Kind::Garrison,
    Kind::Extractor,
    Kind::Yard,
    Kind::Ark,
    Kind::Pioneer,
    Kind::Labor,
    Kind::Food,
    Kind::Metal,
    Kind::Energy,
];

// ---------------------------------------------------------------------------------------
// Families
// ---------------------------------------------------------------------------------------

/// A name for several kinds at once.
///
/// **Not a parent class.** Membership is a list, so `unit` is something an Ark and a Pioneer
/// both carry rather than something they inherit from. `spec/invariants.md` has every kind
/// of thing be data, and a hierarchy would be the one shape that is not.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Family {
    Thing,
    Unit,
    Resource,
    Place,
}

impl Family {
    pub fn name(self) -> &'static str {
        match self {
            Family::Thing => "thing",
            Family::Unit => "unit",
            Family::Resource => "resource",
            Family::Place => "place",
        }
    }

    pub fn members(self) -> Vec<Kind> {
        match self {
            Family::Thing => KINDS.to_vec(),
            Family::Unit => vec![Kind::Ark, Kind::Pioneer],
            Family::Resource => vec![Kind::Food, Kind::Metal, Kind::Energy],
            Family::Place => vec![Kind::Territory, Kind::Orbit],
        }
    }

    pub fn members_written(self) -> String {
        match self {
            Family::Thing => "every kind above".to_string(),
            other => other
                .members()
                .iter()
                .map(|kind| kind.name())
                .collect::<Vec<_>>()
                .join(", "),
        }
    }

    pub fn covers(self, kind: Kind) -> bool {
        self.members().contains(&kind)
    }
}

pub const FAMILIES: [Family; 4] = [Family::Thing, Family::Unit, Family::Resource, Family::Place];

// ---------------------------------------------------------------------------------------
// Where things are
// ---------------------------------------------------------------------------------------

/// One sort of capacity a thing can be held in.
///
/// **Every thing is in another thing**, and each sort of capacity has a limit - which is why
/// *is there room* never needs to be an ingredient. `build extractor` used to take
/// `node, unworked` and no longer does, because the general rule already answers it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Capacity {
    pub what: &'static str,
    pub holds: &'static str,
    pub up_to: &'static str,
}

pub const CAPACITIES: [Capacity; 3] = [
    Capacity {
        what: "a territory's total capacity for a kind",
        holds: "that kind",
        up_to: "its total capacity for that kind",
    },
    Capacity {
        what: "an extractor's catch",
        holds: "the resource it was built for",
        up_to: "the territory's density for it",
    },
    Capacity {
        what: "a unit's tank",
        holds: "energy",
        up_to: "the unit's fuel",
    },
];

// ---------------------------------------------------------------------------------------
// Traits
// ---------------------------------------------------------------------------------------

/// Whether a trait is held or worked out.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Held {
    Stored,
    /// Worked out from other things, with the release's own account of how.
    Derived(&'static str),
}

impl Held {
    pub fn written(self) -> String {
        match self {
            Held::Stored => "stored".to_string(),
            Held::Derived(how) => format!("derived: {how}"),
        }
    }
}

/// One of the traits the release declares.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TraitRow {
    pub name: &'static str,
    pub of: &'static str,
    pub values: &'static str,
    pub held: Held,
}

pub const TRAITS: [TraitRow; 18] = [
    TraitRow {
        name: "kind",
        of: "every thing",
        values: "one of the twelve",
        held: Held::Stored,
    },
    TraitRow {
        name: "place",
        of: "every thing",
        values: "the thing it is in",
        held: Held::Stored,
    },
    TraitRow {
        name: "readiness",
        of: "whatever readies",
        values: "ready, exhausted",
        held: Held::Stored,
    },
    TraitRow {
        name: "force",
        of: "citizen, garrison, ark, pioneer",
        values: "a number",
        held: Held::Stored,
    },
    TraitRow {
        name: "fuel",
        of: "a unit",
        values: "how much energy its tank holds",
        held: Held::Stored,
    },
    TraitRow {
        name: "upkeep",
        of: "a thing with upkeep",
        values: "food per turn",
        held: Held::Stored,
    },
    TraitRow {
        name: "metal in it",
        of: "whatever is built",
        values: "a number",
        held: Held::Derived("its binding plus the metal in its parts"),
    },
    TraitRow {
        name: "resource",
        of: "an extractor",
        values: "food, metal or energy",
        held: Held::Stored,
    },
    TraitRow {
        name: "density",
        of: "a territory, per resource",
        values: "a number",
        held: Held::Stored,
    },
    TraitRow {
        name: "total capacity",
        of: "a territory, per kind",
        values: "a number",
        held: Held::Stored,
    },
    TraitRow {
        name: "control",
        of: "a territory",
        values: "held by a player, or unclaimed",
        held: Held::Derived("a citizen of that player is there"),
    },
    TraitRow {
        name: "biome",
        of: "a territory",
        values: "one of the six",
        held: Held::Stored,
    },
    TraitRow {
        name: "force of nature",
        of: "a territory",
        values: "a number",
        held: Held::Stored,
    },
    TraitRow {
        name: "adjacency",
        of: "a place",
        values: "which places it touches, and by which kind of edge",
        held: Held::Stored,
    },
    TraitRow {
        name: "keeps",
        of: "food",
        values: "the number of turns it will last",
        held: Held::Stored,
    },
    TraitRow {
        name: "surplus",
        of: "food",
        values: "yes or no",
        held: Held::Derived("left after every upkeep was paid"),
    },
    TraitRow {
        name: "unpaid",
        of: "a thing with upkeep",
        values: "yes or no",
        held: Held::Derived("its upkeep was not met"),
    },
    TraitRow {
        name: "houses",
        of: "a thing that contains things",
        values: "whether people live in it",
        held: Held::Stored,
    },
];

// ---------------------------------------------------------------------------------------
// Recipes
// ---------------------------------------------------------------------------------------

/// Whose recipe it is, which the release's **Auto** column names.
///
/// **Not where it applies.** It was `here`/`every` and said who a recipe belonged to while
/// reading as though it said where - a player asks for the first sort and the world runs the
/// second whether anyone asks or not.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Owner {
    Player,
    World,
}

impl Owner {
    pub fn written(self) -> &'static str {
        match self {
            Owner::Player => "player",
            Owner::World => "world",
        }
    }
}

/// What one line of a recipe does with the thing it names.
///
/// **The release states this now, and used to leave it to be worked out.** The rule was *an
/// ingredient is consumed exactly when the same thing, with the same traits, does not appear
/// among the results*, which meant four recipes carried an echo row saying nothing except
/// that something survived. A column says it once, and `limit` - *at most none of these* -
/// stopped having to be spelled as a quantity of zero that was also given back.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    /// Has to be there, and still is afterwards.
    Require,
    /// At most this many, which is how a recipe says *unheld ground*.
    Limit,
    /// Has to be there, and is gone afterwards.
    Consume,
    /// What the recipe makes.
    Produce,
}

impl Role {
    pub fn written(self) -> &'static str {
        match self {
            Role::Require => "require",
            Role::Limit => "limit",
            Role::Consume => "consume",
            Role::Produce => "produce",
        }
    }

    /// Whether the line is something the recipe needs rather than something it makes.
    pub fn is_ingredient(self) -> bool {
        !matches!(self, Role::Produce)
    }

    /// Whether the thing named is gone afterwards.
    pub fn consumes(self) -> bool {
        matches!(self, Role::Consume)
    }
}

/// What a recipe line names: one kind, or a family standing for several.
///
/// **There is no third case any more.** `Noun::Territory` sat here because the release used
/// `territory` in its recipes and declared it nowhere, so the crate needed a way to write a
/// name that was not a kind. That is exactly the disagreement this crate exists to prevent,
/// and it survived by being expressible. Now that a territory is a kind, the type says what
/// the release says: a name is a declared kind or a declared family, and there is no way to
/// write one that is neither.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Noun {
    Of(Kind),
    Any(Family),
}

impl Noun {
    /// Whether this names somewhere rather than something in it.
    ///
    /// **A name is bound by requiring a place**, and until `P-196` the only place a recipe
    /// could require was a territory - so this read `== Noun::Of(Kind::Territory)` and
    /// stopped finding `move`'s two names the moment `move` began taking places. The
    /// question was always *is this somewhere*, and it was written as the one answer that
    /// existed at the time.
    pub fn is_a_place(self) -> bool {
        match self {
            Noun::Of(kind) => Family::Place.covers(kind),
            Noun::Any(family) => family == Family::Place,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Noun::Of(kind) => kind.name(),
            Noun::Any(family) => family.name(),
        }
    }
}

/// What distinguishes one of a noun from another, as the release writes it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Qualifier {
    pub written: &'static str,
    /// The declared trait this is a value of, and `None` when the release declares none.
    pub of_trait: Option<&'static str>,
}

const fn by(written: &'static str, of_trait: &'static str) -> Qualifier {
    Qualifier {
        written,
        of_trait: Some(of_trait),
    }
}

/// How many.
///
/// `releases/first-release.md`: *a quantity is a whole number. It is written in the recipe,
/// read from a trait of one of the ingredients, or read from a trait of a named ingredient.*
///
/// The third way exists because of this crate. The sentence said two until `P-151`, and
/// `work` yields the territory's density while a territory is not among its ingredients -
/// so the sentence was false of a row three lines below it. `work` names the territory
/// `$where` now, which is how a recipe reaches past what it consumes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Quantity {
    Exactly(u32),
    /// Read from a trait of something the recipe names.
    OfATrait(&'static str),
}

impl Quantity {
    pub fn written(self) -> String {
        match self {
            Quantity::Exactly(count) => count.to_string(),
            Quantity::OfATrait(how) => how.to_string(),
        }
    }
}

/// One line of a recipe: a role, a quantity, a kind, what distinguishes it, and where.
///
/// **Where is a column now, and used to be a qualifier.** `in $where` read as a trait of the
/// thing and was one - `place` - but every recipe that acts somewhere puts every line there,
/// so it was written once per line to say something true of the whole recipe.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Line {
    pub role: Role,
    pub quantity: Quantity,
    pub noun: Noun,
    pub traits: &'static [Qualifier],
    /// The **Where** column: a name the recipe binds or refers to, or somewhere it points.
    pub place: Option<&'static str>,
}

impl Line {
    pub fn traits_written(&self) -> String {
        self.traits
            .iter()
            .map(|qualifier| qualifier.written)
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub fn is_ingredient(&self) -> bool {
        self.role.is_ingredient()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Recipe {
    pub name: &'static str,
    pub owner: Owner,
    pub lines: &'static [Line],
}

impl Recipe {
    /// The names this recipe binds, in the order it binds them.
    ///
    /// A name is bound by requiring a territory somewhere, and referred to by every other
    /// line that mentions it. That is how `move` says *this territory* and *that one*
    /// without the table having a column for either.
    pub fn binds(&self) -> Vec<&'static str> {
        self.lines
            .iter()
            .filter(|line| line.role == Role::Require && line.noun.is_a_place())
            .filter_map(|line| line.place)
            .collect()
    }

    /// Everything this recipe writes in a column a `$name` could appear in.
    pub fn mentions(&self) -> Vec<String> {
        let mut out = Vec::new();
        for line in self.lines {
            out.push(line.quantity.written());
            out.push(line.traits_written());
            out.push(line.place.unwrap_or_default().to_string());
        }
        out
    }
}

const fn just(role: Role, count: u32, noun: Noun) -> Line {
    Line {
        role,
        quantity: Quantity::Exactly(count),
        noun,
        traits: &[],
        place: None,
    }
}

const fn traited(role: Role, count: u32, noun: Noun, traits: &'static [Qualifier]) -> Line {
    Line {
        role,
        quantity: Quantity::Exactly(count),
        noun,
        traits,
        place: None,
    }
}

const fn placed(
    role: Role,
    count: u32,
    noun: Noun,
    traits: &'static [Qualifier],
    place: &'static str,
) -> Line {
    Line {
        role,
        quantity: Quantity::Exactly(count),
        noun,
        traits,
        place: Some(place),
    }
}

const fn measured(role: Role, quantity: Quantity, noun: Noun) -> Line {
    Line {
        role,
        quantity,
        noun,
        traits: &[],
        place: None,
    }
}

use Kind::*;
use Owner::{Player, World};
use Quantity::OfATrait;
use Role::{Consume, Limit, Produce, Require};

const FOR_FOOD: [Qualifier; 1] = [by("food", "resource")];
const FOR_METAL: [Qualifier; 1] = [by("metal", "resource")];
const FOR_ENERGY: [Qualifier; 1] = [by("energy", "resource")];
const JOINED_TO_FROM: [Qualifier; 1] = [by(
    "joined to `$from` by an edge the unit crosses",
    "adjacency",
)];
const READY: [Qualifier; 1] = [by("ready", "readiness")];
const EXHAUSTED: [Qualifier; 1] = [by("exhausted", "readiness")];
const SURPLUS: [Qualifier; 1] = [by("surplus", "surplus")];
const HOUSES: [Qualifier; 1] = [by("houses", "houses")];
const WITH_UPKEEP: [Qualifier; 1] = [by("with upkeep", "upkeep")];
const UPKEEP_UNPAID: [Qualifier; 1] = [by("whose upkeep is unpaid", "unpaid")];
const KEEPS_NONE: [Qualifier; 1] = [by("keeps 0", "keeps")];
const KEEPS_SOME: [Qualifier; 1] = [by("keeps at least 1", "keeps")];
const KEEPS_LESS: [Qualifier; 1] = [by("keeps one less", "keeps")];

const TERRITORY: Noun = Noun::Of(Kind::Territory);
const UNIT: Noun = Noun::Any(Family::Unit);
const RESOURCE: Noun = Noun::Any(Family::Resource);
const THING: Noun = Noun::Any(Family::Thing);
const PLACE: Noun = Noun::Any(Family::Place);

/// The seventeen recipes of `releases/first-release.md`.
pub const RECIPES: &[Recipe] = &[
    Recipe {
        name: "deploy ark",
        owner: Player,
        lines: &[
            placed(Require, 1, TERRITORY, &[], "`$where`"),
            placed(Consume, 1, Noun::Of(Ark), &[], "the orbit above `$where`"),
            just(Limit, 0, Noun::Of(Garrison)),
            just(Produce, 1, Noun::Of(Garrison)),
            just(Produce, 2, Noun::Of(Citizen)),
            traited(Produce, 1, Noun::Of(Extractor), &FOR_FOOD),
            traited(Produce, 1, Noun::Of(Extractor), &FOR_METAL),
        ],
    },
    Recipe {
        name: "move",
        owner: Player,
        lines: &[
            placed(Require, 1, PLACE, &[], "`$from`"),
            placed(Require, 1, PLACE, &JOINED_TO_FROM, "`$to`"),
            placed(Consume, 1, UNIT, &READY, "`$from`"),
            placed(Consume, 1, Noun::Of(Energy), &[], "that unit"),
            placed(Produce, 1, UNIT, &EXHAUSTED, "`$to`"),
        ],
    },
    Recipe {
        name: "found by land",
        owner: Player,
        lines: &[
            just(Consume, 1, Noun::Of(Pioneer)),
            just(Limit, 0, Noun::Of(Garrison)),
            just(Produce, 1, Noun::Of(Garrison)),
            just(Produce, 2, Noun::Of(Citizen)),
            traited(Produce, 1, Noun::Of(Extractor), &FOR_FOOD),
            traited(Produce, 1, Noun::Of(Extractor), &FOR_METAL),
        ],
    },
    Recipe {
        name: "build food extractor",
        owner: Player,
        lines: &[
            just(Consume, 1, Noun::Of(Labor)),
            just(Consume, 1, Noun::Of(Metal)),
            traited(Produce, 1, Noun::Of(Extractor), &FOR_FOOD),
        ],
    },
    Recipe {
        name: "build metal extractor",
        owner: Player,
        lines: &[
            just(Consume, 1, Noun::Of(Labor)),
            just(Consume, 1, Noun::Of(Metal)),
            traited(Produce, 1, Noun::Of(Extractor), &FOR_METAL),
        ],
    },
    Recipe {
        name: "build energy extractor",
        owner: Player,
        lines: &[
            just(Consume, 1, Noun::Of(Labor)),
            just(Consume, 1, Noun::Of(Metal)),
            traited(Produce, 1, Noun::Of(Extractor), &FOR_ENERGY),
        ],
    },
    Recipe {
        name: "build yard",
        owner: Player,
        lines: &[
            just(Consume, 1, Noun::Of(Labor)),
            just(Consume, 15, Noun::Of(Metal)),
            just(Produce, 1, Noun::Of(Yard)),
        ],
    },
    Recipe {
        name: "produce pioneer",
        owner: Player,
        lines: &[
            just(Consume, 3, Noun::Of(Metal)),
            just(Consume, 6, Noun::Of(Energy)),
            just(Consume, 2, Noun::Of(Citizen)),
            just(Produce, 1, Noun::Of(Pioneer)),
        ],
    },
    Recipe {
        name: "produce ark",
        owner: Player,
        lines: &[
            just(Consume, 3, Noun::Of(Metal)),
            just(Consume, 12, Noun::Of(Energy)),
            just(Consume, 2, Noun::Of(Citizen)),
            just(Require, 1, Noun::Of(Yard)),
            just(Produce, 1, Noun::Of(Ark)),
        ],
    },
    Recipe {
        name: "create labor",
        owner: Player,
        lines: &[
            traited(Consume, 1, Noun::Of(Citizen), &READY),
            traited(Produce, 1, Noun::Of(Citizen), &EXHAUSTED),
            just(Produce, 1, Noun::Of(Labor)),
        ],
    },
    Recipe {
        name: "work",
        owner: Player,
        lines: &[
            placed(Require, 1, TERRITORY, &[], "`$where`"),
            just(Consume, 1, Noun::Of(Labor)),
            traited(Consume, 1, Noun::Of(Extractor), &READY),
            traited(Produce, 1, Noun::Of(Extractor), &EXHAUSTED),
            measured(
                Produce,
                OfATrait("`$where`'s density for that resource"),
                RESOURCE,
            ),
        ],
    },
    Recipe {
        name: "upkeep",
        owner: World,
        lines: &[
            traited(Require, 1, THING, &WITH_UPKEEP),
            measured(Consume, OfATrait("the thing's upkeep"), Noun::Of(Food)),
        ],
    },
    Recipe {
        name: "grow",
        owner: World,
        lines: &[
            traited(Consume, 1, Noun::Of(Food), &SURPLUS),
            traited(Require, 1, THING, &HOUSES),
            just(Produce, 1, Noun::Of(Citizen)),
        ],
    },
    Recipe {
        name: "perish",
        owner: World,
        lines: &[
            traited(Consume, 1, THING, &UPKEEP_UNPAID),
            measured(Produce, OfATrait("the thing's metal"), Noun::Of(Metal)),
        ],
    },
    Recipe {
        name: "spoil",
        owner: World,
        lines: &[traited(Consume, 1, Noun::Of(Food), &KEEPS_NONE)],
    },
    Recipe {
        name: "age",
        owner: World,
        lines: &[
            traited(Consume, 1, Noun::Of(Food), &KEEPS_SOME),
            traited(Produce, 1, Noun::Of(Food), &KEEPS_LESS),
        ],
    },
    Recipe {
        name: "ready",
        owner: World,
        lines: &[
            traited(Consume, 1, THING, &EXHAUSTED),
            traited(Produce, 1, THING, &READY),
        ],
    },
];

// ---------------------------------------------------------------------------------------
// Units and structures
// ---------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub struct Producible {
    pub kind: Kind,
    pub force: Option<u32>,
    pub fuel: Option<u32>,
    pub a_move: Option<u32>,
    pub upkeep: Option<(u32, Kind)>,
    pub costs: &'static [(u32, Kind)],
    /// What holds it together, and what `perish` gives back before its parts are counted.
    pub binding: Option<u32>,
    /// Which kinds of edge it may move along, which is what decides where it can ever be.
    ///
    /// **An Ark's life has no land-to-land move in it**: it crosses `orbit border` and
    /// `ascent` and not `border`, so it is produced on the ground, ascends once, moves in
    /// orbit to choose a site, and deploys. Sean's rule that an Ark cannot move between two
    /// territories is now a consequence of this column rather than something merely obeyed.
    pub crosses: Option<&'static str>,
    pub requires: Option<&'static str>,
    pub readies: bool,
}

impl Producible {
    /// The **Metal in it** column, which the Traits table calls derived: *its binding plus
    /// the metal in its parts*.
    ///
    /// Derived here rather than stored, so that the two cannot disagree - which is what the
    /// trait table says it is.
    pub fn metal_in_it(&self) -> Option<u32> {
        let binding = self.binding?;
        let parts: u32 = self
            .costs
            .iter()
            .filter(|(_, kind)| *kind == Kind::Citizen)
            .map(|_| 0)
            .sum();
        Some(binding + parts)
    }

    /// The **Costs to produce** column.
    ///
    /// *Two citizens*, and *15 metal*: a resource is a mass noun and labor is one too, so
    /// only the things you can count take a plural. The release writes it that way and this
    /// is compared with the release, so the rule lives here rather than being smoothed over.
    pub fn cost_written(&self) -> String {
        self.costs
            .iter()
            .map(|(count, kind)| {
                let mass = Family::Resource.covers(*kind) || *kind == Kind::Labor;
                let plural = if *count == 1 || mass { "" } else { "s" };
                format!("{count} {}{plural}", kind.name())
            })
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub fn upkeep_written(&self) -> String {
        match self.upkeep {
            Some((count, kind)) => format!("{count} {} per turn", kind.name()),
            None => String::new(),
        }
    }
}

pub const PRODUCIBLE: &[Producible] = &[
    Producible {
        kind: Citizen,
        force: Some(1),
        fuel: None,
        a_move: None,
        upkeep: Some((1, Food)),
        costs: &[],
        binding: None,
        crosses: None,
        requires: None,
        readies: true,
    },
    Producible {
        kind: Garrison,
        force: Some(1),
        fuel: None,
        a_move: None,
        upkeep: None,
        costs: &[(1, Labor), (1, Metal)],
        binding: Some(1),
        crosses: None,
        requires: None,
        readies: false,
    },
    Producible {
        kind: Extractor,
        force: None,
        fuel: None,
        a_move: None,
        upkeep: None,
        costs: &[(1, Labor), (1, Metal)],
        binding: Some(1),
        crosses: None,
        requires: None,
        readies: true,
    },
    Producible {
        kind: Yard,
        force: None,
        fuel: None,
        a_move: None,
        upkeep: None,
        costs: &[(1, Labor), (15, Metal)],
        binding: Some(15),
        crosses: None,
        requires: None,
        readies: false,
    },
    Producible {
        kind: Ark,
        force: Some(2),
        fuel: Some(2),
        a_move: Some(1),
        upkeep: None,
        costs: &[(3, Metal), (12, Energy), (2, Citizen)],
        binding: Some(3),
        crosses: Some("orbit border, ascent"),
        requires: Some("a Yard"),
        readies: true,
    },
    Producible {
        kind: Pioneer,
        force: Some(2),
        fuel: Some(2),
        a_move: Some(1),
        upkeep: Some((1, Food)),
        costs: &[(3, Metal), (6, Energy), (2, Citizen)],
        binding: Some(3),
        crosses: Some("border"),
        requires: None,
        readies: true,
    },
];

// ---------------------------------------------------------------------------------------
// Rendering
// ---------------------------------------------------------------------------------------

fn header(cells: &[&str]) -> Vec<String> {
    cells.iter().map(|cell| cell.to_string()).collect()
}

pub fn kinds_table() -> Vec<Vec<String>> {
    let mut rows = vec![header(&["Kind", "What it is"])];
    for kind in KINDS {
        rows.push(vec![
            format!("**{}**", kind.name()),
            kind.what_it_is().to_string(),
        ]);
    }
    rows
}

pub fn families_table() -> Vec<Vec<String>> {
    let mut rows = vec![header(&["Family", "Members"])];
    for family in FAMILIES {
        rows.push(vec![
            format!("**{}**", family.name()),
            family.members_written(),
        ]);
    }
    rows
}

pub fn capacities_table() -> Vec<Vec<String>> {
    let mut rows = vec![header(&["Container", "Holds", "Up to"])];
    for capacity in CAPACITIES {
        rows.push(vec![
            capacity.what.to_string(),
            capacity.holds.to_string(),
            capacity.up_to.to_string(),
        ]);
    }
    rows
}

pub fn traits_table() -> Vec<Vec<String>> {
    let mut rows = vec![header(&["Trait", "Of", "Values", "Stored or derived"])];
    for row in TRAITS {
        rows.push(vec![
            format!("**{}**", row.name),
            row.of.to_string(),
            row.values.to_string(),
            row.held.written(),
        ]);
    }
    rows
}

pub fn capacity_table() -> Vec<Vec<String>> {
    let mut rows = vec![header(&["Kind", "Total capacity"])];
    for kind in CAPACITY_ORDER {
        let capacity = kind
            .total_capacity()
            .unwrap_or_else(|| panic!("{} is a place, and has no capacity row", kind.name()));
        rows.push(vec![format!("**{}**", kind.name()), capacity.to_string()]);
    }
    rows
}

pub fn units_table() -> Vec<Vec<String>> {
    let mut rows = vec![header(&[
        "Thing",
        "Force",
        "Fuel",
        "A move",
        "Upkeep",
        "Costs to produce",
        "Binding",
        "Crosses",
        "Requires",
        "Readies",
    ])];
    for thing in PRODUCIBLE {
        rows.push(vec![
            format!("**{}**", thing.kind.name()),
            thing.force.map(|n| n.to_string()).unwrap_or_default(),
            thing.fuel.map(|n| n.to_string()).unwrap_or_default(),
            thing
                .a_move
                .map(|n| format!("{n} fuel"))
                .unwrap_or_default(),
            thing.upkeep_written(),
            thing.cost_written(),
            thing.binding.map(|n| n.to_string()).unwrap_or_default(),
            thing.crosses.unwrap_or_default().to_string(),
            thing.requires.unwrap_or_default().to_string(),
            if thing.readies { "yes" } else { "" }.to_string(),
        ]);
    }
    rows
}

pub fn recipes_table() -> Vec<Vec<String>> {
    let mut rows = vec![header(&[
        "Recipe", "Owner", "Role", "Qty", "Kind", "Traits", "Where",
    ])];
    for recipe in RECIPES {
        for (at, line) in recipe.lines.iter().enumerate() {
            let first = at == 0;
            rows.push(vec![
                if first {
                    format!("**{}**", recipe.name)
                } else {
                    String::new()
                },
                if first {
                    recipe.owner.written().to_string()
                } else {
                    String::new()
                },
                line.role.written().to_string(),
                line.quantity.written(),
                line.noun.name().to_string(),
                line.traits_written(),
                line.place.unwrap_or_default().to_string(),
            ]);
        }
    }
    rows
}
