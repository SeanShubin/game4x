//! Nothing comes back round with more, decided from `spec/data/rules.4x` alone.
//!
//! `cargo run --example nogain` prints the working; `tests/nogain.rs` is what holds it, and
//! `reports/nogain.html` is where it is read.
//!
//! **`spec/invariants.md` -> Nothing comes back round with more**: *whether this holds is decided
//! mechanically, from the rules alone, and stays so however many rules there are.*
//!
//! > There is a weighting of the kinds, and under it no sequence of rules ends holding more
//! > than it began with.
//!
//! So: find a weighting `w` with `w \u{b7} (made - taken) <= 0` at every rule. If one exists, no
//! sequence can gain - each firing is non-increasing in `w`, so a cycle is too. This is a
//! **P-invariant** in the Petri net sense, relaxed from equality to an inequality because
//! disorder is meant: *it may end with less - that is disorder*.
//!
//! # The solver is not new and the reader is
//!
//! **`S-219`**: *what it consumed was rules with weights, and that is what the data holds.* The
//! model and the feasibility solver below are `a8386450`'s, which `D-4` deleted with the four
//! reports that read the old release's *Recipes* table - **rightly, because they read a ruleset
//! that is going.** What they computed was never about that table's shape.
//!
//! **What is new is everything above them**: sixteen rules read from `spec/data/rules.4x`, where a
//! clause carries its quantity as a `{literal ...}` rather than as a Qty cell.
//!
//! # A place is a kind in a state, and that is what keeps this from being vacuous
//!
//! **A kind alone is not enough.** The deleted file found this in its own drawing: at the
//! granularity of *(container, kind)*, `work` takes an extractor and makes an extractor and nets
//! to zero, and `refresh` takes a thing and makes a thing and nets to zero too - **so a weighting
//! over those places would report the whole economy of acting as doing nothing at all.** That is
//! a green run about the wrong question.
//!
//! **So a place is a kind, and a kind with one trait pinned.** `move` is then a unit leaving
//! `moving 1` and arriving at `moving 0`, and `refresh` is what puts it back - which is the pair
//! the invariant is actually about.
//!
//! # Where each reading comes from, and the one that is a guess
//!
//! **Both draws are the invariant's own sentence rather than this reader's idea.**
//! `spec/invariants.md`: *Anything that exhausts draws on time for a turn: it spends a count it
//! carries, and only the turn's end restores that count, the way an extractor draws material out
//! of the planet and is spent doing it.* So:
//!
//! - **a `put` that restores a count draws on time** - `refresh` is the turn's end restoring it,
//!   and that clause is what the sentence describes
//! - **a quantity read out of a `{require}`d deposit draws on the planet** - the density is the
//!   well, and the extractor is the bounded pump in front of it
//!
//! **A source is a place like any other here, and that is what makes the check bite.** Naming one
//! as an exemption would let every rule that touches it out of the arithmetic; giving it a weight
//! with nothing that refills it lets it be weighed as heavily as the solver needs.
//!
//! **What is guessed is which well, and the data cannot say.** The invariant names three sources -
//! the planet, the star and time - and `spec/data/` names none of them. A density reading is the
//! same shape whether it is a mine or a sunlit orbit, so **every density draw here is charged to
//! the planet and the star is drawn on by nothing.** `C-166` is that question.
//!
//! **Charging them all to one well is the strict reading, not a convenient one.** One well means
//! one weight that has to sit above metal, food and energy at once; splitting them could only
//! give the solver more room. So the verdict below holds under a reading at least as demanding as
//! the one the invariant intends.
//!
//! # What a rule does not name, it leaves as it found it
//!
//! **So an unnamed trait moves nothing here**, and that is `spec/invariants.md` rather than an
//! assumption: *a rule acting on a family acts on members that may carry columns it never
//! mentions, and what it does not name it leaves as it found it.*
//!
//! **`move` over an ark is the case it was written for**, and this reader was re-derived against
//! it. `move` names `unit`'s three columns and an ark carries four; `gathering` is named by
//! nothing, and is carried through rather than defaulted - which is why `move` cannot be a way to
//! refresh an ark's gathering without paying the time that `refresh` pays.

use std::collections::{BTreeMap, BTreeSet};

/// Somewhere a thing of one kind, in one state, can be.
///
/// **A kind alone is not enough**, which is the finding that keeps this check from being
/// vacuous. A citizen's `laboring` and its `bearing` are different places, because they are
/// different counts - `P-411` gives each of them its own trait with its own two values, and a
/// weighting blind to the difference would let one pay for the other.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Place {
    pub kind: String,
    /// The trait cell that narrows the kind, normalized. Empty for the kind at rest.
    pub state: String,
}

impl Place {
    pub fn label(&self) -> String {
        if self.state.is_empty() {
            self.kind.clone()
        } else {
            format!("{}, {}", self.kind, self.state)
        }
    }

    /// Whether this is one of the three endless wells rather than something in the game.
    pub fn is_source(&self) -> bool {
        SOURCES.contains(&self.kind.as_str())
    }
}

/// The three `spec/invariants.md` names, spelled as it spells them.
pub const SOURCES: [&str; 3] = ["the planet", "the star", "time"];

/// One rule, ground to constants and to single kinds, and what it does to each place.
#[derive(Clone, Debug)]
pub struct Rule {
    pub name: String,
    /// Made minus taken, per place. A requirement moves nothing and is absent.
    pub delta: BTreeMap<Place, i64>,
}

impl Rule {
    /// Whether this rule touches a source, and which.
    pub fn draws_from(&self) -> Vec<&str> {
        let mut from: Vec<&str> = self
            .delta
            .keys()
            .filter(|place| place.is_source())
            .map(|place| place.kind.as_str())
            .collect();
        from.sort_unstable();
        from
    }
}

