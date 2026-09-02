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

// ---------------------------------------------------------------------------------------
// Kinds
// ---------------------------------------------------------------------------------------

/// The ten kinds the release declares.
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
        }
    }

    /// How many of this kind a territory has room for, as the release writes it.
    pub fn room(self) -> &'static str {
        match self {
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
        }
    }
}

/// In the order the Kinds table lists them.
pub const KINDS: [Kind; 10] = [
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
];

/// In the order the room table lists them, which is not the same order.
pub const ROOM_ORDER: [Kind; 10] = [
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
}

impl Family {
    pub fn name(self) -> &'static str {
        match self {
            Family::Thing => "thing",
            Family::Unit => "unit",
            Family::Resource => "resource",
        }
    }

    pub fn members(self) -> Vec<Kind> {
        match self {
            Family::Thing => KINDS.to_vec(),
            Family::Unit => vec![Kind::Ark, Kind::Pioneer],
            Family::Resource => vec![Kind::Food, Kind::Metal, Kind::Energy],
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

pub const FAMILIES: [Family; 3] = [Family::Thing, Family::Unit, Family::Resource];

// ---------------------------------------------------------------------------------------
// Where things are
// ---------------------------------------------------------------------------------------

/// One sort of room a thing can be in.
///
/// **Every thing is in another thing**, and each sort of room has a capacity - which is why
/// *is there room* never needs to be an ingredient. `build extractor` used to take
/// `node, unworked` and no longer does, because the general rule already answers it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Room {
    pub what: &'static str,
    pub holds: &'static str,
    pub up_to: &'static str,
}

pub const ROOMS: [Room; 3] = [
    Room {
        what: "a territory's room for a kind",
        holds: "that kind",
        up_to: "what the territory has room for",
    },
    Room {
        what: "an extractor's catch",
        holds: "the resource it was built for",
        up_to: "the territory's density for it",
    },
    Room {
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

pub const TRAITS: [TraitRow; 17] = [
    TraitRow {
        name: "kind",
        of: "every thing",
        values: "one of the ten",
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
        name: "room",
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
        of: "a territory",
        values: "which territories touch it",
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

/// Whose recipe it is.
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Noun {
    Of(Kind),
    Any(Family),
    Territory,
}

impl Noun {
    pub fn name(self) -> &'static str {
        match self {
            Noun::Of(kind) => kind.name(),
            Noun::Any(family) => family.name(),
            Noun::Territory => "territory",
        }
    }
}

/// What distinguishes one of a noun from another, as the release writes it.
///
/// Most join with a comma - *ark, in orbit*. Two join as English, *thing with upkeep* and
/// *thing whose upkeep is unpaid*, which is punctuation rather than meaning and is recorded
/// rather than smoothed over.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Qualifier {
    pub written: &'static str,
    pub comma: bool,
    /// The declared trait this is a value of, and `None` when the release declares none.
    pub of_trait: Option<&'static str>,
}

const fn comma(written: &'static str, of_trait: Option<&'static str>) -> Qualifier {
    Qualifier {
        written,
        comma: true,
        of_trait,
    }
}

const fn phrase(written: &'static str, of_trait: Option<&'static str>) -> Qualifier {
    Qualifier {
        written,
        comma: false,
        of_trait,
    }
}

/// A thing a recipe names: what it is, what distinguishes it, and what it is called here.
///
/// **A name is how a recipe says where it acts.** `$where`, `$from` and `$to` are bound by
/// one ingredient and referred to by others, which is what lets `move` say *this territory*
/// and *that one* without the table having a column for either.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Subject {
    pub bound_as: Option<&'static str>,
    pub noun: Noun,
    pub qualifiers: &'static [Qualifier],
}

impl Subject {
    pub fn written(self) -> String {
        let mut out = String::new();
        if let Some(name) = self.bound_as {
            out.push_str(&format!("`${name}` "));
        }
        out.push_str(self.noun.name());
        for qualifier in self.qualifiers {
            if qualifier.comma {
                out.push_str(", ");
            } else {
                out.push(' ');
            }
            out.push_str(qualifier.written);
        }
        out
    }
}

const fn plain(noun: Noun) -> Subject {
    Subject {
        bound_as: None,
        noun,
        qualifiers: &[],
    }
}

const fn of(kind: Kind) -> Subject {
    plain(Noun::Of(kind))
}

const fn that_is(noun: Noun, qualifiers: &'static [Qualifier]) -> Subject {
    Subject {
        bound_as: None,
        noun,
        qualifiers,
    }
}

const fn named(name: &'static str, noun: Noun, qualifiers: &'static [Qualifier]) -> Subject {
    Subject {
        bound_as: Some(name),
        noun,
        qualifiers,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bound {
    AtLeast,
    AtMost,
}

impl Bound {
    pub fn written(self) -> &'static str {
        match self {
            Bound::AtLeast => "at least",
            Bound::AtMost => "at most",
        }
    }
}

/// One line of a recipe.
///
/// **There is no consumed column any more.** `releases/first-release.md`: *an ingredient is
/// consumed exactly when the same thing, with the same traits, does not appear among the
/// results.* So four recipes gained an echo row - `upkeep` takes a thing with upkeep and
/// gives it back - and being consumed became a fact you can work out rather than one stated
/// twice. [`Recipe::consumes`] works it out.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Port {
    In {
        subject: Subject,
        quantity: Quantity,
        bound: Bound,
    },
    Out {
        subject: Subject,
        quantity: Quantity,
    },
}

impl Port {
    pub fn subject(&self) -> Subject {
        match self {
            Port::In { subject, .. } | Port::Out { subject, .. } => *subject,
        }
    }

    pub fn quantity(&self) -> Quantity {
        match self {
            Port::In { quantity, .. } | Port::Out { quantity, .. } => *quantity,
        }
    }

    pub fn is_ingredient(&self) -> bool {
        matches!(self, Port::In { .. })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Recipe {
    pub name: &'static str,
    pub owner: Owner,
    pub ports: &'static [Port],
}

impl Recipe {
    /// Whether an ingredient is consumed, worked out rather than stated.
    ///
    /// The release's rule exactly: consumed when the same thing, with the same traits, does
    /// not appear among the results.
    pub fn consumes(&self, ingredient: &Port) -> bool {
        !self
            .ports
            .iter()
            .any(|port| !port.is_ingredient() && port.subject() == ingredient.subject())
    }
}

const fn takes(subject: Subject, quantity: Quantity, bound: Bound) -> Port {
    Port::In {
        subject,
        quantity,
        bound,
    }
}

const fn gives(subject: Subject, quantity: Quantity) -> Port {
    Port::Out { subject, quantity }
}

use Bound::{AtLeast, AtMost};
use Kind::*;
use Owner::{Player, World};
use Quantity::{Exactly, OfATrait};

const FOR_FOOD: [Qualifier; 1] = [comma("food", Some("resource"))];
const FOR_METAL: [Qualifier; 1] = [comma("metal", Some("resource"))];
const FOR_ENERGY: [Qualifier; 1] = [comma("energy", Some("resource"))];
const IN_WHERE: [Qualifier; 1] = [comma("in `$where`", Some("place"))];
const NEXT_TO_FROM: [Qualifier; 1] = [comma("next to `$from`", Some("adjacency"))];
const IN_FROM_READY: [Qualifier; 2] = [
    comma("in `$from`", Some("place")),
    comma("ready", Some("readiness")),
];
const IN_TO_EXHAUSTED: [Qualifier; 2] = [
    comma("in `$to`", Some("place")),
    comma("exhausted", Some("readiness")),
];
const IN_THAT_UNIT: [Qualifier; 1] = [comma("in that unit", Some("place"))];
const READY: [Qualifier; 1] = [comma("ready", Some("readiness"))];
const EXHAUSTED: [Qualifier; 1] = [comma("exhausted", Some("readiness"))];
const SURPLUS: [Qualifier; 1] = [comma("surplus", Some("surplus"))];
const HOUSES: [Qualifier; 1] = [comma("houses", Some("houses"))];
const WITH_UPKEEP: [Qualifier; 1] = [phrase("with upkeep", Some("upkeep"))];
const UPKEEP_UNPAID: [Qualifier; 1] = [phrase("whose upkeep is unpaid", Some("unpaid"))];

/// The sixteen recipes of `releases/first-release.md`.
pub const RECIPES: &[Recipe] = &[
    Recipe {
        name: "deploy ark",
        owner: Player,
        ports: &[
            takes(named("where", Noun::Territory, &[]), Exactly(1), AtLeast),
            gives(named("where", Noun::Territory, &[]), Exactly(1)),
            takes(that_is(Noun::Of(Ark), &IN_WHERE), Exactly(1), AtLeast),
            takes(of(Garrison), Exactly(0), AtMost),
            gives(of(Garrison), Exactly(1)),
            gives(of(Citizen), Exactly(1)),
            gives(that_is(Noun::Of(Extractor), &FOR_FOOD), Exactly(1)),
            gives(that_is(Noun::Of(Extractor), &FOR_METAL), Exactly(1)),
            gives(that_is(Noun::Of(Extractor), &FOR_ENERGY), Exactly(1)),
        ],
    },
    Recipe {
        name: "move",
        owner: Player,
        ports: &[
            takes(named("from", Noun::Territory, &[]), Exactly(1), AtLeast),
            gives(named("from", Noun::Territory, &[]), Exactly(1)),
            takes(
                named("to", Noun::Territory, &NEXT_TO_FROM),
                Exactly(1),
                AtLeast,
            ),
            gives(named("to", Noun::Territory, &[]), Exactly(1)),
            takes(
                that_is(Noun::Any(Family::Unit), &IN_FROM_READY),
                Exactly(1),
                AtLeast,
            ),
            gives(
                that_is(Noun::Any(Family::Unit), &IN_TO_EXHAUSTED),
                Exactly(1),
            ),
            takes(
                that_is(Noun::Of(Energy), &IN_THAT_UNIT),
                Exactly(1),
                AtLeast,
            ),
        ],
    },
    Recipe {
        name: "found by land",
        owner: Player,
        ports: &[
            takes(of(Pioneer), Exactly(1), AtLeast),
            takes(of(Garrison), Exactly(0), AtMost),
            gives(of(Garrison), Exactly(1)),
            gives(of(Citizen), Exactly(1)),
            gives(that_is(Noun::Of(Extractor), &FOR_FOOD), Exactly(1)),
        ],
    },
    Recipe {
        name: "build food extractor",
        owner: Player,
        ports: &[
            takes(of(Labor), Exactly(1), AtLeast),
            takes(of(Metal), Exactly(1), AtLeast),
            gives(that_is(Noun::Of(Extractor), &FOR_FOOD), Exactly(1)),
        ],
    },
    Recipe {
        name: "build metal extractor",
        owner: Player,
        ports: &[
            takes(of(Labor), Exactly(1), AtLeast),
            takes(of(Metal), Exactly(1), AtLeast),
            gives(that_is(Noun::Of(Extractor), &FOR_METAL), Exactly(1)),
        ],
    },
    Recipe {
        name: "build energy extractor",
        owner: Player,
        ports: &[
            takes(of(Labor), Exactly(1), AtLeast),
            takes(of(Metal), Exactly(1), AtLeast),
            gives(that_is(Noun::Of(Extractor), &FOR_ENERGY), Exactly(1)),
        ],
    },
    Recipe {
        name: "build yard",
        owner: Player,
        ports: &[
            takes(of(Labor), Exactly(1), AtLeast),
            takes(of(Metal), Exactly(15), AtLeast),
            gives(of(Yard), Exactly(1)),
        ],
    },
    Recipe {
        name: "produce pioneer",
        owner: Player,
        ports: &[
            takes(of(Metal), Exactly(2), AtLeast),
            takes(of(Energy), Exactly(6), AtLeast),
            takes(of(Citizen), Exactly(1), AtLeast),
            takes(of(Garrison), Exactly(1), AtLeast),
            gives(of(Pioneer), Exactly(1)),
            gives(of(Garrison), Exactly(1)),
        ],
    },
    Recipe {
        name: "produce ark",
        owner: Player,
        ports: &[
            takes(of(Metal), Exactly(4), AtLeast),
            takes(of(Energy), Exactly(12), AtLeast),
            takes(of(Yard), Exactly(1), AtLeast),
            gives(of(Ark), Exactly(1)),
            gives(of(Yard), Exactly(1)),
        ],
    },
    Recipe {
        name: "spend readiness",
        owner: Player,
        ports: &[
            takes(that_is(Noun::Of(Citizen), &READY), Exactly(1), AtLeast),
            gives(that_is(Noun::Of(Citizen), &EXHAUSTED), Exactly(1)),
            gives(of(Labor), Exactly(1)),
        ],
    },
    Recipe {
        name: "work",
        owner: Player,
        ports: &[
            takes(named("where", Noun::Territory, &[]), Exactly(1), AtLeast),
            gives(named("where", Noun::Territory, &[]), Exactly(1)),
            takes(of(Labor), Exactly(1), AtLeast),
            takes(that_is(Noun::Of(Extractor), &READY), Exactly(1), AtLeast),
            gives(that_is(Noun::Of(Extractor), &EXHAUSTED), Exactly(1)),
            gives(
                plain(Noun::Any(Family::Resource)),
                OfATrait("`$where`'s density for that resource"),
            ),
        ],
    },
    Recipe {
        name: "grow",
        owner: World,
        ports: &[
            takes(that_is(Noun::Of(Food), &SURPLUS), Exactly(1), AtLeast),
            takes(
                that_is(Noun::Any(Family::Thing), &HOUSES),
                Exactly(1),
                AtLeast,
            ),
            gives(of(Citizen), Exactly(1)),
            gives(that_is(Noun::Any(Family::Thing), &HOUSES), Exactly(1)),
        ],
    },
    Recipe {
        name: "spoil",
        owner: World,
        ports: &[takes(
            that_is(Noun::Of(Food), &SURPLUS),
            Exactly(1),
            AtLeast,
        )],
    },
    Recipe {
        name: "ready",
        owner: World,
        ports: &[
            takes(
                that_is(Noun::Any(Family::Thing), &EXHAUSTED),
                Exactly(1),
                AtLeast,
            ),
            gives(that_is(Noun::Any(Family::Thing), &READY), Exactly(1)),
        ],
    },
    Recipe {
        name: "upkeep",
        owner: World,
        ports: &[
            takes(
                that_is(Noun::Any(Family::Thing), &WITH_UPKEEP),
                Exactly(1),
                AtLeast,
            ),
            takes(of(Food), OfATrait("the thing's upkeep"), AtLeast),
            gives(that_is(Noun::Any(Family::Thing), &WITH_UPKEEP), Exactly(1)),
        ],
    },
    Recipe {
        name: "perish",
        owner: World,
        ports: &[
            takes(
                that_is(Noun::Any(Family::Thing), &UPKEEP_UNPAID),
                Exactly(1),
                AtLeast,
            ),
            gives(of(Metal), OfATrait("the thing's metal")),
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

    pub fn cost_written(&self) -> String {
        self.costs
            .iter()
            .map(|(count, kind)| format!("{count} {}", kind.name()))
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
        requires: None,
        readies: false,
    },
    Producible {
        kind: Ark,
        force: Some(2),
        fuel: Some(2),
        a_move: Some(1),
        upkeep: None,
        costs: &[(4, Metal), (12, Energy)],
        binding: Some(4),
        requires: Some("a Yard"),
        readies: true,
    },
    Producible {
        kind: Pioneer,
        force: Some(2),
        fuel: Some(2),
        a_move: Some(1),
        upkeep: Some((1, Food)),
        costs: &[(2, Metal), (6, Energy), (1, Citizen)],
        binding: Some(2),
        requires: Some("a garrison"),
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

pub fn rooms_table() -> Vec<Vec<String>> {
    let mut rows = vec![header(&["Room", "Holds", "Up to"])];
    for room in ROOMS {
        rows.push(vec![
            room.what.to_string(),
            room.holds.to_string(),
            room.up_to.to_string(),
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

pub fn room_table() -> Vec<Vec<String>> {
    let mut rows = vec![header(&["Kind", "Room"])];
    for kind in ROOM_ORDER {
        rows.push(vec![
            format!("**{}**", kind.name()),
            kind.room().to_string(),
        ]);
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
        "Metal in it",
        "Binding",
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
            thing
                .metal_in_it()
                .map(|n| n.to_string())
                .unwrap_or_default(),
            thing.binding.map(|n| n.to_string()).unwrap_or_default(),
            thing.requires.unwrap_or_default().to_string(),
            if thing.readies { "yes" } else { "" }.to_string(),
        ]);
    }
    rows
}

pub fn recipes_table() -> Vec<Vec<String>> {
    let mut rows = vec![header(&[
        "Recipe", "Owner", "Role", "Thing", "Qty", "Bound",
    ])];
    for recipe in RECIPES {
        for (at, port) in recipe.ports.iter().enumerate() {
            let first = at == 0;
            let (role, bound) = match port {
                Port::In { bound, .. } => ("in", bound.written().to_string()),
                Port::Out { .. } => ("out", String::new()),
            };
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
                role.to_string(),
                port.subject().written(),
                port.quantity().written(),
                bound,
            ]);
        }
    }
    rows
}
