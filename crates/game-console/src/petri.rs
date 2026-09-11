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
//! | `limit 0`    | **inhibitor arc** - a zero test | 2 of them |
//!
//! **The two inhibitor arcs are the interesting ones.** Reachability in a plain Petri net is
//! decidable; with inhibitor arcs the net is Turing-complete. Both of this release's are
//! `limit 0 garrison`, and a garrison is bounded by a capacity of 1 - a zero test on a bounded
//! place costs nothing, which is what `C-75` measured and what `X-9` asks the specification to
//! adopt. **So the picture shows where the game sits on that line**, which is the thing a
//! table of the same rows does not.
//!
//! # What cannot be drawn is counted, not dropped
//!
//! A Petri net arc carries a constant weight. **Some of this release's rows do not have one** -
//! `work` produces *`$where`'s density for that resource*, `grow` consumes *the lesser of the
//! surplus food and the citizens here*. A recipe with such a row cannot be drawn as arcs, and
//! **a diagram that quietly left it out would be a picture of a game that is not this one.**
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
pub const UNBOUNDED: [(&str, &str); 5] = [
    (
        "citizen",
        "the food produced here, through upkeep - a rate, not room",
    ),
    ("labor", "the citizens that make it - a rate, not room"),
    (
        "food",
        "no limit: what a territory holds directly is in disorder",
    ),
    (
        "metal",
        "no limit: what a territory holds directly is in disorder",
    ),
    (
        "energy",
        "no limit: what a territory holds directly is in disorder",
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
    /// Every recipe the release declares, drawn or not.
    pub recipes: usize,
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
    let mut places: Vec<Place> = Vec::new();
    let mut transitions: Vec<String> = Vec::new();
    let mut arcs: Vec<Arc> = Vec::new();
    let mut excluded: Vec<Excluded> = Vec::new();

    for (name, lines) in &named {
        // **The whole recipe, or none of it.** An arc without a weight cannot be drawn, and
        // drawing a recipe's other rows without it would show a transition that takes less
        // than it does.
        if let Some(row) = lines.iter().find(|row| number(row).is_none()) {
            excluded.push(Excluded {
                name: name.clone(),
                because: format!(
                    "its {} row is `{}`, which is a state rather than a number",
                    row.get(2).cloned().unwrap_or_default(),
                    row.get(3).cloned().unwrap_or_default()
                ),
            });
            continue;
        }

        let transition = transitions.len();
        transitions.push(name.clone());
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
/// **This is what the exclusions actually cost**, and it is not the same as a place with no
/// arcs - the net only makes a place when an arc needs one, so that set is always empty and a
/// page reporting it would always report nothing.
///
/// `food` is the case and neither this lane nor the specification lane predicted it: every
/// recipe that moves food - `work`, `upkeep`, `grow` - has a state-dependent amount, so all
/// three are excluded and the drawn net has no food in it anywhere. **A reader looking at the
/// picture for it would conclude the game has none.**
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
