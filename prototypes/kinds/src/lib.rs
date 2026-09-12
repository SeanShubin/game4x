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
pub mod recipes;
pub mod release;

// ---------------------------------------------------------------------------------------
// Kinds
// ---------------------------------------------------------------------------------------

/// The seventeen kinds the release declares, and one it uses without declaring.
///
/// **`fertility` is the seventeenth**, arriving with the saturating rewrite: `grow` consumed
/// *the lesser of the surplus food and the citizens here*, and `bear`, `breed` and `renew`
/// are that rule written as smaller ones that fire as many times as they can. Fertility is
/// the thing they pass between them - a citizen's capacity to raise one more.
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
    Store,
    Yard,
    Ark,
    Pioneer,
    Food,
    Metal,
    Energy,
    Labor,
    Territory,
    Orbit,
    Deposit,
    Adjacency,
    Game,
    Fertility,
    /// **`P-414`: what a citizen or a unit musters, and what nature is measured against.**
    ///
    /// **Declared by no Kinds table and used as one by three rows**, which is `C-93`. `muster`
    /// and `stand` produce it, `discard` sweeps it at a turn's end, and a produce row's Kind
    /// column admits a kind or a family and nothing else - so this is the assumption that item
    /// states rather than a reading of the table.
    Force,
}

impl Kind {
    pub fn name(self) -> &'static str {
        match self {
            Kind::Citizen => "citizen",
            Kind::Garrison => "garrison",
            Kind::Extractor => "extractor",
            Kind::Store => "store",
            Kind::Yard => "yard",
            Kind::Ark => "ark",
            Kind::Pioneer => "pioneer",
            Kind::Food => "food",
            Kind::Metal => "metal",
            Kind::Energy => "energy",
            Kind::Labor => "labor",
            Kind::Territory => "territory",
            Kind::Orbit => "orbit",
            Kind::Deposit => "deposit",
            Kind::Adjacency => "adjacency",
            Kind::Game => "game",
            Kind::Fertility => "fertility",
            Kind::Force => "force",
        }
    }

    pub fn what_it_is(self) -> &'static str {
        match self {
            Kind::Citizen => "a person: provides labor, eats, and grows on surplus",
            Kind::Garrison => "what holds a territory; a territory has at most one",
            Kind::Extractor => "built for one resource, and worked to produce it",
            Kind::Store => "built to hold one resource, and holds nothing else",
            Kind::Yard => "where an Ark is produced",
            Kind::Ark => "carries a landing, and can invade from orbit",
            Kind::Pioneer => "founds a territory",
            Kind::Food => "eaten by citizens; expires",
            // **`P-426` reconciled this with the planet being a source.** It read
            // *conserved*, unqualified, while `spec/invariants.md` makes the planet's
            // material endless and `work` mines metal out of it every firing.
            Kind::Metal => {
                "what things are built from; drawn from the planet, and conserved once above ground"
            }
            Kind::Energy => "what moves things; neither conserved nor expiring",
            Kind::Labor => "what working a machine takes; a citizen provides it each turn",
            Kind::Territory => concat!(
                "a place things are in, which has a biome, a force of nature, ",
                "and a density and a total capacity per resource"
            ),
            Kind::Orbit => "a place above one territory, which holds units and nothing else",
            Kind::Deposit => "what a territory's ground offers of one resource, and how richly",
            Kind::Adjacency => "two places that share an edge, held by the thing that holds them",
            Kind::Game => "every thing is in it, and it is the one thing that is in nothing",
            Kind::Fertility => concat!(
                "a citizen's capacity to raise one more, spent by raising one ",
                "and renewed each turn"
            ),
            Kind::Force => "what a citizen or a unit musters, and what nature is measured against",
        }
    }

    /// What bounds this kind in a territory, as the release writes it.
    ///
    /// **It was a number and is now a mechanism.** `P-207` replaced *What a territory has
    /// total capacity for* with *What bounds a kind in a territory*, because most of the
    /// entries were never capacities: a citizen is bounded by the food produced here through
    /// upkeep, and labor by the citizens that make it. Writing those as numbers had already
    /// produced one wrong number - a stated capacity of 8 citizens that the model never
    /// implemented and Sean never intended.
    ///
    /// `None` for the five kinds that are not things in a territory.
    pub fn bounded_by(self) -> Option<&'static str> {
        Some(match self {
            Kind::Citizen => "the food produced here, through upkeep",
            Kind::Garrison => "a capacity of 1",
            Kind::Extractor => "a capacity, from *Territory resources*",
            Kind::Store => "as many as the extractors of its resource",
            Kind::Yard => "a capacity of 1",
            Kind::Ark => "a capacity of 2",
            Kind::Pioneer => "a capacity of 2",
            Kind::Labor => "the citizens that make it, one each per turn",
            // **`P-380` gave `fertility` this row, word for word with `labor`'s above**, and
            // in doing so made `labor`'s own row true: both are transient, and nothing swept
            // the remainder while the sentence said the citizens bounded it. `discard` has
            // four rows now rather than two.
            Kind::Fertility => "the citizens that make it, one each per turn",
            // **`P-258`: a territory declares no capacity for a resource.** It declares
            // capacity for the things that hold them, so what it keeps is what its stores
            // hold - and a territory that has built none keeps nothing. The flat twenty was
            // a property of the wrong thing.
            Kind::Food => "the things in it that hold it, and it keeps for one turn",
            Kind::Metal => "the things in it that hold it",
            Kind::Energy => "the things in it that hold it",
            // **`deposit` is bounded by nothing the release states.** *What bounds a kind
            // in a territory* has eleven rows and none of them is a deposit - it is what the
            // ground *is* rather than something built on it, so there is no capacity for it.
            // The two places are `None` for the same reason: the table is about things in a
            // territory.
            // **`P-351`: `game` is in nothing**, so *what bounds a kind in a territory* cannot
            // be about it at all - it is not in a territory, it holds them. That is a stronger
            // reason than the four above have, and it lands in the same arm.
            // **`force` is bounded by nothing the release states.** It is mustered from the
            // citizens and units that are there and swept at a turn's end, so what bounds it
            // is how many of them there are rather than a capacity a territory declares.
            Kind::Territory
            | Kind::Orbit
            | Kind::Deposit
            | Kind::Adjacency
            | Kind::Game
            | Kind::Force => {
                return None;
            }
        })
    }
}

