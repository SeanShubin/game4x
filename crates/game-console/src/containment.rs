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
//! > **Entries are in the order their descriptions sort in, and the traits
//! > inside a description are in order of relevance**: `id` first, then every other trait
//! > alphabetically, then `occupied`, `free` and `capacity` last. So the same state is always
//! > the same bytes and a description is one string however it was built. **The middle is
//! > alphabetical because nothing has yet needed placing there**, and a trait leaves it by
//! > being named at one end or the other.
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
//! extractors and had built none, so nothing in its file said six; all three are there now -
//! `P-476`. [`Capacity`] below is still computed rather than written, and is a view
//! of what a deposit states plus what the territory holds - which is what
//! `spec/logistics.md` calls occupied and free.

use std::collections::BTreeMap;

use game_model::identity::Resource;
use game_model::territory::HOLDS;
use game_model::thing::{Kind, Thing, Trait};
use game_model::{Game, Phase};

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
    ///
    /// **An empty value is a trait named and not valued**, which `spec/console.md` allows on
    /// a declaration and nowhere else: *a trait of the kind is written with its value and a
    /// trait of the thing with its name*. A state always values what it names, because a state is
    /// about things rather than about kinds - so an empty value here is a declaration's, and
    /// [`state::declarations`] is the only reader that will produce one.
    pub traits: BTreeMap<String, String>,
}

/// The notation's own words, in the order a declaration leads with them.
///
/// > **A declaration leads with `name`, which says which thing it declares.** Then `of` and
/// > `family`, which say what that thing belongs to; then the notation's other words; then its
/// > traits, in the order above. **`name` is to a declaration what `id` is to a thing** - the
/// > difference is that an `id` tells one thing from its siblings and a `name` puts a word into
/// > the language.
///
/// `P-483`, and Sean's reasoning is what made it more than a preference: he asked how *which
/// one* applies to anything but `id`, since `id` exists to answer exactly that. `name:territory`
/// is what makes `territory` writable as a leading word anywhere; `id:1` makes `1` mean nothing
/// outside its own game.
///
/// **These are not traits and are never declared as any** - `P-481`. So they cannot collide
/// with the trait order below, and `no_notation_word_is_a_declared_trait` asserts that rather
/// than leaving it to be true by luck.
pub const NOTATION_WORDS: [&str; 5] = ["name", "of", "family", "admits", "kept"];

/// The traits the order names before the alphabetical middle, in order.
///
/// **Named here rather than written into the comparison**, so that
/// `every_trait_the_ordering_names_is_declared` can read them instead of carrying its own copy
/// of the list - a check against a second copy is checking the copy.
pub const ORDERED_FIRST: [&str; 1] = ["id"];