/// What the search found: a weighting, or the rules that made one impossible to reach.
#[derive(Debug)]
pub enum Found {
    /// A weighting under which no rule gains. Every place is in it.
    Weighting(BTreeMap<Place, i64>),
    /// No weighting was reached, and these rules were still gaining when the search stopped.
    ///
    /// **This one does mean none exists.** Phase I of the simplex ends with the artificial
    /// variables at zero exactly when the system has a solution, so a failure here is a
    /// property of the rules rather than of how long the search ran. The earlier version of
    /// this could only say *not found within a bound*, and said it wrongly.
    NotFound { forcing: Vec<String> },
}

/// Solve for a weighting under which no rule ends with more than it began.
///
/// **Derived and published, never declared** - `S-93`, and Sean chose it that way *exactly
/// because the numbers would otherwise be invented*. Nothing here reads a weight from the
/// release or from this file; every weight is whatever the rules force it to be.
///
/// # Why this is exact rather than a search
///
/// **The first version raised weights until nothing gained, and it reported a false alarm.**
/// It oscillated between `bear` - which needs `fertility` to be worth no more than a fertile
/// citizen - and `breed`, which needs the fertility it spends to be worth *more*. Raising
/// either broke the other, and after two hundred rounds it reported `deploy ark`,
/// `found by land` and `bear` as forcing no weighting to exist. **A weighting does exist**,
/// and one is printed in the report.
///
/// A check that says the game gains when it does not is worse than no check: it would have
/// sent somebody looking for a defect in the rules rather than in the instrument. So this
/// solves the feasibility problem exactly instead of searching for it.
///
/// # The problem, and the arithmetic it is solved in
///
/// Every weight is at least one, so write `w = 1 + x` with `x >= 0`. Each rule's constraint
/// `w · delta <= 0` becomes `delta · x <= -(sum of delta)`, which is a standard linear
/// feasibility problem: `A x <= b`, `x >= 0`. **A floor of one is what stops the answer being
/// empty** - `w = 0` satisfies every rule and says nothing about the game.
///
/// **Exact rationals over `i128`, not floating point.** The weighting is published into a file
/// the gate compares byte for byte, and `CLAUDE.md` requires a generated file to be the same
/// bytes twice. A floating-point pivot would also make *is this zero* a judgement, and every
/// degenerate pivot here would turn on it.
pub fn solve(rules: &[Rule]) -> Found {
    let mut places: Vec<Place> = Vec::new();
    for rule in rules {
        for place in rule.delta.keys() {
            if !places.contains(place) {
                places.push(place.clone());
            }
        }
    }
    places.sort();

    // `A x <= b` with `b = -(row sum)`, which is the `w = 1 + x` substitution written out.
    let rows: Vec<(Vec<i128>, i128)> = rules
        .iter()
        .map(|rule| {
            let row: Vec<i128> = places
                .iter()
                .map(|place| rule.delta.get(place).copied().unwrap_or(0) as i128)
                .collect();
            let sum: i128 = row.iter().sum();
            (row, -sum)
        })
        .collect();

    match feasible(&rows, places.len()) {
        Some(x) => {
            // **Scaled to whole numbers rather than rounded to them, and that distinction
            // cost a wrong answer once.** Rounding each weight up broke `work (metal x8)`:
            // metal came back as three halves, rounding it to two made eight of it worth
            // sixteen against a planet worth ten, and the check reported a gain that the
            // solution it was rounding did not have.
            //
            // **Every constraint is homogeneous** - `w · delta <= 0` says the same thing of
            // `w` and of any positive multiple of `w` - and the only non-homogeneous part is
            // the floor of one, which scaling upward cannot break. So multiplying through by
            // the common denominator is exact where rounding is not.
            let weights: Vec<Ratio> = x.iter().map(|value| Ratio::whole(1).add(*value)).collect();
            let mut scale: i128 = 1;
            for weight in &weights {
                scale = scale / gcd(scale, weight.bottom) * weight.bottom;
            }
            Found::Weighting(
                places
                    .iter()
                    .cloned()
                    .zip(
                        weights
                            .iter()
                            .map(|weight| (weight.top * (scale / weight.bottom)) as i64),
                    )
                    .collect(),
            )
        }
        None => {
            // **Which rules force it is asked of the rules, not of the solver.** A tableau
            // says infeasible without saying why, so the report names the rules that cannot
            // hold together under the weighting everything else settles at - which is what a
            // reader needs to go and look at.
            let forcing = rules
                .iter()
                .filter(|rule| rule.delta.values().sum::<i64>() > 0)
                .map(|rule| rule.name.clone())
                .collect();
            Found::NotFound { forcing }
        }
    }
}

/// One exact rational, kept in lowest terms so that equality is equality.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Ratio {
    top: i128,
    bottom: i128,
}

impl Ratio {
    fn whole(value: i128) -> Self {
        Ratio {
            top: value,
            bottom: 1,
        }
    }

    /// **Always in lowest terms, and zero is `0/1`.**
    ///
    /// The first version returned `0/n` unreduced, because its `gcd` treated a numerator of
    /// zero as one. Denominators then multiplied together unchecked through every pivot and
    /// the tableau overflowed `i128` on a forty-row system. A rational kept reduced is the
    /// whole reason exactness is affordable here.
    fn reduced(top: i128, bottom: i128) -> Self {
        if top == 0 {
            return Ratio { top: 0, bottom: 1 };
        }
        let sign = if bottom < 0 { -1 } else { 1 };
        let (top, bottom) = (top * sign, bottom * sign);
        let divisor = gcd(top, bottom);
        Ratio {
            top: top / divisor,
            bottom: bottom / divisor,
        }
    }

    fn is_zero(self) -> bool {
        self.top == 0
    }

    fn is_negative(self) -> bool {
        self.top < 0
    }

    fn add(self, other: Self) -> Self {
        Ratio::reduced(
            self.top * other.bottom + other.top * self.bottom,
            self.bottom * other.bottom,
        )
    }

