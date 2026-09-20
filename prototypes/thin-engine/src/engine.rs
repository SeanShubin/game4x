//! The whole of the engine: read a command out of the data, check its rule, apply what it changes.
//!
//! # Everything it reads is a row, including what it is
//!
//! `data/given.4x` holds the structure, the structure's own structure, the roles, the rule, the
//! command and the world, all as rows of declared relations. **The engine is handed that one list
//! and nothing else.** There is no separate schema file, no separate rule file and no command
//! typed at it - a command is `{command id:1 rule:move}` with `{argument ...}` rows beside it.
//!
//! # The words it knows
//!
//! `relation`, `column` and `reference`, in [`crate::schema`]; and `role`, `rule`, `input`,
//! `clause`, `binding`, `command` and `argument` here. **Every one is about how a thing is written
//! down** - `territory`, `residency` and `move` appear in `data/` and in the tests and in no line
//! of `src/` that runs, which `tests/isolation.rs` checks.
//!
//! # Why a clause and a binding are two relations
//!
//! A clause says *require this relation*; a binding says *this column of it takes that input*.
//! **They are split because a relation cannot have columns that change with what it points at.**
//! An earlier version wrote `{effect relation:residency what:$what where:$from}`, whose columns
//! are `residency`'s - so `effect` had no fixed columns and was not a relation at all. That is the
//! whole of what *fully normalized* cost here, and it is four rows becoming twelve.
//!
//! # An input is typed, and that is where *no such place* is caught
//!
//! `of:territory` says the value bound to an input is a `territory`'s key, so a command naming
//! territory 9 is refused by the structure before the rule is looked at. **No rule has to say that
//! the destination exists.**

use std::collections::BTreeMap;

use crate::notation::Row;
use crate::refusal::Refused;
use crate::schema::{Malformed, Schema};
use crate::store::Store;

const ROLE: &str = "role";
const RULE: &str = "rule";
const INPUT: &str = "input";
const CLAUSE: &str = "clause";
const BINDING: &str = "binding";
const LITERAL: &str = "literal";
const READING: &str = "reading";
const TAKES: &str = "takes";
const RELATION_OF: &str = "relation-of";
const ID: &str = "id";
const NAME: &str = "name";
const SEQ: &str = "seq";
const OF: &str = "of";
const RELATION: &str = "relation";
const COLUMN: &str = "column";
const VALUE: &str = "value";
const REQUIRE: &str = "require";
const REMOVE: &str = "remove";
const ADD: &str = "add";
const PUT: &str = "put";
const KEEP: &str = "keep";
const ASSIGNS: &str = "assigns";
const TRAIT: &str = "trait";
const CARRIES: &str = "carries";
const KIND: &str = "kind";
const PART: &str = "part";
const ARGUMENT: &str = "argument";
const IS: &str = "is";
const REPEATS: &str = "repeats";
const SCOPE: &str = "scope";

/// Every row there is, and the structure read out of them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Game {
    schema: Schema,
    rows: Store,
}

impl Game {
    /// **The rows are checked against the structure they themselves declare** before anything is
    /// run on them, so a game that exists is one where every row fits and every reference points
    /// at something.
    pub fn of(rows: Vec<Row>) -> Result<Game, Malformed> {
        let schema = Schema::of(&rows)?;
        // **A family where a member belongs becomes one row per member**, before anything else
        // reads them - so no check downstream learns a word, and a template conflicting with a
        // row written out is two rows of one key, which is refused by the check that already
        // asks that.
        let rows = crate::schema::reified(&schema, rows);
        let rows = Store::of(rows);
        crate::schema::check(&schema, &rows)?;
        Ok(Game { schema, rows })
    }

    pub fn schema(&self) -> &Schema {
        &self.schema
    }

    pub fn rows(&self) -> &Store {
        &self.rows
    }

    /// The `name` of the row of `relation` whose id is `id`.
    ///
    /// **The engine branches on names and the data references by ids**, so this is where the two
    /// meet. An id is arbitrary - `role:2` means nothing until the row is looked up - so a value
    /// that selects a code path is resolved here first. **That makes `role.name` and
    /// `relation.name` load-bearing where every other `name` is decoration.**
    pub fn named(&self, relation: &str, id: &str) -> Option<&str> {
        self.rows
            .rows()
            .iter()
            .find(|row| row.relation == relation && row.value(ID) == Some(id))
            .and_then(|row| row.value(NAME))
    }

    fn of_relation(&self, relation: &str) -> Vec<&Row> {
        self.rows
            .rows()
            .iter()
            .filter(|row| row.relation == relation)
            .collect()
    }

    /// Every value that identifies a row of `relation`, in the order the rows are held.
    ///
    /// **The identity rather than the key**, because this is what a reference would carry - a
    /// counted relation has no single value to be named by, so it offers none and an input typed
    /// as one has nothing to range over.
    fn keys_of(&self, relation: &str) -> Vec<String> {
        // **A family ranges over its members, not over its rows**, because it has none. This is
        // what makes `offered` exact by what the game says rather than by what a column shape
        // happens to permit: a deposit is not offered to `move` because it is not a unit.
        if let Some(members) = self.schema.members(relation) {
            return members.to_vec();
        }
        let Some(declared) = self.schema.relation(relation) else {
            return Vec::new();
        };
        if declared.quantity().is_some() {
            return Vec::new();
        }
        self.rows
            .rows()
            .iter()
            .filter(|row| row.relation == relation)
            .filter_map(|row| row.value(declared.identity()))
            .map(str::to_string)
            .collect()
    }

