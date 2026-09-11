//! The rules as a Petri net, read out of the release's *Recipes* table - `S-87`.
//!
//! Sean, 2026-09-10: *lets get a full petri net diagram into the reports, so that I can browse
//! it from github after a deploy.*
//!
//! # The mapping, which is exact rather than an analogy
//!
//! The research lens established the formalism in
//! `lenses/research/2026-09-08-simple-finite-and-decidable.md`: **a recipe network is a Petri
//! net.** The release's four roles are the four arc kinds, and nothing has to be invented to
//! get from one to the other:
//!
//! | Release role | Petri net | In this release |
//! | ------------ | --------- | --------------- |
//! | `consume n`  | input arc of weight n     | taken |
//! | `produce n`  | output arc of weight n    | made |
//! | `require n`  | read arc - present, not taken | 7 of them |
//! | `limit 0`    | **inhibitor arc** - a zero test | 2 rows, and no arc |
//!
//! **The inhibitor arcs were the interesting ones and there are none left.** Reachability in a
//! plain Petri net is decidable; with inhibitor arcs the net is Turing-complete. Both of this
//! release's were `limit 0 garrison`, and `P-374` turned each into a requirement on room -
//! *there is no garrison* and *there is room for a garrison* are the same statement where one
//! is the most a territory can hold. **So the picture shows where the game sits on that line**,
//! which is the thing a table of the same rows does not.
//!
//! # A block is a transition, and a name is not
//!
//! **The release states some recipes more than once** - `stow` twice and `discard` four times,
//! once per kind, because `P-373` makes a rule whose subject is a family a rule for each of
//! them. In a net those are separate transitions: `discard` metal takes metal and `discard`
//! labor takes labor, and one node for both would be a picture of a rule the release does not
//! have. So the partition below is over blocks of rows, the count of distinct names is carried
//! beside it, and **a repeated name is labelled with the kind it acts on** - otherwise the
//! drawing has four nodes called `discard` and no way to tell them apart. `S-88`.
//!
//! # What cannot be drawn is counted, not dropped
//!
//! A Petri net arc carries a constant weight. **Some of this release's rows do not have one** -
//! `work` produces *`$where`'s density for that resource*. A recipe with such a row cannot be
//! drawn as arcs, and **a diagram that quietly left it out would be a picture of a game that is
//! not this one.**
//!
//! **It was four such recipes and it is one.** The saturating rewrite took the others out:
//! `grow` consumed *the lesser of the surplus food and the citizens here* and is gone
//! entirely - `P-379` - and the two capacity clamps became `stow` and `discard`, which carry
//! constant weights. `work` is the last one.
//!
//! So every recipe is accounted for on the page: how many there are, how many are drawn, and
//! which are not with the reason. `S-87` asks for exactly this, and names it as this
//! repository's recurring failure appearing somewhere new - an instrument answering a
//! narrower question than the one asked, and returning something plausible.
//!
//! **Which ones those are is computed, not listed.** `S-87` named five, from the research
//! lens's own re-encoding rather than from the release: two of the five - `refuel` and
//! `end-of-turn losses` - are not recipes in `releases/first-release.md` at all. Deriving the
//! set from the table means the page cannot inherit that.
//!
//! **And what leaving one out costs is computed too, against the excluded rows.** It used to be
//! read off the *Kinds* table, which was right while `food` was the casualty and is wrong now:
//! `work` produces a `resource`, and `resource` is a family rather than a kind, so that reading
//! reports a cost of zero. [`what_exclusion_costs`] asks the narrower question the right way
//! round, and [`kinds_never_drawn`] keeps the other half - four kinds no recipe names at all,
//! which the exclusion did not cause and the page must not blame it for.
//!
//! # Why it is generated
//!
//! `P-368` names the state-dependent amounts as defects, and the rewrites take them out one at
//! a time. A drawing would be wrong the first time one landed and would say nothing; this is
//! read from the release every time it is built, so the count moves on its own.

use std::collections::BTreeMap;

/// What a row of a recipe does to a place.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Role {
    /// Present and not taken - a read arc.
    Require,
    /// A maximum that must not be exceeded. At zero this is a zero test, and a zero test is
    /// an inhibitor arc.
    Limit,
    /// Taken - an input arc.
    Consume,
    /// Made - an output arc.
    Produce,
}