    fn subtract(self, other: Self) -> Self {
        self.add(Ratio {
            top: -other.top,
            bottom: other.bottom,
        })
    }

    fn times(self, other: Self) -> Self {
        Ratio::reduced(self.top * other.top, self.bottom * other.bottom)
    }

    fn over(self, other: Self) -> Self {
        Ratio::reduced(self.top * other.bottom, self.bottom * other.top)
    }
}

fn gcd(a: i128, b: i128) -> i128 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        let next = a % b;
        a = b;
        b = next;
    }
    if a == 0 { 1 } else { a }
}

/// A feasible `x >= 0` with `A x <= b`, by Phase I of the simplex method.
///
/// **Phase I and no Phase II, because nothing is being optimised.** `S-93` asks for *a*
/// weighting and asks for it to be published so that one which is sound and says something
/// Sean would reject is visible. Minimising anything would be this lane choosing what the
/// weighting ought to say.
///
/// Slacks make every row an equation; a row whose `b` is negative is multiplied through and
/// given an artificial variable. Driving the artificials to zero is the whole of Phase I: if
/// they reach zero the remaining basis is a feasible point, and if they cannot the system has
/// no solution at all.
///
/// **Bland's rule for the pivot**, which is slower than choosing the steepest column and
/// cannot cycle. The system is forty-odd rows; the guarantee is worth more than the speed, and
/// a solver that looped would be a gate that hangs.
fn feasible(rows: &[(Vec<i128>, i128)], variables: usize) -> Option<Vec<Ratio>> {
    let height = rows.len();
    // Columns: the real variables, one slack per row, one artificial per negative row.
    let negative: Vec<bool> = rows.iter().map(|(_, b)| *b < 0).collect();
    let artificials = negative.iter().filter(|is| **is).count();
    if artificials == 0 {
        return Some(vec![Ratio::whole(0); variables]);
    }
    let width = variables + height + artificials;

    let mut table: Vec<Vec<Ratio>> = vec![vec![Ratio::whole(0); width + 1]; height + 1];
    let mut basis: Vec<usize> = vec![0; height];
    let mut artificial_at = variables + height;

    for (at, (row, b)) in rows.iter().enumerate() {
        let flip: i128 = if negative[at] { -1 } else { 1 };
        for (column, value) in row.iter().enumerate() {
            table[at][column] = Ratio::whole(value * flip);
        }
        table[at][variables + at] = Ratio::whole(flip);
        table[at][width] = Ratio::whole(b * flip);
        if negative[at] {
            table[at][artificial_at] = Ratio::whole(1);
            basis[at] = artificial_at;
            artificial_at += 1;
        } else {
            basis[at] = variables + at;
        }
    }

    // The Phase I objective: minimise the sum of the artificials, written as the sum of the
    // rows that carry one so that the cost row is expressed in non-basic variables.
    for (at, carries) in negative.iter().enumerate() {
        if !carries {
            continue;
        }
        // Cloned because the cost row and the carried row are both in `table`, and a
        // borrow of one while writing the other is what the index form was working around.
        let carried = table[at].clone();
        for (cell, add) in table[height].iter_mut().zip(carried) {
            *cell = cell.add(add);
        }
    }

    loop {
        // Bland: the lowest-numbered column with a positive cost.
        let entering = (0..width).find(|column| {
            basis.iter().all(|held| held != column)
                && !table[height][*column].is_negative()
                && !table[height][*column].is_zero()
        });
        let Some(entering) = entering else { break };

        // The tightest ratio decides the row, ties broken by the lowest basis index.
        let mut leaving: Option<(usize, Ratio)> = None;
        for at in 0..height {
            let column = table[at][entering];
            if column.is_zero() || column.is_negative() {
                continue;
            }
            let ratio = table[at][width].over(column);
            let better = match &leaving {
                None => true,
                Some((held, best)) => {
                    ratio.subtract(*best).is_negative()
                        || (ratio == *best && basis[at] < basis[*held])
                }
            };
            if better {
                leaving = Some((at, ratio));
            }
        }
        let Some((pivot, _)) = leaving else {
            // Unbounded in a direction that reduces the artificials: cannot happen while
            // every artificial is bounded below by zero, so this is a defect rather than an
            // answer, and saying so beats returning "infeasible" for the wrong reason.
            return None;
        };

        let divisor = table[pivot][entering];
        for cell in table[pivot].iter_mut() {
            *cell = cell.over(divisor);
        }
        for at in 0..=height {
            if at == pivot {
                continue;
            }
            let factor = table[at][entering];
            if factor.is_zero() {
                continue;
            }
            let pivot_row = table[pivot].clone();
            for (cell, from) in table[at].iter_mut().zip(pivot_row) {
                *cell = cell.subtract(from.times(factor));
            }
        }
        basis[pivot] = entering;
    }

    // The artificials are driven out exactly when the objective reaches zero.
    if !table[height][width].is_zero() {
        return None;
    }

    let mut answer = vec![Ratio::whole(0); variables];
    for (at, held) in basis.iter().enumerate() {
        if *held < variables {
            answer[*held] = table[at][width];
        }
    }

    Some(answer)
}

// -- Reading the rules ----------------------------------------------------------------------

use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// One `{...}` row, as its sort and its named cells.
struct Row {
    sort: String,
    cells: BTreeMap<String, String>,
}

impl Row {
    fn at(&self, key: &str) -> Option<&str> {
        self.cells.get(key).map(String::as_str)
    }
}

