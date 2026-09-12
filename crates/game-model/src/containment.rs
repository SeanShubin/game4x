//! The state as a tree of things, which is the shape `spec/logistics.md` says it has.
//!
//! > Every thing is in the game, directly or through what contains it. **The game is the one
//! > thing that is in nothing**, so containment is a tree rather than a scattering
//!
//! And `spec/console.md` says how a tree is written down:
//!
//! > **Each distinct description is its own entry, and an entry is never zero.** A thing
//! > carrying an `id` has a description no other thing shares, so **its quantity is always
//! > one**. **Where a thing is, is where it appears**; nothing states its container.
//! > **Entries are in the order their descriptions sort in**, so the same state is always the
//! > same bytes.
//!
//! # Why this is in the model and not in the console
//!
//! **`C-37`: the arrow was pointing the wrong way.** `scenario/expected/play.4x` was built
//! by iterating `dump::tables`, so the one data file a person validates took its entire
//! vocabulary from a markdown presentation - table names became row names and column names
//! became field names. `docs/process.md` says a presentation is generated from data and is
//! never canonical, and here the data was generated from the presentation.
//!
//! So the tree is built here, from the state itself, and both the data file and the report
//! render *it*. Neither renders the other, and neither can quietly rename the other's words.
//!
//! # Both of a territory's numbers are on the deposit
//!
//! **`C-46` reported both as homeless, `P-322` housed one and `P-331` housed the other.** A
//! description is a flat map from a trait name to one value, and a territory had a `density`
//! per resource and a `total capacity` per kind - so neither could be a trait of a territory
//! *and* be written.
//!
//! **A deposit carries both now**, so a territory contains
//! `{deposit resource:food density:2 total-capacity:6} -> 1` and the release's `6 x 2` is in
//! the file as the two numbers it always meant.
//!
//! **That is what `C-53` was about and it is answered.** Territory 3 offered six food
//! extractors and had built none, so nothing in its file said six; both numbers are there
//! now. [`Capacity`] below is still computed rather than written, and is now a *derived* view
//! of what a deposit states plus what the territory holds - which is what
//! `spec/logistics.md` calls used and available.

use std::collections::BTreeMap;

use crate::identity::Resource;
use crate::territory::HOLDS;
use crate::thing::{Kind, Thing, Trait};
use crate::{Game, Phase};

/// A kind, and every trait of the thing.
///
/// **Of the thing, and not of its kind** - `P-417`. Naming the kind has already said what a
/// trait of the kind is, so a description that repeated it would be saying the same thing
/// twice; `{garrison force:0}` was the entry that lost a word.
///
/// **The key of the map, so two things sharing one are indistinguishable.** That is the
/// point rather than a limitation: `{citizen defending:1} -> 8` says there are eight of them
/// and that nothing in the state tells them apart. Where two things *are* distinguishable
/// they carry a trait that says so, and `id` is the trait that always does.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Description {
    pub kind: &'static str,
    /// Sorted by name, because a description is a map and a map has no order of its own.
    pub traits: BTreeMap<String, String>,
}

impl Description {
    pub fn of(kind: Kind) -> Self {
        Description {
            kind: kind.name(),
            traits: BTreeMap::new(),
        }
    }

    pub fn with(mut self, name: &str, value: impl ToString) -> Self {
        self.traits.insert(name.to_string(), value.to_string());
        self
    }

    /// The description as one word-per-field form: `{kind trait:value ...}`.
    ///
    /// **This is what entries sort on.** `spec/console.md` says entries are in the order
    /// their descriptions sort in, *so the same state is always the same bytes* - and the
    /// order a reader can check is the order of the text in front of them. Sorting on the
    /// struct would be an order only the program can see.
    pub fn written(&self) -> String {
        let mut out = String::from("{");
        out.push_str(self.kind);
        for (name, value) in &self.traits {
            out.push(' ');
            out.push_str(name);
            out.push(':');
            out.push_str(value);
        }
        out.push('}');
        out
    }

    /// Every word this description uses, which is what `P-284` is a rule about.
    pub fn words(&self) -> Vec<String> {
        let mut out = vec![self.kind.to_string()];
        for (name, value) in &self.traits {
            out.push(name.clone());
            out.push(value.clone());
        }
        out
    }
}