    /// Whether any row of `relation` has `value` as its key.
    fn has_key(&self, relation: &str, value: &str) -> bool {
        // **Typed by the family is typed by membership.** `of:unit` admits a relation that is a
        // member of `unit` and nothing else, which is where *no such place* becomes *not one of
        // those*.
        if let Some(members) = self.schema.members(relation) {
            return members.iter().any(|it| it == value);
        }
        let Some(declared) = self.schema.relation(relation) else {
            return false;
        };
        self.rows
            .rows()
            .iter()
            .any(|row| row.relation == relation && row.value(declared.identity()) == Some(value))
    }
}

/// Fire a rule under a binding of its inputs, and give back the game it leaves.
///
/// **Split out of [`run`] so that [`offered`] can ask whether a binding would be refused without
/// a `{command ...}` row existing for it.** Reading a command and firing a rule were one function,
/// and only the first half needs a command. **No logic moved** - this is the second half of `run`
/// with its own name.
fn apply(
    game: &Game,
    of_rule: &str,
    rule: String,
    bound: &BTreeMap<String, String>,
    effect: &mut Effect,
) -> Result<Game, Refused> {
    let mut clauses: Vec<&Row> = game
        .of_relation(CLAUSE)
        .into_iter()
        .filter(|row| row.value(RULE) == Some(of_rule))
        .collect();
    clauses.sort_by_key(|row| row.value(SEQ).unwrap_or_default().to_string());

    // **Requiring happens before anything is applied**, which is what makes the two passes below
    // safe to write as two rather than one.
    //
    // **What each `require` matched, kept by clause.** A later clause can take a value out of it -
    // a quantity read from the world rather than written in the rule, which is what a density is.
    let mut matched: BTreeMap<String, Row> = BTreeMap::new();
    let mut after = game.rows.clone();
    for pass in [REQUIRE, REMOVE] {
        for clause in &clauses {
            let role = clause.value(ROLE).unwrap_or_default();
            let role = game.named(ROLE, role).unwrap_or(role);
            if role != pass {
                continue;
            }
            // **A pattern, not a whole row.** `require` and `remove` match on what they name,
            // so a clause can ask for *an adjacency from here to there* without naming which one.
            // **`add` is the other case and must name every column**, because a row that does not
            // fit the structure cannot be put into the world - including the `id` it will be
            // known by.
            // **A clause is done once for each relation its argument denotes.** One where
            // the argument is a kind, and one per member where it is a family - which is the
            // same sentence for every role, so no role is a special case.
            let relations = relations_of(game, clause, bound);
            let alone = relations.len() == 1;
            for relation in &relations {
                let wanted = row_of(game, clause, bound, &rule, PATTERN, &matched, relation)?;
                match role {
                    REQUIRE => {
                        let found = game.rows.matching(&wanted);
                        if found.is_empty() {
                            return Err(Refused::NotSo {
                                rule,
                                wanted: game.schema.write(&wanted),
                            });
                        }
                        // **One match is remembered and several are not.** A clause nothing reads
                        // from does not care either way; one that is read from refuses below
                        // rather than picking, which is where the non-determinism would have been.
                        //
                        // **A clause over several relations remembers none of them**, for that
                        // same reason one step out: which member a reading meant would be
                        // whichever was walked last.
                        if let [one] = found[..]
                            && alone
                        {
                            let id = clause.value(ID).unwrap_or_default().to_string();
                            matched.insert(id, one.clone());
                        }
                    }
                    _ => {
                        // **A pattern that several rows answer has not said which**, so it is
                        // refused rather than taken from whichever the store holds first. Sean,
                        // 2026-09-19: *it should be possible to structure the code to make
                        // nondeterminism impossible by raising an error instead.* **`NotOne`
                        // already does this for a reading**; this is the same answer for a take.
                        let how_many =
                            after.how_many_match(&wanted, counted(game, &wanted).as_deref());
                        if how_many > 1 {
                            return Err(Refused::NotOneToTake {
                                rule,
                                wanted: game.schema.write(&wanted),
                                found: how_many,
                            });
                        }
                        let Some(took) = after.take(&wanted, counted(game, &wanted).as_deref())
                        else {
                            return Err(Refused::NothingToRemove {
                                rule,
                                wanted: game.schema.write(&wanted),
                            });
                        };
                        effect.took.push(took);
                    }
                }
            }
        }
    }
    // **A put finds rows and says what is true of them afterwards.** It is the third thing a rule
    // can do to the world and the one the other two cannot express: `remove` refuses when nothing
    // matches, and refreshing something already topped off has to be a no-op.
    //
    // **Sean, 2026-09-18**, on why that is the right answer rather than a convenience: *This is not
    // a transformation recipe, it is creation. It is similar in nature to something generated by
    // time. The resources that come from the ground and sun are infinite but rate limited. So are
    // moves.*
    //
    // **Bindings and literals match; `{assigns ...}` says what changes.** The alternative offered -
    // bindings match and literals assign - cannot work, because a literal already matches:
    // `{literal id:13 clause:clause-3 column:98 value:1}` is how `move` picks the `moving:1` group
    // out of a place holding both, on a clause whose role is `remove`. **Which a value came from
    // cannot decide what it is for.**
    //
    // **The evidence first given for this was stale by one increment** - `{literal ... value:labor}`,
    // which the kinds unification deleted when a clause began naming its relation directly. Found
    // by the quality lens re-deriving the claim and finding nothing there.
    //
    // **A put conserves the count**, which is what separates it from a remove and an add that
    // could drop or duplicate: three scouts before, three after, and only their descriptions
    // changed. Two groups assigned the same description become one, because a store is a set and
    // `put` sums what it joins.
    for clause in &clauses {
        let role = clause.value(ROLE).unwrap_or_default();
        if game.named(ROLE, role).unwrap_or(role) != PUT {
            continue;
        }
        let id = clause.value(ID).unwrap_or_default().to_string();
        for relation in relations_of(game, clause, bound) {
            // **What is assigned is worked out once per relation, because it can refuse.** A
            // trait names the column it restores, so the kind either carries one or the rule is
            // being asked for something that does not exist.
            let mut assignments: Vec<(String, String)> = Vec::new();
            for assign in game
                .of_relation(ASSIGNS)
                .into_iter()
                .filter(|it| it.value(CLAUSE) == Some(id.as_str()))
            {
                let input = assign.value(INPUT).unwrap_or_default();
                let Some(given) = bound.get(input) else {
                    continue;
                };
                let carried = game.named(TRAIT, given).unwrap_or(given).to_string();
                let Some(value) = assign.value(VALUE) else {
                    continue;
                };
                if !carries(game, &relation, &carried) {
                    return Err(Refused::DoesNotCarry {
                        rule,
                        relation,
                        carried,
                    });
                }
                assignments.push((carried, value.to_string()));
            }
            let wanted = row_of(game, clause, bound, &rule, PATTERN, &matched, &relation)?;
            let found: Vec<Row> = after.matching(&wanted).into_iter().cloned().collect();
            for row in found {
                let mut made = row.clone();
                for (column, value) in &assignments {
                    made.values.insert(column.clone(), value.clone());
                }
                // **Already so is nothing done**, which is the no-op this role exists for.
                if made == row {
                    continue;
                }
                after.take(&row, None);
                after.put(made.clone(), counted(game, &made).as_deref());
                effect.took.push(row);
                effect.made.push(made);
            }
        }
    }

    for clause in &clauses {
        let role = clause.value(ROLE).unwrap_or_default();
        match game.named(ROLE, role).unwrap_or(role) {
            REQUIRE | REMOVE | PUT | KEEP => continue,
            ADD => {
                for relation in relations_of(game, clause, bound) {
                    let made = row_of(game, clause, bound, &rule, WHOLE, &matched, &relation)?;
                    after.put(made.clone(), counted(game, &made).as_deref());
                    effect.made.push(made);
                }
            }
            other => {
                return Err(Refused::NoSuchRole {
                    rule,
                    clause: clause.value(ID).unwrap_or_default().to_string(),
                    role: other.to_string(),
                });
            }
        }
    }

    // **What is over its capacity is taken**, which is the fifth role and the only one that
    // reads an amount out of the world rather than out of the rule. `require` and `remove` match,
    // `add` makes, `put` assigns; this one takes away what will not fit.
    //
    // **Only what is loose can be over**, and a kind that is not loose is bounded by the world
    // check rather than trimmed here - so a `keep` on a structure finds nothing to do, which is
    // right rather than a special case.
    //
    // **The same arithmetic the check reads**, from `crate::schema::rooming`, so the two cannot
    // disagree about what fits.
    for clause in &clauses {
        let role = clause.value(ROLE).unwrap_or_default();
        if game.named(ROLE, role).unwrap_or(role) != KEEP {
            continue;
        }
        for relation in relations_of(game, clause, bound) {
            let schema = Schema::of(after.rows()).map_err(|why| Refused::Broke {
                rule: rule.clone(),
                why: Box::new(why),
            })?;
            for asked in crate::schema::rooming(&schema, &after) {
                if asked.held != relation {
                    continue;
                }
                for (place, used) in &asked.used {
                    let there = asked.room.get(place).copied().unwrap_or(0);
                    if *used <= there {
                        continue;
                    }
                    // **One row holds it, and nothing here has to make that true.** A place holds
                    // one number of a kind - `spec/logistics.md` - and `Malformed::NotOneNumber`
                    // refuses a world where a loose kind is in two rows of one place. **So there
                    // is an excess and a row to take it from, and no choice between them.**
                    let Some((_, column)) = crate::schema::place_of(&schema, &relation, &asked.per)
                    else {
                        continue;
                    };
                    let Some(only) = after
                        .rows()
                        .iter()
                        .find(|row| {
                            row.relation == relation && row.value(&column) == Some(place.as_str())
                        })
                        .cloned()
                    else {
                        continue;
                    };
                    let only = &only;
                    let Some(quantity) = counted(game, only) else {
                        continue;
                    };
                    let mut kept = only.clone();
                    kept.values.insert(quantity.clone(), there.to_string());
                    after.take(only, Some(&quantity));
                    if there > 0 {
                        after.put(kept.clone(), Some(&quantity));
                        effect.made.push(kept);
                    }
                    effect.took.push(only.clone());
                }
            }
        }
    }

    let schema = Schema::of(after.rows()).map_err(|why| Refused::Broke {
        rule: rule.clone(),
        why: Box::new(why),
    })?;
    crate::schema::check(&schema, &after).map_err(|why| Refused::Broke {
        rule,
        why: Box::new(why),
    })?;
    Ok(Game {
        schema,
        rows: after,
    })
}