impl Role {
    fn parse(cell: &str) -> Option<Self> {
        match cell {
            "require" => Some(Self::Require),
            "limit" => Some(Self::Limit),
            "consume" => Some(Self::Consume),
            "produce" => Some(Self::Produce),
            _ => None,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Require => "require",
            Self::Limit => "limit",
            Self::Consume => "consume",
            Self::Produce => "produce",
        }
    }

    /// Whether the arc runs from the place into the transition.
    pub fn into_transition(self) -> bool {
        !matches!(self, Self::Produce)
    }
}

/// A place: somewhere a kind can be, which is a container and a kind together.
///
/// **A kind alone is not a place.** `spec/console.md` and the release's *Where things are*
/// give this release three sorts of container - a territory's capacity for a kind, a store,
/// and a unit's tank - and energy in a tank is not energy in a territory. The *Where* column
/// of a recipe row is what says which.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Place {
    pub container: String,
    pub kind: String,
    /// Whether this is the room left for that kind rather than the count of it.
    ///
    /// **`P-374`: what is stored is the room left.** Used capacity is what is there and total
    /// capacity is the two added, recorded nowhere so that nothing can disagree with it. Room
    /// is therefore state, and state is a place - drawing the count and leaving the room out
    /// would be the diagram omitting part of the state, which is the thing its own accounting
    /// paragraph exists to prevent.
    pub room: bool,
}

impl Place {
    pub fn label(&self) -> String {
        let what = if self.container == "a territory" {
            self.kind.clone()
        } else {
            format!("{} in {}", self.kind, self.container)
        };
        if self.room {
            format!("room for {what}")
        } else {
            what
        }
    }
}

/// The kinds a territory declares room for, and what the release says bounds each.
///
/// **A list with a check over it, rather than a rule that reads the words.** The bound is
/// stated in prose - *a capacity of 1*, *as many as the extractors of its resource* - and a
/// predicate matching on `capacity` would classify the store as unbounded because its
/// sentence does not use the word. That is the narrow-predicate failure this repository keeps
/// recording, so the classification is written down and
/// `every_bound_the_release_states_is_classified` asserts that every row of the table is in
/// one list or the other.
pub const BOUNDED: [(&str, &str); 6] = [
    ("garrison", "a capacity of 1"),
    ("extractor", "a capacity, from Territory resources"),
    ("store", "as many as the extractors of its resource"),
    ("yard", "a capacity of 1"),
    ("ark", "a capacity of 2"),
    ("pioneer", "a capacity of 2"),
];

/// The kinds a territory bounds by something other than room, and why that is not capacity.
///
/// **A bound is not a capacity.** Room is spent and given back - `P-374` - and none of these
/// works that way: nothing frees a citizen's worth of room by eating, and `P-372` says a
/// territory declares **no limit** for a resource at all.
///
/// **Six since `P-381`, and three of the labels quoted a sentence that is gone.** They said
/// *what a territory holds directly is in disorder*, which `P-381` replaced: a raw material is
/// in one of three states - its source, disorder, or a container - and what a territory
/// declares capacity for is the things that hold it rather than the material. **`fertility` is
/// the sixth**, given a bound row by `P-380` where it had none. `S-88` named both.
///
/// **`labor` and `fertility` are transient**, which the release says of exactly those two:
/// neither has a source and nothing holds either, so both are always in disorder and neither
/// survives the turn's end. That is a rate over a turn rather than a room that fills.
pub const UNBOUNDED: [(&str, &str); 6] = [
    (
        "citizen",
        "the food produced here, through upkeep - a rate, not room",
    ),
    (
        "labor",
        "the citizens that make it, one each per turn - transient, so always in disorder",
    ),
    (
        "fertility",
        "the citizens that make it, one each per turn - transient, so always in disorder",
    ),
    (
        "food",
        "no limit: a territory declares capacity for the things that hold it, not for it",
    ),
    (
        "metal",
        "no limit: a territory declares capacity for the things that hold it, not for it",
    ),
    (
        "energy",
        "no limit: a territory declares capacity for the things that hold it, not for it",
    ),
];