/// What a thing may contain, and how much of that it holds.
///
/// `spec/logistics.md`:
///
/// > What a thing may contain is a maximum **per kind, per family of kinds, or per kind
/// > carrying a particular value of a trait**. **What is stored is the room left**: how many
/// > more of that kind it could take. **Used capacity** is how many it holds, which is simply
/// > what is there, and **total capacity** is the two added. **Nothing records the total**, so
/// > nothing can disagree with it.
///
/// # `P-374` swapped which of the three is the stored one
///
/// **It used to be the total**, with used and available derived from it, and this type still
/// stores `total`. That is now the derived quantity and `room` is the stored one - the same
/// three numbers with a different one of them written down.
///
/// **The swap answers `C-46` rather than leaving it open.** That item said a total capacity
/// per kind cannot be written into a description, because a description is a flat map and a
/// territory has one total per kind. Room has exactly the same shape, so the difficulty does
/// not go away by itself - but `P-374` adds the half that does: **room is spent and given
/// back**, so it moves by the same rules as anything else a thing holds, and a thing that
/// holds room is a thing a description can carry.
///
/// **This type has not followed yet.** `C-81` carries what it costs to make it, and nothing
/// in the game is wrong meanwhile: the total and the room are each derivable from the other
/// wherever both ends are known, which is everywhere the model looks today.
///
/// **Neither field is in the data file, and they are absent for different reasons.** Saying
/// so is `Q-66`: one account made the omission sound like a rule being obeyed, and the other
/// half of it - the half that is a limitation - is the one a reader has to know.
///
/// - **`used` is derived**, so `spec/console.md` keeps it out: *a derived trait is never part
///   of one*. That is the rule working.
/// - **`total` was the stored one** when this was written, so nothing excused its absence.
///   It is out because a description is a flat map and a territory has a total capacity per
///   kind, which the map form has no way to write. `C-46`.
///
/// Both are here because `S-54` asks for `used/total` on a collapsed summary line, and a
/// container that cannot say whether it is full defeats the reason for collapsing it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Capacity {
    /// The kind, or the kind carrying a trait value, that is bounded.
    pub of: Description,
    pub total: u32,
    pub used: u32,
}

impl Capacity {
    pub fn available(&self) -> u32 {
        self.total.saturating_sub(self.used)
    }
}

/// One entry of a containment map: a description, how many there are, and what they contain.
///
/// **Quantity and contents together, because grouping by description is what the form
/// says.** Two things with one description are one entry, so they had better be holding the
/// same thing - and if they are not, the state is one the form cannot write down. [`group`]
/// refuses rather than picking a winner.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    pub description: Description,
    /// Never zero. `spec/console.md`: *an entry is never zero.*
    pub quantity: u32,
    pub contents: Vec<Entry>,
    /// Not in the data file. `used` is derived and `total` is stored - see [`Capacity`] for
    /// why each is absent, because the reasons are not the same one.
    pub capacity: Vec<Capacity>,
}

impl Entry {
    fn leaf(description: Description) -> Self {
        Entry {
            description,
            quantity: 1,
            contents: Vec::new(),
            capacity: Vec::new(),
        }
    }
}

/// Group things that share a description into one entry each, in description order.
///
/// **It refuses when two things share a description and contain different things**, because
/// that state has no written form at all: the description is the key of the map, so the two
/// would have to be one entry, and one entry has one set of contents. Refusing is the honest
/// answer - the alternative is a file that reads back as a state the game was never in.
///
/// `C-34` is the record of what was writable before this existed, held open so the claim
/// this makes can be measured against a population rather than asserted.
fn group(things: Vec<Entry>) -> Vec<Entry> {
    let mut by_description: BTreeMap<String, Entry> = BTreeMap::new();
    for thing in things {
        let key = thing.description.written();
        match by_description.get_mut(&key) {
            Some(already) => {
                assert_eq!(
                    already.contents, thing.contents,
                    "two things are described by {key} and contain different things, which \
                     the map form cannot write down - a description is the key of the map"
                );
                // `spec/logistics.md`: **there is never a quantity of a thing with an
                // `id`** - it is one thing, and anything that holds it holds exactly it.
                //
                // **`C-34`'s second entry, and refusing is the whole of what changed.**
                // Two units sharing one id were writable before this and are writable
                // still: `Game.units` is a `Vec` and nothing in it is keyed. What they can
                // no longer do is produce a file, because grouping them would write
                // `{ark id:1} -> 2` - a plausible line stating a rule the specification
                // forbids, which is worse than a crash and quieter.
                assert!(
                    !already.description.traits.contains_key("id"),
                    "{key} twice, and there is never a quantity of a thing with an `id`"
                );
                already.quantity += 1;
            }
            None => {
                by_description.insert(key, thing);
            }
        }
    }
    by_description.into_values().collect()
}