/// The column a relation counts by, where it counts.
fn counted(game: &Game, row: &Row) -> Option<String> {
    game.schema
        .relation(&row.relation)
        .and_then(|it| it.quantity())
        .map(str::to_string)
}

/// Whether a clause must name every column of its relation, or only the ones it constrains.
const WHOLE: bool = true;
const PATTERN: bool = false;

/// The relations a clause works on, in the order the data states them.
///
/// **A clause's relation is its own, or an argument's.** `{relation-of clause:12 input:9}` says
/// the second - the rule works on whichever kind the command named, which is what lets one `work`
/// serve every resource now that a kind is a relation rather than a value.
///
/// **An argument denotes one relation or several.** A kind denotes itself; a family denotes its
/// members, because a family has no rows of its own and a rule that worked on it would find
/// nothing. That is what makes `{refresh what:unit trait:moving}` the group command and
/// `{refresh what:scout trait:moving}` the separate one, with no second mechanism for grouping -
/// Sean, 2026-09-18: *I should be able to declare separate things with separate commands, as well
/// as explicitly declare group commands.*
///
/// **Read before the columns are bound**, because which columns exist depends on it.
fn relations_of(game: &Game, clause: &Row, bound: &BTreeMap<String, String>) -> Vec<String> {
    let id = clause.value(ID).unwrap_or_default();
    let given = match game
        .of_relation(RELATION_OF)
        .into_iter()
        .find(|row| row.value(CLAUSE) == Some(id))
        .and_then(|row| row.value(INPUT))
        .and_then(|input| bound.get(input))
    {
        Some(given) => given.clone(),
        None => clause.value(RELATION).unwrap_or_default().to_string(),
    };
    let named = game.named(RELATION, &given).unwrap_or(&given).to_string();
    match game.schema.members(&named) {
        // **Members are ids, because that is what a value carries** - so they are turned into
        // names here, which is what everything downstream of this reads.
        Some(members) => members
            .iter()
            .map(|it| game.named(RELATION, it).unwrap_or(it).to_string())
            .collect(),
        None => vec![named],
    }
}

