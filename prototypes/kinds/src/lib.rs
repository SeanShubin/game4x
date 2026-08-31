//! The kinds and transformations of the first release, as Rust data.
//!
//! **The question.** What do the inputs to the gameplay logic actually look like? The
//! release says it in two markdown tables, which is the right form for deciding it and the
//! wrong one for finding out whether it holds together. This is the same content in a form
//! that compiles, so that the shape can be read before it is built into the model.
//!
//! It does not play. There is no turn, no board, no rule and no state - only what a thing
//! is, and what turns into what.
//!
//! # What is here
//!
//! - [`Kind`] - every noun the release names, including the three that stand for a family
//!   rather than a thing: `unit`, `thing` and `resource`.
//! - [`Trait`] - what distinguishes two of the same kind. *In orbit*, *unworked*, *ready*.
//! - [`Transformation`] - fifteen of them, each a scope and a list of [`Port`]s.
//! - [`PRODUCIBLE`] - force, cells, upkeep and cost per kind.
//!
//! # It is checked against the release, not merely derived from it
//!
//! `tests/against_the_release.rs` renders this data back into the two tables and compares
//! them with `releases/first-release.md` on disk. Two copies of fifteen transformations
//! would be one copy and one guess otherwise; this way neither can move without the other.

/// Every noun the release names.
///
/// Three of these are not things but families, and that is the first thing writing this
/// down made obvious. `work` outputs a `resource` without saying which; `ready` readies a
/// `thing` whatever kind it is; `move` moves a `unit`. **The table quantifies over kinds**,
/// so a kind is either a leaf or a family, and an implementation needs to know which.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Citizen,
    Garrison,
    Extractor,
    Yard,
    Ark,
    Pioneer,
    Labor,
    Metal,
    Energy,
    Food,
    Node,
    Cell,
    /// A family: anything that can move.
    Unit,
    /// A family: anything that can be exhausted and readied.
    Thing,
    /// A family: whichever of food, metal or energy is being spoken of.
    Resource,
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
            Kind::Labor => "labor",
            Kind::Metal => "metal",
            Kind::Energy => "energy",
            Kind::Food => "food",
            Kind::Node => "node",
            Kind::Cell => "cell",
            Kind::Unit => "unit",
            Kind::Thing => "thing",
            Kind::Resource => "resource",
        }
    }

    /// Whether this stands for a family of kinds rather than for one kind.
    pub fn is_family(self) -> bool {
        matches!(self, Kind::Unit | Kind::Thing | Kind::Resource)
    }
}

/// What distinguishes two things of the same kind.
///
/// **The answer to *how is a trait that varies per instance typed*.** `move` takes
/// *unit, here* and yields *unit, there*, which is the same unit and the same kind: the
/// location is a property of the instance. So a trait is not a kind and not a container -
/// it is a second axis, and a transformation names the trait it requires and the trait it
/// leaves behind.
///
/// Two of these are **derived** rather than stored, and [`Trait::is_derived`] says which.
/// A node is *unworked* when the extractors on it are fewer than the nodes; food is
/// *surplus* when it is more than the citizens. Neither is a fact anything holds - each is
/// a comparison between two counts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trait {
    InOrbit,
    Here,
    There,
    Arriving,
    OnThatUnit,
    /// Which resource an extractor is on.
    Food,
    Unworked,
    Ready,
    Exhausted,
    Surplus,
    Unfed,
}

impl Trait {
    pub fn name(self) -> &'static str {
        match self {
            Trait::InOrbit => "in orbit",
            Trait::Here => "here",
            Trait::There => "there",
            Trait::Arriving => "arriving",
            Trait::OnThatUnit => "on that unit",
            Trait::Food => "food",
            Trait::Unworked => "unworked",
            Trait::Ready => "ready",
            Trait::Exhausted => "exhausted",
            Trait::Surplus => "surplus",
            Trait::Unfed => "unfed",
        }
    }

    /// Whether this is a comparison between two counts rather than something stored.
    pub fn is_derived(self) -> bool {
        matches!(self, Trait::Unworked | Trait::Surplus)
    }
}

/// A kind, and the trait it must have or is left with.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Subject {
    pub kind: Kind,
    pub distinguished_by: Option<Trait>,
}

