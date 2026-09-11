//! Nothing comes back round with more, decided from the rules alone - `S-93`.
//!
//! Sean wants `spec/invariants.md` -> *Nothing comes back round with more* **enforced rather
//! than believed**, and that section says how: *whether this holds is decided mechanically,
//! from the rules alone*.
//!
//! > There is a weighting of the kinds, and under it no sequence of rules ends holding more
//! > than it began with.
//!
//! So: find a weighting `w` with `w · (made - taken) <= 0` at every rule. If one exists, no
//! sequence can gain - each firing is non-increasing in `w`, so a cycle is too. This is a
//! **P-invariant** in the Petri net sense, relaxed from equality to an inequality because
//! disorder is meant: *it may end with less - that is disorder*.
//!
//! # Why this is a second derivation and not a layer on the drawing
//!
//! `crates/game-console/src/petri.rs` draws places at `(container, kind)`. **At that
//! granularity `work` takes an extractor and makes an extractor and nets to zero**, and
//! `refresh` takes a thing and makes a thing and nets to zero too - so a weighting over those
//! places would report the readiness economy as doing nothing at all. That is a green run
//! about the wrong question, which is this repository's recurring failure.
//!
//! **So the places here are `(kind, state)`**, and readiness is a state. Two readings of one
//! table at two granularities, and the finer one is this. Found in the drawing rather than
//! predicted - `S-93` was corrected by it.
//!
//! # The three sources
//!
//! `spec/invariants.md`, since `P-388`:
//!
//! > There are three sources: the planet, the star, and time. The planet's material and the
//! > star's energy are endless, and so are time's turns. **Anything that exhausts is a
//! > readiness extractor for a turn**: it draws one readiness out of time and is spent doing
//! > it, the way an extractor draws material out of the planet and is spent doing it
//!
//! **A source is a place like any other, and that is what makes the check bite.** Naming a
//! source as an exemption would let every rule that touches it out of the arithmetic; naming
//! it as a place a rule *takes from* keeps the rule in, and says only that the well behind it
//! is endless. So `work` takes one from the planet and `refresh` takes one from time, and
//! everything else has to balance without them.
//!
//! **`refresh` is therefore not an exception carved out of the check** - `S-93` is explicit
//! about this, and it is Sean's own framing rather than a reading of it: readiness is gathered
//! rather than made, and what bounds the gathering is the count of things that exhaust.

use std::collections::BTreeMap;

/// Somewhere a thing of one kind, in one state, can be.
///
/// **A kind alone is not enough**, which is the finding that keeps this check from being
/// vacuous. `citizen` and `citizen, spent` are different places: `bear` moves one to the other
/// and a weighting blind to the difference sees `bear` doing nothing.
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

/// Which kinds a family stands for, read from the release's *Families* table.
///
/// **`P-373`: a rule whose subject is a family of kinds is a rule for each of them.** So a
/// family is not a place - it is a way of writing several rules at once, and grounding it is
/// reading the specification rather than modelling anything. `X-29` names the same move from
/// the other side: *colours are the families, grounding is unfolding*.
///
/// **`thing` is written *every kind above* rather than as a list**, which both joins in
/// `prototypes/kinds` once split on commas and missed - `C-71`. It is read as membership here.
pub fn family(document: &str, name: &str) -> Option<Vec<String>> {
    for row in crate::recipes::body_under(document, "## Families") {
        if crate::recipes::plain(row.first().map(String::as_str).unwrap_or_default()) != name {
            continue;
        }
        let members = row.get(1).cloned().unwrap_or_default();
        if members.contains("every kind above") {
            return Some(
                crate::recipes::body_under(document, "## Kinds")
                    .iter()
                    .filter_map(|row| row.first())
                    .map(|cell| crate::recipes::plain(cell))
                    .filter(|kind| !kind.is_empty())
                    .collect(),
            );
        }
        return Some(
            members
                .split(',')
                .map(|word| word.trim().to_string())
                .filter(|word| !word.is_empty())
                .collect(),
        );
    }
    None
}

/// The kinds the release says readiness applies to.
///
/// **Read from the *Readies* column, and the release closes the set itself**: *Nothing outside
/// this table readies.* So grounding `refresh`'s `thing` against every kind would invent three
/// readiness economies the game does not have - a garrison, a store and a yard that exhaust.
pub fn readies(document: &str) -> Vec<String> {
    crate::recipes::body_under(document, "## Units and structures")
        .iter()
        .filter(|row| row.get(8).map(|cell| cell.trim() == "yes").unwrap_or(false))
        .filter_map(|row| row.first())
        .map(|cell| crate::recipes::plain(cell))
        .collect()
}