/// Whether the release declares room for this kind in this container.
///
/// **A kind is not bounded or unbounded on its own**, which the first version of this got
/// wrong in both directions at once. Energy in a territory has no limit - `P-372` - and
/// energy in a unit's tank is bounded by that unit's fuel; an ark in a territory has a
/// capacity of 2 and an ark in an orbit has none declared at all, so the net grew a *room for
/// ark in an orbit* that the release never says exists.
///
/// **Every container is answered here rather than defaulting**, and
/// `every_container_the_net_uses_is_classified` asserts the net produces no other.
pub fn bounded(container: &str, kind: &str) -> bool {
    match container {
        // `releases/first-release.md`, *What bounds a kind in a territory*.
        "a territory" => BOUNDED.iter().any(|(name, _)| *name == kind),
        // *Where things are*: a store holds the resource it was built for, up to 10; a unit's
        // tank holds energy, up to the unit's fuel.
        "a store" | "a unit's tank" => true,
        // *An orbit holds units and nothing else*, and states no capacity. The capacity of 2
        // for an ark is stated of a territory.
        "an orbit" => false,
        // The game holds the twelve territories and declares no capacity to hold anything,
        // which is this lane's own `C-68`.
        "the game" => false,
        other => panic!("`{other}` holds `{kind}` and nothing says whether it is bounded"),
    }
}

/// Every container the release gives things to be in.
///
/// Listed so a test can assert the net produces no other, which is what makes the `panic!` in
/// [`bounded`] a guard rather than a hazard.
pub const CONTAINERS: [&str; 4] = ["a territory", "a store", "a unit's tank", "an orbit"];

/// The capacity the release states as a number, where it states one.
///
/// **Only needed to check one translation.** A `limit 0` row becomes a requirement on room,
/// and that is equivalent only where the capacity is exactly one: at a capacity of two,
/// *there is none* and *there is room for one* are different claims.
pub fn stated_capacity(kind: &str) -> Option<u32> {
    match kind {
        "garrison" | "yard" => Some(1),
        "ark" | "pioneer" => Some(2),
        _ => None,
    }
}

/// One arc: a transition, a place, what it does and how much.
#[derive(Clone, Debug)]
pub struct Arc {
    pub transition: usize,
    pub place: usize,
    pub role: Role,
    pub weight: u32,
    /// The traits the row names, which narrow the kind without changing the place.
    pub traits: String,
}

impl Arc {
    /// A zero test, which is what makes a Petri net Turing-complete.
    pub fn is_inhibitor(&self) -> bool {
        self.role == Role::Limit && self.weight == 0
    }
}

/// A recipe that could not be drawn, and the row that stopped it.
#[derive(Clone, Debug)]
pub struct Excluded {
    pub name: String,
    pub because: String,
}

/// The whole net, and everything left out of it.
#[derive(Debug)]
pub struct Net {
    pub places: Vec<Place>,
    pub transitions: Vec<String>,
    pub arcs: Vec<Arc>,
    pub excluded: Vec<Excluded>,
    /// Every block of rows the release states, drawn or not.
    ///
    /// **A block is a transition and a name is not** - `S-88`. The release states `stow`
    /// twice and `discard` four times, once per kind, because `P-373` makes a rule whose
    /// subject is a family a rule for each of them. In a net those really are six
    /// transitions: `discard` metal takes metal and `discard` labor takes labor, and drawing
    /// them as one node would be a picture of a rule the release does not have.
    ///
    /// So the partition is over blocks, and [`Net::names`] is the other number.
    pub recipes: usize,
    /// How many distinct recipe names those blocks are stated under.
    ///
    /// **Carried so the page can say both.** *Twenty recipes* is what a person reading the
    /// release counts; *twenty-four transitions* is what they count in the drawing, and a
    /// page giving one without the other invites the reader to think something went missing.
    pub names: usize,
}

impl Net {
    /// The arcs touching one transition, in the order the release states them.
    pub fn arcs_of(&self, transition: usize) -> Vec<&Arc> {
        self.arcs
            .iter()
            .filter(|arc| arc.transition == transition)
            .collect()
    }