impl Subject {
    pub const fn plain(kind: Kind) -> Self {
        Self {
            kind,
            distinguished_by: None,
        }
    }

    pub const fn that_is(kind: Kind, distinguished_by: Trait) -> Self {
        Self {
            kind,
            distinguished_by: Some(distinguished_by),
        }
    }

    /// As the release writes it: `ark, in orbit`.
    pub fn written(self) -> String {
        match self.distinguished_by {
            Some(distinction) => format!("{}, {}", self.kind.name(), distinction.name()),
            None => self.kind.name().to_string(),
        }
    }
}

/// How many.
///
/// **The answer to *what type is a quantity*.** Twelve of the fifteen transformations give
/// a number everywhere. Three do not, on four rows between them, and none of the four is
/// missing a number - each is a number known only when the transformation is applied.
/// `work` yields the *density* of the node being worked; `spoil` and `ready` take *any*,
/// meaning however much is there.
///
/// So a quantity is not a `u32`. Anything that reads this data has to handle all three, and
/// that is worth knowing before the model is written rather than after.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Quantity {
    Exactly(u32),
    /// The density of the node being worked.
    Density,
    /// However much is there.
    Any,
}

impl Quantity {
    pub fn written(self) -> String {
        match self {
            Quantity::Exactly(count) => count.to_string(),
            Quantity::Density => "density".to_string(),
            Quantity::Any => "any".to_string(),
        }
    }
}

/// Whether the quantity is a floor or a ceiling.
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

/// One line of a transformation: something going in, or something coming out.
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

/// Where a transformation applies.
///
/// **The answer to *is scope a field or two types*.** A field. Ten are `Here` and five are
/// `Every`, and nothing else about them differs - same ports, same quantities, same bounds -
/// so two types would duplicate the whole shape to carry one bit.
///
/// What it costs is not in the data but in whatever runs it: `Here` needs a place to be
/// told, and `Every` does not.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scope {
    /// One territory, named by whoever asks for it.
    Here,
    /// Everywhere it matches, without being told where.
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

