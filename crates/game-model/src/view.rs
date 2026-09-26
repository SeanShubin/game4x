//! Two ways of reading a world, and neither is the game.
//!
//! **Sean, 2026-09-16**: *any generic utility logic not specific to the game should be elsewhere.*
//! Writing the rows out is not what the engine does; it is what a reader does with what the engine
//! left - so it is here rather than beside the loop.

use crate::engine::Game;

const RELATION: &str = "relation";
const STATE: &str = "state";
const RULE: &str = "rule";
const PART: &str = "part";
const ARGUMENT: &str = "argument";
const INPUT: &str = "input";
const ID: &str = "id";
const NAME: &str = "name";
const SEQ: &str = "seq";
const OF: &str = "of";
const IS: &str = "is";
const VALUE: &str = "value";
const SCOPE: &str = "scope";

impl Game {
    /// Every row, in each relation's declared column order, sorted.
    ///
    /// **Sorted because the rows are a set and the order they are held in is nobody's**, so two
    /// games holding the same rows are the same game however they were built.
    pub fn shown(&self) -> Vec<String> {
        let mut out: Vec<String> = self
            .rows()
            .rows()
            .iter()
            .map(|row| self.schema().write(row))
            .collect();
        out.sort();
        out
    }

    /// The world as indented text: one block per relation the schema marks as state.
    ///
    /// **Sean, 2026-09-16**: *I also like to define an indented text form of the state.* This is
    /// the plainest one the data can say without being told anything new - **grouped, not nested**.
    ///
    /// **Nesting would need one thing nobody has declared**: which column of a relation holds the
    /// thing that contains it. `residency` has `what` and `where` and both are references, and
    /// nothing says `where` is the container - so a tree would be this lane guessing which of two
    /// columns to hang the row from.
    pub fn outline(&self) -> String {
        let mut out = String::from("- root\n");
        let mut of_state: Vec<&str> = self
            .rows()
            .rows()
            .iter()
            .filter(|row| row.relation == STATE)
            .filter_map(|row| row.value(RELATION))
            .filter_map(|id| self.named(RELATION, id))
            .collect();
        of_state.sort_unstable();
        for relation in of_state {
            out.push_str(&format!("  - {relation}\n"));
            let mut written: Vec<String> = self
                .rows()
                .rows()
                .iter()
                .filter(|row| row.relation == relation)
                .map(|row| self.schema().write(row))
                .collect();
            written.sort();
            for row in written {
                out.push_str(&format!("    - {row}\n"));
            }
        }
        out
    }

    /// Every rule, as the tree its parts make, in the order they fire.
    ///
    /// **Sean, 2026-09-18**: *it must be able to organize the entirety of game rules is some type
    /// of acyclic graph or tree. Otherwise it will be impossible for a human player to understand
    /// how to play the game.* This is that tree, written out - and the reason it is worth printing
    /// rather than only checking is his: **the specification had no artifact whose whole structure
    /// could be read at once**, and that is how it got away from him.
    ///
    /// **A root is what a player may choose.** A rule that is somebody's part is fired by that
    /// somebody, so the roots are exactly what [`crate::engine::offered`] will range over - one
    /// fact, read here and applied there, rather than an `owner` written twice.
    ///
    /// **A root shows what it takes and a part shows what it is given.** `what:unit` under a root
    /// is the type of an argument; under a part it is the argument. That is the difference between
    /// a rule and a use of one, and seeing both is what makes the order readable.
    pub fn tree(&self) -> String {
        let mut out = String::from(
            "Every rule, and the tree its parts make.\nA root is what a player may choose; everything under one is fired by it.\n",
        );
        let rules = self.rows().rows().iter().filter(|it| it.relation == RULE);
        let mut ordered: Vec<(&str, &str)> = rules
            .filter_map(|it| Some((it.value(ID)?, it.value(NAME)?)))
            .collect();
        ordered.sort_by_key(|(id, _)| id.parse::<u64>().unwrap_or(u64::MAX));

        for (id, name) in &ordered {
            if self
                .rows()
                .rows()
                .iter()
                .any(|it| it.relation == PART && it.value(IS) == Some(*id))
            {
                continue;
            }
            out.push('\n');
            out.push_str(name);
            let takes = self.takes(id);
            if !takes.is_empty() {
                out.push_str(&format!("  {takes}"));
            }
            out.push_str(&self.per(id));
            out.push('\n');
            self.under(id, 1, &mut out);
        }
        out
    }