/// Whether `relation` declares `carried` as one of its traits.
///
/// **Read from `{carries ...}` rather than from the columns**, because that is where the game
/// says it. `src/schema.rs` checks the two agree in both directions, so reading either is
/// reading both - and this reads the one a person would edit.
fn carries(game: &Game, relation: &str, carried: &str) -> bool {
    game.of_relation(CARRIES).into_iter().any(|row| {
        let kind = row.value(KIND).unwrap_or_default();
        let of = row.value(TRAIT).unwrap_or_default();
        game.named(RELATION, kind).unwrap_or(kind) == relation
            && game.named(TRAIT, of).unwrap_or(of) == carried
    })
}

fn row_of(
    game: &Game,
    clause: &Row,
    bound: &BTreeMap<String, String>,
    rule: &str,
    whole: bool,
    matched: &BTreeMap<String, Row>,
    relation: &str,
) -> Result<Row, Refused> {
    let id = clause.value(ID).unwrap_or_default();
    let bindings: Vec<&Row> = game
        .of_relation(BINDING)
        .into_iter()
        .filter(|row| row.value(CLAUSE) == Some(id))
        .collect();

    let declared = game.schema.relation(relation);
    let mut values = BTreeMap::new();
    for binding in bindings {
        let column = binding.value(COLUMN).unwrap_or_default();
        let Some((_, name)) = game.schema.column(column) else {
            continue;
        };
        let input = binding.value(INPUT).unwrap_or_default();
        let Some(value) = bound.get(input) else {
            continue;
        };
        values.insert(name.to_string(), value.clone());
    }

    // **A literal is a binding whose value is written in the rule rather than passed to it.**
    // An input is typed as a relation and carries one of its keys, so a plain number cannot be
    // one - and a quantity is a plain number. `releases/first-release.md` does the same thing by
    // writing **Qty** in the recipe, so this is the shape the game already has rather than a new
    // idea. **Read after the bindings and into the same map**, because a column takes its value
    // from one place or the other and never both.
    for literal in game
        .of_relation(LITERAL)
        .into_iter()
        .filter(|row| row.value(CLAUSE) == Some(id))
    {
        let column = literal.value(COLUMN).unwrap_or_default();
        let Some((_, name)) = game.schema.column(column) else {
            continue;
        };
        let Some(value) = literal.value(VALUE) else {
            continue;
        };
        values.insert(name.to_string(), value.to_string());
    }

    // **A reading is the third way a column gets its value, and the only one that looks at the
    // world.** A binding takes it from what the caller wrote and a literal from what the rule
    // says; this takes it from the row an earlier clause matched, which is how a rule produces
    // *the density here* rather than a number somebody typed. `spec/data/line.4x` writes that as
    // `$where`'s density for that resource.
    //
    // **Read last, for the same reason a literal is read after a binding**: one column, one
    // source, and the order says which wins if a rule says two things.
    for reading in game
        .of_relation(READING)
        .into_iter()
        .filter(|row| row.value(CLAUSE) == Some(id))
    {
        let column = reading.value(COLUMN).unwrap_or_default();
        let Some((_, name)) = game.schema.column(column) else {
            continue;
        };
        let of = reading.value(OF).unwrap_or_default();
        let Some(source) = matched.get(of) else {
            return Err(Refused::NotOne {
                rule: rule.to_string(),
                clause: of.to_string(),
                found: 0,
            });
        };
        let takes = reading.value(TAKES).unwrap_or_default();
        let Some((_, taken)) = game.schema.column(takes) else {
            continue;
        };
        let Some(value) = source.value(taken) else {
            continue;
        };
        values.insert(name.to_string(), value.to_string());
    }

    // **Every column of the relation has to be bound**, said here rather than left to the
    // structure check - a half-built row would otherwise be reported as one whose columns are
    // wrong, which names the symptom instead of the clause.
    if let Some(declared) = declared
        && whole
    {
        for column in &declared.columns {
            if !values.contains_key(&column.name) {
                return Err(Refused::Unbound {
                    rule: rule.to_string(),
                    clause: id.to_string(),
                    column: column.name.clone(),
                });
            }
        }
    }
    Ok(Row {
        relation: relation.to_string(),
        values,
    })
}