/// One transformation: what it needs and what it leaves.
#[derive(Clone, Copy, Debug)]
pub struct Transformation {
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

use Bound::{AtLeast, AtMost};
use Kind::*;
use Quantity::{Any, Density, Exactly};
use Scope::{Every, Here};

/// The fifteen transformations of `releases/first-release.md`.
pub const TRANSFORMATIONS: &[Transformation] = &[
    Transformation {
        name: "land",
        scope: Here,
        ports: &[
            takes(
                Subject::that_is(Ark, Trait::InOrbit),
                Exactly(1),
                true,
                AtLeast,
            ),
            takes(Subject::plain(Garrison), Exactly(0), false, AtMost),
            yields(Subject::plain(Garrison), Exactly(1)),
            yields(Subject::plain(Citizen), Exactly(1)),
            yields(Subject::that_is(Extractor, Trait::Food), Exactly(1)),
        ],
    },
    Transformation {
        name: "move",
        scope: Here,
        ports: &[
            takes(
                Subject::that_is(Unit, Trait::Here),
                Exactly(1),
                true,
                AtLeast,
            ),
            takes(
                Subject::that_is(Cell, Trait::OnThatUnit),
                Exactly(1),
                true,
                AtLeast,
            ),
            yields(Subject::that_is(Unit, Trait::There), Exactly(1)),
        ],
    },
    Transformation {
        name: "found by land",
        scope: Here,
        ports: &[
            takes(
                Subject::that_is(Pioneer, Trait::Arriving),
                Exactly(1),
                true,
                AtLeast,
            ),
            takes(Subject::plain(Garrison), Exactly(0), false, AtMost),
            yields(Subject::plain(Garrison), Exactly(1)),
            yields(Subject::plain(Citizen), Exactly(1)),
            yields(Subject::that_is(Extractor, Trait::Food), Exactly(1)),
        ],
    },
    Transformation {
        name: "build extractor",
        scope: Here,
        ports: &[
            takes(Subject::plain(Labor), Exactly(1), true, AtLeast),
            takes(
                Subject::that_is(Node, Trait::Unworked),
                Exactly(1),
                false,
                AtLeast,
            ),
            yields(Subject::plain(Extractor), Exactly(1)),
        ],
    },
    Transformation {
        name: "build yard",
        scope: Here,
        ports: &[
            takes(Subject::plain(Metal), Exactly(15), true, AtLeast),
            yields(Subject::plain(Yard), Exactly(1)),
        ],
    },
    Transformation {
        name: "produce pioneer",
        scope: Here,
        ports: &[
            takes(Subject::plain(Metal), Exactly(8), true, AtLeast),
            takes(Subject::plain(Energy), Exactly(6), true, AtLeast),
            takes(Subject::plain(Citizen), Exactly(1), true, AtLeast),
            takes(Subject::plain(Garrison), Exactly(1), false, AtLeast),
            yields(Subject::plain(Pioneer), Exactly(1)),
        ],
    },
    Transformation {
        name: "produce ark",
        scope: Here,
        ports: &[
            takes(Subject::plain(Metal), Exactly(12), true, AtLeast),
            takes(Subject::plain(Energy), Exactly(12), true, AtLeast),
            takes(Subject::plain(Yard), Exactly(1), false, AtLeast),
            yields(Subject::plain(Ark), Exactly(1)),
        ],
    },
    Transformation {
        name: "launch",
        scope: Here,
        ports: &[
            takes(
                Subject::that_is(Ark, Trait::Here),
                Exactly(1),
                true,
                AtLeast,
            ),
            takes(
                Subject::that_is(Cell, Trait::OnThatUnit),
                Exactly(1),
                true,
                AtLeast,
            ),
            yields(Subject::that_is(Ark, Trait::InOrbit), Exactly(1)),
        ],
    },
    Transformation {
        name: "spend readiness",
        scope: Here,
        ports: &[
            takes(
                Subject::that_is(Citizen, Trait::Ready),
                Exactly(1),
                true,
                AtLeast,
            ),
            yields(Subject::that_is(Citizen, Trait::Exhausted), Exactly(1)),
            yields(Subject::plain(Labor), Exactly(1)),
        ],
    },
    Transformation {
        name: "work",
        scope: Here,
        ports: &[
            takes(Subject::plain(Labor), Exactly(1), true, AtLeast),
            takes(Subject::plain(Extractor), Exactly(1), false, AtLeast),
            yields(Subject::plain(Resource), Density),
        ],
    },
    Transformation {
        name: "eat",
        scope: Every,
        ports: &[
            takes(Subject::plain(Citizen), Exactly(1), false, AtLeast),
            takes(Subject::plain(Food), Exactly(1), true, AtLeast),
        ],
    },
    Transformation {
        name: "grow",
        scope: Every,
        ports: &[
            takes(
                Subject::that_is(Food, Trait::Surplus),
                Exactly(1),
                true,
                AtLeast,
            ),
            yields(Subject::plain(Citizen), Exactly(1)),
        ],
    },
    Transformation {
        name: "depart",
        scope: Every,
        ports: &[takes(
            Subject::that_is(Citizen, Trait::Unfed),
            Exactly(1),
            true,
            AtLeast,
        )],
    },
    Transformation {
        name: "spoil",
        scope: Every,
        ports: &[takes(Subject::plain(Food), Any, true, AtLeast)],
    },
    Transformation {
        name: "ready",
        scope: Every,
        ports: &[
            takes(
                Subject::that_is(Thing, Trait::Exhausted),
                Any,
                true,
                AtLeast,
            ),
            yields(Subject::that_is(Thing, Trait::Ready), Any),
        ],
    },
];

/// What a thing is, apart from what turns into it.
///
/// The blank columns are blank in the release too, and they are not oversights: a citizen
/// has a force and no cells because it does not move, and an extractor has neither because
/// it is neither a fighter nor a traveller.
#[derive(Clone, Copy, Debug)]
pub struct Producible {
    pub kind: Kind,
    pub force: Option<u32>,
    pub cells: Option<u32>,
    /// What one move costs, in cells.
    pub a_move: Option<u32>,
    pub upkeep: Option<(u32, Kind)>,
    pub costs: &'static [(u32, Kind)],
    /// What the table says that the figures do not.
    pub aside: Option<&'static str>,
    pub requires: Option<&'static str>,
}

/// Force, cells, upkeep and cost, per kind.
pub const PRODUCIBLE: &[Producible] = &[
    Producible {
        kind: Citizen,
        force: Some(1),
        cells: None,
        a_move: None,
        upkeep: None,
        costs: &[],
        aside: None,
        requires: None,
    },
    Producible {
        kind: Garrison,
        force: Some(1),
        cells: None,
        a_move: None,
        upkeep: None,
        costs: &[],
        aside: Some("not produced; founding gives one"),
        requires: None,
    },
    Producible {
        kind: Extractor,
        force: None,
        cells: None,
        a_move: None,
        upkeep: None,
        costs: &[(1, Labor)],
        aside: Some("and nothing else"),
        requires: None,
    },
    Producible {
        kind: Yard,
        force: None,
        cells: None,
        a_move: None,
        upkeep: None,
        costs: &[(15, Metal)],
        aside: None,
        requires: None,
    },
    Producible {
        kind: Ark,
        force: Some(2),
        cells: Some(2),
        a_move: Some(1),
        upkeep: None,
        costs: &[(12, Metal), (12, Energy)],
        aside: None,
        requires: Some("a Yard"),
    },
    Producible {
        kind: Pioneer,
        force: Some(2),
        cells: Some(2),
        a_move: Some(1),
        upkeep: Some((1, Food)),
        costs: &[(8, Metal), (6, Energy), (1, Citizen)],
        aside: None,
        requires: Some("a garrison"),
    },
];

impl Producible {
    /// The cost column, as the release writes it.
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