// **`ready(bool)` stood here and is deleted with the trait it wrote.** `P-399` removed
// `ready` from the release's *Traits* table: readiness is a kind, and what a data file says
// of one is which action it is `for`. Nothing writes yes-or-no any more.

/// A thing held in a territory, described.
///
/// **Built from what the model stores and nothing else.** A trait the release declares for a
/// kind and the model does not keep is missing from the description, which is a divergence
/// rather than a decision - `tests/descriptions.rs` names every one of them and fails when
/// one is repaired or another appears.
fn describe(thing: &Thing) -> Description {
    // **A `Thing` cannot hold anything, which is why nothing is asserted here.** It carried a
    // list of contained things that nothing wrote; `C-66` deleted it, so a thing appearing in
    // the file as holding nothing is now a fact about the type rather than a guard that has
    // to keep firing. `Q-66`'s review found this path while the field existed, and what it
    // found - a state written down wrongly rather than refused - is unreachable now rather
    // than merely checked. The tree's depth is built below, from `Territory::held` and
    // `Game::units`.
    let mut description = Description::of(thing.kind);
    for (name, value) in &thing.traits {
        // **A count is written by [`counts`] below, under the name its kind gives it.**
        // `P-411` names them apart - `moving` of a unit, `laboring` of a citizen, `working` of
        // an extractor - so one variant cannot carry one name, and the generic loop cannot
        // write them.
        if matches!(name, Trait::Ready | Trait::Spent | Trait::Defending) {
            continue;
        }
        let written = match name {
            // The `resource` trait is stored as an index into `Resource::ALL`, and a data
            // file says the resource. `P-284`: every word is a kind, a trait, or one of a
            // trait's values, and `0` is none of the three.
            Trait::Resource => Resource::ALL
                .get(*value as usize)
                .map(|resource| resource.name().to_string())
                .unwrap_or_else(|| value.to_string()),
            _ => value.to_string(),
        };
        description = description.with(trait_name(*name), written);
    }
    for (name, value) in counts(thing.kind, thing) {
        description = description.with(name, value);
    }
    description
}

// **`P-411` undid `P-399` and `readiness()` went with it.** That function rendered a thing's
// tokens as things it contained, which is what made two citizens differing only in readiness
// share a description - `C-90`, filed by this lane and the reason the release turned back. A
// count the thing carries is part of its description again, so `describe` writes it and
// nothing synthesises contents.

/// What a trait is called in a data file.
///
/// **The release's name, not the variant's.** `founded` is the standing example of what
/// happens when a program names a thing one way and the release another: the dump printed a
/// field the release had never declared, and nothing compared the two lists in either
/// direction until something went looking.
pub fn trait_name(name: Trait) -> &'static str {
    match name {
        Trait::Resource => "resource",
        Trait::Force => "force",
        Trait::Multiplier => "multiplier",
        Trait::Density => "density",
        // **Dashed, because a name is one word.** `spec/console.md` joins the words of a
        // name that needs more than one, which is the rule `P-328` applied to commands and
        // this applies to a trait. **`C-25` dissolves with it** - it reported the dump
        // printing `capacity` where the release declared `total capacity`, and there is one
        // name spelled one way now.
        Trait::TotalCapacity => "total-capacity",
        Trait::From => "from",
        Trait::To => "to",
        // **These three are never written through here** - [`counts`] writes them under the
        // name the release gives them for the kind carrying them, which is why `readiness`
        // is not one of them: `P-399` made it a kind and `P-411` made it a count again.
        // Kept so the match is total and a new trait cannot be added without a name.
        Trait::Ready | Trait::Spent | Trait::Defending => "a count, written by `counts`",
    }
}

/// The counts a thing of this kind carries, under the names `P-411` gives them.
///
/// **One trait per action, and the name depends on the kind.** `spec/console.md`: a
/// description is a kind and **every trait of that thing**, and **no trait of the thing may be
/// left out** - `{citizen defending:1} -> 8` and `{citizen defending:0} -> 6`. So a count is
/// always written, with its value, rather than omitted when it is full.
///
/// **Absent means one**, which is how the model has always stored readiness and is why a thing
/// made this turn needs no trait to be able to act. That is storage; what a data file says is
/// the number either way.
fn counts(kind: Kind, thing: &Thing) -> Vec<(&'static str, u32)> {
    let held = |name: Trait| thing.trait_of(name).unwrap_or(1);
    match kind {
        Kind::Citizen => vec![
            ("bearing", held(Trait::Spent)),
            ("defending", held(Trait::Defending)),
            ("laboring", held(Trait::Ready)),
        ],
        Kind::Extractor => vec![("working", held(Trait::Ready))],
        _ => Vec::new(),
    }
}