/// Every row of a friendly `.4x`.
///
/// **A hand parser rather than the engine's, and reading `spec/data/` rather than what runs.**
/// `C-165` records those being two different files. What Sean writes is the thing the invariant
/// is about, so it is the thing this reads.
fn rows_of(at: &str) -> Vec<Row> {
    let text = std::fs::read_to_string(root().join(at)).unwrap_or_else(|why| panic!("{at}: {why}"));
    let mut out = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if !line.starts_with('{') {
            continue;
        }
        let Some(close) = line.rfind('}') else {
            continue;
        };
        let mut parts = line[1..close].split_whitespace();
        let Some(sort) = parts.next() else { continue };
        let mut cells = BTreeMap::new();
        for part in parts {
            if let Some((key, value)) = part.split_once(':') {
                cells.insert(key.to_string(), value.to_string());
            }
        }
        out.push(Row {
            sort: sort.to_string(),
            cells,
        });
    }
    // **A count over nothing is the same failure with the sign flipped** - `CLAUDE.md`. Reading
    // no rows would make every rule move nothing, and a weighting for nothing always exists.
    assert!(out.len() > 50, "{at} parsed to {} row(s)", out.len());
    out
}

/// What `spec/data/schema.4x` says about kinds, so a clause can be ground to them.
struct Schema {
    /// A column id to its name.
    column: BTreeMap<String, String>,
    /// A family to its members.
    members: BTreeMap<String, Vec<String>>,
    /// A kind to the traits it carries.
    carries: BTreeMap<String, Vec<String>>,
    /// Every name declared by `{trait ...}`, which is what makes a cell a state.
    traits: BTreeSet<String>,
}

impl Schema {
    fn read() -> Schema {
        let mut it = Schema {
            column: BTreeMap::new(),
            members: BTreeMap::new(),
            carries: BTreeMap::new(),
            traits: BTreeSet::new(),
        };
        for row in rows_of("spec/data/schema.4x") {
            match row.sort.as_str() {
                "column" => {
                    if let (Some(id), Some(name)) = (row.at("id"), row.at("name")) {
                        it.column.insert(id.into(), name.into());
                    }
                }
                "member" => {
                    if let (Some(kind), Some(family)) = (row.at("kind"), row.at("family")) {
                        it.members
                            .entry(family.into())
                            .or_default()
                            .push(kind.into());
                    }
                }
                "carries" => {
                    if let (Some(kind), Some(what)) = (row.at("kind"), row.at("trait")) {
                        it.carries.entry(kind.into()).or_default().push(what.into());
                    }
                }
                "trait" => {
                    if let Some(name) = row.at("name") {
                        it.traits.insert(name.into());
                    }
                }
                _ => {}
            }
        }
        assert!(
            !it.traits.is_empty(),
            "the schema declares no traits, so every place would be a bare kind and `move` \
             would net to nothing"
        );
        it
    }

    fn named(&self, id: &str) -> &str {
        self.column
            .get(id)
            .unwrap_or_else(|| panic!("`spec/data/schema.4x` has no column {id}"))
    }

    /// The kinds a relation stands for - itself, or every member if it is a family.
    ///
    /// **A family is not a place.** It is a way of writing one rule for each of its members, so
    /// `move` over `unit` is four rules and the weighting has to satisfy every one.
    fn kinds_of(&self, relation: &str) -> Vec<String> {
        match self.members.get(relation) {
            Some(kinds) => kinds.clone(),
            None => vec![relation.to_string()],
        }
    }

    fn carries_it(&self, kind: &str, what: &str) -> bool {
        self.carries
            .get(kind)
            .is_some_and(|all| all.iter().any(|it| it == what))
    }
}

/// One clause, with its columns resolved to names.
struct Clause {
    name: String,
    seq: i64,
    role: String,
    relation: String,
    /// Column name to value, from `{literal ...}`.
    fixed: BTreeMap<String, String>,
    /// Whether a `{reading ...}` fills this clause's quantity from a row matched earlier.
    quantity_is_read: bool,
    /// The input a `{relation-of ...}` takes this clause's relation from, where there is one.
    ///
    /// **This is what says a rule is about a family rather than a kind.** `work`'s add clause is
    /// declared over `resource` and carries one, so the rule is three rules; its extractor and
    /// labor clauses carry none, and are the kinds they name.
    relation_of: Option<String>,
    /// A `{put ...}`'s `{assigns input:<name> value:<v>}`.
    assigns: Option<(String, String)>,
}

/// A rule as the data states it, before it is ground to kinds.
struct Stated {
    name: String,
    clauses: Vec<Clause>,
    /// Every distinct way a `{part ...}` calls it, as the arguments passed.
    called_with: Vec<BTreeMap<String, String>>,
}