    /// The upkeep column, as the release writes it.
    pub fn upkeep_written(&self) -> String {
        match self.upkeep {
            Some((count, kind)) => format!("{count} {} per turn", kind.name()),
            None => String::new(),
        }
    }
}

/// The **Units and structures** table, as rows of cells, header included.
///
/// Rendered rather than stored, so that what this crate says and what the release says are
/// the same sentence rather than two sentences that agree today.
pub fn units_table() -> Vec<Vec<String>> {
    let mut rows = vec![
        [
            "Thing",
            "Force",
            "Cells",
            "A move",
            "Upkeep",
            "Costs to produce",
            "Requires",
        ]
        .iter()
        .map(|cell| cell.to_string())
        .collect::<Vec<_>>(),
    ];
    for thing in PRODUCIBLE {
        rows.push(vec![
            format!("**{}**", thing.kind.name()),
            thing.force.map(|n| n.to_string()).unwrap_or_default(),
            thing.cells.map(|n| n.to_string()).unwrap_or_default(),
            thing
                .a_move
                .map(|n| format!("{n} cell{}", if n == 1 { "" } else { "s" }))
                .unwrap_or_default(),
            thing.upkeep_written(),
            thing.cost_written(),
            thing.requires.unwrap_or_default().to_string(),
        ]);
    }
    rows
}

/// The **Transformations** table, as rows of cells, header included.
///
/// The scope and the name appear on a transformation's first row and are blank on the rest,
/// which is the table's own shape: a transformation is one thing with several ports, not
/// several things that share a name.
pub fn transformations_table() -> Vec<Vec<String>> {
    let mut rows = vec![
        [
            "Transformation",
            "Scope",
            "Role",
            "Thing",
            "Qty",
            "Consumed",
            "Bound",
        ]
        .iter()
        .map(|cell| cell.to_string())
        .collect::<Vec<_>>(),
    ];
    for transformation in TRANSFORMATIONS {
        for (at, port) in transformation.ports.iter().enumerate() {
            let first = at == 0;
            let (role, subject, quantity, consumed, bound) = match port {
                Port::In {
                    subject,
                    quantity,
                    consumed,
                    bound,
                } => (
                    "in",
                    subject,
                    quantity,
                    if *consumed { "yes" } else { "no" }.to_string(),
                    bound.written().to_string(),
                ),
                Port::Out { subject, quantity } => {
                    ("out", subject, quantity, String::new(), String::new())
                }
            };
            rows.push(vec![
                if first {
                    format!("**{}**", transformation.name)
                } else {
                    String::new()
                },
                if first {
                    transformation.scope.written().to_string()
                } else {
                    String::new()
                },
                role.to_string(),
                subject.written(),
                quantity.written(),
                consumed,
                bound,
            ]);
        }
    }
    rows
}