/// Whether a kind may contain things at all.
///
/// `spec/logistics.md` draws the line and says why it matters:
///
/// > **A kind declares one of three things about what it may hold.** It may declare **no
/// > capacity**, and then it holds nothing of that sort and never can. It may declare a
/// > **limit**, and then it holds up to that many and may happen to be empty - so a thing
/// > holding nothing today is not thereby a thing that never could. Or it may declare **no
/// > limit**, and then it holds any number, and there is no room to record because nothing can
/// > be short of it
///
/// **So this is a fact about the kind and not a count of what is there.** A reader who
/// cannot tell *empty* from *never* is being shown the opposite of the rule, and the only
/// place the difference can come from is here.
///
/// **`P-391` made it three cases where it was two, and this function still answers two.** It
/// says whether a kind may hold anything at all, which separates *no capacity* from the other
/// two and is all any caller here asks. **The third case is the one to watch**: a kind
/// declaring *no limit* holds any number and has no room to record, so anything printing a
/// `used/total` for one would be printing a total the specification says does not exist.
/// `crates/game-console/src/tree.rs` is where that would show.
///
/// Three kinds, from `releases/first-release.md` -> *Where things are*, which gives this
/// release exactly three sorts of capacity: **a territory**, for the kinds it has total
/// capacity for; **a store**, for the resource it was built for; and **an orbit**, which
/// *holds units and nothing else*.
///
/// **A unit is not among them, and that is the release's own arrangement rather than an
/// omission.** *Where things are* gives a unit's tank as a sort of capacity, and the *Traits*
/// table gives `fuel` as *how much energy its tank holds* - so in this release the tank is a
/// number on the unit rather than a thing with a description, and a unit contains nothing.
/// **`game` is the fourth, and it is an assumption rather than a reading - `C-68`.** `P-351`
/// made `game` a kind and added no row to *Where things are*, so the rule quoted above says a
/// kind declaring no capacity *contains nothing, and never can* - while `tree` puts twelve
/// territories inside it and `spec/logistics.md` requires exactly that. **The release and the
/// code cannot both be read literally**, and drawing the root as a thing that never could hold
/// would contradict the tree it is the root of. So this proceeds, and the question is filed.
pub fn may_contain(kind: Kind) -> bool {
    matches!(
        kind,
        Kind::Territory | Kind::Store | Kind::Orbit | Kind::Game
    )
}

/// Whether a kind readies.
///
/// `releases/first-release.md` -> *Units and structures* has a **Readies** column, and says
/// underneath it that **nothing outside this table readies**. So this is four kinds, and
/// `tests/descriptions.rs` reads the column rather than trusting the list.
pub fn readies(kind: Kind) -> bool {
    matches!(
        kind,
        Kind::Citizen | Kind::Extractor | Kind::Ark | Kind::Pioneer
    )
}