/// The kinds made with a `keeps` counter, which is what `age` and `spoil` act on.
///
/// **One sentence in the release and it names one kind**: *Food is made with `keeps` 1.* So
/// `age` and `spoil`, whose subject is `thing`, ground to food alone - and a version that
/// ground them against every kind would have twelve kinds ageing, which is a bigger game than
/// this release specifies.
pub fn keeps(document: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in document.lines() {
        // Matched on the sentence rather than on the word, because `keeps` appears in the
        // Traits table and in three recipe rows, and neither is a declaration of which kinds
        // have one.
        //
        // **Found within the line rather than at its end**, because the sentence shares a line
        // with the next one: *Food is made with `keeps` 1. The force nature holds a territory
        // with.* A `strip_suffix` matched nothing, and the guard in `groundings` is what said
        // so - it would otherwise have ground `age` and `spoil` to no kind and dropped two
        // rules from the arithmetic without a word.
        let Some(at) = line.find(" is made with `keeps` ") else {
            continue;
        };
        let subject = line[..at]
            .trim()
            .rsplit(['.', '*'])
            .next()
            .unwrap_or_default();
        let subject = subject.trim().to_lowercase();
        if !subject.is_empty() {
            out.push(subject);
        }
    }
    out
}

/// Every rule, ground to constants and to single kinds.
///
/// **Three groundings, each one a rule the specification states:**
///
/// - **A family becomes its members** - `P-373` - narrowed by what the release says the trait
///   applies to. `refresh` names `thing` and readiness is closed to the *Readies* column;
///   `age` and `spoil` name `thing` and `keeps` is declared of food alone.
/// - **A density becomes its cases** - `P-376` - one per `(resource, density)` the planet
///   offers, which [`crate::petri::densities`] reads.
/// - **A repeated name is separate rules already**: `stow` and `discard` are stated once per
///   kind, so nothing has to be done for them.
pub fn rules(document: &str) -> Vec<Rule> {
    let rows = crate::recipes::body_under(document, "## Recipes");
    let readies = readies(document);
    let keeps = keeps(document);
    let capable = capable(document);

    // Gathered by block, the way the table states them: the name is on the first row only.
    let mut blocks: Vec<(String, Vec<Vec<String>>)> = Vec::new();
    for row in &rows {
        let name = crate::recipes::plain(row.first().map(String::as_str).unwrap_or_default());
        if !name.is_empty() {
            blocks.push((name, Vec::new()));
        }
        let Some((_, lines)) = blocks.last_mut() else {
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

    let mut out = Vec::new();
    for (name, lines) in &blocks {
        for (suffix, ground) in groundings(document, name, lines, &readies, &keeps, &capable) {
            let mut delta: BTreeMap<Place, i64> = BTreeMap::new();
            for (place, change) in ground {
                *delta.entry(place).or_insert(0) += change;
            }
            // **A change that cancels is dropped and the rule is kept.** A rule whose every
            // change cancels is exactly what a weighting has nothing to say about, and
            // dropping the rule here would hide it from a reader looking for one.
            delta.retain(|_, change| *change != 0);

            // **A rule that refills a capacity without making the thing draws on time** -
            // `P-388`: *anything that exhausts is a readiness extractor for a turn ... it
            // draws one readiness out of time*.
            //
            // **Without the second half this catches every rule that makes anything.** A new
            // citizen arrives ready and fertile, so `breed` and `deploy ark` raise a capacity
            // place too - and their capacity came with the thing, which was paid for. What
            // marks a draw is raising the capacity of things that were **already there**: the
            // count of the kind does not move and its capacity does.
            let refills: Vec<Place> = delta
                .iter()
                .filter(|(place, change)| {
                    **change > 0 && CAPACITIES.iter().any(|(held, _)| place.state == *held)
                })
                .map(|(place, _)| place.clone())
                .filter(|place| {
                    let kind = Place {
                        kind: place.kind.clone(),
                        state: String::new(),
                    };
                    delta.get(&kind).copied().unwrap_or(0) <= 0
                })
                .collect();
            if !refills.is_empty() {
                let drawn: i64 = refills
                    .iter()
                    .map(|place| delta.get(place).copied().unwrap_or(0))
                    .sum();
                *delta
                    .entry(Place {
                        kind: "time".to_string(),
                        state: String::new(),
                    })
                    .or_insert(0) -= drawn;
            }
            out.push(Rule {
                name: format!("{name}{suffix}"),
                delta,
            });
        }
    }
    out
}

/// One block, as the one or several rules it stands for, each a list of place changes.
fn groundings(
    document: &str,
    name: &str,
    lines: &[Vec<String>],
    readies: &[String],
    keeps: &[String],
    capable: &std::collections::BTreeSet<(String, String)>,
) -> Vec<(String, Vec<(Place, i64)>)> {
    // **The density cases first**, because a block with one is `work` and its other rows are
    // the same in every case.
    let per_density = lines.iter().any(|row| {
        row.get(3)
            .map(|cell| cell.trim() == crate::petri::PER_DENSITY)
            .unwrap_or(false)
    });
    if per_density {
        return crate::petri::densities(document)
            .into_iter()
            .map(|(resource, density)| {
                let mut changes = Vec::new();
                for row in lines {
                    changes.extend(changed(row, Some((&resource, density)), None, capable));
                }
                // **The planet is what the material came from.** `work` draws it out of the
                // ground and is spent doing so - the extractor it took is not ready afterwards
                // - so the source is taken from rather than exempted, and the rule stays in
                // the arithmetic instead of being let out of it.
                changes.push((source_for(&resource), -1));
                (format!(" ({resource} x{density})"), changes)
            })
            .collect();
    }

    // **Any family in a row's Kind column becomes the members the release admits** - `P-373`,
    // and it applies to every family rather than to `thing` alone.
    //
    // **`unit` was left as a family and that was the same blindness one level down.** `move`
    // spends a unit's readiness and `refresh` puts an ark's back, so a place called
    // *unit, ready* and a place called *ark, ready* were two unconnected economies - readiness
    // going out of one and into the other, and a weighting with nothing to say about either.
    // Found by reading the report rather than by a test, which is why the report is generated
    // before it is trusted.
    let families: Vec<String> = ["thing", "unit", "resource", "place"]
        .iter()
        .map(|name| (*name).to_string())
        .collect();
    //
    // **Asked of the rows that move something, which is not every row.** `move` names two
    // families: `place`, on the two `require` rows that say where it goes, and `unit`, on the
    // rows that actually take and make. Grounding on the first family found gave
    // `move (territory)` and `move (orbit)` - one rule per *destination*, each having lost the
    // unit entirely. A `require` moves nothing, so it cannot be what a rule is ground over.
    let named_family = lines.iter().find_map(|row| {
        let role = row.get(2).map(String::as_str).unwrap_or_default().trim();
        if role != "produce" && role != "consume" {
            return None;
        }
        let kind = crate::recipes::plain(row.get(4).map(String::as_str).unwrap_or_default());
        families.contains(&kind).then_some(kind)
    });
    if let Some(named_family) = named_family {
        // Which members: readiness is closed to the *Readies* column, and `keeps` is declared
        // of food alone. Both narrow `thing`, which is every kind and would otherwise give
        // this release nine more things that expire and three more that exhaust.
        let about = |word: &str| {
            lines.iter().any(|row| {
                row.get(5)
                    .map(|traits| traits.contains(word))
                    .unwrap_or(false)
            })
        };
        let members: Vec<String> = if named_family == "thing" && about("ready") {
            readies.to_vec()
        } else if named_family == "thing" && about("keeps") {
            keeps.to_vec()
        } else {
            family(document, &named_family).unwrap_or_default()
        };
        assert!(
            !members.is_empty(),
            "`{name}` names the family `{named_family}` and this ground it to no kind at all, \
             which would drop the rule rather than spell it out"
        );
        return members
            .into_iter()
            .map(|member| {
                let mut changes = Vec::new();
                for row in lines {
                    changes.extend(changed(row, None, Some(&member), capable));
                }
                // The draw on time is added in `rules`, once the whole delta is known: it
                // depends on whether the kind's own count moved, which one row cannot say.
                (format!(" ({member})"), changes)
            })
            .collect();
    }

    let mut changes = Vec::new();
    for row in lines {
        changes.extend(changed(row, None, None, capable));
    }
    vec![(String::new(), changes)]
}

/// Which source an endless material comes out of.
///
/// **`spec/invariants.md` names three and assigns two of them directly**: *the planet's
/// material and the star's energy*. Food and metal are material; energy is the star's.
fn source_for(resource: &str) -> Place {
    Place {
        kind: if resource == "energy" {
            "the star".to_string()
        } else {
            "the planet".to_string()
        },
        state: String::new(),
    }
}

/// What one row does to one place, if it moves anything.
///
/// **A requirement moves nothing**, which is what `require` means: *present and not taken*. It
/// is absent from the arithmetic rather than entered as a zero, because a zero would make the
/// row look like something a weighting had weighed and found to balance.
fn changed(
    row: &[String],
    density: Option<(&str, u32)>,
    member: Option<&str>,
    capable: &std::collections::BTreeSet<(String, String)>,
) -> Vec<(Place, i64)> {
    let role = row.get(2).map(String::as_str).unwrap_or_default().trim();
    let sign: i64 = match role {
        "produce" => 1,
        "consume" => -1,
        // `require` takes nothing, and `limit` has no instance in the release - `P-385`.
        _ => return Vec::new(),
    };

    let kind = crate::recipes::plain(row.get(4).map(String::as_str).unwrap_or_default());
    let traits = crate::recipes::plain(row.get(5).map(String::as_str).unwrap_or_default());
    if kind.is_empty() {
        return Vec::new();
    }

    let (kind, amount) = match (density, kind.as_str()) {
        (Some((resource, density)), "resource") => (resource.to_string(), density as i64),
        _ => {
            let Some(amount) = row.get(3).and_then(|cell| cell.trim().parse::<i64>().ok()) else {
                return Vec::new();
            };
            (kind, amount)
        }
    };
    // A family row being ground names its member instead.
    // **The member the family row is being ground to.** Matched on the row naming *a*
    // family rather than on the word `thing`, so `move`'s `unit` grounds the same way.
    let kind = match member {
        Some(member) if ["thing", "unit", "resource", "place"].contains(&kind.as_str()) => {
            member.to_string()
        }
        _ => kind,
    };

    // **Two places, not one: the kind, and the state it is in.**
    //
    // The first version made `(kind, state)` a single place and it did not compose. A citizen
    // has two independent states - readiness and fertility - so `citizen, ready` and
    // `citizen, fertile` were different places with no relation, and a row naming neither
    // made a *third*. A built extractor landed in plain `extractor`, which `work` could not
    // take because `work` takes `extractor, ready`: the readiness economy was disconnected
    // from the thing that builds into it, and the check would have been solving a game where
    // a new extractor can never be worked.
    //
    // **A state is *yes or no*, so the place is how many are in the `yes`.** The release's
    // Traits table says exactly that of `ready` and of `spent`. So a thing is counted once
    // under its kind, and once more under each state it is in - and a rule that moves a thing
    // between states leaves the kind alone and moves the state place, which is what `work`,
    // `bear`, `renew` and `refresh` all do. No product of axes is needed and none is invented.
    let mut out = vec![(
        Place {
            kind: kind.clone(),
            state: String::new(),
        },
        sign * amount,
    )];

    let said = traits.trim();
    // **A capacity place moves when the row says the thing has that capacity**, which is
    // either by naming it or by naming nothing: a thing made and not described is at rest, and
    // at rest is where a capacity is full. `Thing::is_ready` in the model reads an absent
    // trait the same way.
    for (held, spent) in CAPACITIES {
        if said == spent {
            continue;
        }
        if !said.is_empty() && said != held && !COUNTERS.contains(&said) {
            continue;
        }
        // Only for a kind the release gives that capacity to. Deciding it per row instead
        // would give a metal a readiness the moment some rule mentioned one without a trait.
        if !capable.contains(&(kind.clone(), held.to_string())) {
            continue;
        }
        out.push((
            Place {
                kind: kind.clone(),
                state: held.to_string(),
            },
            sign * amount,
        ));
    }

    if COUNTERS.contains(&said) {
        out.push((
            Place {
                kind,
                state: said.to_string(),
            },
            sign * amount,
        ));
    }
    out
}

/// Which kinds hold which capacity, read from the release before any rule is ground.
///
/// **Two sources, because the release states the two capacities in two places.** Readiness is
/// the *Readies* column of *Units and structures*, and the release closes the set itself -
/// *Nothing outside this table readies*. Fertility is not a column, so it is read from the
/// rules: a kind some rule names as `fertile` or `spent` is a kind that has it.
///
/// **Decided here rather than row by row**, because a row naming a kind and no trait would
/// otherwise hand that kind every capacity in the list - a metal with a readiness, from a rule
/// that merely failed to describe it.
pub fn capable(document: &str) -> std::collections::BTreeSet<(String, String)> {
    let mut out = std::collections::BTreeSet::new();
    for kind in readies(document) {
        out.insert((kind, "ready".to_string()));
    }
    for row in crate::recipes::body_under(document, "## Recipes") {
        let kind = crate::recipes::plain(row.get(4).map(String::as_str).unwrap_or_default());
        let said = crate::recipes::plain(row.get(5).map(String::as_str).unwrap_or_default());
        if kind.is_empty() || kind == "thing" {
            continue;
        }
        for (held, spent) in CAPACITIES {
            if said == held || said == spent {
                out.insert((kind.clone(), held.to_string()));
            }
        }
    }
    out
}

/// A capacity to act, and the state a thing is in once it has been spent.
///
/// **The place counts the capacity, not its absence, and getting that backwards is what the
/// first two attempts did.** Counting things that are *not ready* makes `create labor` read as
/// pure gain - it makes a labor and a not-ready citizen and takes nothing - when what actually
/// paid for the labor is the citizen's readiness. `spec/invariants.md` says it directly:
/// **anything that exhausts is a readiness extractor for a turn**, *it draws one readiness out
/// of time and is spent doing it*. A thing that is drawn from and spent is a place that goes
/// **down**.
///
/// So a ready citizen holds one readiness, `create labor` spends it, and `refresh` draws
/// another out of time. Same shape for fertility: a fertile citizen holds one, `bear` spends
/// it, `renew` draws another. **`renew` is a readiness extractor too**, which falls out of the
/// rule rather than being decided here - the release calls fertility *renewed each turn*.
pub const CAPACITIES: [(&str, &str); 2] = [("ready", "not ready"), ("fertile", "spent")];

/// The states that are neither a capacity nor a thing at rest, kept as places of their own.
///
/// **`keeps` does not fit the pattern above and is not forced into it.** It is a counter
/// rather than a yes-or-no: `age` moves food from *keeps at least 1* to *keeps one less*, and
/// `spoil` takes food at *keeps 0*. Nothing in the release makes those three the same axis, so
/// they are three places and the weighting is left to say what it can about them. What that
/// exposes is real - see the report's own note about which of them nothing fills.
pub const COUNTERS: [&str; 4] = [
    "keeps at least 1",
    "keeps one less",
    "keeps 0",
    "whose upkeep is unpaid",
];

/// The report: what the rules ground to, and the weighting they force.
///
/// **Published rather than merely computed, which is `S-93`'s first requirement.** Sean chose
/// a derived weighting over a declared one *exactly because the numbers would otherwise be
/// invented* - and the other half of that choice is that the numbers it does arrive at are
/// where he can see them. A weighting that is technically sound and says something he would
/// reject, labor weighed at nothing being the example he gave, is visible here rather than
/// silent behind a green test.
pub fn markdown(document: &str) -> String {
    let rules = rules(document);
    let found = solve(&rules);

    let mut out = String::from("# Nothing comes back round with more\n\n");
    out.push_str(
        "**Generated. Do not edit.** Read out of the release's *Recipes* table by \
         `crates/game-console/src/nogain.rs`.\n\n",
    );
    out.push_str(
        "`spec/invariants.md` says there is a weighting of the kinds under which no sequence \
         of rules ends holding more than it began with, and that **whether this holds is \
         decided mechanically, from the rules alone**. This is that decision. `S-93`.\n\n",
    );
    out.push_str(
        "**Nothing here is declared.** The weighting below is solved for, not read from \
         anywhere: the only inputs are the release's rules. So a number that looks wrong is a \
         fact about the rules rather than about somebody's judgement of them - and one that is \
         sound but says something you would reject is visible for that reason.\n\n",
    );

    let sources: Vec<&Rule> = rules
        .iter()
        .filter(|rule| !rule.draws_from().is_empty())
        .collect();
    out.push_str(&format!(
        "**{} rules**, ground from **{}** blocks of recipe rows: a family becomes its members, \
         a density becomes its cases. **{} of them draw on a source** - the planet, the star or \
         time - and a source is a place they take from rather than an exemption from the \
         arithmetic. That is what keeps them in the check instead of out of it.\n\n",
        rules.len(),
        crate::recipes::body_under(document, "## Recipes")
            .iter()
            .filter(
                |row| !crate::recipes::plain(row.first().map(String::as_str).unwrap_or(""))
                    .is_empty()
            )
            .count(),
        sources.len(),
    ));

    match &found {
        Found::Weighting(weighting) => {
            out.push_str(
                "## A weighting exists, so nothing comes back round with more\n\n\
                 Every rule is non-increasing under the weighting below, so any sequence of \
                 them is too - a cycle included. **It may end with less**, which is disorder \
                 and is meant.\n\n",
            );
            let mut rows: Vec<Vec<String>> = vec![vec![
                "Place".to_string(),
                "Weight".to_string(),
                "What it is".to_string(),
            ]];
            let mut sorted: Vec<(&Place, &i64)> = weighting.iter().collect();
            sorted.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
            for (place, weight) in sorted {
                rows.push(vec![
                    place.label(),
                    weight.to_string(),
                    if place.is_source() {
                        "an endless well".to_string()
                    } else if CAPACITIES.iter().any(|(held, _)| place.state == *held) {
                        "a capacity, spent by acting".to_string()
                    } else if place.state.is_empty() {
                        "a thing".to_string()
                    } else {
                        "a state a thing is in".to_string()
                    },
                ]);
            }
            out.push_str(&padded(&rows));
            out.push('\n');
        }
        Found::NotFound { forcing } => {
            out.push_str(&format!(
                "## No weighting exists\n\n**The rules can come back round with more.** Phase I \
                 of the simplex ends with its artificial variables at zero exactly when a \
                 solution exists, so this is a property of the rules and not of how long \
                 anything searched.\n\n**{} rule(s) make more than they take** and are where to \
                 look first: {}.\n\n",
                forcing.len(),
                forcing
                    .iter()
                    .map(|name| format!("`{name}`"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
    }

    out.push_str("## Every rule, and what it nets\n\n");
    out.push_str(
        "Made minus taken, per place. A `require` row moves nothing and is absent rather than \
         entered as a zero - a zero would read as something the weighting had weighed.\n\n",
    );
    let mut rows: Vec<Vec<String>> = vec![vec![
        "Rule".to_string(),
        "Nets".to_string(),
        "Worth".to_string(),
    ]];
    for rule in &rules {
        let mut said: Vec<String> = rule
            .delta
            .iter()
            .map(|(place, change)| format!("{change:+} {}", place.label()))
            .collect();
        said.sort();
        let worth = match &found {
            Found::Weighting(weighting) => rule
                .delta
                .iter()
                .map(|(place, change)| change * weighting.get(place).copied().unwrap_or(1))
                .sum::<i64>()
                .to_string(),
            Found::NotFound { .. } => "-".to_string(),
        };
        rows.push(vec![
            rule.name.clone(),
            if said.is_empty() {
                "nothing".to_string()
            } else {
                // **Not a comma**, because a place label carries one - `citizen, fertile` -
                // and a comma-separated list of comma-containing labels cannot be read back.
                said.join(" · ")
            },
            worth,
        ]);
    }
    out.push_str(&padded(&rows));

    out.push_str(
        "\n**A rule that nets nothing is kept rather than dropped.** `stow` moves a resource \
         into a store, and a store is a container rather than a state - so at this granularity \
         it does nothing, and a reader looking for it should find it saying so rather than \
         find it missing.\n",
    );
    out
}

/// A markdown table, already padded, so that the padder finds nothing to change.
fn padded(rows: &[Vec<String>]) -> String {
    let columns = rows.iter().map(Vec::len).max().unwrap_or(0);
    let mut widths = vec![0usize; columns];
    for row in rows {
        for (at, cell) in row.iter().enumerate() {
            widths[at] = widths[at].max(cell.chars().count());
        }
    }
    let line = |row: &[String]| {
        let mut out = String::from("|");
        for (at, width) in widths.iter().enumerate() {
            let cell = row.get(at).map(String::as_str).unwrap_or("");
            out.push_str(&format!(
                " {}{} |",
                cell,
                " ".repeat(width - cell.chars().count())
            ));
        }
        out.push('\n');
        out
    };
    let mut out = line(&rows[0]);
    out.push('|');
    for width in &widths {
        out.push_str(&format!(" {} |", "-".repeat(*width)));
    }
    out.push('\n');
    for row in &rows[1..] {
        out.push_str(&line(row));
    }
    out
}