/// The sixteen rules, read but not yet ground.
fn stated(schema: &Schema) -> Vec<Stated> {
    let rows = rows_of("spec/data/rules.4x");

    let mut named: Vec<String> = Vec::new();
    let mut of_rule: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut clause: BTreeMap<String, Clause> = BTreeMap::new();
    for row in &rows {
        match row.sort.as_str() {
            "rule" => named.push(row.at("name").expect("a rule has a name").into()),
            "clause" => {
                let (Some(name), Some(rule), Some(role), Some(relation), Some(seq)) = (
                    row.at("name"),
                    row.at("rule"),
                    row.at("role"),
                    row.at("relation"),
                    row.at("seq"),
                ) else {
                    continue;
                };
                of_rule.entry(rule.into()).or_default().push(name.into());
                clause.insert(
                    name.into(),
                    Clause {
                        name: name.into(),
                        seq: seq.parse().unwrap_or(0),
                        role: role.into(),
                        relation: relation.into(),
                        fixed: BTreeMap::new(),
                        quantity_is_read: false,
                        relation_of: None,
                        assigns: None,
                    },
                );
            }
            _ => {}
        }
    }

    for row in &rows {
        let Some(one) = row.at("clause").and_then(|it| clause.get_mut(it)) else {
            continue;
        };
        match row.sort.as_str() {
            "literal" => {
                if let (Some(id), Some(value)) = (row.at("column"), row.at("value")) {
                    one.fixed.insert(schema.named(id).into(), value.into());
                }
            }
            "reading" => {
                if row.at("column").map(|id| schema.named(id)) == Some("quantity") {
                    one.quantity_is_read = true;
                }
            }
            "assigns" => {
                if let (Some(input), Some(value)) = (row.at("input"), row.at("value")) {
                    one.assigns = Some((input.into(), value.into()));
                }
            }
            "relation-of" => {
                if let Some(input) = row.at("input") {
                    one.relation_of = Some(input.into());
                }
            }
            _ => {}
        }
    }

    // **How a `{part ...}` calls a rule is how that rule grounds.** `refresh` is written once and
    // `end-turn` calls it six times, each naming a kind and a trait - and nothing else in the
    // data says which traits it is for. A rule nobody calls with arguments grounds as itself.
    let mut arguments: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
    for row in &rows {
        if row.sort == "argument"
            && let (Some(part), Some(input), Some(value)) =
                (row.at("part"), row.at("input"), row.at("value"))
        {
            arguments
                .entry(part.into())
                .or_default()
                .insert(input.into(), value.into());
        }
    }
    let mut called_with: BTreeMap<String, Vec<BTreeMap<String, String>>> = BTreeMap::new();
    for row in &rows {
        if row.sort == "part"
            && let (Some(name), Some(is)) = (row.at("name"), row.at("is"))
            && let Some(with) = arguments.get(name).filter(|it| !it.is_empty())
        {
            let ways = called_with.entry(is.into()).or_default();
            if !ways.contains(with) {
                ways.push(with.clone());
            }
        }
    }

    let mut out = Vec::new();
    for name in named {
        let mut mine: Vec<Clause> = of_rule
            .remove(&name)
            .unwrap_or_default()
            .into_iter()
            .filter_map(|it| clause.remove(&it))
            .collect();
        mine.sort_by_key(|it| it.seq);
        out.push(Stated {
            called_with: called_with.remove(&name).unwrap_or_default(),
            clauses: mine,
            name,
        });
    }
    out
}

// -- Grounding a rule to places -----------------------------------------------------------------

/// How much of a thing a clause moves.
enum Many {
    /// A `{literal ...}` on the quantity column.
    Fixed(i64),
    /// A `{reading ...}` of a required row - a deposit's density, and the one unbounded draw.
    Read,
}

/// What a rule does, and how it was arrived at, so the report can show the second.
pub struct Ground {
    pub rule: Rule,
    /// The line under the rule in the report: how this grounding was reached.
    pub how: String,
}

/// **Rules that move nothing**, named rather than dropped - `end-turn`, which is only its parts,
/// and `discard-disorder`, whose one clause is a `keep`.
pub struct Read {
    pub ground: Vec<Ground>,
    pub moves_nothing: Vec<String>,
    pub rules_read: usize,
}

/// Every rule in `spec/data/rules.4x`, ground to kinds and to the places it moves things between.
pub fn read() -> Read {
    let schema = Schema::read();
    let all = stated(&schema);
    let rules_read = all.len();
    let mut ground = Vec::new();
    let mut moves_nothing = Vec::new();

    for one in &all {
        // A rule with no arguments grounds once, as itself.
        let ways: Vec<BTreeMap<String, String>> = if one.called_with.is_empty() {
            vec![BTreeMap::new()]
        } else {
            one.called_with.clone()
        };
        let mut any = false;
        for with in &ways {
            for (_name, rule, how) in grounded(&schema, one, with) {
                any = true;
                ground.push(Ground { rule, how });
            }
        }
        if !any {
            moves_nothing.push(one.name.clone());
        }
    }

    assert_eq!(
        rules_read, 16,
        "`spec/data/rules.4x` states {rules_read} rule(s) and this was written against sixteen - \
         a rule has been added or removed, and the reader should be looked at before its answer \
         is believed"
    );
    Read {
        ground,
        moves_nothing,
        rules_read,
    }
}