    /// Where a rule is scoped, as ` per:territory` - and nothing at all where it is not.
    ///
    /// **A scoped input is not in `takes`**, because nothing hands it one: the engine fires the
    /// rule once per row of what it is typed as. **So the tree would otherwise show `upkeep` as a
    /// step with no arguments**, which is true of what it is given and silent about the one thing
    /// a person reading the turn needs - that it happens in every territory rather than once.
    fn per(&self, of_rule: &str) -> String {
        self.rows()
            .rows()
            .iter()
            .filter(|it| it.relation == SCOPE && it.value(RULE) == Some(of_rule))
            .filter_map(|it| it.value(INPUT))
            .filter_map(|input| {
                let declared = self
                    .rows()
                    .rows()
                    .iter()
                    .find(|it| it.relation == INPUT && it.value(ID) == Some(input))?;
                let of = declared.value(OF)?;
                Some(format!("  per:{}", self.named(RELATION, of).unwrap_or(of)))
            })
            .collect()
    }

    /// What a rule takes, as `name:type` in the order a player would say them.
    fn takes(&self, of_rule: &str) -> String {
        let mut inputs: Vec<(u64, String)> = self
            .rows()
            .rows()
            .iter()
            .filter(|it| it.relation == INPUT && it.value("rule") == Some(of_rule))
            .filter_map(|it| {
                let of = it.value(OF)?;
                Some((
                    crate::schema::ordinal(it.value(SEQ)?),
                    format!(
                        "{}:{}",
                        it.value(NAME)?,
                        self.named(RELATION, of).unwrap_or(of)
                    ),
                ))
            })
            .collect();
        inputs.sort();
        inputs
            .into_iter()
            .map(|(_, it)| it)
            .collect::<Vec<String>>()
            .join(" ")
    }

    /// The parts of one rule, indented, each with what it is given.
    fn under(&self, of_rule: &str, depth: usize, out: &mut String) {
        let mut parts: Vec<(u64, &str, &str)> = self
            .rows()
            .rows()
            .iter()
            .filter(|it| it.relation == PART && it.value(OF) == Some(of_rule))
            .filter_map(|it| {
                Some((
                    crate::schema::ordinal(it.value(SEQ)?),
                    it.value(ID)?,
                    it.value(IS)?,
                ))
            })
            .collect();
        parts.sort();
        for (seq, part, is) in parts {
            let named = self.named(RULE, is).unwrap_or(is);
            out.push_str(&"    ".repeat(depth));
            out.push_str(&format!("{seq}. {named}"));
            let given = self.given(part, is);
            if !given.is_empty() {
                out.push_str(&format!("  {given}"));
            }
            out.push_str(&self.per(is));
            out.push('\n');
            self.under(is, depth + 1, out);
        }
    }

    /// What a part hands the rule it names, as `input:value` in the rule's own input order.
    ///
    /// **The value is written in the words of whatever the input is typed as**, which is the one
    /// lookup nothing else can do: `{argument ... input:11 value:26}` is `what:unit` because input
    /// 11 is typed `of:relation` and relation 26 is the family `unit`.
    fn given(&self, part: &str, of_rule: &str) -> String {
        let mut shown: Vec<(u64, String)> = Vec::new();
        for input in self
            .rows()
            .rows()
            .iter()
            .filter(|it| it.relation == INPUT && it.value("rule") == Some(of_rule))
        {
            let (Some(id), Some(name), Some(of), Some(seq)) = (
                input.value(ID),
                input.value(NAME),
                input.value(OF),
                input.value(SEQ),
            ) else {
                continue;
            };
            let Some(value) = self
                .rows()
                .rows()
                .iter()
                .filter(|it| it.relation == ARGUMENT && it.value(PART) == Some(part))
                .find(|it| it.value(INPUT) == Some(id))
                .and_then(|it| it.value(VALUE))
            else {
                continue;
            };
            let of = self.named(RELATION, of).unwrap_or(of);
            let shows = self.named(of, value).unwrap_or(value);
            shown.push((
                seq.parse::<u64>().unwrap_or(u64::MAX),
                format!("{name}:{shows}"),
            ));
        }
        shown.sort();
        shown
            .into_iter()
            .map(|(_, it)| it)
            .collect::<Vec<String>>()
            .join(" ")
    }
}