/// What one command took out of the world and what it made.
///
/// **Sean, 2026-09-16**: *I like to implement my programs as some form of (old-state, commands) ->
/// (new-state, effects), sometimes omitting effects depending on the architecture.* **This is the
/// effects half**, and it was there all along without a name - `take` and `put` knew what they had
/// done and nobody asked them.
///
/// **It says what the command did, not what the world now holds.** One scout of five leaving is
/// one taken; the four that stayed are nobody's effect.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Effect {
    /// The command that caused it, as the player wrote it.
    pub command: Row,
    pub took: Vec<Row>,
    pub made: Vec<Row>,
}

/// Play a list of commands against a world: **(old state, commands) -> (new state, effects)**.
///
/// **The whole of what the engine does, in one signature.** Everything else here is how. A refusal
/// stops the fold and gives back neither a world nor a list, because a command that was refused
/// leaves no state to have arrived at.
///
/// **One effect per command, in the order they were played**, so reading the list beside the two
/// states is reading what happened rather than inferring it from the difference.
pub fn play(game: &Game, commands: &[Row]) -> Result<(Game, Vec<Effect>), Refused> {
    let mut after = game.clone();
    let mut effects = Vec::new();
    for command in commands {
        let (next, effect) = fire(&after, command, 1)?;
        after = next;
        effects.push(effect);
    }
    Ok((after, effects))
}

/// Fire a command written as a rule-named row: `{move what:1 from:1 to:2}`.
///
/// **The row is the command**, so there is no `{command ...}` and no `{argument ...}` to mint and
/// none to read. Its relation is the rule's name and each value is named for one of the rule's
/// inputs. **This is the same shape [`offered`] gives back**, so what the engine says a player may
/// do is what the player writes.
///
/// **`repeat` is how many times it fires** - `spec/console.md`: *A command may carry a `repeat`,
/// which is how many times it fires.* And: *a command without one fires once.*
pub fn fire(game: &Game, command: &Row, repeat: usize) -> Result<(Game, Effect), Refused> {
    let Some(rule) = game
        .of_relation(RULE)
        .into_iter()
        .find(|row| row.value(NAME) == Some(command.relation.as_str()))
    else {
        return Err(Refused::NoSuchCommand {
            id: command.relation.clone(),
        });
    };
    let of_rule = rule.value(ID).unwrap_or_default().to_string();
    let named = command.relation.clone();

    // **Bound by the input's name, which is what the row writes.** A value the structure cannot
    // place is refused here, exactly as it is when a command is read from rows.
    let mut bound: BTreeMap<String, String> = BTreeMap::new();
    let filled = scoped(game, &of_rule);
    for input in game
        .of_relation(INPUT)
        .into_iter()
        .filter(|row| row.value(RULE) == Some(of_rule.as_str()))
    {
        let id = input.value(ID).unwrap_or_default();
        let name = input.value(NAME).unwrap_or_default();
        // **A scoped input is not the caller's to give.** `{upkeep}` is written with no place
        // because the engine supplies every place, which is what makes a test firing it by hand
        // and the turn firing it do the same thing.
        if filled.iter().any(|(it, _)| it == id) {
            continue;
        }
        let Some(given) = command.value(name) else {
            return Err(Refused::Missing {
                rule: named,
                input: name.to_string(),
            });
        };
        let of = input.value(OF).unwrap_or_default();
        let of = game.named(RELATION, of).unwrap_or(of).to_string();
        if !game.has_key(&of, given) {
            return Err(Refused::WrongType {
                rule: named,
                input: name.to_string(),
                value: given.to_string(),
                of,
            });
        }
        bound.insert(id.to_string(), given.to_string());
    }

    let mut after = game.clone();
    let mut effect = Effect {
        command: command.clone(),
        took: Vec::new(),
        made: Vec::new(),
    };
    for _ in 0..repeat {
        after = run(&after, &of_rule, named.clone(), &bound, &mut effect)?;
    }
    Ok((after, effect))
}