/// The traits the order names after the alphabetical middle, in order.
pub const ORDERED_LAST: [&str; 3] = ["occupied", "free", "capacity"];

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

    /// The traits in the order a reader wants to meet them, which is not the order the map
    /// holds them in.
    ///
    /// **Sean, 2026-09-13, stating the order of relevance:** *the most important is the type,
    /// second most important is id, the least important is capacity, second least is free,
    /// third least is occupied.* The kind is already first because it names the thing; `id`
    /// follows it, because which one it is, is the next question anybody has. What is left
    /// sorts by name in the middle, and the three that describe how full something is fall to
    /// the end in the order he gave.
    ///
    /// **Why those three are last is worth keeping.** `occupied`, `free` and `capacity` are
    /// one fact said three ways - the two derived from the third - so they are the part of a
    /// line a reader checks rather than reads. Alphabetically they led:
    /// `{deposit capacity:3 density:4 free:2 occupied:1 resource:food}` opened on the least
    /// interesting number and buried what the deposit is *of*.
    ///
    /// **A declaration's own words come first, ahead of any trait** - `P-483`. `name`, then
    /// `of` and `family`, then `admits` and `kept`, then the traits in the order above. A
    /// declaration is a description like any other, so this is one order over both rather than
    /// a second rule for declarations.
    ///
    /// **The determinism `spec/console.md` asks for is untouched.** This is a total order and a
    /// function of the description alone, so the same state is the same bytes and a description
    /// is one string however it was built.
    pub fn ordered(&self) -> Vec<(&String, &String)> {
        /// Where the alphabetical middle sits, so the two named ends fall either side of it.
        const MIDDLE: u8 = ORDERED_FIRST.len() as u8;

        // **A rank rather than a list of every trait**, because the middle is everything the
        // release declares and a list here would go stale the moment a trait is added -
        // silently, since an unlisted name would simply fall somewhere.
        // `every_trait_the_ordering_names_is_declared` guards the other direction, which is the
        // one a partial list is exposed to: a trait renamed, and an end of the order quietly
        // naming nothing.
        //
        // **It reads the name alone now.** It used to read the value too, so that a valueless
        // `id` on a declaration would not displace `name:territory`. `P-481` established that
        // `name` is not a trait at all and `P-483` put it first outright, so the narrowing
        // stood on nothing and is gone: `{kind name:territory family:place id biome control
        // nature}` leads with `name` because `name` ranks ahead of every trait, not because
        // `id` was held back.
        fn rank(name: &str) -> u8 {
            if let Some(at) = NOTATION_WORDS.iter().position(|it| *it == name) {
                return at as u8;
            }
            let traits = NOTATION_WORDS.len() as u8;
            if let Some(at) = ORDERED_FIRST.iter().position(|it| *it == name) {
                return traits + at as u8;
            }
            match ORDERED_LAST.iter().position(|it| *it == name) {
                Some(at) => traits + MIDDLE + 1 + at as u8,
                None => traits + MIDDLE,
            }
        }
        let mut out: Vec<(&String, &String)> = self.traits.iter().collect();
        out.sort_by(|(left, _), (right, _)| {
            rank(left).cmp(&rank(right)).then_with(|| left.cmp(right))
        });
        out
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
        for (name, value) in self.ordered() {
            out.push(' ');
            out.push_str(name);
            // **A named trait with no value writes as its name**, which is the form it was
            // read in. `{kind biome family:place id name:territory nature}` is a kind
            // declaring two traits of its own and three it has by name.
            if !value.is_empty() {
                out.push(':');
                out.push_str(value);
            }
        }
        out.push('}');
        out
    }

    /// Every word this description uses, which is what `P-284` is a rule about.
    ///
    /// In the same order as [`Description::written`], so that a vocabulary failure names the
    /// words in the order the reader will find them on the line.
    pub fn words(&self) -> Vec<String> {
        let mut out = vec![self.kind.to_string()];
        for (name, value) in self.ordered() {
            out.push(name.clone());
            // A trait named and not valued contributes its name and no value, rather than
            // an empty string that every vocabulary check would then have to admit.
            if !value.is_empty() {
                out.push(value.clone());
            }
        }
        out
    }
}

/// What a thing may contain, and how much of that it holds.
///
/// `spec/logistics.md`:
///
/// > What a thing may contain is a maximum **per kind, per family of kinds, or per kind
/// > carrying a particular value of a trait**. **Three names describe it and there are two
/// > facts**: its **capacity** for that kind, how much of that capacity is **occupied**, and
/// > how much is **free**. **Any two give the third, so only two are ever held** and nothing
/// > can disagree with anything. A capacity of four extractors is a maximum of four, so
/// > nothing a player builds ever crowds out something of another kind
///
/// # Which two are held, and why this type holds those two
///
/// **`P-476` named the three and the rule chose none of them**, which is deliberate: any two
/// give the third. This holds `free` and `used`, so `capacity()` is their sum.
///
/// **`P-374` and `P-474` are where that choice was made and `C-81` is what it cost.** It held
/// the total, with used and available derived - and the total is the one number the release
/// then said nothing records, so this printed a figure the specification had stopped having.
/// Sean found it reading `reports/recipes.md` while `C-81` sat in an outbox saying the same
/// thing.
///
/// **Two written numbers can disagree and these cannot**, because `used` is not a number this
/// holds at all: it is how many are there. That is the whole of why the choice matters.
///
/// **Neither field is in the data file, and they are absent for different reasons.** Saying
/// so is `Q-66`: one account made the omission sound like a rule being obeyed, and the other
/// half of it - the half that is a limitation - is the one a reader has to know.
///
/// - **`used` is not a number this holds** - it is how many are there - so there was never
///   anything to leave out. `P-476` took the word `derived` out of the release's column and
///   `P-477` took the rule that said so out of `spec/console.md`; the fact is unchanged.
/// - **`capacity` was the held one** when this was written, so nothing excused its absence.
///   It is out because a description is a flat map and a territory has a capacity per kind,
///   which the map form has no way to write. `C-46`.
///
/// Both are here because `S-54` asks for `used/total` on a collapsed summary line, and a
/// container that cannot say whether it is full defeats the reason for collapsing it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Capacity {
    /// The kind, or the kind carrying a trait value, that is bounded.
    pub of: Description,
    /// **How many more will fit, which is the one of the three that is stored** - `P-474`,
    /// and `spec/logistics.md` has said it since before this release: *what is stored is the
    /// room left*, used is what is there, and **nothing records the total**.
    ///
    /// **This held `total` until `P-474`, and that is `C-81`.** Storing the total beside the
    /// used count writes two of the three, and two written numbers can disagree; storing the
    /// room alone means nothing can, because used is not a number this holds at all - it is
    /// how many are there.
    pub room: u32,
    pub used: u32,
}

