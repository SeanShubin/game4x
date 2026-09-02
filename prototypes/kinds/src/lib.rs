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
//! # It stopped guessing
//!
//! The first version of this crate inferred the kinds from the recipes, because nothing
//! declared them - and said so: the table quantifies over kinds, and a reader supplies the
//! generality without noticing. The release declares them now, and this crate holds all six
//! of its tables and is checked against every one.
//!
//! Two of the kinds it had inferred were not kinds at all. `node` became a trait of a
//! territory and `cell` became fuel in a unit's tank, which is what a declaration is for.
//!
//! # Checked against the release, not merely derived from it
//!
//! `tests/against_the_release.rs` renders this data back into all six tables and compares
//! them with `releases/first-release.md` on disk. Two copies of eighteen recipes would be
//! one copy and one guess; this way neither can move without the other.

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

    /// What the release says it is.
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
}

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

    /// The members column, as the release writes it.
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

/// A place a thing can be.
///
/// **Every thing is in a bin**, and a bin has a capacity - which is why *is there room*
/// never needs to be an ingredient. `build extractor` used to take `node, unworked` and no
/// longer does, because the general rule already answers it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bin {
    pub what: &'static str,
    pub holds: &'static str,
    pub up_to: &'static str,
}

pub const BINS: [Bin; 3] = [
    Bin {
        what: "a territory's room for a kind",
        holds: "that kind",
        up_to: "what the territory has room for",
    },
    Bin {
        what: "an extractor's catch",
        holds: "the resource it was built for",
        up_to: "the territory's density for it",
    },
    Bin {
        what: "a unit's tank",
        holds: "energy",
        up_to: "the unit's fuel",
    },
];

// ---------------------------------------------------------------------------------------
// Traits
// ---------------------------------------------------------------------------------------

/// Whether a trait is held or worked out, and whether it survives the turn.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Held {
    Stored,
    /// Worked out from other things, with the release's own account of how.
    Derived(&'static str),
    /// Stored, and wiped when the turn ends.
    ClearedAtEndTurn,
}

impl Held {
    pub fn written(self) -> String {
        match self {
            Held::Stored => "stored".to_string(),
            Held::ClearedAtEndTurn => "stored, cleared at end turn".to_string(),
            Held::Derived(how) => format!("derived: {how}"),
        }
    }
}

/// One of the thirteen traits the release declares.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TraitRow {
    pub name: &'static str,
    pub of: &'static str,
    pub values: &'static str,
    pub held: Held,
}

pub const TRAITS: [TraitRow; 13] = [
    TraitRow {
        name: "kind",
        of: "every thing",
        values: "one of the ten",
        held: Held::Stored,
    },
    TraitRow {
        name: "place",
        of: "every thing",
        values: "the bin it is in",
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
        of: "a unit",
        values: "food per turn",
        held: Held::Stored,
    },
    TraitRow {
        name: "metal in it",
        of: "whatever is built",
        values: "a number",
        held: Held::Stored,
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
        name: "arriving",
        of: "a pioneer",
        values: "yes or no",
        held: Held::ClearedAtEndTurn,
    },
    TraitRow {
        name: "surplus",
        of: "food",
        values: "yes or no",
        held: Held::Derived("left after everything ate"),
    },
    TraitRow {
        name: "unfed",
        of: "a citizen",
        values: "yes or no",
        held: Held::Derived("it did not eat"),
    },
];

// ---------------------------------------------------------------------------------------
// Recipes
// ---------------------------------------------------------------------------------------

/// What a recipe's Thing column names.
///
/// **Three sorts of noun, and only two of them are things.** A kind or a family names
/// something that is in a bin; `revert` names a *territory*, which is where things are
/// rather than a thing that is anywhere. The release's own *Where things are* says every
/// thing is in a bin, and a territory is a bin.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Noun {
    Of(Kind),
    Any(Family),
    /// The place itself. `revert` is the only recipe that names one.
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
/// Most join with a comma - *ark, in orbit*. Two join as English, *unit with upkeep* and
/// *unit whose upkeep is unpaid*, which is punctuation rather than meaning and is recorded
/// rather than smoothed over.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Qualifier {
    pub written: &'static str,
    pub comma: bool,
}