/// A rule's parts, in the order they fire.
fn parts_of<'a>(game: &'a Game, of_rule: &str) -> Vec<&'a Row> {
    let mut found: Vec<&Row> = game
        .of_relation(PART)
        .into_iter()
        .filter(|row| row.value(OF) == Some(of_rule))
        .collect();
    found.sort_by_key(|row| row.value(SEQ).unwrap_or_default().to_string());
    found
}

/// Fire a rule everywhere it is scoped to, which is once where it is scoped to nothing.
///
/// **A `{scope ...}` input is filled here and nowhere else**, so a rule that declares one is fired
/// once per row of what it is typed as whether a command named it or a part did. **Two scoped
/// inputs are a product**, which is the same enumeration [`offered`] does over a player's choices
/// and is done by the same function.
///
/// **The fan-out is outside the repetition**, so `upkeep` feeds one territory to exhaustion before
/// it looks at the next. Either order gives the same world while these rules are place-local, and
/// this one is the order a person would read the turn in.
///
/// **An effect is the whole command's**, not one part's and not one place's: ending a turn took and
/// made whatever its parts did, in the order they did it, so a test reads one list rather than five.
fn run(
    game: &Game,
    of_rule: &str,
    rule: String,
    bound: &BTreeMap<String, String>,
    effect: &mut Effect,
) -> Result<Game, Refused> {
    let over = scoped(game, of_rule);
    if over.is_empty() {
        return once(game, of_rule, rule, bound, effect);
    }
    let choices: Vec<(String, String, Vec<String>)> = over
        .into_iter()
        .map(|(input, of)| {
            let keys = game.keys_of(&of);
            (input, of, keys)
        })
        .collect();
    let mut after = game.clone();
    for places in every_binding(&choices) {
        let mut bound = bound.clone();
        for ((input, _, _), place) in choices.iter().zip(places) {
            bound.insert(input.clone(), place);
        }
        after = once(&after, of_rule, rule.clone(), &bound, effect)?;
    }
    Ok(after)
}

/// The inputs a rule's `{scope ...}` rows say the engine fills, each with what it ranges over.
///
/// **In the order the rows are stated**, because two scoped inputs are a product and which one
/// varies fastest would otherwise be whatever a map happened to hold first.
fn scoped(game: &Game, of_rule: &str) -> Vec<(String, String)> {
    game.of_relation(SCOPE)
        .into_iter()
        .filter(|row| row.value(RULE) == Some(of_rule))
        .filter_map(|row| row.value(INPUT))
        .filter_map(|input| {
            let declared = game
                .of_relation(INPUT)
                .into_iter()
                .find(|row| row.value(ID) == Some(input))?;
            let of = declared.value(OF).unwrap_or_default();
            let of = game.named(RELATION, of).unwrap_or(of).to_string();
            Some((input.to_string(), of))
        })
        .collect()
}

/// Fire a rule once for one binding of its inputs: its parts, or its repetition, or its clauses.
///
/// **A rule is a leaf or a composite and never both**, which `src/schema.rs` refuses - so this
/// reads the parts and stops if there are any, rather than doing both and leaving the order
/// between them to whichever this function happened to write first.
///
/// **The recursion terminates because the structure says so.** `{part ...}` is checked for cycles
/// and for a rule with two parents when the world is read, so a composite cannot reach itself and
/// this needs no depth counter. **That is the check doing the work a guard would otherwise do**,
/// and it is why Sean's *acyclic graph or tree* is a property of the data rather than advice.
///
/// **A repetition is a leaf's business.** `Malformed::NeverStops` refuses one on a composite,
/// because what a composite consumes is its parts' to say - so the two branches below cannot both
/// be taken and the order between them decides nothing.
fn once(
    game: &Game,
    of_rule: &str,
    rule: String,
    bound: &BTreeMap<String, String>,
    effect: &mut Effect,
) -> Result<Game, Refused> {
    let parts = parts_of(game, of_rule);
    if !parts.is_empty() {
        let mut after = game.clone();
        for part in parts {
            let id = part.value(ID).unwrap_or_default();
            let of = part.value(IS).unwrap_or_default().to_string();
            let named = game.named(RULE, &of).unwrap_or(&of).to_string();
            let given = arguments_of(game, id, &of, &named)?;
            after = run(&after, &of, named, &given, effect)?;
        }
        return Ok(after);
    }
    if game
        .of_relation(REPEATS)
        .into_iter()
        .any(|row| row.value(RULE) == Some(of_rule))
    {
        return repeatedly(game, of_rule, rule, bound, effect);
    }
    apply(game, of_rule, rule, bound, effect)
}