impl Capacity {
    /// How many more will fit, which is what is stored.
    pub fn available(&self) -> u32 {
        self.room
    }

    /// **Derived, because nothing records it** - `spec/logistics.md`. The two added.
    pub fn total(&self) -> u32 {
        self.room + self.used
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
    /// Not in the data file: `used` is not a number anything holds - it is how many are
    /// there - and `capacity()` is their sum. See [`Capacity`]; the reasons differ.
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
        if matches!(
            name,
            Trait::Ready | Trait::Spent | Trait::Defending | Trait::Met | Trait::Paid
        ) {
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
        Trait::Strength => "strength",
        Trait::Multiplier => "multiplier",
        Trait::Density => "density",
        // **Dashed, because a name is one word.** `spec/console.md` joins the words of a
        // name that needs more than one, which is the rule `P-328` applied to commands and
        // this applies to a trait. **`C-25` dissolves with it** - it reported the dump
        // printing `capacity` where the release declared `total capacity`, and there is one
        // name spelled one way now.
        Trait::Capacity => "capacity",
        Trait::Occupied => "occupied",
        Trait::Free => "free",
        Trait::From => "from",
        Trait::To => "to",
        // **These three are never written through here** - [`counts`] writes them under the
        // name the release gives them for the kind carrying them, which is why `readiness`
        // is not one of them: `P-399` made it a kind and `P-411` made it a count again.
        // Kept so the match is total and a new trait cannot be added without a name.
        Trait::Ready | Trait::Spent | Trait::Defending | Trait::Met | Trait::Paid => {
            "a count, written by `counts`"
        }
    }
}

/// The counts a thing of this kind carries, under the names `P-411` gives them.
///
/// How many of each action a thing may take in a turn, which every kind declares as one.
///
/// **The release declares it now, and this constant is checked against it rather than
/// standing in for it.** `P-459` was the gap: `refresh` read *at its maximum* six times and
/// the *Readies* column said `yes`, one cell for a citizen's three actions, where
/// `spec/turn.md` asks for a number. The column is the count per action now - *bearing 1,
/// defending 1, laboring 1* - so there is a declared maximum to read.
///
/// **It is one for all eight pairs**, and `the_readies_column_declares_the_maximum_this_reads`
/// in `game-console` holds this constant against the column. **That check is why this stays a
/// constant**: the model does not read the release, deliberately, so what keeps the two in
/// step is a check rather than a lookup - and the day a kind declares two of an action, the
/// check fails and names this.
///
/// **It was `MAXIMUM_PER_ACTION` until `P-459` landed.** The name was true and stopped being
/// true, which is the thing a doc comment cannot notice about itself - so the check that
/// noticed is the one that mattered rather than the comment that described.
pub const MAXIMUM_PER_ACTION: u32 = 1;

/// **One trait per action, and the name depends on the kind.** `spec/console.md`: a
/// description is a kind and **every trait of that thing**, and **no trait of the thing may be
/// left out** - `{citizen defending:1} -> 8` and `{citizen defending:0} -> 6`. So a count is
/// always written, with its value, rather than omitted when it is full.
///
/// **Absent means one**, which is how the model has always stored readiness and is why a thing
/// made this turn needs no trait to be able to act. That is storage; what a data file says is
/// the number either way.
///
/// # `met` is written here and is not a readiness
///
/// **What this function is for is the *always written* half**, and `met` needs exactly that:
/// `P-494` gives a nature a count of `0 or 1` and `spec/console.md` says no trait of a thing
/// may be left out, so `{nature met:0}` and `{nature met:1}` are two descriptions and neither
/// is spelled `{nature}`.
///
/// **Its default is the other way round, and that is the difference rather than an
/// inconsistency.** A readiness is absent when it is *full*, because `refresh` restores it and
/// a thing at rest can act. A nature is absent when it is *zero*, because `renew` clears it
/// and a nature at rest has had nothing spent on it. In both cases absent is what a committed
/// state holds - which is the property that makes a data file short, and it points opposite
/// ways for the two because the recipe that resets them does.
fn counts(kind: Kind, thing: &Thing) -> Vec<(&'static str, u32)> {
    let held = |name: Trait| thing.trait_of(name).unwrap_or(MAXIMUM_PER_ACTION);
    match kind {
        Kind::Citizen => vec![
            ("bearing", held(Trait::Spent)),
            ("defending", held(Trait::Defending)),
            ("laboring", held(Trait::Ready)),
            // **`paid` defaults to zero and the three above it default to one**, which is the
            // same split [`Trait::Met`] records: a readiness is absent when it is full and a
            // mark is absent when it is zero, because the recipe that resets each points the
            // other way. `spec/data/carries.4x` is what says a citizen carries this at all.
            ("paid", thing.trait_of(Trait::Paid).unwrap_or(0)),
        ],
        Kind::Extractor => vec![("working", held(Trait::Ready))],
        Kind::Nature => vec![("met", thing.trait_of(Trait::Met).unwrap_or(0))],
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
/// > limit**, and then it holds any number, and there is no free capacity to record because
/// > nothing can be short of it
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
/// **`game` is the fourth, and `P-433` made it a reading rather than an assumption.**
/// `spec/logistics.md` -> Containment now says it outright: **the game declares no limit, for
/// every kind** - *it contains everything, there is no room to record because nothing can be
/// short of it, and it is the one thing that is in nothing, so the tree has a root that no
/// rule has to except.*
///
/// **This was `C-68` and it stood open for five days.** `P-351` made `game` a kind and added
/// no row to *Where things are*, so the rule quoted above said a kind declaring no capacity
/// *contains nothing, and never can* - while `tree` put twelve territories inside it. This
/// lane proceeded and filed rather than drawing a root that never could hold what it holds.
///
/// **The release did not change and did not need to.** *Where things are* was right to omit
/// the game all along: a kind that declares no limit has no capacity row to write, which is a
/// different thing from a kind nothing has looked at.
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
        // **A territory's force of nature is no longer one of its traits** - `P-494` made
        // it a kind, so it appears below among the things this territory holds, as
        // `{nature met:0} -> 1`. The release's *Traits* table declares no `nature`, and a
        // description writing one would be a word the release does not have.
        let mut entry = Entry::leaf(
            Description::of(Kind::Territory)
                .with("id", place.id)
                .with("biome", place.biome.name()),
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
            let occupied = place.extractors_for(resource).len() as u32;
            held.push(Entry::leaf(
                Description::of(Kind::Deposit)
                    .with("resource", resource.name())
                    .with("density", offered.density)
                    // **Three names and two facts** - `spec/logistics.md`, `P-476`. Any two
                    // give the third, so the model holds the free capacity and how many are
                    // there; all three are written, because a description carries every trait
                    // of the thing and `P-477` removed the rule that kept a derived one out.
                    .with("capacity", offered.capacity)
                    .with("occupied", occupied)
                    .with("free", offered.capacity.saturating_sub(occupied)),
            ));
        }
        held.extend(game.units_on(place.id).into_iter().map(entry_for_unit));
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
                .filter(|unit| unit.location == game_model::Location::Orbit(place.id))
                .map(entry_for_unit)
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
        let from = game_model::TerritoryId::from_index(at);
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
    /// **A tree read back from a data file has no capacity**, because the file states none.
    /// `used` is not a number anything holds, and a capacity per kind is a thing a flat map
    /// cannot write. See [`Capacity`].
    ///
    /// **A deposit's own three are in the file** and this is about a territory's capacities,
    /// which are a different bound and still unwritten.
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
fn describe_unit(unit: &game_model::Unit) -> Description {
    Description::of(match unit.kind {
        game_model::UnitKind::Ark => Kind::Ark,
        game_model::UnitKind::Pioneer => Kind::Pioneer,
    })
    .with("id", unit.id)
    // **A unit's counts, under `P-411`'s names.** `moving` is what the model has stored as
    // `exhausted` all along - absent means one - and `defending` is `P-414`'s, which a unit
    // spends to stand. `fuel` left the description with `P-407`: it is a trait of the kind.
    .with("moving", u32::from(!unit.exhausted))
    .with("defending", u32::from(!unit.stood))
}

/// A unit, with what is in its bin.
///
/// **A bin is containment, so the fuel is an entry rather than a trait** - `P-485`, which began
/// as the report that this dump showed nothing inside a pioneer while the entity view said
/// `fuel 2`. One of the two was wrong about a thing that holds something, and it was this one.
///
/// > A mobile unit that moves over the ground has a bin for fuel. **It is built with that bin
/// > full, and the energy is paid where it is built.** Moving burns a unit of it, and one with
/// > an empty bin cannot move
///
/// **`fuel` stays a trait of the kind and is a different fact.** The Units table's `Fuel` is how
/// big the bin is - a pioneer's is 2 - and what is written here is what is in it now. A unit
/// that has moved once reads `{energy} -> 1` under a kind whose `fuel` is still 2.
///
/// **An empty bin writes nothing**, because `spec/console.md` says an entry is never zero. So a
/// pioneer that cannot move holds nothing, which is the same shape as a territory with no food.
fn entry_for_unit(unit: &game_model::Unit) -> Entry {
    let mut entry = Entry::leaf(describe_unit(unit));
    if unit.cells > 0 {
        entry.contents = vec![Entry {
            description: Description::of(Kind::Energy),
            quantity: unit.cells,
            contents: Vec::new(),
            capacity: Vec::new(),
        }];
    }
    entry
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
fn capacities_of(game: &Game, place: &game_model::Territory) -> Vec<Capacity> {
    let _ = game;
    let mut out = Vec::new();
    // **Room rather than total** - `P-474`. The caller states the bound because that is how
    // the release states it, and what is kept is what is left.
    let mut fixed = |kind: Kind, total: u32, used: u32| {
        out.push(Capacity {
            of: Description::of(kind),
            room: total.saturating_sub(used),
            used,
        });
    };
    fixed(Kind::Garrison, 1, place.count_of(Kind::Garrison));
    fixed(Kind::Yard, 1, place.yards());

    for resource in Resource::ALL {
        let offered = place.deposit(resource);
        out.push(Capacity {
            of: Description::of(Kind::Extractor).with("resource", resource.name()),
            room: offered
                .capacity
                .saturating_sub(place.extractors_for(resource).len() as u32),
            used: place.extractors_for(resource).len() as u32,
        });
        out.push(Capacity {
            of: Description::of(Kind::Store).with("resource", resource.name()),
            room: (place.store_capacity(resource) as u32)
                .saturating_sub(place.stores(resource) as u32),
            used: place.stores(resource) as u32,
        });
        // What the stores of a resource can hold between them. *Where things are*: a store
        // holds the resource it was built for, up to 10.
        out.push(Capacity {
            of: Description::of(Kind::from_resource(resource)),
            room: (place.stores(resource) as u32 * HOLDS).saturating_sub(place.store(resource)),
            used: place.store(resource),
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use game_model::identity::TerritoryId;

    fn a_world() -> Game {
        let mut game = Game::new();
        game.phase = Phase::Play;
        game.territories.push(game_model::Territory::empty(
            TerritoryId(1),
            game_model::Biome::Grassland,
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
            "{citizen bearing:1 defending:1 laboring:1 paid:0}"
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
                (
                    "{citizen bearing:1 defending:1 laboring:0 paid:0}".to_string(),
                    6
                ),
                (
                    "{citizen bearing:1 defending:1 laboring:1 paid:0}".to_string(),
                    8
                ),
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
        game.units.push(game_model::Unit::new(
            game_model::UnitId(1),
            game_model::UnitKind::Ark,
            TerritoryId(1),
        ));
        game.units.push(game_model::Unit::new(
            game_model::UnitId(2),
            game_model::UnitKind::Ark,
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
        game.territories.push(game_model::Territory::empty(
            TerritoryId(2),
            game_model::Biome::Ice,
        ));
        let mut landed = game_model::Unit::new(
            game_model::UnitId(1),
            game_model::UnitKind::Pioneer,
            TerritoryId(2),
        );
        landed.location = game_model::Location::On(TerritoryId(2));
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
        // `id` leads, which is Sean's order of relevance and not the alphabet - see
        // [`Description::ordered`].
        //
        // **And there is no `nature:` here since `P-494`**, which made it a kind: a
        // territory that resists with nothing holds no `nature` at all, where it used to
        // carry the number zero. An entry is never zero - `spec/console.md` - and this is
        // that rule reaching a fact that had been exempt from it by being a trait.
        assert_eq!(holder, vec!["{territory id:2 biome:ice}"]);
    }

    /// Capacity is the total and the used, and the used is counted rather than kept.
    #[test]
    fn a_territory_says_what_it_can_hold_and_what_it_holds() {
        let mut game = a_world();
        game.territories[0].deposits.insert(
            Resource::Food,
            game_model::Deposit {
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
        // **Room is what is kept and the total is the two added** - `P-474`. Asserted in
        // that order so a reader sees which of the three is stored.
        assert_eq!((food.room, food.used, food.total()), (2, 1, 3));
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
        let mut orphan = game_model::Unit::new(
            game_model::UnitId(1),
            game_model::UnitKind::Ark,
            TerritoryId(99),
        );
        orphan.location = game_model::Location::On(TerritoryId(99));
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
            game.units.push(game_model::Unit::new(
                game_model::UnitId(1),
                game_model::UnitKind::Ark,
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
