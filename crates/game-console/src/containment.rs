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

use std::collections::{BTreeMap, BTreeSet};

use game_model::engine::Game;
use game_model::notation::Row;

/// A kind, and every trait of the thing.
///
/// **Of the thing, and not of its kind** - `P-417`. Naming the kind has already said what a
/// trait of the kind is, so a description that repeated it would be saying the same thing
/// twice; `{garrison force:0}` was the entry that lost a word.
///
/// **The key of the map, so two things sharing one are indistinguishable.** That is the
/// point rather than a limitation: `{citizen laboring:1} -> 8` says there are eight of them
/// and that nothing in the state tells them apart. Where two things *are* distinguishable
/// they carry a trait that says so, and `id` is the trait that always does.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Description {
    pub kind: String,
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
pub use friendly_notation::NOTATION_WORDS;

/// The traits the order names before the alphabetical middle, in order.
///
/// **Named here rather than written into the comparison**, so that
/// `every_trait_the_ordering_names_is_declared` can read them instead of carrying its own copy
/// of the list - a check against a second copy is checking the copy.
pub use friendly_notation::ORDERED_FIRST;

/// The traits the order names after the alphabetical middle, in order.
pub use friendly_notation::ORDERED_LAST;

impl Description {
    /// **A kind is a relation's name, read from the data rather than chosen from a list.**
    ///
    /// **It was a `Kind` until 2026-09-29**, an enum of the old model's nine - which could not
    /// name a relation the data added, so a world with a new kind in it rendered as a world
    /// without one. `D-1` is that being impossible: what a thing may be is what
    /// `spec/data/schema.4x` declares.
    pub fn of(kind: &str) -> Self {
        Description {
            kind: kind.to_string(),
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
        out.push_str(&self.kind);
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
    ///
    /// **Signed since `P-504`**, which says `free <kind> of x` may be less than zero:
    /// `spec/console.md` - *its free capacity for that kind is the shortfall written as a
    /// negative number*, and the same paragraph says that shortfall is what the turn's end
    /// takes. It was a `u32` clamped by `saturating_sub`, so a place over its capacity read
    /// as having no room rather than as being short, and **the number the rule is about could
    /// not be written down at all**.
    pub room: i64,
    pub used: u32,
}

impl Capacity {
    /// How many more will fit, which is what is stored. Negative where it is over.
    pub fn available(&self) -> i64 {
        self.room
    }

    /// **Derived, because nothing records it** - `spec/logistics.md`. The two added.
    ///
    /// **Still what the container declares, even where the room is negative**: a store for
    /// ten holds ten whatever is piled in the territory, so `room + used` gives the ten back
    /// and the shortfall is the part that reads below zero.
    pub fn total(&self) -> i64 {
        self.room + self.used as i64
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

impl Entry {}

// **`ready(bool)` stood here and is deleted with the trait it wrote.** `P-399` removed
// `ready` from the release's *Traits* table: readiness is a kind, and what a data file says
// of one is which action it is `for`. Nothing writes yes-or-no any more.

/// The whole world as a tree of things, which is the shape `spec/logistics.md` says it has.
///
/// # Read from the rows, since 2026-09-29
///
/// **It walked `game.territories`, `units_on`, `room_in` and `adjacency` until the port**, which
/// is the old model's structure rather than the game's. The engine holds rows, so this reads
/// them: a thing says where it is with a `where` column, a place says which territory with `of`,
/// and what is left is in the game directly.
///
/// **What counts as a thing is `{state relation:...}`**, which the schema declares - so a
/// relation added to the data appears here with nothing edited, and one that is machinery for
/// the engine does not appear at all.
pub fn tree(game: &Game) -> Entry {
    let rows = game.rows().rows();
    let state: BTreeSet<&str> = rows
        .iter()
        .filter(|it| it.relation == "state")
        .filter_map(|it| it.value("relation"))
        .collect();
    // **Relation ids, because the foundation names nothing twice.** `{state relation:7}` says
    // the seventh relation holds things, and its name is what a row of it is called.
    let named: BTreeMap<&str, &str> = rows
        .iter()
        .filter(|it| it.relation == "relation")
        .filter_map(|it| Some((it.value("id")?, it.value("name")?)))
        .collect();
    let holds: BTreeSet<&str> = state
        .iter()
        .filter_map(|id| named.get(id).copied())
        .collect();

    let thing = |row: &Row| -> Entry {
        let mut description = Description::of(&row.relation);
        for (name, value) in &row.values {
            // **A quantity is how many, not a trait**, so it becomes the entry's count and
            // `spec/console.md`'s *an entry is never zero* is what drops the row if it is.
            // **`where` is where a thing appears and is never written**, which the same
            // document says: *where a thing is, is where it appears; nothing states its
            // container.*
            if name == "quantity" || name == "where" {
                continue;
            }
            description = description.with(name, value);
        }
        let quantity = row
            .value("quantity")
            .and_then(|it| it.parse::<u32>().ok())
            .unwrap_or(1);
        Entry {
            description,
            quantity,
            contents: Vec::new(),
            capacity: Vec::new(),
        }
    };

    let of_relation =
        |name: &str| -> Vec<&Row> { rows.iter().filter(|it| it.relation == name).collect() };

    // A place holds everything whose `where` names it.
    let mut place_entries: BTreeMap<String, Entry> = BTreeMap::new();
    for place in of_relation("place") {
        let Some(id) = place.value("id") else {
            continue;
        };
        let mut entry = thing(place);
        for row in rows {
            if !holds.contains(row.relation.as_str()) {
                continue;
            }
            if row.value("where") == Some(id) {
                entry.contents.push(thing(row));
            }
        }
        entry
            .contents
            .sort_by(|a, b| a.description.cmp(&b.description));
        place_entries.insert(id.to_string(), entry);
    }

    // A territory holds its places.
    let mut territory_entries: Vec<Entry> = Vec::new();
    for territory in of_relation("territory") {
        let Some(id) = territory.value("id") else {
            continue;
        };
        let mut entry = thing(territory);
        for place in of_relation("place") {
            if place.value("of") == Some(id)
                && let Some(held) = place.value("id").and_then(|it| place_entries.remove(it))
            {
                entry.contents.push(held);
            }
        }
        entry
            .contents
            .sort_by(|a, b| a.description.cmp(&b.description));
        territory_entries.push(entry);
    }

    // The planet holds its territories, and the game holds the planet and everything left.
    let mut root = Entry {
        description: Description::of("game"),
        quantity: 1,
        contents: Vec::new(),
        capacity: Vec::new(),
    };
    for planet in of_relation("planet") {
        let mut entry = thing(planet);
        entry.contents.append(&mut territory_entries);
        entry
            .contents
            .sort_by(|a, b| a.description.cmp(&b.description));
        root.contents.push(entry);
    }
    // **A world with no planet still has its territories**, so they are not lost when there is
    // nothing to hang them on - a state that dropped them would read as a world without any.
    root.contents.append(&mut territory_entries);

    // **What is left is in the game directly.** A capacity, a provision and an adjacency are
    // things the data states and are in no place, which is what `spec/logistics.md` means by
    // *the game is the one thing nothing holds*.
    for row in rows {
        if !holds.contains(row.relation.as_str()) {
            continue;
        }
        if row.relation == "place" || row.relation == "territory" || row.relation == "planet" {
            continue;
        }
        if row.value("where").is_some() {
            continue;
        }
        root.contents.push(thing(row));
    }
    root.contents
        .sort_by(|a, b| a.description.cmp(&b.description));
    root
}

impl Entry {
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

#[cfg(test)]
mod tests {
    use super::*;

    /// A world, stated as the rows the engine reads.
    ///
    /// **It built a `Game` and put things in it until the port.** The engine holds rows, so a
    /// test world is rows - which is also what makes these tests say what they assert: a line
    /// here is a line a player could have typed.
    fn world(lines: &[&str]) -> Game {
        let mut rows = game_model::foundation::rows();
        for line in lines {
            rows.push(
                game_model::notation::read(line)
                    .unwrap_or_else(|why| panic!("`{line}`: {why}"))
                    .pop()
                    .unwrap_or_else(|| panic!("`{line}` is no row")),
            );
        }
        Game::of(rows).unwrap_or_else(|why| panic!("the world does not hold: {why}"))
    }

    /// One territory with a surface, which is the least a thing can sit in.
    fn a_world(also: &[&str]) -> Game {
        let mut lines = vec!["{territory id:1}", "{place id:1 of:1 layer:surface}"];
        lines.extend_from_slice(also);
        world(&lines)
    }

    /// The entry for a place, found by its kind rather than by where it sits.
    ///
    /// **Not `contents[0]`.** Entries are in the order their descriptions sort in, so a test
    /// indexing by position is asserting the sort order while claiming to assert something else.
    fn place_in(tree: &Entry, id: u32) -> Entry {
        tree.walk()
            .into_iter()
            .find(|entry| {
                entry.description.kind == "place"
                    && entry.description.traits.get("id") == Some(&id.to_string())
            })
            .cloned()
            .unwrap_or_else(|| panic!("place {id} is in the tree"))
    }

    /// **Two things that cannot be told apart are one entry, and the quantity is the count.**
    ///
    /// `spec/console.md`: *each distinct description is its own entry, and an entry is never
    /// zero.*
    #[test]
    fn things_that_cannot_be_told_apart_are_one_entry_with_a_quantity() {
        let game = a_world(&["{citizen where:1 hungry:0 bearing:1 laboring:1 quantity:8}"]);
        let place = place_in(&tree(&game), 1);
        assert_eq!(place.contents.len(), 1, "one description, one entry");
        assert_eq!(place.contents[0].quantity, 8);
        assert_eq!(
            place.contents[0].description.written(),
            "{citizen bearing:1 hungry:0 laboring:1}"
        );
        assert!(
            place.contents[0].contents.is_empty(),
            "a citizen holds nothing; its counts are traits it carries"
        );
    }

    /// **A trait that tells two things apart makes two entries, at their own counts.**
    ///
    /// **This is the form `spec/console.md`'s own example uses**: `{citizen laboring:1} -> 8`
    /// and `{citizen laboring:0} -> 6`, never `{citizen} -> 14`.
    #[test]
    fn a_trait_that_tells_two_things_apart_makes_two_entries() {
        let game = a_world(&[
            "{citizen where:1 hungry:0 bearing:1 laboring:1 quantity:8}",
            "{citizen where:1 hungry:0 bearing:1 laboring:0 quantity:6}",
        ]);
        let held = place_in(&tree(&game), 1).contents;
        let written: Vec<(String, u32)> = held
            .iter()
            .map(|entry| (entry.description.written(), entry.quantity))
            .collect();
        assert_eq!(
            written,
            vec![
                ("{citizen bearing:1 hungry:0 laboring:0}".to_string(), 6),
                ("{citizen bearing:1 hungry:0 laboring:1}".to_string(), 8),
            ]
        );
    }

    /// **An entry is never zero**, which the data says by not holding a row for nothing.
    #[test]
    fn an_entry_is_never_zero() {
        let game = a_world(&["{metal where:1 quantity:3}"]);
        for entry in tree(&game).walk() {
            assert!(
                entry.quantity > 0,
                "`{}` is written with a quantity of nothing",
                entry.description.written()
            );
        }
    }

    /// **A thing carrying an `id` has a description no other thing shares**, so its quantity is
    /// always one.
    #[test]
    fn a_thing_with_an_id_is_always_one() {
        let game = a_world(&["{place id:2 of:1 layer:orbit}"]);
        let mut checked = 0;
        for entry in tree(&game).walk() {
            if entry.description.traits.contains_key("id") {
                assert_eq!(
                    entry.quantity,
                    1,
                    "`{}` carries an id and is written {} times",
                    entry.description.written(),
                    entry.quantity
                );
                checked += 1;
            }
        }
        assert!(checked >= 3, "only {checked} thing(s) carry an id");
    }

    /// **Where a thing is, is where it appears; nothing states its container.**
    #[test]
    fn nothing_states_its_container() {
        let game = a_world(&["{metal where:1 quantity:2}"]);
        let place = place_in(&tree(&game), 1);
        let metal = place
            .contents
            .iter()
            .find(|it| it.description.kind == "metal")
            .expect("the metal is in the place");
        assert!(
            !metal.description.traits.contains_key("where"),
            "`{}` says where it is, and where it is, is where it appears",
            metal.description.written()
        );
    }

    /// **A place holds what says it is there**, and a thing in another place is not in it.
    #[test]
    fn a_place_holds_what_says_it_is_there() {
        let game = world(&[
            "{territory id:1}",
            "{place id:1 of:1 layer:surface}",
            "{place id:2 of:1 layer:orbit}",
            "{metal where:1 quantity:2}",
            "{ark where:2 moving:1 gathering:1 quantity:1}",
        ]);
        let whole = tree(&game);
        let surface = place_in(&whole, 1);
        let orbit = place_in(&whole, 2);
        assert_eq!(surface.contents.len(), 1);
        assert_eq!(surface.contents[0].description.kind, "metal");
        assert_eq!(orbit.contents.len(), 1);
        assert_eq!(orbit.contents[0].description.kind, "ark");
    }

    /// **A place is in its territory and a territory is in the planet**, which is the tree
    /// `spec/logistics.md` describes: *every thing but the game is in another thing.*
    #[test]
    fn a_thing_is_in_what_holds_it_all_the_way_up() {
        let game = world(&[
            "{planet id:1 energy:3}",
            "{territory id:1}",
            "{place id:1 of:1 layer:surface}",
            "{metal where:1 quantity:2}",
        ]);
        let whole = tree(&game);
        assert_eq!(whole.description.kind, "game");
        let planet = whole
            .contents
            .iter()
            .find(|it| it.description.kind == "planet")
            .expect("the planet is in the game");
        let territory = planet
            .contents
            .iter()
            .find(|it| it.description.kind == "territory")
            .expect("the territory is in the planet");
        let place = territory
            .contents
            .iter()
            .find(|it| it.description.kind == "place")
            .expect("the place is in the territory");
        assert!(
            place
                .contents
                .iter()
                .any(|it| it.description.kind == "metal"),
            "the metal is in the place"
        );
    }

    /// **What is in no place is in the game**, which is what a capacity is.
    #[test]
    fn what_is_in_no_place_is_in_the_game() {
        let game = a_world(&[]);
        let whole = tree(&game);
        assert!(
            whole
                .contents
                .iter()
                .any(|it| it.description.kind == "territory"),
            "a world with no planet still has its territories"
        );
    }
}