/// The whole state as one tree, the game at the root.
///
/// **`turn` is not here, and that is `S-48`.** `P-288` says `phase` is a declared trait and
/// `turn` is not, so the game's description is `{game phase:play}` and nothing else. The
/// counts that used to sit beside it - `territories`, `units` - are the contents, which is
/// where a count of things belongs.
///
/// **`game` is a kind now, and `P-351` is what made it one.** `spec/logistics.md` needed a
/// thing that is in nothing for containment to be a tree, and the *Kinds* table did not declare
/// one - so this function wrote the word by hand, and `C-46` reported it as a word the code
/// writes anyway. The root is [`Kind::Game`] rather than a string, so the one place that
/// decides how a kind is spelled decides this one too.
pub fn tree(game: &Game) -> Entry {
    let mut root = Entry::leaf(Description::of(Kind::Game)).with_trait(
        "phase",
        match game.phase {
            Phase::Design => "design",
            Phase::Play => "play",
        },
    );

    let mut children = Vec::new();
    for place in &game.territories {
        let mut entry = Entry::leaf(
            Description::of(Kind::Territory)
                .with("id", place.id)
                .with("biome", place.biome.name())
                .with("nature", place.force_of_nature),
        );
        let mut held: Vec<Entry> = place
            .held
            .iter()
            .map(|t| Entry::leaf(describe(t)))
            .collect();
        // **`P-322`: a deposit is a thing, so a territory contains one per resource its
        // ground offers.** `{deposit resource:food density:4} -> 1`, which is where `density`
        // lives now that it is a trait of a deposit rather than of a territory.
        //
        // **Only where the ground offers something.** A resource with no deposit is a
        // territory that has none of it, and an entry is never zero - `spec/console.md`. The
        // release says territory 6 has no metal, and the file says so by not mentioning it.
        for resource in Resource::ALL {
            let offered = place.deposit(resource);
            if offered.capacity == 0 && offered.density == 0 {
                continue;
            }
            held.push(Entry::leaf(
                Description::of(Kind::Deposit)
                    .with("resource", resource.name())
                    .with("density", offered.density)
                    .with("total-capacity", offered.capacity),
            ));
        }
        held.extend(
            game.units_on(place.id)
                .into_iter()
                .map(|unit| Entry::leaf(describe_unit(unit))),
        );
        entry.contents = group(held);
        entry.capacity = capacities_of(game, place);
        children.push(entry);

        // **An orbit is a place beside its territory, not inside it.** `spec/orbit.md`: *a
        // planet has an orbit above each of its territories*, and *an orbit is next to the
        // territory below it* - next to, which is adjacency rather than containment.
        //
        // **It carries the id of the territory it is above**, which is derivation rather
        // than a second copy: there is exactly one orbit per territory, so naming the pair
        // twice is what would be able to disagree. Twelve orbits holding different units
        // have to be named individually, which is what `releases/first-release.md` says an
        // `id` is for.
        let mut above = Entry::leaf(Description::of(Kind::Orbit).with("id", place.id));
        above.contents = group(
            game.units
                .iter()
                .filter(|unit| unit.location == crate::Location::Orbit(place.id))
                .map(|unit| Entry::leaf(describe_unit(unit)))
                .collect(),
        );
        children.push(above);
    }

    // **`P-334`: the game holds the adjacencies, beside its places rather than inside them.**
    // `spec/logistics.md`: *a thing says which of the things in it are next to which. That is
    // a fact about the container rather than about its contents.*
    //
    // **Written once.** Adjacency is symmetric - `Game.adjacency` says so in its own comment -
    // so `{adjacency from:1 to:2}` and `{adjacency from:2 to:1}` are one fact, and the lower
    // id is `from`. Thirty entries for a tiny planet rather than sixty, and the same state is
    // the same bytes.
    //
    // **Orbital adjacency is not here**, because the release derives it: *an orbit is next to
    // its territory and to the orbits above that territory's neighbours, so stating it would
    // be a second copy that can disagree.*
    for (at, near) in game.adjacency.iter().enumerate() {
        let from = crate::TerritoryId::from_index(at);
        for to in near {
            if from.0 >= to.0 {
                continue;
            }
            children.push(Entry::leaf(
                Description::of(Kind::Adjacency)
                    .with("from", from)
                    .with("to", *to),
            ));
        }
    }

    root.contents = group(children);

    // **Every unit is somewhere, and this is what says so.**
    //
    // **`C-34`'s first entry.** `Location::On(TerritoryId(99))` constructs and refers to
    // nothing; `game.rs` guards the ids it is handed and the struct admits any number. The
    // tree is built by asking each place what is on it, so an orphan matches no place and
    // **vanishes** - the file would say there is no Ark while the game held one, which is a
    // data file that is quietly wrong rather than one that fails.
    //
    // Measured rather than assumed: before this line, a game with one orphaned Ark produced
    // a tree with none, and nothing said so.
    let placed = root
        .walk()
        .into_iter()
        .filter(|entry| {
            entry.description.kind == Kind::Ark.name()
                || entry.description.kind == Kind::Pioneer.name()
        })
        .map(|entry| entry.quantity as usize)
        .sum::<usize>();
    assert_eq!(
        placed,
        game.units.len(),
        "{} units are in the game and {placed} are in a place - one is somewhere the tree \
         cannot reach, and writing it out would lose it silently",
        game.units.len()
    );

    root
}

impl Entry {
    fn with_trait(mut self, name: &str, value: impl ToString) -> Self {
        self.description = self.description.with(name, value);
        self
    }