/// In the order the Kinds table lists them.
pub const KINDS: [Kind; 17] = [
    Kind::Citizen,
    Kind::Garrison,
    Kind::Extractor,
    Kind::Yard,
    Kind::Store,
    Kind::Ark,
    Kind::Pioneer,
    Kind::Food,
    Kind::Metal,
    Kind::Energy,
    Kind::Labor,
    Kind::Territory,
    Kind::Orbit,
    Kind::Deposit,
    Kind::Adjacency,
    Kind::Game,
    Kind::Fertility,
    // **`Kind::Force` is deliberately not here** - `C-93`. `KINDS` renders the release's
    // *Kinds* table back and is compared to it cell for cell, and the release declares
    // seventeen kinds without `force`. The variant exists because three recipe rows use
    // `force` in the Kind column and a row has nowhere else to put it; the table says what
    // the release says. **The two disagree, and that is the finding rather than a bug here.**
];

/// In the order the bounds table lists them, which is not the Kinds order.
pub const BOUND_ORDER: [Kind; 12] = [
    Kind::Citizen,
    Kind::Garrison,
    Kind::Extractor,
    Kind::Store,
    Kind::Yard,
    Kind::Ark,
    Kind::Pioneer,
    Kind::Labor,
    Kind::Fertility,
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
    // **`P-260` and `P-265`.** This row said *an extractor's catch*, holding up to the
    // territory's density - and the same document said four lines later that an extractor
    // holds nothing. `C-26` was that contradiction; `P-265` resolved it toward the prose,
    // because the model had never given an extractor a capacity and the other reading would
    // have cost a field, production routed into catches, and a bound per extractor.
    Capacity {
        what: "a store",
        holds: "the resource it was built for",
        up_to: "10",
    },
    Capacity {
        what: "a unit's tank",
        holds: "energy",
        up_to: "the unit's fuel",
    },
    // **`P-411` took the fourth row out.** Readiness is a count a thing carries again, so
    // what bounds it is the trait's own values - `0 or 1` - rather than a capacity to hold
    // something. Three sorts of capacity, as before `P-399`.
];

// ---------------------------------------------------------------------------------------
// Traits
// ---------------------------------------------------------------------------------------

/// Whether a trait is held or worked out.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Held {
    Stored,
    /// **`P-407`: a fact about the kind rather than about any one of them.**
    ///
    /// `force`, `fuel`, `upkeep`, `keeps` and `movable`. Every citizen's force is the same
    /// number, so it is not something one citizen carries - and `P-417` follows from it: a
    /// description carries the traits of the thing and not those of its kind, which is why
    /// `{garrison force:0}` lost its word.
    OfTheKind,
    /// Worked out from other things, with the release's own account of how.
    Derived(&'static str),
}