    /// Every zero test in the net.
    pub fn inhibitors(&self) -> Vec<&Arc> {
        self.arcs.iter().filter(|arc| arc.is_inhibitor()).collect()
    }
}

/// Which container a row's *Where* cell names.
///
/// **Blank means the one place the recipe acts**, which the release states directly; a named
/// parameter is a territory bound by another row; `that unit` is a tank; an orbit is an orbit.
fn container_of(where_cell: &str, kind: &str) -> String {
    let cell = where_cell.trim().trim_matches('`');
    if cell.contains("orbit") {
        "an orbit".to_string()
    } else if cell.contains("unit") {
        "a unit's tank".to_string()
    } else if kind == "territory" || kind == "place" || kind == "orbit" {
        // A territory is not *in* a territory; it is the thing the rest are in.
        "the game".to_string()
    } else {
        "a territory".to_string()
    }
}

/// Reads the release's *Recipes* table as a Petri net.
pub fn net(document: &str) -> Net {
    let rows = crate::recipes::body_under(document, "## Recipes");

    // Gathered first, because a recipe's name is on its first row only and a row whose
    // quantity is not a number disqualifies the whole recipe rather than one arc.
    let mut named: Vec<(String, Vec<Vec<String>>)> = Vec::new();
    for row in &rows {
        let name = crate::recipes::plain(row.first().map(String::as_str).unwrap_or_default());
        if !name.is_empty() {
            named.push((name, Vec::new()));
        }
        let Some((_, lines)) = named.last_mut() else {
            continue;
        };
        if row
            .get(2)
            .map(String::as_str)
            .unwrap_or_default()
            .is_empty()
        {
            continue;
        }
        lines.push(row.clone());
    }

    let recipes = named.len();
    // **Distinct names, which is the number a person counts in the release.** Deduplicating
    // here rather than folding the blocks together, because the blocks are what the net draws
    // - `tests/fired.rs` made the same distinction for what an `end turn` fires, and for the
    // same reason: a name repeated is one rule applied to two kinds.
    let mut seen: Vec<&str> = Vec::new();
    for (name, _) in &named {
        if !seen.contains(&name.as_str()) {
            seen.push(name);
        }
    }
    let names = seen.len();

    // **A repeated name is labelled by what it acts on, or the drawing has four nodes called
    // `discard`.** The disambiguator is the kind on the block's first row, which is exactly
    // the member of the family the rule was instantiated for - so it is the thing that made
    // the block a separate block rather than a suffix invented to tell them apart.
    let labels: Vec<String> = named
        .iter()
        .map(|(name, lines)| {
            if named.iter().filter(|(other, _)| other == name).count() < 2 {
                return name.clone();
            }
            let kind = lines
                .first()
                .and_then(|row| row.get(4))
                .map(|cell| crate::recipes::plain(cell))
                .unwrap_or_default();
            assert!(
                !kind.is_empty(),
                "`{name}` is stated more than once and its first row names no kind, so the \
                 two cannot be told apart on the page"
            );
            format!("{name} ({kind})")
        })
        .collect();
    assert_eq!(
        labels.len(),
        labels
            .iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        "two blocks share a label, so the drawing would show one node for two transitions: \
         {labels:?}"
    );

    let mut places: Vec<Place> = Vec::new();
    let mut transitions: Vec<String> = Vec::new();
    let mut arcs: Vec<Arc> = Vec::new();
    let mut excluded: Vec<Excluded> = Vec::new();

    for ((_, lines), label) in named.iter().zip(&labels) {
        // **The whole recipe, or none of it.** An arc without a weight cannot be drawn, and
        // drawing a recipe's other rows without it would show a transition that takes less
        // than it does.
        if let Some(row) = lines.iter().find(|row| number(row).is_none()) {
            excluded.push(Excluded {
                name: label.clone(),
                because: format!(
                    "its {} row is `{}`, which is a state rather than a number",
                    row.get(2).cloned().unwrap_or_default(),
                    row.get(3).cloned().unwrap_or_default()
                ),
            });
            continue;
        }

        let transition = transitions.len();
        transitions.push(label.clone());
        for row in lines {
            let Some(role) = Role::parse(row.get(2).map(String::as_str).unwrap_or_default()) else {
                continue;
            };
            let kind = crate::recipes::plain(row.get(4).map(String::as_str).unwrap_or_default());
            if kind.is_empty() {
                continue;
            }
            let container = container_of(row.get(6).map(String::as_str).unwrap_or_default(), &kind);
            let weight = number(row).unwrap_or_default();
            let traits = row.get(5).cloned().unwrap_or_default();

            let find = |places: &mut Vec<Place>, room: bool| {
                let place = Place {
                    container: container.clone(),
                    kind: kind.clone(),
                    room,
                };
                match places.iter().position(|it| *it == place) {
                    Some(at) => at,
                    None => {
                        places.push(place);
                        places.len() - 1
                    }
                }
            };

            // **A zero test becomes a claim on room** - `P-374`, and it is exact only at a
            // capacity of one. *There is no garrison* and *there is room for a garrison* are
            // the same statement when one is the most it can hold, and different statements
            // at two. So the translation is made only where the release states a capacity of
            // one, and refused loudly otherwise rather than quietly approximated.
            if role == Role::Limit && weight == 0 && bounded(&container, &kind) {
                assert_eq!(
                    stated_capacity(&kind),
                    Some(1),
                    "`limit 0 {kind}` is being read as a requirement on room, which says the \
                     same thing only where the capacity is one - and {kind}'s is not"
                );
                let at = find(&mut places, true);
                arcs.push(Arc {
                    transition,
                    place: at,
                    role: Role::Require,
                    weight: 1,
                    traits,
                });
                continue;
            }

            let at = find(&mut places, false);
            arcs.push(Arc {
                transition,
                place: at,
                role,
                weight,
                traits: traits.clone(),
            });

            // **Making takes room and destroying gives it back** - `P-374`, and the two never
            // come apart because the total is only ever their sum. A requirement takes
            // nothing and makes nothing, so it moves no room.
            if !bounded(&container, &kind) {
                continue;
            }
            let room = match role {
                Role::Produce => Some(Role::Consume),
                Role::Consume => Some(Role::Produce),
                Role::Require | Role::Limit => None,
            };
            if let Some(role) = room {
                let at = find(&mut places, true);
                arcs.push(Arc {
                    transition,
                    place: at,
                    role,
                    weight,
                    traits,
                });
            }
        }
    }

    Net {
        places,
        transitions,
        arcs,
        excluded,
        recipes,
        names,
    }
}