impl Qualifier {
    pub const fn after_comma(written: &'static str) -> Self {
        Self {
            written,
            comma: true,
        }
    }

    pub const fn as_phrase(written: &'static str) -> Self {
        Self {
            written,
            comma: false,
        }
    }
}

/// A noun, and what distinguishes it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Subject {
    pub noun: Noun,
    pub qualified_by: Option<Qualifier>,
}

impl Subject {
    pub const fn plain(noun: Noun) -> Self {
        Self {
            noun,
            qualified_by: None,
        }
    }

    pub const fn that_is(noun: Noun, qualified_by: Qualifier) -> Self {
        Self {
            noun,
            qualified_by: Some(qualified_by),
        }
    }

    pub fn written(self) -> String {
        match self.qualified_by {
            Some(qualifier) if qualifier.comma => {
                format!("{}, {}", self.noun.name(), qualifier.written)
            }
            Some(qualifier) => format!("{} {}", self.noun.name(), qualifier.written),
            None => self.noun.name().to_string(),
        }
    }
}

/// How many.
///
/// `releases/first-release.md`: *a quantity is a whole number. It is written in the recipe,
/// or read from a trait of one of the ingredients.*
///
/// **Two of the three that are read are ingredients and one is not**, which is why the
/// variants are cut this way rather than by what is read. `upkeep` reads the unit's upkeep
/// and `perish` reads the unit's metal, and in both the unit is an ingredient of the recipe.
/// `work` reads the *territory's* density, and the territory is not an ingredient of `work` -
/// its ingredients are labor and an extractor. It is the only one of eighteen that reads
/// past its own ingredients, and it does so because density moved from a thing to a place.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Quantity {
    Exactly(u32),
    /// Read from a trait of one of the recipe's own ingredients.
    OfAnIngredient(&'static str),
    /// Read from a trait of the place the recipe runs in.
    OfThePlace(&'static str),
}

impl Quantity {
    pub fn written(self) -> String {
        match self {
            Quantity::Exactly(count) => count.to_string(),
            Quantity::OfAnIngredient(how) | Quantity::OfThePlace(how) => how.to_string(),
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

/// One line of a recipe: something going in, or something coming out.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Port {
    In {
        subject: Subject,
        quantity: Quantity,
        consumed: bool,
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

/// Where a recipe applies.
///
/// A field rather than two types: ten are `Here` and eight are `Every`, and nothing else
/// about them differs, so two types would duplicate the whole shape to carry one bit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scope {
    Here,
    Every,
}

impl Scope {
    pub fn written(self) -> &'static str {
        match self {
            Scope::Here => "here",
            Scope::Every => "every",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Recipe {
    pub name: &'static str,
    pub scope: Scope,
    pub ports: &'static [Port],
}

const fn takes(subject: Subject, quantity: Quantity, consumed: bool, bound: Bound) -> Port {
    Port::In {
        subject,
        quantity,
        consumed,
        bound,
    }
}

const fn yields(subject: Subject, quantity: Quantity) -> Port {
    Port::Out { subject, quantity }
}

const fn of(kind: Kind) -> Subject {
    Subject::plain(Noun::Of(kind))
}

const fn any(family: Family) -> Subject {
    Subject::plain(Noun::Any(family))
}

use Bound::{AtLeast, AtMost};
use Kind::*;
use Quantity::{Exactly, OfAnIngredient, OfThePlace};
use Scope::{Every, Here};

const IN_ORBIT: Qualifier = Qualifier::after_comma("in orbit");
const HERE_AT: Qualifier = Qualifier::after_comma("here");
const THERE: Qualifier = Qualifier::after_comma("there");
const ARRIVING: Qualifier = Qualifier::after_comma("arriving");
const IN_THAT_UNIT: Qualifier = Qualifier::after_comma("in that unit");
const FOR_FOOD: Qualifier = Qualifier::after_comma("food");
const READY: Qualifier = Qualifier::after_comma("ready");
const EXHAUSTED: Qualifier = Qualifier::after_comma("exhausted");
const SURPLUS: Qualifier = Qualifier::after_comma("surplus");
const UNFED: Qualifier = Qualifier::after_comma("unfed");
const WITH_UPKEEP: Qualifier = Qualifier::as_phrase("with upkeep");
const UPKEEP_UNPAID: Qualifier = Qualifier::as_phrase("whose upkeep is unpaid");
const BELOW_NATURE: Qualifier = Qualifier::after_comma("force below its force of nature");
const UNCLAIMED: Qualifier = Qualifier::after_comma("unclaimed");

/// The eighteen recipes of `releases/first-release.md`.
pub const RECIPES: &[Recipe] = &[
    Recipe {
        name: "land",
        scope: Here,
        ports: &[
            takes(
                Subject::that_is(Noun::Of(Ark), IN_ORBIT),
                Exactly(1),
                true,
                AtLeast,
            ),
            takes(of(Garrison), Exactly(0), false, AtMost),
            yields(of(Garrison), Exactly(1)),
            yields(of(Citizen), Exactly(1)),
            yields(Subject::that_is(Noun::Of(Extractor), FOR_FOOD), Exactly(1)),
        ],
    },
    Recipe {
        name: "move",
        scope: Here,
        ports: &[
            takes(
                Subject::that_is(Noun::Any(Family::Unit), HERE_AT),
                Exactly(1),
                true,
                AtLeast,
            ),
            takes(
                Subject::that_is(Noun::Of(Energy), IN_THAT_UNIT),
                Exactly(1),
                true,
                AtLeast,
            ),
            yields(Subject::that_is(Noun::Any(Family::Unit), THERE), Exactly(1)),
        ],
    },
    Recipe {
        name: "found by land",
        scope: Here,
        ports: &[
            takes(
                Subject::that_is(Noun::Of(Pioneer), ARRIVING),
                Exactly(1),
                true,
                AtLeast,
            ),
            takes(of(Garrison), Exactly(0), false, AtMost),
            yields(of(Garrison), Exactly(1)),
            yields(of(Citizen), Exactly(1)),
            yields(Subject::that_is(Noun::Of(Extractor), FOR_FOOD), Exactly(1)),
        ],
    },
    Recipe {
        name: "build extractor",
        scope: Here,
        ports: &[
            takes(of(Labor), Exactly(1), true, AtLeast),
            yields(of(Extractor), Exactly(1)),
        ],
    },
    Recipe {
        name: "build yard",
        scope: Here,
        ports: &[
            takes(of(Metal), Exactly(15), true, AtLeast),
            yields(of(Yard), Exactly(1)),
        ],
    },
    Recipe {
        name: "produce pioneer",
        scope: Here,
        ports: &[
            takes(of(Metal), Exactly(8), true, AtLeast),
            takes(of(Energy), Exactly(6), true, AtLeast),
            takes(of(Citizen), Exactly(1), true, AtLeast),
            takes(of(Garrison), Exactly(1), false, AtLeast),
            yields(of(Pioneer), Exactly(1)),
        ],
    },
    Recipe {
        name: "produce ark",
        scope: Here,
        ports: &[
            takes(of(Metal), Exactly(12), true, AtLeast),
            takes(of(Energy), Exactly(12), true, AtLeast),
            takes(of(Yard), Exactly(1), false, AtLeast),
            yields(of(Ark), Exactly(1)),
        ],
    },
    Recipe {
        name: "launch",
        scope: Here,
        ports: &[
            takes(
                Subject::that_is(Noun::Of(Ark), HERE_AT),
                Exactly(1),
                true,
                AtLeast,
            ),
            takes(
                Subject::that_is(Noun::Of(Energy), IN_THAT_UNIT),
                Exactly(1),
                true,
                AtLeast,
            ),
            yields(Subject::that_is(Noun::Of(Ark), IN_ORBIT), Exactly(1)),
        ],
    },
    Recipe {
        name: "spend readiness",
        scope: Here,
        ports: &[
            takes(
                Subject::that_is(Noun::Of(Citizen), READY),
                Exactly(1),
                true,
                AtLeast,
            ),
            yields(Subject::that_is(Noun::Of(Citizen), EXHAUSTED), Exactly(1)),
            yields(of(Labor), Exactly(1)),
        ],
    },
    Recipe {
        name: "work",
        scope: Here,
        ports: &[
            takes(of(Labor), Exactly(1), true, AtLeast),
            takes(of(Extractor), Exactly(1), false, AtLeast),
            yields(
                any(Family::Resource),
                OfThePlace("the territory's density for that resource"),
            ),
        ],
    },
    Recipe {
        name: "eat",
        scope: Every,
        ports: &[
            takes(of(Citizen), Exactly(1), false, AtLeast),
            takes(of(Food), Exactly(1), true, AtLeast),
        ],
    },
    Recipe {
        name: "grow",
        scope: Every,
        ports: &[
            takes(
                Subject::that_is(Noun::Of(Food), SURPLUS),
                Exactly(1),
                true,
                AtLeast,
            ),
            yields(of(Citizen), Exactly(1)),
        ],
    },
    Recipe {
        name: "depart",
        scope: Every,
        ports: &[takes(
            Subject::that_is(Noun::Of(Citizen), UNFED),
            Exactly(1),
            true,
            AtLeast,
        )],
    },
    Recipe {
        name: "spoil",
        scope: Every,
        ports: &[takes(of(Food), Exactly(1), true, AtLeast)],
    },
    Recipe {
        name: "ready",
        scope: Every,
        ports: &[
            takes(
                Subject::that_is(Noun::Any(Family::Thing), EXHAUSTED),
                Exactly(1),
                true,
                AtLeast,
            ),
            yields(
                Subject::that_is(Noun::Any(Family::Thing), READY),
                Exactly(1),
            ),
        ],
    },
    Recipe {
        name: "upkeep",
        scope: Every,
        ports: &[
            takes(
                Subject::that_is(Noun::Any(Family::Unit), WITH_UPKEEP),
                Exactly(1),
                false,
                AtLeast,
            ),
            takes(of(Food), OfAnIngredient("the unit's upkeep"), true, AtLeast),
        ],
    },
    Recipe {
        name: "perish",
        scope: Every,
        ports: &[
            takes(
                Subject::that_is(Noun::Any(Family::Unit), UPKEEP_UNPAID),
                Exactly(1),
                true,
                AtLeast,
            ),
            yields(of(Metal), OfAnIngredient("the unit's metal")),
        ],
    },
    Recipe {
        name: "revert",
        scope: Every,
        ports: &[
            takes(
                Subject::that_is(Noun::Territory, BELOW_NATURE),
                Exactly(1),
                false,
                AtLeast,
            ),
            yields(Subject::that_is(Noun::Territory, UNCLAIMED), Exactly(1)),
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
    /// What one move costs, in fuel.
    pub a_move: Option<u32>,
    pub upkeep: Option<(u32, Kind)>,
    pub costs: &'static [(u32, Kind)],
    /// What the table says that the figures do not.
    pub aside: Option<&'static str>,
    /// What `perish` gives back. Metal is conserved, so this is what went in.
    pub metal_in_it: Option<u32>,
    pub requires: Option<&'static str>,
    pub readies: bool,
}

pub const PRODUCIBLE: &[Producible] = &[
    Producible {
        kind: Citizen,
        force: Some(1),
        fuel: None,
        a_move: None,
        upkeep: None,
        costs: &[],
        aside: None,
        metal_in_it: None,
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
        aside: None,
        metal_in_it: Some(1),
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
        aside: None,
        metal_in_it: Some(1),
        requires: None,
        readies: true,
    },
    Producible {
        kind: Yard,
        force: None,
        fuel: None,
        a_move: None,
        upkeep: None,
        costs: &[(15, Metal)],
        aside: None,
        metal_in_it: Some(15),
        requires: None,
        readies: false,
    },
    Producible {
        kind: Ark,
        force: Some(2),
        fuel: Some(2),
        a_move: Some(1),
        upkeep: None,
        costs: &[(12, Metal), (12, Energy)],
        aside: None,
        metal_in_it: Some(12),
        requires: Some("a Yard"),
        readies: true,
    },
    Producible {
        kind: Pioneer,
        force: Some(2),
        fuel: Some(2),
        a_move: Some(1),
        upkeep: Some((1, Food)),
        costs: &[(8, Metal), (6, Energy), (1, Citizen)],
        aside: None,
        metal_in_it: Some(8),
        requires: Some("a garrison"),
        readies: true,
    },
];

impl Producible {
    pub fn cost_written(&self) -> String {
        let figures: Vec<String> = self
            .costs
            .iter()
            .map(|(count, kind)| format!("{count} {}", kind.name()))
            .collect();
        match (figures.is_empty(), self.aside) {
            (true, None) => String::new(),
            (true, Some(aside)) => aside.to_string(),
            (false, None) => figures.join(", "),
            (false, Some(aside)) => format!("{}, {aside}", figures.join(", ")),
        }
    }

    pub fn upkeep_written(&self) -> String {
        match self.upkeep {
            Some((count, kind)) => format!("{count} {} per turn", kind.name()),
            None => String::new(),
        }
    }
}

// ---------------------------------------------------------------------------------------
// Rendering: the six tables, as rows of cells
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

pub fn bins_table() -> Vec<Vec<String>> {
    let mut rows = vec![header(&["Bin", "Holds", "Up to"])];
    for bin in BINS {
        rows.push(vec![
            bin.what.to_string(),
            bin.holds.to_string(),
            bin.up_to.to_string(),
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

pub fn units_table() -> Vec<Vec<String>> {
    let mut rows = vec![header(&[
        "Thing",
        "Force",
        "Fuel",
        "A move",
        "Upkeep",
        "Costs to produce",
        "Metal in it",
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
            thing.metal_in_it.map(|n| n.to_string()).unwrap_or_default(),
            thing.requires.unwrap_or_default().to_string(),
            if thing.readies { "yes" } else { "" }.to_string(),
        ]);
    }
    rows
}

pub fn recipes_table() -> Vec<Vec<String>> {
    let mut rows = vec![header(&[
        "Recipe", "Scope", "Role", "Thing", "Qty", "Consumed", "Bound",
    ])];
    for recipe in RECIPES {
        for (at, port) in recipe.ports.iter().enumerate() {
            let first = at == 0;
            let (role, consumed, bound) = match port {
                Port::In {
                    consumed, bound, ..
                } => (
                    "in",
                    if *consumed { "yes" } else { "no" }.to_string(),
                    bound.written().to_string(),
                ),
                Port::Out { .. } => ("out", String::new(), String::new()),
            };
            rows.push(vec![
                if first {
                    format!("**{}**", recipe.name)
                } else {
                    String::new()
                },
                if first {
                    recipe.scope.written().to_string()
                } else {
                    String::new()
                },
                role.to_string(),
                port.subject().written(),
                port.quantity().written(),
                consumed,
                bound,
            ]);
        }
    }
    rows
}