    /// Every entry in the tree, the root first and each thing before what it contains.
    pub fn walk(&self) -> Vec<&Entry> {
        let mut out = vec![self];
        for held in &self.contents {
            out.extend(held.walk());
        }
        out
    }

    /// The same tree with every capacity dropped.
    ///
    /// **A tree read back from a data file has no capacity**, because the file states none -
    /// `used` by the rule that keeps derived traits out, and `total` because the map form
    /// cannot hold a stored trait a thing has one of per kind. See [`Capacity`].
    ///
    /// So a round trip is compared against this rather than against the tree the model built,
    /// **and what that concedes is the second of those two.** The comparison is text against
    /// tree, not text against the game: reading the file back cannot rebuild a territory's
    /// numbers, because they were never written. `C-46`.
    pub fn contained(&self) -> Entry {
        Entry {
            description: self.description.clone(),
            quantity: self.quantity,
            contents: self.contents.iter().map(Entry::contained).collect(),
            capacity: Vec::new(),
        }
    }

    /// How many of a description this entry holds directly.
    pub fn holding(&self, description: &Description) -> u32 {
        self.contents
            .iter()
            .filter(|entry| &entry.description == description)
            .map(|entry| entry.quantity)
            .sum()
    }
}

/// A unit, described. Its `fuel` is what its tank holds.
///
/// `releases/first-release.md` -> *Traits*: **fuel**, of a unit, *how much energy its tank
/// holds*. So the number is the trait, and the tank is not a separate thing in this
/// release: *Where things are* gives a unit's tank as one of the three sorts of capacity
/// rather than as something with a description of its own.
fn describe_unit(unit: &crate::Unit) -> Description {
    Description::of(match unit.kind {
        crate::UnitKind::Ark => Kind::Ark,
        crate::UnitKind::Pioneer => Kind::Pioneer,
    })
    .with("id", unit.id)
    // **A unit's counts, under `P-411`'s names.** `moving` is what the model has stored as
    // `exhausted` all along - absent means one - and `defending` is `P-414`'s, which a unit
    // spends to stand. `fuel` left the description with `P-407`: it is a trait of the kind.
    .with("moving", u32::from(!unit.exhausted))
    .with("defending", u32::from(!unit.stood))
}