/// A row's quantity, when it is a whole number.
///
/// **The test is the same one `recipes.rs` makes** - a quantity is a number or it is an
/// expression - stated here as a parse so that the failure carries the cell.
fn number(row: &[String]) -> Option<u32> {
    let cell = row.get(3)?.trim();
    if cell.is_empty() {
        return None;
    }
    cell.parse().ok()
}

/// The incidence matrix: places down, transitions across.
///
/// **This is what the checks operate on**, and it is the form that needs no layout at all. It
/// is also the diffable sibling `R-9` requires of the drawing - a change to one weight is one
/// changed cell, where the same change in an SVG moves every coordinate after it.
pub fn matrix(net: &Net) -> Vec<Vec<String>> {
    let mut out = Vec::new();
    let mut header = vec!["Place".to_string()];
    header.extend(net.transitions.iter().cloned());
    out.push(header);

    for (at, place) in net.places.iter().enumerate() {
        let mut row = vec![place.label()];
        for transition in 0..net.transitions.len() {
            let mut cell = String::new();
            for arc in &net.arcs {
                if arc.place != at || arc.transition != transition {
                    continue;
                }
                if !cell.is_empty() {
                    cell.push_str(", ");
                }
                // **The sign is the direction and the letter is the kind of arc**, so a cell
                // says what the row says: `-1` is taken, `+2` is made, `r1` is read and not
                // taken, and `0!` is the zero test.
                cell.push_str(&match arc.role {
                    Role::Consume => format!("-{}", arc.weight),
                    Role::Produce => format!("+{}", arc.weight),
                    Role::Require => format!("r{}", arc.weight),
                    Role::Limit if arc.weight == 0 => "0!".to_string(),
                    Role::Limit => format!("<={}", arc.weight),
                });
            }
            row.push(cell);
        }
        out.push(row);
    }
    out
}