/// One stated rule, as every rule it stands for.
fn grounded(
    schema: &Schema,
    one: &Stated,
    with: &BTreeMap<String, String>,
) -> Vec<(String, Rule, String)> {
    // **What a rule is about is what its `{relation-of ...}` clause is declared over**, and the
    // argument where a `{part ...}` gives one. `work` is about `resource`, so it is three rules;
    // `refresh` is declared over `unit` and called with `extractor`, and the call is what says so.
    //
    // **Summing the members instead would be a different rule.** `work` adds one resource, and a
    // reading that merged its three groundings had it add metal and food and energy at once,
    // against three of the planet - a rule nobody wrote, and one whose arithmetic happens to
    // balance where the real three might not.
    let subject = with.get("what").cloned().or_else(|| {
        one.clauses
            .iter()
            .find(|it| it.relation_of.is_some())
            .map(|it| it.relation.clone())
    });

    // One spelling per member of the subject - or, where the rule has no subject, one spelling
    // in which every clause is the kind it names.
    let spellings: Vec<Option<String>> = match &subject {
        Some(what) => schema.kinds_of(what).into_iter().map(Some).collect(),
        None => vec![None],
    };

    let mut out = Vec::new();
    for spelling in spellings {
        let mut delta: BTreeMap<Place, i64> = BTreeMap::new();
        let mut reads: Vec<(Place, Place)> = Vec::new();
        let mut said: Vec<String> = Vec::new();

        for clause in &one.clauses {
            // **A clause takes the subject's kind only if it is the clause the subject came
            // from.** `work`'s extractor and labor clauses are those kinds whichever resource
            // is being worked, and reading them as the resource would have `work (energy)`
            // consuming an energy rather than an extractor.
            let kind = match (&spelling, &clause.relation_of) {
                (Some(kind), Some(_)) => kind.clone(),
                _ => {
                    assert!(
                        !(schema.members.contains_key(&clause.relation)
                            && matches!(clause.role.as_str(), "add" | "remove")),
                        "`{}` in `{}` moves a `{}`, which is a family, and names no \
                         `{{relation-of ...}}` to say which member - so there is no one kind \
                         this rule is about and the reader will not choose one",
                        clause.name,
                        one.name,
                        clause.relation
                    );
                    clause.relation.clone()
                }
            };
            let kinds = vec![kind];
            let quantity = many(clause);
            for kind in kinds {
                match clause.role.as_str() {
                    "require" | "keep" => {}
                    "remove" | "add" => {
                        let sign = if clause.role == "add" { 1 } else { -1 };
                        // **A remove that states no quantity takes the whole row it matched**,
                        // which is `perish` and nothing else. How many that is depends on the
                        // world, and counting it as one is safe in the direction that matters:
                        // every weight is at least one, so a larger draw only lowers the sum
                        // further. **An add is not like that**, and this refuses to guess at one.
                        let quantity = quantity.as_ref().unwrap_or_else(|| {
                            assert_eq!(
                                clause.role, "remove",
                                "`{}` in `{}` adds an amount it neither states nor reads, and \
                                 this reader will not guess at how much",
                                clause.name, one.name
                            );
                            &Many::Fixed(1)
                        });
                        match quantity {
                            Many::Fixed(many) => {
                                for place in places(schema, &kind, &clause.fixed) {
                                    *delta.entry(place).or_insert(0) += sign * many;
                                }
                            }
                            Many::Read => {
                                // **A quantity read out of a required deposit draws on the
                                // planet**, at the same rate as it arrives. The density is the
                                // well; the extractor in front of it is what bounds the rate.
                                let made = Place {
                                    kind: kind.clone(),
                                    state: String::new(),
                                };
                                let from = Place {
                                    kind: "the planet".into(),
                                    state: String::new(),
                                };
                                *delta.entry(made.clone()).or_insert(0) += sign;
                                *delta.entry(from.clone()).or_insert(0) -= sign;
                                reads.push((made, from));
                                said.push(format!(
                                    "`{}` reads its quantity from a required row, so it is \
                                     counted once here and again on its own",
                                    clause.name
                                ));
                            }
                        }
                    }
                    "put" => {
                        let Some((input, to)) = &clause.assigns else {
                            continue;
                        };
                        // The trait is named by an argument where the call gives one.
                        let what = with.get(input).map(String::as_str).unwrap_or(input);
                        if !schema.carries_it(&kind, what) {
                            continue;
                        }
                        // **A put raises a count, and the invariant says what pays for it**:
                        // *anything that exhausts draws on time for a turn*.
                        *delta
                            .entry(Place {
                                kind: "time".into(),
                                state: String::new(),
                            })
                            .or_insert(0) -= 1;
                        *delta
                            .entry(Place {
                                kind: kind.clone(),
                                state: format!("{what} 0"),
                            })
                            .or_insert(0) -= 1;
                        *delta
                            .entry(Place {
                                kind: kind.clone(),
                                state: format!("{what} {to}"),
                            })
                            .or_insert(0) += 1;
                    }
                    other => panic!(
                        "`{}` has role `{other}`, which this reader does not know",
                        clause.name
                    ),
                }
            }
        }

        delta.retain(|_, by| *by != 0);
        if delta.is_empty() {
            continue;
        }

        let name = match (&spelling, with.is_empty()) {
            (Some(kind), _) => {
                let rest: Vec<&str> = with
                    .iter()
                    .filter(|(key, _)| key.as_str() != "what")
                    .map(|(_, value)| value.as_str())
                    .collect();
                if rest.is_empty() {
                    format!("{} ({kind})", one.name)
                } else {
                    format!("{} ({kind}, {})", one.name, rest.join(", "))
                }
            }
            (None, _) => one.name.clone(),
        };
        let how = if said.is_empty() {
            String::new()
        } else {
            said.join(" ")
        };
        out.push((
            name.clone(),
            Rule {
                name: name.clone(),
                delta,
            },
            how,
        ));

        // **The unbounded draw gets a rule of its own.** A clause whose quantity is read moves
        // `n` of something and `n` from a source, for an `n` nobody here knows. Counting it once
        // above fixes `n = 1`; this row is the coefficient of `n` alone, and the two together
        // cover every `n` - because a weighting non-increasing at `n = 1` and non-increasing per
        // further unit is non-increasing at all of them.
        for (made, from) in reads {
            let mut only: BTreeMap<Place, i64> = BTreeMap::new();
            *only.entry(made).or_insert(0) += 1;
            *only.entry(from).or_insert(0) -= 1;
            out.push((
                name.clone(),
                Rule {
                    name: format!("{name}, per unit of density"),
                    delta: only,
                },
                "the coefficient of the density, which fixing it at one would not cover".into(),
            ));
        }
    }
    out
}

/// How much a clause moves, or nothing where it moves no count at all.
fn many(clause: &Clause) -> Option<Many> {
    if clause.quantity_is_read {
        return Some(Many::Read);
    }
    clause
        .fixed
        .get("quantity")
        .and_then(|it| it.parse().ok())
        .map(Many::Fixed)
}

/// The places one clause of one kind touches.
///
/// **The bare kind, and one place per trait the clause pins.** A weighting over the bare count
/// alone would report `move` as doing nothing, because it takes a unit and makes a unit.
///
/// **Only a `{trait ...}` makes a state, and `where` never does.** Merging places can only give
/// the solver less to work with - a weighting found over merged places is one that happens to
/// agree across them, which is still a weighting over the finer ones. So the error this can
/// make is a false alarm, never a false pass.
fn places(schema: &Schema, kind: &str, fixed: &BTreeMap<String, String>) -> Vec<Place> {
    let mut out = vec![Place {
        kind: kind.to_string(),
        state: String::new(),
    }];
    for (column, value) in fixed {
        if schema.traits.contains(column) {
            out.push(Place {
                kind: kind.to_string(),
                state: format!("{column} {value}"),
            });
        }
    }
    out
}

// -- The decision, and the page it is read on ---------------------------------------------------

/// What the rules were read to be, and what the solver made of them.
pub struct Decision {
    pub read: Read,
    pub found: Found,
}