/// What a territory may contain, per kind, with what it holds now.
///
/// Every bound is read from `releases/first-release.md` -> *What bounds a kind*, and the
/// two that are not a fixed number are read from the territory:
///
/// - **extractor** - *a capacity, from Territory resources*, which is per resource
/// - **store** - *as many as the extractors of its resource*
///
/// **The kinds bounded by food are not here**, because a bound through upkeep is not a
/// containment capacity: `citizen` is bounded by *the food produced here, through upkeep*
/// and a territory does not declare a number for it.
fn capacities_of(game: &Game, place: &crate::Territory) -> Vec<Capacity> {
    let _ = game;
    let mut out = Vec::new();
    let mut fixed = |kind: Kind, total: u32, used: u32| {
        out.push(Capacity {
            of: Description::of(kind),
            total,
            used,
        });
    };
    fixed(Kind::Garrison, 1, place.count_of(Kind::Garrison));
    fixed(Kind::Yard, 1, place.yards());

    for resource in Resource::ALL {
        let offered = place.deposit(resource);
        out.push(Capacity {
            of: Description::of(Kind::Extractor).with("resource", resource.name()),
            total: offered.capacity,
            used: place.extractors_for(resource).len() as u32,
        });
        out.push(Capacity {
            of: Description::of(Kind::Store).with("resource", resource.name()),
            total: place.store_capacity(resource) as u32,
            used: place.stores(resource) as u32,
        });
        // What the stores of a resource can hold between them. *Where things are*: a store
        // holds the resource it was built for, up to 10.
        out.push(Capacity {
            of: Description::of(Kind::from_resource(resource)),
            total: place.stores(resource) as u32 * HOLDS,
            used: place.store(resource),
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identity::TerritoryId;

    fn a_world() -> Game {
        let mut game = Game::new();
        game.phase = Phase::Play;
        game.territories.push(crate::Territory::empty(
            TerritoryId(1),
            crate::Biome::Grassland,
        ));
        game
    }

    /// The entry for a territory, found by its kind.
    ///
    /// **Not `contents[0]`.** Entries are in the order their descriptions sort in, and
    /// `{orbit ...}` sorts before `{territory ...}` - so a test indexing by position is
    /// asserting the sort order while claiming to assert something else.
    fn territory_in(tree: &Entry, id: u32) -> Entry {
        tree.contents
            .iter()
            .find(|entry| {
                entry.description.kind == "territory"
                    && entry.description.traits.get("id") == Some(&id.to_string())
            })
            .cloned()
            .unwrap_or_else(|| panic!("territory {id} is in the tree"))
    }

    /// Two things with one description are one entry, and the quantity is the count.
    ///
    /// **The counts are back in the description since `P-411`, one per action.** A citizen
    /// carries `bearing`, `defending` and `laboring`, each `0 or 1`, and `spec/console.md`
    /// says no trait of the thing may be left out - so a count is written with its value
    /// rather than omitted when it is full.
    #[test]
    fn things_that_cannot_be_told_apart_are_one_entry_with_a_quantity() {
        let mut game = a_world();
        game.territories[0].put(Kind::Citizen, 8);
        let territory = territory_in(&tree(&game), 1);
        assert_eq!(territory.contents.len(), 1, "one description, one entry");
        assert_eq!(territory.contents[0].quantity, 8);
        assert_eq!(
            territory.contents[0].description.written(),
            "{citizen bearing:1 defending:1 laboring:1}"
        );
        assert!(
            territory.contents[0].contents.is_empty(),
            "a citizen holds nothing; its counts are traits it carries"
        );
    }

    /// A trait that distinguishes two things makes two entries, at their own counts.
    ///
    /// **`C-90` is answered and this test is the evidence.** For one afternoon readiness was a
    /// kind a thing held, and two citizens differing only in it had the same description and
    /// different contents - which a map keyed on the description cannot say. This lane filed
    /// that; `P-411` turned readiness back into a count the thing carries, and the two
    /// citizens are two descriptions again.
    ///
    /// **So the state that had no written form has one**, and it is the form
    /// `spec/console.md`'s own example uses: `{citizen defending:1} -> 8` and
    /// `{citizen defending:0} -> 6`, never `{citizen} -> 14`.
    #[test]
    fn a_trait_that_tells_two_things_apart_makes_two_entries() {
        let mut game = a_world();
        game.territories[0].put(Kind::Citizen, 14);
        for thing in game.territories[0]
            .held
            .iter_mut()
            .filter(|t| t.kind == Kind::Citizen)
            .take(6)
        {
            thing.set(Trait::Ready, 0);
        }

        let held = territory_in(&tree(&game), 1).contents;
        let written: Vec<(String, u32)> = held
            .iter()
            .map(|entry| (entry.description.written(), entry.quantity))
            .collect();
        assert_eq!(
            written,
            vec![
                ("{citizen bearing:1 defending:1 laboring:0}".to_string(), 6),
                ("{citizen bearing:1 defending:1 laboring:1}".to_string(), 8),
            ],
            "eight that can labor and six that cannot, in the order their descriptions sort in"
        );
    }

    /// No entry is zero: a kind with none of it here is absent rather than present at nought.
    #[test]
    fn an_entry_is_never_zero() {
        let game = a_world();
        let tree = tree(&game);
        assert!(
            tree.walk().iter().all(|entry| entry.quantity > 0),
            "`spec/console.md`: an entry is never zero"
        );
        assert!(
            territory_in(&tree, 1).contents.is_empty(),
            "an empty territory contains nothing, rather than nought of thirteen kinds"
        );
    }

    /// A thing with an `id` has a description no other thing shares, so its quantity is one.
    #[test]
    fn a_thing_with_an_id_is_always_one() {
        let mut game = a_world();
        game.units.push(crate::Unit::new(
            crate::UnitId(1),
            crate::UnitKind::Ark,
            TerritoryId(1),
        ));
        game.units.push(crate::Unit::new(
            crate::UnitId(2),
            crate::UnitKind::Ark,
            TerritoryId(1),
        ));
        let orbit = tree(&game)
            .contents
            .iter()
            .find(|entry| entry.description.kind == "orbit")
            .cloned()
            .expect("the orbit above territory 1");
        assert_eq!(orbit.contents.len(), 2, "two arks, two entries");
        let identified = orbit
            .contents
            .iter()
            .filter(|entry| entry.description.traits.contains_key("id"))
            .count();
        assert_eq!(identified, 2, "and both carry an id");
        assert!(
            orbit.contents.iter().all(|entry| entry.quantity == 1),
            "there is never a quantity of a thing with an `id`"
        );
    }

    /// Where a thing is, is where it appears - and nothing says where it is.
    ///
    /// **The rule this checks is the one `P-311` broke and `P-320` restored.** A unit used to
    /// name its container with `in-kind` and `in-id`; `spec/console.md` says nothing states
    /// its container, so the only thing that says where a unit is, is which entry it is in.
    #[test]
    fn nothing_states_its_container() {
        let mut game = a_world();
        game.territories
            .push(crate::Territory::empty(TerritoryId(2), crate::Biome::Ice));
        let mut landed =
            crate::Unit::new(crate::UnitId(1), crate::UnitKind::Pioneer, TerritoryId(2));
        landed.location = crate::Location::On(TerritoryId(2));
        game.units.push(landed);

        let tree = tree(&game);
        let mut words = Vec::new();
        for entry in tree.walk() {
            words.extend(entry.description.traits.keys().cloned());
        }
        for named in ["in-kind", "in-id", "place", "territory", "location"] {
            assert!(
                !words.contains(&named.to_string()),
                "`{named}` states a container, and nothing may"
            );
        }

        // And the pioneer is found by looking inside territory 2 and nowhere else.
        let holder: Vec<&str> = tree
            .contents
            .iter()
            .filter(|entry| {
                entry
                    .contents
                    .iter()
                    .any(|c| c.description.kind == "pioneer")
            })
            .map(|entry| entry.description.written())
            .map(|w| Box::leak(w.into_boxed_str()) as &str)
            .collect();
        assert_eq!(holder, vec!["{territory biome:ice id:2 nature:0}"]);
    }

    /// Capacity is the total and the used, and the used is counted rather than kept.
    #[test]
    fn a_territory_says_what_it_can_hold_and_what_it_holds() {
        let mut game = a_world();
        game.territories[0].deposits.insert(
            Resource::Food,
            crate::Deposit {
                capacity: 3,
                density: 4,
            },
        );
        game.territories[0].add_extractor(Resource::Food);
        let territory = territory_in(&tree(&game), 1);
        let food = territory
            .capacity
            .iter()
            .find(|c| c.of.written() == "{extractor resource:food}")
            .expect("a capacity for food extractors");
        assert_eq!((food.total, food.used, food.available()), (3, 1, 2));
    }

    /// A unit in a place the tree cannot reach stops the tree rather than vanishing from it.
    ///
    /// **`C-34`'s first entry, exhibited.** It said an orphan *holds unconditionally*, and it
    /// still does - the struct admits any territory number. What changed is what happens
    /// next: the state has no written form now, where before it had a plausible one that was
    /// missing a unit.
    #[test]
    #[should_panic(expected = "cannot reach")]
    fn a_unit_in_a_place_that_is_not_there_has_no_written_form() {
        let mut game = a_world();
        let mut orphan = crate::Unit::new(crate::UnitId(1), crate::UnitKind::Ark, TerritoryId(99));
        orphan.location = crate::Location::On(TerritoryId(99));
        game.units.push(orphan);
        tree(&game);
    }

    /// Two things with one `id` have no written form either.
    ///
    /// **`C-34`'s second entry, and its condition is met.** The entry was conditional on
    /// identity becoming positional; it did not - `Thing` still has no id and `game.rs` still
    /// selects units by `UnitId`. So the state is writable exactly as it was, and what it can
    /// no longer do is be written down: grouping produces `{ark id:1} -> 2`, which
    /// `spec/logistics.md` forbids outright.
    #[test]
    #[should_panic(expected = "never a quantity of a thing with an `id`")]
    fn two_things_sharing_an_id_have_no_written_form() {
        let mut game = a_world();
        for _ in 0..2 {
            game.units.push(crate::Unit::new(
                crate::UnitId(1),
                crate::UnitKind::Ark,
                TerritoryId(1),
            ));
        }
        tree(&game);
    }

    /// Two things sharing a description and holding different things has no written form.
    ///
    /// **The refusal is the point.** A description is the key of the map, so the two would
    /// have to be one entry and one entry has one set of contents - picking either would
    /// write a file that reads back as a state the game was never in.
    #[test]
    #[should_panic(expected = "the map form cannot write down")]
    fn two_things_with_one_description_holding_different_things_is_refused() {
        let one = Entry::leaf(Description::of(Kind::Store).with("resource", "food"));
        let mut other = one.clone();
        other.contents = vec![Entry::leaf(Description::of(Kind::Food))];
        group(vec![one, other]);
    }
}