impl Held {
    pub fn written(self) -> String {
        match self {
            Held::Stored => "stored".to_string(),
            Held::OfTheKind => "of the kind".to_string(),
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

pub const TRAITS: [TraitRow; 23] = [
    // **`P-417` deleted the `kind` row**, because a kind is not a trait: `spec/console.md`
    // lists them as different categories and no recipe writes `kind:`.
    // **`P-285` and `P-286`: a thing is not located by a trait.** `place` said *the thing it
    // is in*, which made location something a thing carries rather than something its
    // container says. What holds it is what says where it is - so the trait is gone and
    // nothing replaces it. The `place` **family** - territory, orbit - is untouched, and a
    // search for the word finds both; only one of them went.
    TraitRow {
        name: "id",
        of: "a thing that must be named individually",
        values: "a number, unique among things of its kind",
        held: Held::Stored,
    },
    // **`P-411` undid `P-399` and `C-90` is why.** Readiness was a kind a thing held, and
    // two citizens differing only in it had the same description - so the map form could not
    // tell them apart. It is a count the thing carries again, one per action, and the actions
    // are separate traits rather than values of one.
    TraitRow {
        name: "moving",
        of: "a unit",
        values: "0 or 1",
        held: Held::Stored,
    },
    TraitRow {
        name: "laboring",
        of: "a citizen",
        values: "0 or 1",
        held: Held::Stored,
    },
    TraitRow {
        name: "working",
        of: "an extractor",
        values: "0 or 1",
        held: Held::Stored,
    },
    TraitRow {
        name: "bearing",
        of: "a citizen",
        values: "0 or 1",
        held: Held::Stored,
    },
    // **`P-414`: force is mustered rather than computed**, and this is what a thing spends to
    // muster it. `spec/control.md` has no *highest* case any more.
    TraitRow {
        name: "defending",
        of: "a citizen or a unit",
        values: "0 or 1",
        held: Held::Stored,
    },
    TraitRow {
        name: "resource",
        of: "an extractor or a store",
        values: "one of the resources",
        held: Held::Stored,
    },
    TraitRow {
        name: "force",
        of: "citizen, garrison, ark, pioneer",
        values: "a number",
        held: Held::OfTheKind,
    },
    TraitRow {
        name: "fuel",
        of: "a unit",
        values: "how much energy its tank holds",
        held: Held::OfTheKind,
    },
    TraitRow {
        name: "upkeep",
        of: "a thing with upkeep",
        values: "food per turn",
        held: Held::OfTheKind,
    },
    TraitRow {
        name: "metal in it",
        of: "whatever is built",
        values: "a number",
        held: Held::Derived("its binding plus the metal in its parts"),
    },
    TraitRow {
        name: "density",
        of: "a deposit",
        values: "a number",
        held: Held::Stored,
    },
    TraitRow {
        name: "total capacity",
        of: "a deposit",
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
        values: "one of the biomes",
        held: Held::Stored,
    },
    TraitRow {
        name: "nature",
        of: "a territory",
        values: "a number",
        held: Held::Stored,
    },
    TraitRow {
        // **`P-311`/`P-314`: adjacency is a fact the container holds, not one a place carries.**
        // It read *a place / which places it touches*, which put the relation on each end of
        // it - so the same edge was stated twice and could disagree with itself.
        name: "from",
        of: "an adjacency",
        values: "a place",
        held: Held::Stored,
    },
    TraitRow {
        name: "to",
        of: "an adjacency",
        values: "a place",
        held: Held::Stored,
    },
    TraitRow {
        name: "keeps",
        of: "thing",
        values: "the number of turns it will last",
        held: Held::OfTheKind,
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
    // **`P-288`: `phase` is a declared trait and `turn` is not.** The release grew this row
    // and the gate went red until this list followed, which is what `P-263` says a promotion
    // into a table the code generates from does. `turn` gets no row here and needs no rule
    // forbidding one: `P-284` admits only kinds, traits and trait values into a data file,
    // and `turn` is none of the three.
    // **`P-308` names the values.** The cell read *before it starts, or once it has*, which
    // describes them and names neither - so `design` and `play` appeared nowhere in the
    // release, and `play` was a forbidden word under `P-284` while sitting in the data file.
    TraitRow {
        name: "phase",
        of: "the game",
        values: "design or play",
        held: Held::Stored,
    },
    // **`P-355`.** It replaces nothing: the `A move` column `P-346` deleted was a cost in
    // fuel, and this is whether the thing moves at all. `move` still consumes a literal 1
    // energy, and selecting by this trait rather than by the `unit` family waits on `C-56`.
    TraitRow {
        name: "movable",
        of: "whatever moves",
        values: "yes or no",
        held: Held::OfTheKind,
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
    /// **`P-399`: the thing moves, and is neither taken nor made.**
    ///
    /// `move` used to consume a unit in one place and produce one in another, which said the
    /// unit that arrived was a different unit. It is put instead - the same thing, somewhere
    /// else - and `P-396` is why that mattered: under the token model a created thing arrives
    /// holding its tokens, so a produced unit would arrive able to move again.
    ///
    /// **Its quantity cell is blank and that is not a zero.** The release: *a blank is not a
    /// zero. It says the row has no such number* - and a named thing is not a quantity.
    Put,
}

impl Role {
    pub fn written(self) -> &'static str {
        match self {
            Role::Require => "require",
            Role::Limit => "limit",
            Role::Consume => "consume",
            Role::Produce => "produce",
            Role::Put => "put",
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
    /// No number at all, which `put` is the only role to write.
    ///
    /// **A blank is not a zero**, which the release states directly: *it says the row has no
    /// such number, and a quantity read from one produces nothing.* Writing `Exactly(0)` here
    /// would render as `0` and say the recipe moves no units.
    None,
}

impl Quantity {
    pub fn written(self) -> String {
        match self {
            Quantity::Exactly(count) => count.to_string(),
            Quantity::OfATrait(how) => how.to_string(),
            Quantity::None => String::new(),
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

/// A `put` row: the thing stays, with one of its counts changed.
///
/// **`P-411`'s shape, and the quantity is blank rather than zero.** The release says a blank is
/// not a zero - it says the row has no such number - and a named thing is not a quantity.
const fn put(noun: Noun, traits: &'static [Qualifier]) -> Line {
    Line {
        role: Role::Put,
        quantity: Quantity::None,
        noun,
        traits,
        place: None,
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

// **`measured_traited` was here and is gone with `grow`.** It existed for the one row that
// needed an expression and a qualifier at once - *the lesser of the surplus food and the
// citizens here*, on food that is `surplus`. The saturating rewrite removed that row, so
// every remaining expression is `work`'s and carries no qualifier. Deleted rather than kept
// against a future caller: an unused constructor is a shape the release no longer has.

use Kind::*;
use Owner::{Player, World};
use Quantity::OfATrait;
use Role::{Consume, Produce, Require};

// **`P-334` turned `adjacency` from a trait into a kind, and this is what it left behind.**
// The phrase distinguishes a place by there being an adjacency whose `from` is `$from` and
// whose `to` is this place - so `to` is the trait that expresses it, and `adjacency` is no
// longer a trait at all. **A reading rather than a rule**, filed as `C-60`: the release still
// words it as though adjacency were a property of a place.
const JOINED_TO_FROM: [Qualifier; 1] = [by("joined to `$from` by an edge the unit crosses", "to")];
const FOR_FOOD: [Qualifier; 1] = [by("food", "resource")];
const FOR_METAL: [Qualifier; 1] = [by("metal", "resource")];
const OF_RESOURCE: [Qualifier; 1] = [by("`$resource`", "resource")];
// **`P-399`: a readiness names the action it is for.** These replace `ready`, `not ready`,
// `fertile` and `spent`, which were four qualifiers over two yes-or-no traits; there is one
// trait now and its value is the action.
// **`P-411`: a count a thing carries, one trait per action.** A rule requires at least one and
// puts the thing back with one less; `refresh` puts it back at its maximum. Four rows where the
// token model had one, and two citizens differing only in readiness are two descriptions again
// - which is `C-90`, and why `P-399` was undone.
const MOVING_SOME: [Qualifier; 1] = [by("moving at least 1", "moving")];
const MOVING_LESS: [Qualifier; 1] = [by("moving one less", "moving")];
const MOVING_FULL: [Qualifier; 1] = [by("moving at its maximum", "moving")];
const LABORING_SOME: [Qualifier; 1] = [by("laboring at least 1", "laboring")];
const LABORING_LESS: [Qualifier; 1] = [by("laboring one less", "laboring")];
const LABORING_FULL: [Qualifier; 1] = [by("laboring at its maximum", "laboring")];
const WORKING_SOME: [Qualifier; 1] = [by("working at least 1", "working")];
const WORKING_LESS: [Qualifier; 1] = [by("working one less", "working")];
const WORKING_FULL: [Qualifier; 1] = [by("working at its maximum", "working")];
const BEARING_SOME: [Qualifier; 1] = [by("bearing at least 1", "bearing")];
const BEARING_LESS: [Qualifier; 1] = [by("bearing one less", "bearing")];
const BEARING_FULL: [Qualifier; 1] = [by("bearing at its maximum", "bearing")];
const DEFENDING_SOME: [Qualifier; 1] = [by("defending at least 1", "defending")];
const DEFENDING_LESS: [Qualifier; 1] = [by("defending one less", "defending")];
const DEFENDING_FULL: [Qualifier; 1] = [by("defending at its maximum", "defending")];
// **`fertile` and `spent` are one trait read both ways** - `spent`, yes or no. `bear` takes a
// citizen that is not spent and leaves one that is; `renew` does the reverse, once per turn.

const UPKEEP_UNPAID: [Qualifier; 1] = [by("whose upkeep is unpaid", "unpaid")];
const KEEPS_NONE: [Qualifier; 1] = [by("keeps 0", "keeps")];
const KEEPS_SOME: [Qualifier; 1] = [by("keeps at least 1", "keeps")];
const KEEPS_LESS: [Qualifier; 1] = [by("keeps one less", "keeps")];

const TERRITORY: Noun = Noun::Of(Kind::Territory);
const UNIT: Noun = Noun::Any(Family::Unit);
const RESOURCE: Noun = Noun::Any(Family::Resource);
const THING: Noun = Noun::Any(Family::Thing);
const PLACE: Noun = Noun::Any(Family::Place);
const EXTRACTOR: Noun = Noun::Of(Kind::Extractor);

/// The seventeen recipes of `releases/first-release.md`.
pub const RECIPES: &[Recipe] = &[
    Recipe {
        name: "deploy ark",
        owner: Player,
        lines: &[
            placed(Require, 1, TERRITORY, &[], "`$where`"),
            placed(Consume, 1, Noun::Of(Ark), &[], "the orbit above `$where`"),
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
            // **Put, not consumed and produced** - `P-399`, and `P-396` is the reason: a
            // created thing arrives holding its tokens, so a produced unit would arrive able
            // to move again.
            placed(Require, 1, UNIT, &MOVING_SOME, "`$from`"),
            Line {
                role: Role::Put,
                quantity: Quantity::None,
                noun: UNIT,
                traits: &MOVING_LESS,
                place: Some("`$to`"),
            },
            placed(Consume, 1, Noun::Of(Energy), &[], "that unit"),
        ],
    },
    Recipe {
        name: "found by land",
        owner: Player,
        lines: &[
            just(Consume, 1, Noun::Of(Pioneer)),
            just(Produce, 1, Noun::Of(Garrison)),
            just(Produce, 2, Noun::Of(Citizen)),
            traited(Produce, 1, Noun::Of(Extractor), &FOR_FOOD),
            traited(Produce, 1, Noun::Of(Extractor), &FOR_METAL),
        ],
    },
    Recipe {
        name: "build extractor",
        owner: Player,
        lines: &[
            just(Consume, 1, Noun::Of(Labor)),
            just(Consume, 1, Noun::Of(Metal)),
            traited(Produce, 1, Noun::Of(Extractor), &OF_RESOURCE),
        ],
    },
    // **`P-260`: one recipe with a `$resource`, not three.** The same shape `build
    // extractor` has had since `P-234`, which is what makes `store` one kind with a trait.
    Recipe {
        name: "build store",
        owner: Player,
        lines: &[
            just(Consume, 1, Noun::Of(Labor)),
            just(Consume, 1, Noun::Of(Metal)),
            traited(Produce, 1, Noun::Of(Store), &OF_RESOURCE),
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
        // P-342: one recipe where there were two, and it produces nothing. Launching is
        // not a move - the cost is paid at a Yard and nothing comes back.
        name: "launch ark",
        owner: Player,
        // **`P-362` put the Ark back.** `P-342` had dropped `produce 1 ark` because there was
        // no way to name where it went; `P-334` made adjacency a kind, so an orbit carries
        // `below` and the destination can be written. The territory row comes with it, since
        // the orbit is named relative to `$where`.
        lines: &[
            placed(Require, 1, TERRITORY, &[], "`$where`"),
            just(Consume, 3, Noun::Of(Metal)),
            just(Consume, 12, Noun::Of(Energy)),
            just(Consume, 2, Noun::Of(Citizen)),
            just(Require, 1, Noun::Of(Yard)),
            placed(Produce, 1, Noun::Of(Ark), &[], "the orbit above `$where`"),
        ],
    },
    Recipe {
        name: "create labor",
        owner: Player,
        lines: &[
            traited(Require, 1, Noun::Of(Citizen), &LABORING_SOME),
            put(Noun::Of(Citizen), &LABORING_LESS),
            just(Produce, 1, Noun::Of(Labor)),
        ],
    },
    Recipe {
        name: "work",
        owner: Player,
        lines: &[
            placed(Require, 1, TERRITORY, &[], "`$where`"),
            traited(Require, 1, EXTRACTOR, &WORKING_SOME),
            put(EXTRACTOR, &WORKING_LESS),
            just(Consume, 1, Noun::Of(Labor)),
            measured(
                Produce,
                OfATrait("`$where`'s density for that resource"),
                RESOURCE,
            ),
        ],
    },
    // **The world's ten, in `P-379`'s stated order** - `upkeep`, then `bear`, `breed` and
    // `renew`, then `perish`, then `age`, then `spoil`, then `stow` and `discard`, then
    // `refresh`. It was six, and the saturating rewrite is what changed them: `P-373` says a
    // rule whose quantity is read from the state is written as a smaller rule that fires as
    // many times as it can.
    Recipe {
        // **Every quantity is a constant now.** It was *require 1 thing with upkeep* and
        // *consume the thing's upkeep food*; a citizen is the only thing in this release with
        // upkeep and its upkeep is one food, so the rule that fires once per citizen says the
        // same thing without reading a trait for the amount.
        name: "upkeep",
        owner: World,
        lines: &[
            just(Require, 1, Noun::Of(Citizen)),
            just(Consume, 1, Noun::Of(Food)),
        ],
    },
    // **`grow` is gone and these three replace it.** It consumed *the lesser of the surplus
    // food and the citizens here*, which is a quantity read from the state. `bear` makes one
    // fertility per fertile citizen and turns that citizen spent; `breed` turns a fertility
    // and a food into a citizen; `renew` makes a spent citizen fertile again. **The `spent`
    // trait is what bounds the increase at the number of citizens** - the job `grow`'s
    // expression used to do - because a citizen that has borne cannot bear again this turn.
    Recipe {
        name: "bear",
        owner: World,
        lines: &[
            traited(Require, 1, Noun::Of(Citizen), &BEARING_SOME),
            put(Noun::Of(Citizen), &BEARING_LESS),
            just(Produce, 1, Noun::Of(Fertility)),
        ],
    },
    Recipe {
        name: "breed",
        owner: World,
        lines: &[
            just(Consume, 1, Noun::Of(Fertility)),
            just(Consume, 1, Noun::Of(Food)),
            just(Produce, 1, Noun::Of(Citizen)),
        ],
    },
    Recipe {
        // **One row, and it used to have two.** The second produced *the thing's metal* in
        // metal, which is a quantity read from a trait; a citizen is what perishes and a
        // citizen has no metal in it, so the row gave back nothing and said it in an
        // expression.
        name: "perish",
        owner: World,
        lines: &[traited(Consume, 1, Noun::Of(Citizen), &UPKEEP_UNPAID)],
    },
    Recipe {
        // **`P-340`: `age` fires before `spoil`, and both name `thing`.** Under the old
        // order a food made with `keeps` 1 was aged to 0 at one turn's end and removed at the
        // next - a two-turn life, against `spec/turn.md` and against the release's own *food
        // keeps for one turn*. The model discarded it at the first ending and was right; this
        // is the release catching up.
        name: "age",
        owner: World,
        // **`P-431` rewrote both rows**, answering the research lens's `X-30`. It was
        // `consume` a thing with `keeps at least 1` and `produce` one with `keeps one less`,
        // which destroyed and recreated the thing - and, under *a rule fires as many times as
        // it can*, the produce satisfied its own consume whenever `keeps` was 2 or more, so a
        // thing declared to last three turns aged to nothing in one.
        lines: &[
            traited(Require, 1, THING, &KEEPS_SOME),
            put(THING, &KEEPS_LESS),
        ],
    },
    Recipe {
        name: "spoil",
        owner: World,
        lines: &[traited(Consume, 1, THING, &KEEPS_NONE)],
    },
    // **`stow` and `discard` are each stated once per kind, which is `P-373` again.** A rule
    // whose subject is a family is a rule for each member, and writing *a resource* here
    // would be one rule with a quantity that depends on which member it caught. So `stow` has
    // two blocks and `discard` four - and `discard`'s four are not a family at all: metal and
    // energy are resources returning to their source, `labor` and `fertility` are transient
    // and have no source to return to.
    Recipe {
        name: "stow",
        owner: World,
        lines: &[
            just(Consume, 1, Noun::Of(Metal)),
            placed(Produce, 1, Noun::Of(Metal), &[], "a store for metal"),
        ],
    },
    Recipe {
        name: "stow",
        owner: World,
        lines: &[
            just(Consume, 1, Noun::Of(Energy)),
            placed(Produce, 1, Noun::Of(Energy), &[], "a store for energy"),
        ],
    },
    Recipe {
        name: "discard",
        owner: World,
        lines: &[just(Consume, 1, Noun::Of(Metal))],
    },
    Recipe {
        name: "discard",
        owner: World,
        lines: &[just(Consume, 1, Noun::Of(Energy))],
    },
    // **`P-380` added these two**, and they are what makes `labor`'s and `fertility`'s bound
    // rows true: both are transient, always in disorder, and nothing swept the remainder
    // before. A fertility that outlived the turn let a territory with no citizens repopulate
    // from stock - `C-83`.
    Recipe {
        name: "discard",
        owner: World,
        lines: &[just(Consume, 1, Noun::Of(Labor))],
    },
    Recipe {
        name: "discard",
        owner: World,
        lines: &[just(Consume, 1, Noun::Of(Fertility))],
    },
    Recipe {
        // **One block per action, not one block of four rows** - `P-411`. `refresh` puts each
        // thing back at its maximum rather than making a readiness, because readiness is a
        // count the thing carries again and there is nothing to make. A repeated name is one
        // rule applied to several kinds, which is `stow` and `discard`'s shape already.
        name: "refresh",
        owner: World,
        lines: &[put(UNIT, &MOVING_FULL)],
    },
    Recipe {
        name: "refresh",
        owner: World,
        lines: &[put(Noun::Of(Citizen), &LABORING_FULL)],
    },
    Recipe {
        name: "refresh",
        owner: World,
        lines: &[put(Noun::Of(Citizen), &BEARING_FULL)],
    },
    Recipe {
        name: "refresh",
        owner: World,
        lines: &[put(EXTRACTOR, &WORKING_FULL)],
    },
    // **`P-414`: force is mustered rather than computed**, and `P-416` removed the *highest*
    // case from `spec/control.md` entirely. A citizen musters only where a garrison stands; a
    // unit stands wherever it is. **A territory with no garrison presents no force at all**,
    // where the model takes the maximum today.
    Recipe {
        name: "muster",
        owner: World,
        lines: &[
            just(Require, 1, Noun::Of(Garrison)),
            traited(Require, 1, Noun::Of(Citizen), &DEFENDING_SOME),
            put(Noun::Of(Citizen), &DEFENDING_LESS),
            measured(Produce, OfATrait("that citizen's force"), Noun::Of(Force)),
        ],
    },
    Recipe {
        name: "stand",
        owner: World,
        lines: &[
            traited(Require, 1, UNIT, &DEFENDING_SOME),
            put(UNIT, &DEFENDING_LESS),
            measured(Produce, OfATrait("that unit's force"), Noun::Of(Force)),
        ],
    },
    Recipe {
        name: "refresh",
        owner: World,
        lines: &[put(Noun::Of(Citizen), &DEFENDING_FULL)],
    },
    Recipe {
        name: "refresh",
        owner: World,
        lines: &[put(UNIT, &DEFENDING_FULL)],
    },
    // Force is swept at a turn's end like the other transients, so what a territory presents
    // is what it mustered this turn rather than what it has ever mustered.
    Recipe {
        name: "discard",
        owner: World,
        lines: &[just(Consume, 1, Noun::Of(Force))],
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
    pub upkeep: Option<(u32, Kind)>,
    pub costs: &'static [(u32, Kind)],
    /// What holds it together, and what `perish` gives back before its parts are counted.
    pub binding: Option<u32>,
    /// **`P-355`: whatever moves, yes or no, stored.** Filled for an ark and a pioneer only.
    ///
    /// **Three columns now name exactly those two - Fuel, Crosses and this - and that is a
    /// decision rather than duplication.** Sean approved it knowing the sets coincide today,
    /// because they are expected to diverge: fuel in a tank is one way to move and there will
    /// be others.
    pub movable: bool,
    /// Which kinds of edge it may move along, which is what decides where it can ever be.
    ///
    /// **An Ark crosses one kind of edge and it is not `border`**: `orbit border`, between
    /// two orbits, which is what an arriving Ark uses to reach the orbit above its landing
    /// zone. Sean's rule that an Ark cannot move between two territories is a consequence of
    /// this column rather than something merely obeyed.
    ///
    /// **`P-344` took `ascent` out of this cell and left `P-71`'s third edge kind with no
    /// user.** An ascent is between the ground and the orbit above it, and there is no Ark on
    /// the ground to make one: `P-342` made producing and launching a single act that pays an
    /// Ark's cost at a Yard and puts nothing into orbit. So the edge kind still exists and
    /// nothing crosses it.
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
        upkeep: Some((1, Food)),
        costs: &[],
        movable: false,
        binding: None,
        crosses: None,
        requires: None,
        readies: true,
    },
    Producible {
        kind: Garrison,
        // **`P-277`: a garrison has no force of its own.** `P-276` says what it does
        // instead - it lets the citizens of that territory sum their force, by existing,
        // and nothing has to work it. Its cost is unchanged.
        force: Some(0),
        fuel: None,
        upkeep: None,
        costs: &[(1, Labor), (1, Metal)],
        movable: false,
        binding: Some(1),
        crosses: None,
        requires: None,
        readies: false,
    },
    Producible {
        kind: Extractor,
        force: None,
        fuel: None,
        upkeep: None,
        costs: &[(1, Labor), (1, Metal)],
        movable: false,
        binding: Some(1),
        crosses: None,
        requires: None,
        readies: true,
    },
    Producible {
        kind: Yard,
        force: None,
        fuel: None,
        upkeep: None,
        costs: &[(1, Labor), (15, Metal)],
        movable: false,
        binding: Some(15),
        crosses: None,
        requires: None,
        readies: false,
    },
    // **`P-260`: one kind with a `resource` trait, not three that differ in one word.**
    // A store costs a labor and a metal and holds ten of what it was built for, which is
    // in *Where things are* rather than here because it is a fact about the kind.
    Producible {
        kind: Store,
        force: None,
        fuel: None,
        upkeep: None,
        costs: &[(1, Labor), (1, Metal)],
        movable: false,
        binding: Some(1),
        crosses: None,
        requires: None,
        readies: false,
    },
    Producible {
        kind: Ark,
        force: Some(2),
        fuel: Some(2),
        upkeep: None,
        costs: &[(3, Metal), (12, Energy), (2, Citizen)],
        movable: true,
        binding: Some(3),
        crosses: Some("orbit border"),
        requires: Some("a Yard"),
        readies: true,
    },
    Producible {
        kind: Pioneer,
        force: Some(2),
        fuel: Some(2),
        // `P-339`: a pioneer's Upkeep cell is empty, and a citizen is the only thing in
        // the release with one.
        upkeep: None,
        costs: &[(3, Metal), (6, Energy), (2, Citizen)],
        movable: true,
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

pub fn bounds_table() -> Vec<Vec<String>> {
    let mut rows = vec![header(&["Kind", "Bounded by"])];
    for kind in BOUND_ORDER {
        let bound = kind
            .bounded_by()
            .unwrap_or_else(|| panic!("{} is a place, and is not bounded in one", kind.name()));
        rows.push(vec![format!("**{}**", kind.name()), bound.to_string()]);
    }
    rows
}

pub fn units_table() -> Vec<Vec<String>> {
    let mut rows = vec![header(&[
        "Thing",
        "Force",
        "Fuel",
        "Upkeep",
        "Costs to produce",
        "Binding",
        "Crosses",
        "Requires",
        "Readies",
        "Movable",
    ])];
    for thing in PRODUCIBLE {
        rows.push(vec![
            format!("**{}**", thing.kind.name()),
            thing.force.map(|n| n.to_string()).unwrap_or_default(),
            thing.fuel.map(|n| n.to_string()).unwrap_or_default(),
            thing.upkeep_written(),
            thing.cost_written(),
            thing.binding.map(|n| n.to_string()).unwrap_or_default(),
            thing.crosses.unwrap_or_default().to_string(),
            thing.requires.unwrap_or_default().to_string(),
            if thing.readies { "yes" } else { "" }.to_string(),
            if thing.movable { "yes" } else { "" }.to_string(),
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