/// How many arcs of each role, for a page that says what it is showing.
pub fn by_role(net: &Net) -> BTreeMap<&'static str, usize> {
    let mut out = BTreeMap::new();
    for arc in &net.arcs {
        *out.entry(arc.role.name()).or_insert(0) += 1;
    }
    out
}

/// Kinds the release declares that the drawn net never names.
///
/// **Not the same as a place with no arcs** - the net only makes a place when an arc needs
/// one, so that set is always empty and a page reporting it would always report nothing.
///
/// **This used to be what the exclusions cost, and the saturating rewrite took that away.**
/// `food` was the case: every recipe that moved it - `work`, `upkeep`, `grow` - had a
/// state-dependent amount, so all three were excluded and the drawn net had no food in it
/// anywhere. `grow` is gone since `P-379` and `upkeep` carries a constant weight, so food is
/// drawn now and the sentence the page used to carry about it is false.
///
/// **What is left in this set is a different fact, and the page must not present it as the
/// old one.** `orbit`, `deposit`, `adjacency` and `game` are absent because **no recipe names
/// them as a kind** - they are where things are and how places relate, not things a recipe
/// moves. That would be true of a net with nothing excluded at all. What the exclusion costs
/// is [`what_exclusion_costs`], which is a different set computed a different way.
pub fn kinds_never_drawn(net: &Net, document: &str) -> Vec<String> {
    let declared = crate::recipes::body_under(document, "## Kinds");
    let drawn: Vec<&str> = net.places.iter().map(|place| place.kind.as_str()).collect();
    declared
        .iter()
        .filter_map(|row| row.first())
        .map(|cell| crate::recipes::plain(cell))
        .filter(|kind| !kind.is_empty() && !drawn.contains(&kind.as_str()))
        .collect()
}

/// What the excluded recipes cost the drawing: what they name that nothing drawn names.
///
/// **The measurement [`kinds_never_drawn`] used to make and no longer does.** A recipe left
/// out costs the page nothing if every place it touches is reached by some other recipe, and
/// costs a reader a great deal if it is the only way into one. So the question is asked
/// against the excluded recipes themselves rather than against the list of kinds: what do
/// their rows name, and which of those does the drawn net never name?
///
/// **It is over kinds and families both**, which is the reason this is not a filter on the
/// *Kinds* table. `work` produces a `resource`, and `resource` is a family - so a check
/// reading the *Kinds* table would find nothing missing and report a cost of zero, which is
/// this repository's recurring failure: a right number about a narrower question.
///
/// **A zero here is a real answer and says what it counted against.** With one recipe
/// excluded and its rows naming four things, *none of them is missing* means something; with
/// nothing excluded it would mean nothing, and the caller has [`Net::excluded`] to tell the
/// two apart.
pub fn what_exclusion_costs(net: &Net, document: &str) -> Vec<String> {
    let rows = crate::recipes::body_under(document, "## Recipes");
    let out_of: Vec<&str> = net.excluded.iter().map(|one| one.name.as_str()).collect();
    let drawn: Vec<&str> = net.places.iter().map(|place| place.kind.as_str()).collect();

    let mut current = String::new();
    let mut lost: Vec<String> = Vec::new();
    for row in &rows {
        let name = crate::recipes::plain(row.first().map(String::as_str).unwrap_or_default());
        if !name.is_empty() {
            current = name;
        }
        // The excluded names are the net's labels, and a label is the name where the name is
        // stated once. A repeated name that became excluded would be `stow (metal)` here and
        // `stow` in the table, so this matches on the opening rather than on equality.
        if !out_of
            .iter()
            .any(|label| label.starts_with(current.as_str()))
        {
            continue;
        }
        let kind = crate::recipes::plain(row.get(4).map(String::as_str).unwrap_or_default());
        if kind.is_empty() || drawn.contains(&kind.as_str()) || lost.contains(&kind) {
            continue;
        }
        lost.push(kind);
    }
    lost
}