impl Decision {
    /// Every ground rule, in the order the report shows them.
    pub fn rules(&self) -> Vec<Rule> {
        self.read.ground.iter().map(|it| it.rule.clone()).collect()
    }

    /// What the weighting makes of one rule: made minus taken, weighed.
    ///
    /// **This is the whole claim, per rule.** A reader who does not believe the verdict can add
    /// a row up by hand from the table above it, which is why both are on the page.
    fn margin(&self, weights: &BTreeMap<Place, i64>, rule: &Rule) -> i64 {
        rule.delta
            .iter()
            .map(|(place, by)| by * weights.get(place).copied().unwrap_or(0))
            .sum()
    }
}

/// Read the rules and decide.
pub fn decided() -> Decision {
    let read = read();
    let found = solve(
        &read
            .ground
            .iter()
            .map(|it| it.rule.clone())
            .collect::<Vec<_>>(),
    );
    Decision { read, found }
}

/// One cell of a table, and whether it is a figure.
struct Cell {
    said: String,
    class: &'static str,
}

fn text(said: impl Into<String>) -> Cell {
    Cell {
        said: said.into(),
        class: "",
    }
}

/// A part of the page: a heading, a paragraph, or a table written once and rendered twice.
enum Part {
    Said(String),
    Table {
        heading: String,
        said: String,
        columns: Vec<&'static str>,
        rows: Vec<Vec<Cell>>,
    },
}