/// Fire a rule as many times as it can, each firing drawing from what the last one left behind.
///
/// **The pool it draws from is the world as it was when the repetition began, and it only ever
/// shrinks.** What a firing makes is held aside until the repetition ends, so a rule cannot spend
/// its own output - which is what Sean, 2026-09-19, asked for: *repeat should always have
/// consume-once rather than consume semantics.*
///
/// **That is the whole termination proof.** The pool is finite and each firing takes something out
/// of it, so a bound falls out of the structure rather than out of a counter. `upkeep` feeds as
/// many citizens as there is food for and stops when either runs out, and nothing computes which.
///
/// **A firing that is refused leaves no trace.** Its effect goes into a list of its own and is
/// dropped, so the world the repetition gives back is the one the last firing that stood left.
fn repeatedly(
    game: &Game,
    of_rule: &str,
    rule: String,
    bound: &BTreeMap<String, String>,
    effect: &mut Effect,
) -> Result<Game, Refused> {
    let mut pool = game.clone();
    let mut held: Vec<Row> = Vec::new();
    loop {
        let mut aside = Effect {
            command: effect.command.clone(),
            took: Vec::new(),
            made: Vec::new(),
        };
        let next = match apply(&pool, of_rule, rule.clone(), bound, &mut aside) {
            Ok(next) => next,
            Err(why) if ran_out(&why) => break,
            Err(why) => return Err(why),
        };
        let mut rows = next.rows;
        for made in &aside.made {
            let quantity = counted(&pool, made);
            rows.take(made, quantity.as_deref());
        }
        // **It stops when the pool stops shrinking**, which no rule the structure admits should
        // reach - `Malformed::NeverStops` refuses a repetition with nothing to consume. It is here
        // because a loop that cannot be shown to end from inside itself is not one to run.
        if same(&rows, &pool.rows) {
            break;
        }
        pool.rows = rows;
        held.extend(aside.made.clone());
        effect.took.extend(aside.took);
        effect.made.extend(aside.made);
    }

    // **And everything held aside goes back**, joined with whatever is there, which is the one
    // world anybody outside this function sees.
    let mut rows = pool.rows;
    for made in held {
        let quantity = counted(game, &made);
        rows.put(made, quantity.as_deref());
    }
    let schema = Schema::of(rows.rows()).map_err(|why| Refused::Broke {
        rule: rule.clone(),
        why: Box::new(why),
    })?;
    crate::schema::check(&schema, &rows).map_err(|why| Refused::Broke {
        rule,
        why: Box::new(why),
    })?;
    Ok(Game { schema, rows })
}

/// Whether a refusal means *the world does not have it* rather than *the rule is wrong*.
///
/// **A repetition stops on the first and reports the second.** Firing as many times as it can ends
/// when what it needs has run out - and a clause bound to no input has not run out of anything, it
/// is malformed. **Swallowing that would lose the one message that says so**, which is the
/// opposite of what Sean asked for: *I want errors detectible with good error messages.*
fn ran_out(why: &Refused) -> bool {
    matches!(
        why,
        Refused::NotSo { .. } | Refused::NothingToRemove { .. } | Refused::Broke { .. }
    )
}

/// Whether two stores hold the same rows, in whatever order they hold them.
///
/// **Taking a row and putting it back moves it to the end**, so comparing the two as lists would
/// call a firing that changed nothing a change, and the repetition above would not stop.
fn same(these: &Store, those: &Store) -> bool {
    these.rows().len() == those.rows().len()
        && these.rows().iter().all(|row| those.rows().contains(row))
}

/// What a part hands the rule it names, by that rule's input ids.
///
/// **Every input gets an argument and none may be of the wrong sort**, which are the two things
/// [`fire`] checks for a command - said here for a part, because a part is where a rule is called
/// from when a player is not the one calling it.
///
/// **`{argument ...}`'s value is the one reference the schema cannot state.** What sort of thing it
/// is follows the input's `of` rather than a `{reference ...}` row on the column, so nothing checks
/// it when the world is read and this is where it is checked instead.
fn arguments_of(
    game: &Game,
    part: &str,
    of_rule: &str,
    named: &str,
) -> Result<BTreeMap<String, String>, Refused> {
    let mut bound = BTreeMap::new();
    let filled = scoped(game, of_rule);
    for input in game
        .of_relation(INPUT)
        .into_iter()
        .filter(|row| row.value(RULE) == Some(of_rule))
    {
        let id = input.value(ID).unwrap_or_default();
        let name = input.value(NAME).unwrap_or_default();
        // **A scoped input takes no argument**, for the same reason a command gives none: it is
        // the engine's to fill, and a part that supplied one would be overruled.
        if filled.iter().any(|(it, _)| it == id) {
            continue;
        }
        let Some(given) = game
            .of_relation(ARGUMENT)
            .into_iter()
            .filter(|row| row.value(PART) == Some(part))
            .find(|row| row.value(INPUT) == Some(id))
            .and_then(|row| row.value(VALUE))
        else {
            return Err(Refused::Missing {
                rule: named.to_string(),
                input: name.to_string(),
            });
        };
        let of = input.value(OF).unwrap_or_default();
        let of = game.named(RELATION, of).unwrap_or(of).to_string();
        if !game.has_key(&of, given) {
            return Err(Refused::WrongType {
                rule: named.to_string(),
                input: name.to_string(),
                value: given.to_string(),
                of,
            });
        }
        bound.insert(id.to_string(), given.to_string());
    }
    Ok(bound)
}