fn escaped(said: &str) -> String {
    said.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// `**bold**` and `` `code` `` into spans, so one string serves both forms.
fn marked(said: &str) -> String {
    let mut out = escaped(said);
    for (mark, open, close) in [("**", "<strong>", "</strong>"), ("`", "<code>", "</code>")] {
        let mut made = String::new();
        let mut rest = out.as_str();
        let mut opening = true;
        while let Some(at) = rest.find(mark) {
            made.push_str(&rest[..at]);
            made.push_str(if opening { open } else { close });
            opening = !opening;
            rest = &rest[at + mark.len()..];
        }
        made.push_str(rest);
        // An odd number of marks would leave a tag unclosed, so the text is left as it was.
        out = if opening { made } else { out };
    }
    out
}

/// The page, written once as parts and rendered into both forms.
fn page(parts: &[Part], title: &str, note: &str) -> (String, String) {
    let mut html = format!(
        "<!doctype html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n\
         <title>{}</title>\n<link rel=\"stylesheet\" href=\"report.css\">\n</head>\n<body>\n\
         <h1>{}</h1>\n<p class=\"note\">{}</p>\n\
         <p class=\"where\"><a href=\"index.html\">The index</a> \u{b7} \
         <a href=\"nogain.md\">this page as markdown</a></p>\n",
        escaped(title),
        escaped(title),
        marked(note)
    );
    let mut markdown = format!("# {title}\n\n{note}\n\n[The index](index.md)\n");

    for part in parts {
        match part {
            Part::Said(said) => {
                html.push_str(&format!("<p>{}</p>\n", marked(said)));
                markdown.push_str(&format!("\n{said}\n"));
            }
            Part::Table {
                heading,
                said,
                columns,
                rows,
            } => {
                html.push_str(&format!(
                    "<h2>{}  ({})</h2>\n<p>{}</p>\n<table>\n<tr>",
                    escaped(heading),
                    rows.len(),
                    marked(said)
                ));
                markdown.push_str(&format!("\n## {heading}  ({})\n\n{said}\n\n", rows.len()));
                for (at, column) in columns.iter().enumerate() {
                    let class = rows
                        .first()
                        .and_then(|row| row.get(at))
                        .map(|cell| cell.class)
                        .unwrap_or("");
                    html.push_str(&format!(
                        "<th{}>{}</th>",
                        if class.is_empty() {
                            String::new()
                        } else {
                            format!(" class=\"{class}\"")
                        },
                        escaped(column)
                    ));
                }
                html.push_str("</tr>\n");
                markdown.push_str(&format!("| {} |\n", columns.join(" | ")));
                markdown.push_str(&format!(
                    "| {} |\n",
                    columns
                        .iter()
                        .map(|_| "---")
                        .collect::<Vec<_>>()
                        .join(" | ")
                ));
                for row in rows {
                    html.push_str("<tr>");
                    for cell in row {
                        html.push_str(&format!(
                            "<td{}>{}</td>",
                            if cell.class.is_empty() {
                                String::new()
                            } else {
                                format!(" class=\"{}\"", cell.class)
                            },
                            marked(&cell.said)
                        ));
                    }
                    html.push_str("</tr>\n");
                    markdown.push_str(&format!(
                        "| {} |\n",
                        row.iter()
                            .map(|it| it.said.replace('|', "\\|"))
                            .collect::<Vec<_>>()
                            .join(" | ")
                    ));
                }
                html.push_str("</table>\n");
            }
        }
    }
    html.push_str("</body>\n</html>\n");
    (html, markdown)
}

/// What one rule takes and makes, as the report spells it.
fn moved(rule: &Rule, sign: i64) -> String {
    let mut said: Vec<String> = rule
        .delta
        .iter()
        .filter(|(_, by)| by.signum() == sign)
        .map(|(place, by)| {
            let many = by.abs();
            if many == 1 {
                place.label()
            } else {
                format!("{many} \u{d7} {}", place.label())
            }
        })
        .collect();
    said.sort();
    if said.is_empty() {
        "\u{2014}".to_string()
    } else {
        // **Not a comma, because a place's own label has one in it.** `extractor, working 1 ·
        // labor · the planet` is three things; written with commas it reads as four.
        said.join(" \u{b7} ")
    }
}

/// Write `reports/nogain.html` and `reports/nogain.md`, and say how many moved.
pub fn write_report(decision: &Decision) -> usize {
    let mut parts = Vec::new();

    let weights = match &decision.found {
        Found::Weighting(weights) => Some(weights),
        Found::NotFound { .. } => None,
    };

    parts.push(Part::Said(format!(
        "**{}** rules in `spec/data/rules.4x` ground to **{}**, over **{}** places. \
         A rule over a family is one rule for each of its members, and a clause whose quantity is \
         read gets a second row for the coefficient of what it reads.",
        decision.read.rules_read,
        decision.read.ground.len(),
        weights.map(|it| it.len()).unwrap_or(0)
    )));

    match &decision.found {
        Found::Weighting(_) => parts.push(Part::Said(
            "**A weighting exists, so nothing comes back round with more.** Every rule below is \
             non-increasing under it, so any sequence of them is too - a cycle included. That is \
             the whole of the argument: it does not depend on which rules a player picks, or on \
             how many times, because each one alone never raises the total."
                .to_string(),
        )),
        Found::NotFound { forcing } => parts.push(Part::Said(format!(
            "**No weighting exists, so some sequence of rules gains.** The rules that make \
             something without taking anything of equal weight are the place to look: {}.",
            forcing
                .iter()
                .map(|it| format!("`{it}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ))),
    }

    if let Some(weights) = weights {
        let tight = decision
            .read
            .ground
            .iter()
            .filter(|it| decision.margin(weights, &it.rule) == 0)
            .count();
        parts.push(Part::Table {
            heading: "Every rule under the weighting".to_string(),
            said: format!(
                "Made minus taken, weighed. **Every figure is at or below zero**, and **{tight}** \
                 of them are exactly zero - the loops the weighting is tight around, where what \
                 comes back is worth precisely what went in. A rule that could gain would show a \
                 figure above zero, and none can, because the weighting was solved for rather \
                 than chosen."
            ),
            columns: vec!["Rule", "Takes", "Makes", "Weighed"],
            rows: decision
                .read
                .ground
                .iter()
                .map(|one| {
                    let margin = decision.margin(weights, &one.rule);
                    vec![
                        text(format!("`{}`", one.rule.name)),
                        text(moved(&one.rule, -1)),
                        text(moved(&one.rule, 1)),
                        Cell {
                            said: margin.to_string(),
                            class: if margin == 0 {
                                "figure tight"
                            } else {
                                "figure"
                            },
                        },
                    ]
                })
                .collect(),
        });

        parts.push(Part::Table {
            heading: "The weighting".to_string(),
            said: "**Nothing here is declared.** These numbers are solved for, and the only input \
                   is `spec/data/rules.4x` - so they are a consequence of the rules rather than a \
                   statement about them. Any positive multiple of them would do as well; these \
                   are the smallest whole numbers the solver reached."
                .to_string(),
            columns: vec!["Place", "Weight"],
            rows: weights
                .iter()
                .map(|(place, weight)| {
                    vec![
                        text(place.label()),
                        Cell {
                            said: weight.to_string(),
                            class: if SOURCES.contains(&place.kind.as_str()) {
                                "figure slack"
                            } else {
                                "figure"
                            },
                        },
                    ]
                })
                .collect(),
        });
    }

    if !decision.read.moves_nothing.is_empty() {
        parts.push(Part::Table {
            heading: "Rules that move nothing".to_string(),
            said: "**Named rather than dropped**, so that the count above can be reconciled with \
                   the sixteen in the data. Neither is a gap: `end-turn` is exactly its parts, \
                   and each of those is a rule in its own right above; `discard-disorder` keeps \
                   what it matches and takes nothing away."
                .to_string(),
            columns: vec!["Rule"],
            rows: decision
                .read
                .moves_nothing
                .iter()
                .map(|name| vec![text(format!("`{name}`"))])
                .collect(),
        });
    }

    parts.push(Part::Said(
        "**What this reads, and the one thing it guesses, is at the top of \
         `crates/game-model/examples/nogain.rs`.** The short of it: a place is a kind, or a kind \
         with one trait pinned, because a weighting over bare kinds would call `move` and \
         `refresh` no-ops and report the whole economy as doing nothing. A `put` draws on time \
         and a density reading draws on the planet, which is `spec/invariants.md`'s own sentence \
         about what exhausts. **Which well a density reading draws on is the guess** - the data \
         names no sources, so every one is charged to the planet and the star is drawn on by \
         nothing, which is `C-166`."
            .to_string(),
    ));

    let (html, markdown) = page(
        &parts,
        "Nothing comes back round with more",
        "Generated from `spec/data/rules.4x` by `scripts/reports.sh`. **Not canonical** - the \
         rules are, and this is what follows from them.",
    );

    let out = root().join("reports");
    std::fs::create_dir_all(&out).expect("reports/");
    let mut written = 0;
    for (name, text) in [("nogain.html", html), ("nogain.md", markdown)] {
        let at = out.join(name);
        if std::fs::read_to_string(&at).unwrap_or_default() != text {
            std::fs::write(&at, &text).unwrap_or_else(|why| panic!("{name}: {why}"));
            written += 1;
        }
    }
    written
}

fn main() {
    let decision = decided();
    let weights = match &decision.found {
        Found::Weighting(weights) => weights.clone(),
        Found::NotFound { forcing } => {
            eprintln!("No weighting exists. Look at: {}", forcing.join(", "));
            BTreeMap::new()
        }
    };
    // **The working is printed on request rather than always**, so that `scripts/reports.sh`
    // says what moved without burying it under thirty-two rules.
    if std::env::args().any(|it| it == "--working") {
        for one in &decision.read.ground {
            let margin = decision.margin(&weights, &one.rule);
            println!("{:>6}  {}", margin, one.rule.name);
            for (place, by) in &one.rule.delta {
                println!("        {by:+3}  {}", place.label());
            }
        }
    }
    let written = write_report(&decision);
    println!(
        "reports/nogain: {} rule(s) read, {} ground, {} place(s); {written} page file(s) rewritten",
        decision.read.rules_read,
        decision.read.ground.len(),
        weights.len()
    );
}