/// Every command the player could fire right now, as rows in the friendly command form.
///
/// `spec/invariants.md`, of a recipe: *The player's are offered wherever their inputs are present,
/// to take or to leave.* And of a choice: *What may be chosen is whatever the game holds, and the
/// offering is derived rather than listed.* **This is that sentence, run.**
///
/// **Offerable means would not be refused**, which is the strongest reading and the cheapest: a
/// candidate is bound and fired, and kept if firing succeeds. **No second copy of what legal
/// means** - `docs/process.md` calls a check that reads a copy of the population a check of the
/// copy, and a predicate written beside [`apply`] would be exactly that.
///
/// **It adds no word to the engine.** Every row it reads - `rule`, `input`, `clause`, `binding` -
/// is vocabulary the engine already had, so the boundary `data/engine.4x` draws does not move.
///
/// **A row's relation is the rule's name and its values are the input names**, so what comes back
/// is what a player would type: `{move what:scout from:territory-1 to:territory-2}`.
///
/// **The cost is a product over every input's type**, which is fine at three territories and is
/// the thing to watch as a world grows. It is measured rather than guarded against, because what
/// this prototype is for is finding out where the data explodes.
pub fn offered(game: &Game) -> Vec<Row> {
    let mut out = Vec::new();
    for rule in game.of_relation(RULE) {
        let Some(of_rule) = rule.value(ID) else {
            continue;
        };
        // **A rule that is somebody's part is fired by that somebody.** So the roots of the tree
        // are the player's menu, and nothing has to declare an owner: `spec/data/block.4x` writes
        // `owner:world` and here the structure says it, which is one fact rather than two that can
        // disagree.
        //
        // **Offered and fireable are two questions.** Sean, 2026-09-18: *Why can't refresh be both
        // a player command and part of the turn [...] it will be easier to test the end turn
        // command itself if i can test its parts.* A test names `refresh` and fires it; a player
        // is not shown it. This lane had collapsed the two into one.
        if game
            .of_relation(PART)
            .into_iter()
            .any(|row| row.value(IS) == Some(of_rule))
        {
            continue;
        }
        let named = rule.value(NAME).unwrap_or(of_rule).to_string();

        let mut inputs: Vec<&Row> = game
            .of_relation(INPUT)
            .into_iter()
            .filter(|row| row.value(RULE) == Some(of_rule))
            .collect();
        inputs.sort_by_key(|row| row.value(SEQ).unwrap_or_default().to_string());

        // **Every key of the relation each input is typed as.** That is what *wherever their
        // inputs are present* ranges over, and the type is what bounds it.
        let choices: Vec<(String, String, Vec<String>)> = inputs
            .iter()
            .map(|input| {
                let of = input.value(OF).unwrap_or_default();
                let of = game.named(RELATION, of).unwrap_or(of).to_string();
                let keys = game.keys_of(&of);
                (
                    input.value(ID).unwrap_or_default().to_string(),
                    input.value(NAME).unwrap_or_default().to_string(),
                    keys,
                )
            })
            .collect();

        for bound in every_binding(&choices) {
            let by_id: BTreeMap<String, String> = choices
                .iter()
                .map(|(id, _, _)| id.clone())
                .zip(bound.iter().cloned())
                .collect();
            // **Offering does not care what a command would do**, only that it could - so the
            // effect it would have is built and dropped.
            let mut aside = Effect {
                command: Row {
                    relation: named.clone(),
                    values: BTreeMap::new(),
                },
                took: Vec::new(),
                made: Vec::new(),
            };
            if apply(game, of_rule, named.clone(), &by_id, &mut aside).is_ok() {
                out.push(Row {
                    relation: named.clone(),
                    values: choices
                        .iter()
                        .map(|(_, name, _)| name.clone())
                        .zip(bound)
                        .collect(),
                });
            }
        }
    }
    out.sort_by_key(|row| (row.relation.clone(), format!("{:?}", row.values)));
    out
}

/// Every way of choosing one value for each input, in declared order.
///
/// **Written with loops because the closure form needed the word `move`**, which is a Rust keyword
/// and also the name of the game's one rule. `tests/isolation.rs` reads `src/` for the game's nouns
/// and cannot tell a keyword from a noun - and should not try, because the day it does is the day
/// it stops catching the thing it is for. **The check was right and the code moved.**
fn every_binding(choices: &[(String, String, Vec<String>)]) -> Vec<Vec<String>> {
    let mut all: Vec<Vec<String>> = vec![Vec::new()];
    for (_, _, keys) in choices {
        let mut next = Vec::new();
        for so_far in &all {
            for key in keys {
                let mut one = so_far.clone();
                one.push(key.clone());
                next.push(one);
            }
        }
        all = next;
    }
    all
}

/// The roles a clause may have, so that a test can assert the data uses all of them and no others.
pub fn roles() -> [&'static str; 5] {
    [REQUIRE, REMOVE, ADD, PUT, KEEP]
}
