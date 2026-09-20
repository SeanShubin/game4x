//! What relations there are, what columns each has, and which columns point at another relation.
//!
//! **This module is the answer to *be clear about the data structure*.** The first test was typed
//! in relational notation - `residency (what fk thing, where fk territory)` - and everything here
//! is that made explicit enough for the engine to enforce.
//!
//! # It is read from the same rows as everything else
//!
//! `{relation ...}`, `{column ...}` and `{reference ...}` are rows in `data/given.4x` beside
//! `{territory id:1}`, and they are declared there too, so **the structure describes itself and is
//! checked against its own description**. That is what makes *everything is data* a thing a test
//! can fail rather than a thing to say.
//!
//! # Three things a schema decides
//!
//! **What may be stated.** A row whose relation is not declared, or whose columns are not exactly
//! the declared ones, is refused. There is no open row.
//!
//! **What a value means.** A `{reference column:residency.what to:thing}` row says the value in
//! that column is a `thing`'s key, and the engine checks such a row exists. **A relation's key is
//! its first column** - a convention rather than a dependency, which is why there is no `key`
//! relation.
//!
//! **What order a row is written in.** The order is the relation's own, stated at `seq`, and a row
//! reads back in it rather than alphabetically. That is `P-513`'s question from the data side.

use std::collections::BTreeMap;

use crate::notation::Row;
use crate::store::Store;

const RELATION: &str = "relation";
const COLUMN: &str = "column";
const REFERENCE: &str = "reference";
const ID: &str = "id";
const QUANTITY: &str = "quantity";
const NAME: &str = "name";
const SEQ: &str = "seq";
const TO: &str = "to";
const ATTRIBUTE: &str = "attribute";
const LIMIT: &str = "limit";
const HELD: &str = "held";
const BY: &str = "by";
const FAMILY: &str = "family";
const MEMBER: &str = "member";
const KIND: &str = "kind";
const CAPACITY: &str = "capacity";
const STATE: &str = "state";
const LOOSE: &str = "loose";
const FOR: &str = "for";
const PART: &str = "part";
const IS: &str = "is";
const OF: &str = "of";
const RULE: &str = "rule";
const CLAUSE: &str = "clause";
const REPEATS: &str = "repeats";
const ROLE: &str = "role";
const REMOVE: &str = "remove";
const TRAIT: &str = "trait";
const CARRIES: &str = "carries";
const SUPPLY: &str = "supply";
const PROVIDES: &str = "provides";
const CONSUMES: &str = "consumes";
const WHAT: &str = "what";
const PER: &str = "per";

/// One column: its id, what it is called, and what it points at if anything.
///
/// **The id is how a `binding` names a column**, which is why a column has one at all - `name` is
/// unique only within its relation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Column {
    pub id: String,
    pub name: String,
    pub references: Option<String>,
}

/// One relation: its name, and its columns in the order it declares them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Relation {
    pub name: String,
    pub columns: Vec<Column>,
    /// Columns that are neither the key nor the quantity: facts about the row rather than what
    /// tells it from another.
    ///
    /// **The default is that there are none**, which is why they are marked one at a time rather
    /// than the key being declared. `spec/console.md` - *a description is a kind and every trait
    /// of that thing* - is still what a key is, and this is the exception saying so out loud.
    pub attributes: Vec<String>,
}

impl Relation {
    /// The column a reference names this relation's rows by - always the first.
    ///
    /// **This is identity, not uniqueness**, and the two parted company when a quantity arrived.
    /// A reference needs one value to carry, so a relation something points at is identified by
    /// one column. **A relation nothing points at need not be**, which is what [`Relation::key`]
    /// is about.
    pub fn identity(&self) -> &str {
        &self.columns[0].name
    }

    /// The column holding how many things each row stands for, where there is one.
    ///
    /// **Named rather than declared**, exactly as `id` already is. Sean, 2026-09-15: *it would
    /// make no sense to have both an id and a quantity in the same logical model* - they are one
    /// slot, how a relation tells its rows apart, and `id` had occupied it by name since the
    /// beginning. A table saying which column is the quantity would have stated one half of an
    /// exclusive pair as data and left the other half a convention.
    pub fn quantity(&self) -> Option<&str> {
        self.columns
            .iter()
            .find(|it| it.name == QUANTITY)
            .map(|it| it.name.as_str())
    }

    /// The columns that together tell one row from another.
    ///
    /// **An id where there is one; otherwise every column but the quantity and the attributes** -
    /// `spec/console.md`: *a description is a kind and every trait of that thing* [...] *no trait
    /// of the thing may be left out*. So the key is not a subset anybody chooses, and a column
    /// added to the relation joins it by existing.
    ///
    /// **It used to be the first column wherever there was no quantity**, which made the quantity
    /// decide the shape of the rule rather than being one more column the key excludes. That was
    /// right for three relations and wrong for two: `member` was keyed by `kind` alone, so a kind
    /// could belong to one family, and `limit` by `held` alone, so a thing could be held by one
    /// container. **Neither was a decision** - Sean, 2026-09-18, on being told a scout could not
    /// also be a refreshable: *Why not? Many languages have multiple inheritance.* It was an
    /// artefact of this function presented as a property of the model.
    ///
    /// **The two it was right about are kept right by marking a column**, which is what
    /// `{attribute ...}` is for and costs a row each: `attribute.relation` and `relation-of.input`
    /// are facts about the row rather than part of what it is, so a column is an attribute of one
    /// relation and a clause takes its relation from one input, as before.
    pub fn key(&self) -> Vec<&str> {
        if let Some(id) = self.columns.iter().find(|it| it.name == ID) {
            return vec![id.name.as_str()];
        }
        let quantity = self.quantity();
        self.columns
            .iter()
            .map(|it| it.name.as_str())
            .filter(|name| Some(*name) != quantity)
            .filter(|name| !self.attributes.iter().any(|it| it == name))
            .collect()
    }
}

/// Every way the data can fail to fit the structure, said about the data and not about the reader.
#[derive(Debug, PartialEq, Eq)]
pub enum Malformed {
    /// A `{column ...}` row naming a relation nothing declares.
    ColumnOfNothing { relation: String, column: String },
    /// A `{reference ...}` row naming a column nothing declares.
    ReferenceOfNothing { column: String },
    /// A relation declared with no columns at all, so it has no key.
    NoColumns { relation: String },
    /// Columns whose `seq` is not 1, 2, 3 and so on.
    BadOrder { relation: String, seq: Vec<String> },
    /// A reference pointing at a relation nothing declares.
    ReferencesNothing { column: String, to: String },
    /// A row of a relation nothing declares.
    NoSuchRelation { relation: String },
    /// A row whose columns are not the declared ones.
    WrongColumns {
        relation: String,
        wanted: String,
        given: String,
    },
    /// Two rows of one relation with the same key.
    ///
    /// **A reference names a row by its key, so a key naming two rows names neither.** Nothing
    /// checked this until it was looked for: two `{thing id:1 ...}` rows were accepted, and
    /// `{residency what:1}` then pointed at both of them.
    TwoWithOneKey {
        relation: String,
        /// Each key column and the value this row carries in it, in declared order.
        ///
        /// **A list because a key may be several columns.** One entry reads exactly as it did
        /// when a key was always one column, so the message did not change for the case that
        /// already existed.
        key: Vec<(String, String)>,
    },
    /// A relation declaring both an `id` and a `quantity`.
    ///
    /// **Sean, 2026-09-15**: *it would make no sense to have both an id and a quantity in the
    /// same logical model.* They are one slot - whether a row is one thing or a count of them -
    /// so carrying both says a row is identified and counted at once, and nothing can be.
    IdAndQuantity { relation: String },
    /// More of a held thing somewhere than there is room for it.
    ///
    /// **One variant for both ways of having no room**, because a row at quantity zero is not
    /// written: a deposit that is full and a deposit that does not exist differ only in the
    /// number, and `wanted` says which by naming the row that would have had to be there.
    Overfull {
        held: String,
        by: String,
        /// The row of `by` that would have had to exist, written out.
        wanted: String,
        /// How much room there actually is, which is `0` where the row is absent.
        room: String,
    },
    /// More of a supply consumed in a place than is provided there.
    ///
    /// **One number for many kinds**: two transports and two scouts are six berths, and the
    /// refusal says so rather than naming whichever row was read last.
    Crowded {
        supply: String,
        /// The place it was counted in, as its key.
        at: String,
        /// The providing that would have had to be there, written out.
        wanted: String,
        /// What is provided.
        room: String,
    },
    /// A relation belongs to a family and does not declare one of the family's columns.
    UnlikeShape {
        family: String,
        member: String,
        column: String,
    },
    /// A kind carries a trait and declares no column of that name.
    ///
    /// **A trait is the column that holds how many are left**, so this is the half of the check
    /// that says a `carries` row names something real.
    CarriesNothing { relation: String, carried: String },
    /// A relation declares a column named for a trait and does not carry it.
    ///
    /// **The other half, and neither implies the other.** Without the first, a `carries` row
    /// could name a column that does not exist; without this one, a column could hold an
    /// allowance that no rule can reach, because `refresh` finds a kind through `carries` and
    /// not through its columns. **Checked in both directions is what keeps the two from drifting
    /// apart** while each stays individually true.
    DoesNotCarry { relation: String, carried: String },
    /// A rule is a part of two different composites.
    ///
    /// **A tree and not a graph** - Sean, 2026-09-18: *it must be able to organize the entirety of
    /// game rules is some type of acyclic graph or tree. Otherwise it will be impossible for a
    /// human player to understand how to play the game.* **One composite may name a rule twice**,
    /// which is two steps of one order rather than two parents, and is not this.
    TwoParents { rule: String, parents: Vec<String> },
    /// A composite reaches itself through its parts.
    ///
    /// **This is what lets the engine recurse with no depth counter.** A cycle here would be a
    /// rule that fires forever, which is the unboundedness the whole shape exists to refuse - so
    /// it is refused when the world is read rather than guarded against when it runs.
    CycleOfParts { rules: Vec<String> },
    /// A rule with clauses of its own and parts as well.
    ///
    /// **A rule is a leaf or a composite.** Both would make *what does this rule do* need two
    /// answers, and would leave the order between its clauses and its parts to whatever the
    /// engine happened to do first.
    BothLeafAndComposite { rule: String },
    /// A rule repeats and takes nothing out of the world, so the repetition has no bound.
    ///
    /// **A repetition draws from a pool that only shrinks**, and this is the rule with no pool: it
    /// would be able to fire again every time it fired, forever. **So the bound is a property of
    /// the data rather than a counter in the engine**, which is the same trade `CycleOfParts`
    /// makes - a structure that cannot run away is checked once when the world is read.
    ///
    /// **`remove` is the only role that shrinks it.** `require` reads, `add` and `put` make, and
    /// `keep` takes only what a capacity overflowed - which is nothing at all in a world that
    /// fits, so a repetition resting on it would stop after one firing or never.
    NeverStops { rule: String },
    /// A place holds more of something than what stands in it has room for.
    ///
    /// **Named in the words a person would use to ask about it**: the place, the thing, and the
    /// two numbers. Sean, 2026-09-18: *I want errors detectible with good error messages* - and
    /// *I am imagining a live recipe editor that will be able to reject invalid recipes and give
    /// the reason.*
    NoRoom {
        at: String,
        contained: String,
        used: i64,
        room: i64,
        /// The capacity that would have had to be there, written out.
        ///
        /// **The same answer `Overfull` and `Crowded` give**, and for the same reason: not *this
        /// is too many* but *there is no capacity this large*. It is what a `{refused}` section
        /// states, so a test can say what was missing rather than quoting a sentence.
        wanted: String,
    },
    /// A kind that is not fungible is declared loose.
    ///
    /// **`spec/logistics.md`**: *What a place holds of a kind is one number.* A kind keyed by
    /// anything more could hold two numbers in one place, and then taking what is over capacity
    /// would have to choose which row to take it from. **So the declaration is refused rather
    /// than the situation**, and `keep` has nothing to decide.
    ///
    /// Sean, 2026-09-19: *I am expecting that we can compute the amount of room for something, we
    /// can compute the excess, and discard the rest. I don't imagine we need to choose anything
    /// here.* **Two earlier versions of this put the choice where it would bite** - first a
    /// refusal when `keep` fired, then a count of the rows in a world - and both guarded a
    /// situation instead of forbidding what allows it.
    LooseAndNotFungible { relation: String, by: String },
    /// A limit between two relations whose keys are not the same columns.
    ///
    /// **Held and holder are compared key for key**, so a limit between relations that do not
    /// agree about what a row is keyed by has nothing to compare.
    CannotLimit { held: String, by: String },
    /// A value in a column that points at a row nothing states.
    NoSuchRow {
        relation: String,
        column: String,
        value: String,
        to: String,
    },
}

impl std::fmt::Display for Malformed {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Malformed::ColumnOfNothing { relation, column } => {
                write!(
                    out,
                    "`{column}` is a column of `{relation}`, which is not declared"
                )
            }
            Malformed::ReferenceOfNothing { column } => {
                write!(
                    out,
                    "a reference names the column `{column}`, which is not declared"
                )
            }
            Malformed::NoColumns { relation } => {
                write!(out, "`{relation}` declares no columns, so it has no key")
            }
            Malformed::Overfull {
                held,
                by,
                wanted,
                room,
            } => {
                write!(
                    out,
                    "`{held}` needs {wanted} and there is room for {room} in `{by}`"
                )
            }
            Malformed::Crowded {
                supply,
                at,
                wanted,
                room,
            } => {
                write!(
                    out,
                    "`{at}` consumes {supply} enough for {wanted} and is provided {room}"
                )
            }
            Malformed::UnlikeShape {
                family,
                member,
                column,
            } => {
                write!(out, "`{member}` is a `{family}` and declares no `{column}`")
            }
            Malformed::CarriesNothing { relation, carried } => {
                write!(
                    out,
                    "`{relation}` carries `{carried}` and declares no such column"
                )
            }
            Malformed::DoesNotCarry { relation, carried } => {
                write!(
                    out,
                    "`{relation}` declares `{carried}` and does not carry it"
                )
            }
            Malformed::TwoParents { rule, parents } => {
                write!(
                    out,
                    "`{rule}` is a part of {parents:?}, and a rule has one parent"
                )
            }
            Malformed::CycleOfParts { rules } => {
                write!(out, "these reach themselves through their parts: {rules:?}")
            }
            Malformed::BothLeafAndComposite { rule } => {
                write!(
                    out,
                    "`{rule}` has clauses and parts, and a rule has one or the other"
                )
            }
            Malformed::NeverStops { rule } => {
                write!(
                    out,
                    "`{rule}` repeats and removes nothing, so it would never stop"
                )
            }
            Malformed::NoRoom {
                at,
                contained,
                used,
                room,
                ..
            } => write!(
                out,
                "`{at}` holds {used} `{contained}` and has room for {room}"
            ),
            Malformed::LooseAndNotFungible { relation, by } => write!(
                out,
                "`{relation}` is not fungible - it is told apart by `{by}` - so it may not lie loose"
            ),
            Malformed::CannotLimit { held, by } => {
                write!(
                    out,
                    "`{held}` is limited by `{by}` and the two are not keyed alike"
                )
            }
            Malformed::BadOrder { relation, seq } => {
                write!(
                    out,
                    "`{relation}` numbers its columns {seq:?}, and they run from 1"
                )
            }
            Malformed::ReferencesNothing { column, to } => {
                write!(out, "`{column}` points at `{to}`, which is not declared")
            }
            Malformed::NoSuchRelation { relation } => {
                write!(out, "nothing declares a relation `{relation}`")
            }
            Malformed::WrongColumns {
                relation,
                wanted,
                given,
            } => {
                write!(
                    out,
                    "`{relation}` is ({wanted}) and this row gives ({given})"
                )
            }
            Malformed::IdAndQuantity { relation } => {
                write!(
                    out,
                    "`{relation}` declares both an `id` and a `quantity`, and a row is one or the other"
                )
            }
            Malformed::TwoWithOneKey { relation, key } => {
                let said = key
                    .iter()
                    .map(|(column, value)| format!("`{column}` of `{value}`"))
                    .collect::<Vec<String>>()
                    .join(" and ");
                write!(
                    out,
                    "two `{relation}` rows have {said}, so it names neither"
                )
            }

            Malformed::NoSuchRow {
                relation,
                column,
                value,
                to,
            } => {
                write!(
                    out,
                    "`{relation}`.`{column}` is `{value}`, and no `{to}` has that key"
                )
            }
        }
    }
}

/// A `seq` as the number it is, with anything that is not a number last.
///
/// **Read as text, `10` sorts between `1` and `2`.** Every ordering in the engine sorted the string
/// and `src/view.rs` sorted the number, so at ten parts the turn would have run one order while
/// `tree.txt` showed another - silently, because the test that compares that file compares the tree
/// against itself. **It is latent rather than live**: nothing has ten of anything yet.
///
/// **Found by Sean asking whether the cycle message was nondeterminism**, 2026-09-20. It is not,
/// and neither is this; both are the same smaller thing, an answer settled by something incidental
/// rather than by what was asked.
///
/// **A `seq` that is not a number sorts last rather than being refused.** Refusing it would make
/// every `seq` in the data load-bearing for a reason that has nothing to do with order, and
/// `tests/mutation.rs` asks exactly that question by writing `mutated` into one - so the strict
/// reading would blind the instrument that measures it.
pub fn ordinal(seq: &str) -> u64 {
    seq.parse().unwrap_or(u64::MAX)
}

/// Every relation there is.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Schema {
    relations: BTreeMap<String, Relation>,
    /// A column's id to the relation it belongs to and what it is called there.
    by_id: BTreeMap<String, (String, String)>,
    /// A family's name to the names of the relations that belong to it.
    ///
    /// **A family is an abstract relation: columns and no rows.** The columns are the shape its
    /// members share and the members are the set - Sean, 2026-09-17, having considered both an
    /// exists/not-exists trait and a set of kinds: *they are the same mechanism*, and
    /// `spec/data/families.4x` writes it as one.
    ///
    /// **Kept here rather than worked out twice.** Both the engine, deciding what an input ranges
    /// over, and the reference check, deciding whether a value is of the right kind, ask the same
    /// question - so it is answered in the one place that already turns ids into names.
    families: BTreeMap<String, Vec<String>>,
    /// The relations the world states, by name.
    ///
    /// **Templating is for the world and not for the structure.** `{member kind:scout
    /// family:unit}` names a family as *itself* and must not become one row per member;
    /// `{capacity ... for:resource}` names one where a member belongs and must. **What tells them
    /// apart is `{state ...}`**, which the data already says - so the rule is read from the data
    /// rather than written as a list of exceptions in here.
    stated: std::collections::BTreeSet<String>,
    /// A relation's id to its name.
    ///
    /// **Worked out while reading and then thrown away**, which meant every caller that needed it
    /// built it again from the rows it happened to hold - and `reified` was handed a `then`
    /// section with no `{relation ...}` rows in it and quietly expanded nothing. **A map built
    /// from the caller's rows answers a question about the caller's rows**, which is not the
    /// question anybody was asking.
    named: BTreeMap<String, String>,
    /// Which relation is held by which: `(extractor, deposit)` says there cannot be more
    /// extractors somewhere than there are deposits to hold them.
    ///
    /// **A constraint on the world rather than on a rule.** Sean, 2026-09-17: *We can't place an
    /// extractor if there are no available deposits* - and *the situation should be detectible and
    /// therefore preventable*. Detectable is this; preventable follows, because every rule already
    /// refuses the world it would leave if that world does not fit.
    limits: Vec<(String, String)>,
}

impl Schema {
    /// Read a schema from the `{relation ...}`, `{column ...}` and `{reference ...}` rows among
    /// whatever else is there.
    pub fn of(rows: &[Row]) -> Result<Schema, Malformed> {
        // **A relation is named by its id everywhere except in its own declaration.** The rows
        // that follow say `relation:16`, not `relation:residency`, so this book is what turns one
        // into the other - and it is read first because everything else depends on it.
        let mut named: BTreeMap<String, String> = BTreeMap::new();
        let mut relations: BTreeMap<String, Relation> = BTreeMap::new();
        for row in rows.iter().filter(|row| row.relation == RELATION) {
            let name = row.value(NAME).unwrap_or_default().to_string();
            named.insert(row.value(ID).unwrap_or_default().to_string(), name.clone());
            relations.insert(
                name.clone(),
                Relation {
                    name,
                    columns: Vec::new(),
                    attributes: Vec::new(),
                },
            );
        }

        // **What each column points at, gathered before the columns are built**, so that a
        // reference to a column that is declared later is not an error of ordering.
        let mut points_at: BTreeMap<String, String> = BTreeMap::new();
        for row in rows.iter().filter(|row| row.relation == REFERENCE) {
            let to = row.value(TO).unwrap_or_default();
            points_at.insert(
                row.value(COLUMN).unwrap_or_default().to_string(),
                named.get(to).cloned().unwrap_or_else(|| to.to_string()),
            );
        }

        let mut numbered: BTreeMap<String, Vec<(String, Column)>> = BTreeMap::new();
        let mut by_id: BTreeMap<String, (String, String)> = BTreeMap::new();
        for row in rows.iter().filter(|row| row.relation == COLUMN) {
            let of = row.value(RELATION).unwrap_or_default();
            let of = named.get(of).cloned().unwrap_or_else(|| of.to_string());
            let name = row.value(NAME).unwrap_or_default().to_string();
            let id = row.value(ID).unwrap_or_default().to_string();
            if !relations.contains_key(&of) {
                return Err(Malformed::ColumnOfNothing {
                    relation: of,
                    column: name,
                });
            }
            by_id.insert(id.clone(), (of.clone(), name.clone()));
            numbered.entry(of).or_default().push((
                row.value(SEQ).unwrap_or_default().to_string(),
                Column {
                    id: id.clone(),
                    name,
                    references: points_at.get(&id).cloned(),
                },
            ));
        }

        for column in points_at.keys() {
            if !by_id.contains_key(column) {
                return Err(Malformed::ReferenceOfNothing {
                    column: column.clone(),
                });
            }
        }

        // **Which columns are facts about a row rather than part of what it is.** Read before the
        // columns are attached, because `key()` asks the relation and the relation has to know.
        let mut attributes: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for row in rows.iter().filter(|row| row.relation == ATTRIBUTE) {
            let column = row.value(COLUMN).unwrap_or_default();
            let Some((of, name)) = by_id.get(column) else {
                return Err(Malformed::ReferenceOfNothing {
                    column: column.to_string(),
                });
            };
            attributes.entry(of.clone()).or_default().push(name.clone());
        }

        // **A family and its members, resolved to names here** so that nothing downstream has
        // to turn an id into a relation again.
        let mut families: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for row in rows.iter().filter(|row| row.relation == FAMILY) {
            let of = row.value(RELATION).unwrap_or_default();
            if let Some(name) = named.get(of) {
                families.entry(name.clone()).or_default();
            }
        }
        for row in rows.iter().filter(|row| row.relation == MEMBER) {
            let (Some(kind), Some(family)) = (row.value(KIND), row.value(FAMILY)) else {
                continue;
            };
            // **Members are kept as ids, because that is what a value carries.** A command says
            // `what:28`, a reference says `to:27`, and every other value in the data is an id -
            // so a family that answered in names would be the one place that did not.
            let Some(family) = named.get(family) else {
                continue;
            };
            if let Some(members) = families.get_mut(family) {
                members.push(kind.to_string());
            }
        }

        let mut limits: Vec<(String, String)> = Vec::new();
        for row in rows.iter().filter(|row| row.relation == LIMIT) {
            let held = row.value(HELD).unwrap_or_default();
            let by = row.value(BY).unwrap_or_default();
            limits.push((
                named.get(held).cloned().unwrap_or_else(|| held.to_string()),
                named.get(by).cloned().unwrap_or_else(|| by.to_string()),
            ));
        }

        for (of, mut columns) in numbered {
            columns.sort_by_key(|(seq, _)| ordinal(seq));
            let seq: Vec<String> = columns.iter().map(|it| it.0.clone()).collect();
            let wanted: Vec<String> = (1..=columns.len()).map(|it| it.to_string()).collect();
            if seq != wanted {
                return Err(Malformed::BadOrder { relation: of, seq });
            }
            let relation = relations.get_mut(&of).expect("declared above");
            relation.columns = columns.into_iter().map(|it| it.1).collect();
            relation.attributes = attributes.get(&of).cloned().unwrap_or_default();
        }

        for relation in relations.values() {
            if relation.columns.is_empty() {
                return Err(Malformed::NoColumns {
                    relation: relation.name.clone(),
                });
            }
            // **Identified or counted, never both.** Checked here rather than left to the key
            // computation, which would otherwise quietly drop `id` out of the key and go on.
            if relation.quantity().is_some()
                && relation.columns.iter().any(|column| column.name == ID)
            {
                return Err(Malformed::IdAndQuantity {
                    relation: relation.name.clone(),
                });
            }

            for column in &relation.columns {
                if let Some(to) = &column.references
                    && !relations.contains_key(to)
                {
                    return Err(Malformed::ReferencesNothing {
                        column: column.id.clone(),
                        to: to.clone(),
                    });
                }
            }
        }

        // **Every member declares the columns its family does.** That is what makes a family a
        // shape rather than only a set: a clause whose relation comes from an argument binds the
        // family's columns, and it can only do that if every member has them.
        for (family, members) in &families {
            let Some(shape) = relations.get(family) else {
                continue;
            };
            for member in members {
                let Some(member) = named.get(member) else {
                    continue;
                };
                let Some(declared) = relations.get(member) else {
                    continue;
                };
                for column in &shape.columns {
                    if !declared.columns.iter().any(|it| it.name == column.name) {
                        return Err(Malformed::UnlikeShape {
                            family: family.clone(),
                            member: member.to_string(),
                            column: column.name.clone(),
                        });
                    }
                }
            }
        }

        let stated = rows
            .iter()
            .filter(|row| row.relation == STATE)
            .filter_map(|row| named.get(row.value(RELATION)?).cloned())
            .collect();

        Ok(Schema {
            stated,
            named: named.clone(),
            relations,
            by_id,
            families,
            limits,
        })
    }

    /// The relations belonging to `family`, or `None` where it is not a family.
    /// Whether the world states rows of this relation, rather than it describing the structure.
    pub fn stated(&self, relation: &str) -> bool {
        self.stated.contains(relation)
    }

    /// The name of the relation with this id.
    pub fn relation_named(&self, id: &str) -> Option<&str> {
        self.named.get(id).map(String::as_str)
    }

    pub fn members(&self, family: &str) -> Option<&[String]> {
        self.families.get(family).map(Vec::as_slice)
    }

    pub fn relation(&self, name: &str) -> Option<&Relation> {
        self.relations.get(name)
    }

    pub fn names(&self) -> Vec<&str> {
        self.relations.keys().map(String::as_str).collect()
    }

    /// The relation a column id belongs to, and what it is called there.
    pub fn column(&self, id: &str) -> Option<(&str, &str)> {
        self.by_id
            .get(id)
            .map(|(relation, name)| (relation.as_str(), name.as_str()))
    }

    /// A row written back in its relation's declared column order.
    ///
    /// **Not alphabetically, which is what the notation would do on its own.** A `BTreeMap` holds
    /// the values by key and has no idea what order the relation wants them in; this is the only
    /// place that does.
    pub fn write(&self, row: &Row) -> String {
        let Some(relation) = self.relation(&row.relation) else {
            return crate::notation::write(row);
        };
        let mut out = String::from("{");
        out.push_str(&row.relation);
        for column in &relation.columns {
            if let Some(value) = row.value(&column.name) {
                out.push(' ');
                out.push_str(&column.name);
                out.push(':');
                out.push_str(value);
            }
        }
        out.push('}');
        out
    }

    /// Whether one row fits: a declared relation, and exactly its columns.
    ///
    /// **Exactly, rather than at least.** A row with a column nobody declared is as wrong as one
    /// missing a column, and both are the data saying something the structure does not allow.
    pub fn fits(&self, row: &Row) -> Result<&Relation, Malformed> {
        let Some(relation) = self.relation(&row.relation) else {
            return Err(Malformed::NoSuchRelation {
                relation: row.relation.clone(),
            });
        };
        let wanted: Vec<&str> = relation.columns.iter().map(|it| it.name.as_str()).collect();
        let given: Vec<&str> = row.values.keys().map(String::as_str).collect();
        let mut sorted = wanted.clone();
        sorted.sort_unstable();
        if sorted != given {
            return Err(Malformed::WrongColumns {
                relation: row.relation.clone(),
                wanted: wanted.join(" "),
                given: given.join(" "),
            });
        }
        Ok(relation)
    }
}

/// Every row, with a family named where a member belongs replaced by one row per member.
///
/// **A family named where a member is expected means each member.** That is the rule
/// `{refresh what:unit trait:moving}` already runs on, applied to a row instead of to a command -
/// so `{capacity of:bin for:resource what:resource per:territory} -> 10` is three rows, one per
/// resource. **Named twice in one row it means the same member**, which is what makes a bin hold
/// what it carries rather than everything.
///
/// **Sean, 2026-09-18**: *if we declared resource = [food, metal, energy], we could have
/// bin[resource] and transport[resource], which would need to be reified to a leaf resource by
/// some mechanic.* This is that mechanic, and it is substitution rather than computation: the
/// expansion is bounded by the family's size and cannot reach itself, so **nothing downstream
/// learns a new word** - every check reads the plain rows it always read.
///
/// **A value is a slot only where its column points at the family it names.** `{member kind:28
/// family:26}` says `26` in a column pointing at `relation`, not at `unit`, so nothing expands
/// there - which is the difference between a value that *is* a family and a value that *names one
/// where a member belongs*.
pub fn reified(schema: &Schema, rows: Vec<Row>) -> Vec<Row> {
    let mut out = Vec::new();
    for row in rows {
        let Some(declared) = schema.relations.get(&row.relation) else {
            out.push(row);
            continue;
        };
        // **Only what the world states.** A row describing the structure names a family as the
        // thing it is - `{member kind:scout family:unit}` - and expanding that would turn one
        // membership into two. A row of the world names one where a member belongs.
        if !schema.stated(&row.relation) {
            out.push(row);
            continue;
        }
        // **A column that points at something, holding the name of a family.** The column's own
        // reference is not enough to go on: `capacity.for` points at `relation`, because what a
        // thing has room for may be any kind - so what says *each member* is the **value**, and
        // the reference only says the value is a name rather than a number.
        let mut slots: Vec<(String, String)> = Vec::new();
        for column in &declared.columns {
            if column.references.is_none() {
                continue;
            }
            let Some(value) = row.value(&column.name) else {
                continue;
            };
            let Some(family) = schema.relation_named(value) else {
                continue;
            };
            if schema.members(family).is_some() {
                slots.push((column.name.clone(), family.to_string()));
            }
        }
        if slots.is_empty() {
            out.push(row);
            continue;
        }

        // **One variable per family, however many columns name it.** Two columns naming
        // `resource` are one choice made twice, not two choices.
        let mut families: Vec<String> = Vec::new();
        for (_, family) in &slots {
            if !families.contains(family) {
                families.push(family.clone());
            }
        }

        let mut made = vec![row];
        for family in &families {
            let members = schema.members(family).unwrap_or(&[]).to_vec();
            let mut next = Vec::new();
            for row in &made {
                for member in &members {
                    let mut one = row.clone();
                    for (column, of) in &slots {
                        if of == family {
                            one.values.insert(column.clone(), member.clone());
                        }
                    }
                    next.push(one);
                }
            }
            made = next;
        }
        out.extend(made);
    }
    out
}

/// Every row fits its relation, its key is its own, and every reference points at a row.
pub fn check(schema: &Schema, rows: &Store) -> Result<(), Malformed> {
    // **A key names one row.** A reference is a key, so a key naming two rows is a reference that
    // names neither - and nothing checked it until it was looked for.
    let mut taken: BTreeMap<(&str, Vec<&str>), usize> = BTreeMap::new();
    for row in rows.rows() {
        let Some(relation) = schema.relation(&row.relation) else {
            continue;
        };
        // **A relation is identified or counted and never both**, so a key of several columns and
        // a key of one are read the same way here: `key()` says which columns, and this counts
        // what the row carries in them.
        let key = relation.key();
        let Some(values) = key
            .iter()
            .map(|column| row.value(column))
            .collect::<Option<Vec<&str>>>()
        else {
            continue;
        };
        let seen = taken
            .entry((relation.name.as_str(), values.clone()))
            .or_default();
        *seen += 1;
        if *seen > 1 {
            return Err(Malformed::TwoWithOneKey {
                relation: row.relation.clone(),
                key: key
                    .iter()
                    .zip(values)
                    .map(|(column, value)| (column.to_string(), value.to_string()))
                    .collect(),
            });
        }
    }

    for row in rows.rows() {
        let relation = schema.fits(row)?;
        for column in &relation.columns {
            let Some(to) = &column.references else {
                continue;
            };
            let value = row.value(&column.name).unwrap_or_default();
            // **A reference to a family is a reference to its members.** A family has no rows, so
            // asking whether one of them carries this key would refuse everything; what the value
            // names is a relation, and the question is whether that relation belongs.
            if let Some(members) = schema.members(to) {
                if members.iter().any(|it| it == value) {
                    continue;
                }
                return Err(Malformed::NoSuchRow {
                    relation: row.relation.clone(),
                    column: column.name.clone(),
                    value: value.to_string(),
                    to: to.clone(),
                });
            }
            let declared = schema
                .relation(to)
                .expect("checked when the schema was read");
            let there = rows
                .rows()
                .iter()
                .any(|it| it.relation == *to && it.value(declared.identity()) == Some(value));
            if !there {
                return Err(Malformed::NoSuchRow {
                    relation: row.relation.clone(),
                    column: column.name.clone(),
                    value: value.to_string(),
                    to: to.clone(),
                });
            }
        }
    }

    // **A trait is checked against the columns both ways.** `{carries kind:scout trait:moving}`
    // says a scout has a `moving` to spend, and `{column ... name:moving}` is where the number
    // lives - two statements of one fact, which is the shape that drifts.
    //
    // **Sean, 2026-09-18**, on why the duplication is allowed to stand: *One reason I resist
    // duplication is to guard against the inconsistency. Another reason is to keep the model
    // simple. Inconsistency can be mitigated by automated checks. Simplicity is more important
    // from the expression side that I audit than it is for the implementation details.* This is
    // that mitigation, and the expression side keeps both words.
    //
    // **After the references, for the reason the block below is after them too.** Point
    // `carries.kind` at a key nothing has and the kind carrying that trait is simply gone - so a
    // check running first answers *`unit` declares `moving` and does not carry it* to a question
    // about a dangling reference. **The narrower fault is the one to report**, and the mutation
    // sweep is what said so, twice.
    let named: BTreeMap<&str, &str> = rows
        .rows()
        .iter()
        .filter(|row| row.relation == RELATION)
        .filter_map(|row| Some((row.value(ID)?, row.value(NAME)?)))
        .collect();
    let traits: Vec<&str> = rows
        .rows()
        .iter()
        .filter(|row| row.relation == TRAIT)
        .filter_map(|row| row.value(NAME))
        .collect();
    let mut carried: Vec<(&str, &str)> = Vec::new();
    for row in rows.rows().iter().filter(|row| row.relation == CARRIES) {
        let (Some(kind), Some(of)) = (row.value(KIND), row.value(TRAIT)) else {
            continue;
        };
        // **A value naming nothing is skipped, because somebody else refuses it.** Both columns
        // carry a `{reference ...}` row, and the loop above has already walked them - so a kind
        // that is not a relation, or a trait that is not a trait, has been reported by the check
        // whose subject that is. **What is left here is the pair**, which is this check's.
        //
        // **By id and not also by name**, unlike everywhere else in this file: `check` is reached
        // only from `Game::of`, which is handed the foundation rows. A by-name arm was written
        // here while this block still sat in `Schema::of`, where `tests/directories.rs` does hand
        // it the friendly rows - and it came along when the block moved, still carrying the
        // reason it had there. **The code was right and the reason had stopped being true**,
        // which is the failure `docs/working-with-an-assistant.md` is about.
        let Some(kind) = named.get(kind) else {
            continue;
        };
        let Some(of) = rows
            .rows()
            .iter()
            .find(|it| it.relation == TRAIT && it.value(ID) == Some(of))
            .and_then(|it| it.value(NAME))
        else {
            continue;
        };
        carried.push((*kind, of));
    }
    for (relation, of) in &carried {
        let declares = schema
            .relation(relation)
            .is_some_and(|it| it.columns.iter().any(|column| column.name == *of));
        if !declares {
            return Err(Malformed::CarriesNothing {
                relation: relation.to_string(),
                carried: of.to_string(),
            });
        }
    }
    for name in schema.names() {
        let declared = schema.relation(name).expect("named just above");
        for column in &declared.columns {
            if traits.contains(&column.name.as_str())
                && !carried
                    .iter()
                    .any(|(kind, of)| *kind == name && *of == column.name)
            {
                return Err(Malformed::DoesNotCarry {
                    relation: name.to_string(),
                    carried: column.name.clone(),
                });
            }
        }
    }

    // **The rules are a tree, and that is what makes them readable and the engine safe.** Sean,
    // 2026-09-18: *it must be able to organize the entirety of game rules is some type of acyclic
    // graph or tree. Otherwise it will be impossible for a human player to understand how to play
    // the game.*
    //
    // **Two graphs and only one of them can be this.** What a rule produces that another consumes
    // is cyclic here already - `build-extractor` takes metal and makes an extractor, `work` takes
    // an extractor and makes metal - and that cycle is the economy. **What is checked here is
    // containment**, which rule is a step of which, and a weighting is what makes the other one
    // safe.
    //
    // **After the references**, for the reason every check below is: a `part` pointing at no rule
    // is that reference's to refuse, and a check answering first would say *two parents* about a
    // dangling id.
    let mut parent: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for row in rows.rows().iter().filter(|row| row.relation == PART) {
        let (Some(of), Some(is)) = (row.value(OF), row.value(IS)) else {
            continue;
        };
        let seen = parent.entry(is).or_default();
        // **Named twice by one composite is one parent.** `end-turn` refreshes once for `moving`
        // and once for `working`, and a person reading the tree sees two steps rather than two
        // owners.
        if !seen.contains(&of) {
            seen.push(of);
        }
    }
    // **A rule's id is not a relation's id**, and `named` above is the relation map - so reading
    // a rule through it turns `refresh` into `state`, which is the name of whatever relation
    // happens to share the number. Found by the test asserting the words rather than the ids.
    let rule_named: BTreeMap<&str, &str> = rows
        .rows()
        .iter()
        .filter(|row| row.relation == RULE)
        .filter_map(|row| Some((row.value(ID)?, row.value(NAME)?)))
        .collect();
    let shown = |id: &str| rule_named.get(id).unwrap_or(&id).to_string();
    for (rule, parents) in &parent {
        if parents.len() > 1 {
            return Err(Malformed::TwoParents {
                rule: shown(rule),
                parents: parents.iter().map(|it| shown(it)).collect(),
            });
        }
    }

    // **Every rule has at most one parent now, so a cycle is a walk upwards that repeats.** No
    // general graph search is needed, and the walk is bounded by the number of rules.
    for start in parent.keys() {
        let mut walked: Vec<&str> = vec![start];
        let mut at = *start;
        while let Some(above) = parent.get(at).and_then(|it| it.first()) {
            // **From where the walk met itself, and not from where it began.** A walk that starts
            // below a cycle passes through rules that are not in it, and naming those in the
            // message sends a reader to look at a rule that is fine. Found by `adjust-population`
            // landing above `end-turn` in the walk order and joining a cycle it is not part of.
            if let Some(joined) = walked.iter().position(|it| it == above) {
                // **Rotated to a fixed start, so the message names the cycle and not the walk.**
                // A cycle of two rules can be written two ways and they mean the same thing, and
                // which one came back depended on where the walk began - so an unrelated rule
                // arriving reordered a message about a fault it was not part of. **That is not
                // nondeterminism**, because the same data always gave the same answer; it is the
                // smaller thing next to it, an answer settled by something incidental. Sean,
                // 2026-09-20: *while canonicalise is not as important as nondeterminism, I see no
                // reason not to be just as strict about it.*
                let mut cycle: Vec<String> = walked[joined..].iter().map(|it| shown(it)).collect();
                let first = cycle
                    .iter()
                    .enumerate()
                    .min_by(|left, right| left.1.cmp(right.1))
                    .map(|(at, _)| at)
                    .unwrap_or(0);
                cycle.rotate_left(first);
                return Err(Malformed::CycleOfParts { rules: cycle });
            }
            walked.push(above);
            at = above;
        }
    }

    // **A rule is a leaf or a composite.**
    for row in rows.rows().iter().filter(|row| row.relation == RULE) {
        let Some(id) = row.value(ID) else { continue };
        let composite = rows
            .rows()
            .iter()
            .any(|it| it.relation == PART && it.value(OF) == Some(id));
        let leaf = rows
            .rows()
            .iter()
            .any(|it| it.relation == CLAUSE && it.value(RULE) == Some(id));
        if composite && leaf {
            return Err(Malformed::BothLeafAndComposite { rule: shown(id) });
        }
    }

    // **A repetition needs something to consume.** It fires as many times as it can, and what
    // makes *as many as it can* a number is that every firing takes something out of a pool that
    // began finite. A rule that removes nothing would answer *forever*.
    //
    // **A composite reaches this too, and that is the right answer rather than a near miss.** It
    // has no clauses of its own, so it removes nothing of its own - what its parts consume is
    // their business, and a repetition of the whole would be reasoning about a pool nobody here
    // can see.
    let role_named: BTreeMap<&str, &str> = rows
        .rows()
        .iter()
        .filter(|row| row.relation == ROLE)
        .filter_map(|row| Some((row.value(ID)?, row.value(NAME)?)))
        .collect();
    for row in rows.rows().iter().filter(|row| row.relation == REPEATS) {
        let Some(rule) = row.value(RULE) else {
            continue;
        };
        let shrinks = rows.rows().iter().any(|it| {
            it.relation == CLAUSE
                && it.value(RULE) == Some(rule)
                && it
                    .value(ROLE)
                    .map(|role| *role_named.get(role).unwrap_or(&role) == REMOVE)
                    .unwrap_or(false)
        });
        if !shrinks {
            return Err(Malformed::NeverStops { rule: shown(rule) });
        }
    }

    // **Last, because breaking a reference breaks this too.** Point a deposit's `where` at a key
    // nothing has and the extractors over it are suddenly over nothing - so a check that ran
    // first would answer *too many extractors* to a question about a dangling reference, and
    // `tests/mutation.rs` said exactly that. **The narrower fault is the one to report.**
    only_what_is_fungible_lies_loose(schema, rows)?;
    held_within_what_holds_it(schema, rows)?;
    nothing_crowds_a_place(schema, rows)?;
    nothing_holds_more_than_there_is_room_for(schema, rows)?;
    Ok(())
}

/// What has room for what, where, and how much of it there is.
///
/// **One question, however many kinds answer it.** A place's room for a thing is the sum of what
/// stands in it that can hold that thing, so the capacity rows are grouped before anything is
/// counted - `spec/logistics.md`: *a place's capacity for a kind is the sum of what is in it that
/// can hold that kind*.
///
/// **Read by two callers and written once**, which is the point of it being here rather than
/// inside either: [`check`] refuses a world where something that may not exceed its room does, and
/// the `keep` role takes away what a loose kind has beyond it. **Two readings of one arithmetic
/// cannot disagree about what fits.**
pub struct Rooming {
    /// The relation the room is for.
    pub held: String,
    /// The trait value it is for, where the held kind carries one.
    pub what: String,
    /// The relation whose rows the room is counted in.
    pub per: String,
    /// How much room there is, by the key of the place.
    pub room: BTreeMap<String, i64>,
    /// How much is there, by the key of the place.
    pub used: BTreeMap<String, i64>,
    /// The capacity rows that answered, as `(container, how much each)`.
    pub gives: Vec<(String, i64)>,
    /// The values of this question's columns, as the rows write them.
    pub written: (String, String, String),
}

/// Where a row of `relation` stands, as a column of it and whether it is the place itself.
///
/// **A kind that *is* the place is one of itself in itself**; a kind that references the place says
/// which one in the column that points at it.
pub fn place_of(schema: &Schema, relation: &str, per: &str) -> Option<(bool, String)> {
    let declared = schema.relation(relation)?;
    if relation == per {
        return Some((true, declared.identity().to_string()));
    }
    let column = declared
        .columns
        .iter()
        .find(|it| it.references.as_deref() == Some(per))?;
    Some((false, column.name.clone()))
}

/// Every room question the capacity rows ask, answered.
pub fn rooming(schema: &Schema, rows: &Store) -> Vec<Rooming> {
    let named: BTreeMap<&str, &str> = rows
        .rows()
        .iter()
        .filter(|row| row.relation == RELATION)
        .filter_map(|row| Some((row.value(ID)?, row.value(NAME)?)))
        .collect();
    let name_of = |id: &str| named.get(id).copied().unwrap_or(id).to_string();

    // **Grouped by what is held, where, so the containers can be added up.** **In the order the
    // rows are stated**, so a refusal names the same one twice.
    let mut asked: Vec<(String, String, String)> = Vec::new();
    let mut giving: BTreeMap<(String, String, String), Vec<(String, i64)>> = BTreeMap::new();
    for capacity in rows.rows().iter().filter(|it| it.relation == CAPACITY) {
        let (Some(of), Some(held), Some(what), Some(per), Some(each)) = (
            capacity.value(OF),
            capacity.value(FOR),
            capacity.value(WHAT),
            capacity.value(PER),
            capacity
                .value(QUANTITY)
                .and_then(|it| it.parse::<i64>().ok()),
        ) else {
            continue;
        };
        let question = (held.to_string(), what.to_string(), per.to_string());
        if !asked.contains(&question) {
            asked.push(question.clone());
        }
        giving
            .entry(question)
            .or_default()
            .push((of.to_string(), each));
    }

    let mut out = Vec::new();
    for question in asked {
        let (raw_held, raw_what, raw_per) = question.clone();
        let (held, per) = (name_of(&raw_held), name_of(&raw_per));

        // **Totalled per place, at a rate each.** A row that does not carry the trait the capacity
        // names is a different thing and is not counted here.
        let total = |relation: &str, rate: i64| -> BTreeMap<String, i64> {
            let mut found: BTreeMap<String, i64> = BTreeMap::new();
            let Some((is_place, column)) = place_of(schema, relation, &per) else {
                return found;
            };
            let Some(declared) = schema.relation(relation) else {
                return found;
            };
            let carries_what = declared.columns.iter().any(|it| it.name == WHAT);
            for row in rows.rows().iter().filter(|it| it.relation == relation) {
                if carries_what && row.value(WHAT) != Some(raw_what.as_str()) {
                    continue;
                }
                let Some(at) = row.value(&column) else {
                    continue;
                };
                let how_many = match declared.quantity() {
                    Some(quantity) if !is_place => row
                        .value(quantity)
                        .and_then(|it| it.parse::<i64>().ok())
                        .unwrap_or(0),
                    _ => 1,
                };
                *found.entry(at.to_string()).or_default() += how_many * rate;
            }
            found
        };

        let gives = giving
            .get(&question)
            .expect("asked is built from giving")
            .clone();
        let mut room: BTreeMap<String, i64> = BTreeMap::new();
        for (of, each) in &gives {
            for (at, how_much) in total(&name_of(of), *each) {
                *room.entry(at).or_default() += how_much;
            }
        }
        let used = total(&held, 1);
        out.push(Rooming {
            held,
            what: name_of(&raw_what),
            per,
            room,
            used,
            gives,
            written: (raw_held, raw_what, raw_per),
        });
    }
    out
}

/// Whether a kind may lie loose - over its capacity, in disorder - rather than being bounded by it.
///
/// **`{loose kind:resource}` names a family and means its members**, resolved here rather than by
/// reification: a structural row is not `{state ...}` and so is never expanded.
pub fn is_loose(schema: &Schema, rows: &Store, relation: &str) -> bool {
    loose_kinds(schema, rows).iter().any(|it| it == relation)
}

/// Only what is fungible may lie loose.
///
/// # What fungible means here, and why it decides this
///
/// **Two of a fungible kind in one place are interchangeable**, so there is no such thing as
/// *which one*. `spec/logistics.md`: *What a place holds of a kind is one number.* Taking what is
/// over capacity is then arithmetic - compute the room, compute the excess, take it - with nothing
/// to choose between.
///
/// **A kind that carries state is not fungible.** Two scouts differing in `moving` are not
/// interchangeable: one may act and one may not. Taking one away would have to say which, and a
/// rule that picks is a rule nobody wrote. Sean, 2026-09-19: *things without state are fungible in
/// a way things with state are not.*
///
/// **It is read off the key, which is where it already lived.** A kind is fungible when its key is
/// at most where it is and what it is of:
///
/// ```text
/// metal      (where)                 fungible
/// bin        (where, what)           fungible - `what` is what a capacity groups by
/// deposit    (where, what)           fungible - `density` is an attribute and out of the key
/// scout      (where, moving)         not - `moving` is state
/// extractor  (where, what, working)  not - `working` is state
/// territory  (id)                    never - an id is the opposite of fungible
/// ```
///
/// **This is why a scout row looks the way it does.** `{scout where:territory-1 moving:1} -> 2` is
/// two scouts counted as one number precisely because nothing but `moving` tells them apart.
/// **Fungibility did not arrive with disorder** - it is the reason the rows have had a quantity
/// since the quantity landed, and disorder is the first thing that had to name it.
///
/// **`{carries ...}` says the same thing a second way and is not the one to read.** Every kind
/// that carries a trait is told apart by it, so the two agree on every kind the game has - but a
/// state column that is not a declared trait would still divide a kind, and `carries` would not
/// see it. **The key is the fact; `carries` is a use of it.**
///
/// **It is derived and not declared, deliberately.** A `{fungible ...}` row would state twice what
/// the key states once, and unlike `carries` - which `refresh` reads - nothing would read it but
/// the check that can already work it out. Sean's reason for letting `carries` duplicate does not
/// reach it: *inconsistency can be mitigated by automated checks* justifies a second statement
/// that earns something, and this one would earn nothing.
///
/// # What would have to change for a non-fungible kind to be loose
///
/// **A priority, and nothing less.** If units over a berth capacity should be lost rather than
/// refused, the loss has to say *which* - spent before fresh, say. That ordering is the only thing
/// that makes it a rule rather than an accident, and nothing has asked for it.
///
/// # Three versions of this check, and why the first two were wrong
///
/// Sean, 2026-09-19: *I am expecting that we can compute the amount of room for something, we can
/// compute the excess, and discard the rest. I don't imagine we need to choose anything here.*
///
/// **A refusal when `keep` fired** put a choice nobody could make exactly where it would bite.
/// **A count of the rows in a world** guarded the situation instead of forbidding what allows it -
/// and sat inside the loop over capacity questions, so it only saw kinds something already gave
/// room to. **Marking `extractor` loose was accepted silently**, because nothing gives extractors
/// capacity, and the poison being taken is what revealed it. This one reads the `{loose ...}` rows.
fn only_what_is_fungible_lies_loose(schema: &Schema, rows: &Store) -> Result<(), Malformed> {
    for relation in loose_kinds(schema, rows) {
        let Some(declared) = schema.relation(&relation) else {
            continue;
        };
        let mut rest = declared.key();
        rest.retain(|it| *it != WHAT);
        if rest.len() > 1 {
            return Err(Malformed::LooseAndNotFungible {
                relation,
                by: rest[1].to_string(),
            });
        }
    }
    Ok(())
}

/// Every kind that may lie loose, with a family standing for its members.
pub fn loose_kinds(schema: &Schema, rows: &Store) -> Vec<String> {
    let mut out = Vec::new();
    for named in rows
        .rows()
        .iter()
        .filter(|row| row.relation == LOOSE)
        .filter_map(|row| row.value(KIND))
        .filter_map(|kind| schema.relation_named(kind))
    {
        match schema.members(named) {
            Some(members) => out.extend(
                members
                    .iter()
                    .filter_map(|it| schema.relation_named(it))
                    .map(str::to_string),
            ),
            None => out.push(named.to_string()),
        }
    }
    out
}

/// No place holds more of a thing than what stands in it has room for, unless that thing is loose.
///
/// ```text
/// {capacity of:territory for:bin      what:resource per:territory} -> 4
/// {capacity of:bin       for:resource what:resource per:territory} -> 10
/// {loose kind:resource}
/// ```
///
/// **A capacity is a maximum unless the thing it holds is loose**, and then it is a threshold: what
/// is beyond it is in disorder, still spendable, and taken at the turn's end by
/// `discard-disorder`. Sean, 2026-09-19: *structures behave differently from resources in this
/// regard [...] structures are intentionally built for the purpose of having that structure there,
/// while resources are mined to be spent or stored.*
///
/// **No rule mentions any of it**, as with every limit here: a command that would overfill a place
/// leaves a world that does not fit, and every rule already refuses that.
fn nothing_holds_more_than_there_is_room_for(
    schema: &Schema,
    rows: &Store,
) -> Result<(), Malformed> {
    for asked in rooming(schema, rows) {
        if is_loose(schema, rows, &asked.held) {
            continue;
        }
        for (at, used) in &asked.used {
            let there = asked.room.get(at).copied().unwrap_or(0);
            if *used <= there {
                continue;
            }
            // **The refusal names a capacity that would have had to be there**, the way a full
            // deposit and a crowded place already do.
            //
            // **One kind gives room, and it is named**: what its containers would each have to
            // hold to close the whole gap, which is exact.
            //
            // **Several kinds give room, and none of them is the answer** - so rather than pick
            // one, the refusal names the *place itself* granting the shortfall. Sean, 2026-09-15:
            // *We should never have non-determinism from what row happens to be encountered
            // first*, which is why `NotOne` refuses instead of choosing.
            let (raw_held, raw_what, raw_per) = &asked.written;
            let (of, each_would_be) = match asked.gives.as_slice() {
                [(only, _)] => {
                    let mine = schema
                        .relation_named(only)
                        .and_then(|it| place_of(schema, it, &asked.per).map(|_| it))
                        .map(|it| count_in(schema, rows, it, &asked.what, &asked.per, at))
                        .unwrap_or(0);
                    let apiece = match mine {
                        0 => *used,
                        how_many => {
                            used.div_euclid(how_many) + i64::from(used.rem_euclid(how_many) != 0)
                        }
                    };
                    (only.clone(), apiece)
                }
                _ => (raw_per.clone(), used - there),
            };
            let mut wanted = BTreeMap::new();
            wanted.insert(OF.to_string(), of);
            wanted.insert(FOR.to_string(), raw_held.clone());
            wanted.insert(WHAT.to_string(), raw_what.clone());
            wanted.insert(PER.to_string(), raw_per.clone());
            wanted.insert(QUANTITY.to_string(), each_would_be.to_string());
            return Err(Malformed::NoRoom {
                // **The place is a row of `per` and not a relation**, so it is shown as its kind
                // and its key.
                at: format!("{} {at}", asked.per),
                contained: asked.held.clone(),
                used: *used,
                room: there,
                wanted: schema.write(&Row {
                    relation: CAPACITY.to_string(),
                    values: wanted,
                }),
            });
        }
    }
    Ok(())
}

/// How many rows of `relation` carrying `what` stand in one place.
fn count_in(schema: &Schema, rows: &Store, relation: &str, what: &str, per: &str, at: &str) -> i64 {
    let Some((is_place, column)) = place_of(schema, relation, per) else {
        return 0;
    };
    let Some(declared) = schema.relation(relation) else {
        return 0;
    };
    let carries_what = declared.columns.iter().any(|it| it.name == WHAT);
    let mut found = 0;
    for row in rows.rows().iter().filter(|it| it.relation == relation) {
        if carries_what && schema.relation_named(row.value(WHAT).unwrap_or_default()) != Some(what)
        {
            continue;
        }
        if row.value(&column) != Some(at) {
            continue;
        }
        found += match declared.quantity() {
            Some(quantity) if !is_place => row
                .value(quantity)
                .and_then(|it| it.parse::<i64>().ok())
                .unwrap_or(0),
            _ => 1,
        };
    }
    found
}

/// No place consumes more of a supply than is provided there, each kind counting at its own rate.
///
/// **Sean, 2026-09-17**: *there are certain things I always want to see in tests because I need to
/// compute the tests in my head.* So the amounts are rows of the world, stated in a test's `given`
/// and read where the test is read - and a layer that would have kept them out of a scenario is a
/// layer hiding what the test is about. **Comprehension wins and the layer bends**, which is his
/// ruling rather than an inference.
///
/// ```text
/// {provides kind:territory what:berth} -> 6
/// {consumes kind:scout     what:berth} -> 1
/// {consumes kind:transport what:berth} -> 2
/// ```
///
/// **A provider either is the place or is in one.** A territory provides berths at itself; a store
/// would provide room at the territory it stands in, and a place's capacity is then the sum of what
/// is in it that provides - which is `spec/logistics.md`'s sentence, reached without another idea.
///
/// **The place is declared, not guessed.** `{supply ... per:territory}` says what a supply is
/// measured in, so nothing has to work out which relation every provider and consumer has in
/// common.
///
/// **No rule mentions any of it**, as with every limit here: a command that would overfill a place
/// leaves a world that does not fit, and every rule already refuses that.
fn nothing_crowds_a_place(schema: &Schema, rows: &Store) -> Result<(), Malformed> {
    let supplies: Vec<&Row> = rows
        .rows()
        .iter()
        .filter(|it| it.relation == SUPPLY)
        .collect();
    if supplies.is_empty() {
        return Ok(());
    }
    let by_id: BTreeMap<&str, &str> = rows
        .rows()
        .iter()
        .filter(|it| it.relation == RELATION)
        .filter_map(|it| Some((it.value(ID)?, it.value(NAME)?)))
        .collect();

    for supply in supplies {
        let (Some(id), Some(named), Some(per)) =
            (supply.value(ID), supply.value(NAME), supply.value(PER))
        else {
            continue;
        };
        let Some(of_place) = by_id.get(per).map(|it| it.to_string()) else {
            continue;
        };

        // **How much each side is worth, per kind.** A kind named by neither is not in the
        // arithmetic at all, which is how everything that is not a vehicle stays out of it.
        let amounts = |relation: &str| -> BTreeMap<String, i64> {
            let mut found = BTreeMap::new();
            for row in rows.rows().iter().filter(|it| it.relation == relation) {
                if row.value(WHAT) != Some(id) {
                    continue;
                }
                let (Some(kind), Some(rate)) = (row.value(KIND), row.value(QUANTITY)) else {
                    continue;
                };
                let Some(kind) = by_id.get(kind) else {
                    continue;
                };
                found.insert(kind.to_string(), rate.parse().unwrap_or(0));
            }
            found
        };

        // **Rows of a kind, counted into the place each sits in.** A kind that *is* the place is
        // one of itself in itself; a kind that references the place is however many it says.
        let counted = |kinds: &BTreeMap<String, i64>| -> BTreeMap<String, i64> {
            let mut total: BTreeMap<String, i64> = BTreeMap::new();
            for (kind, rate) in kinds {
                let Some(declared) = schema.relation(kind) else {
                    continue;
                };
                if *kind == of_place {
                    for row in rows.rows().iter().filter(|it| it.relation == *kind) {
                        if let Some(at) = row.value(declared.identity()) {
                            *total.entry(at.to_string()).or_default() += rate;
                        }
                    }
                    continue;
                }
                let (Some(at_column), Some(quantity)) = (
                    declared
                        .columns
                        .iter()
                        .find(|it| it.references.as_deref() == Some(of_place.as_str())),
                    declared.quantity(),
                ) else {
                    continue;
                };
                for row in rows.rows().iter().filter(|it| it.relation == *kind) {
                    let (Some(at), Some(how_many)) =
                        (row.value(&at_column.name), row.value(quantity))
                    else {
                        continue;
                    };
                    *total.entry(at.to_string()).or_default() +=
                        how_many.parse::<i64>().unwrap_or(0) * rate;
                }
            }
            total
        };

        let provided = counted(&amounts(PROVIDES));
        let consumed = counted(&amounts(CONSUMES));
        let providers = amounts(PROVIDES);

        for (at, taken) in consumed {
            let room = provided.get(&at).copied().unwrap_or(0);
            if taken <= room {
                continue;
            }
            // **The refusal names the providing that would have had to be there**, which is the
            // answer every limit here gives: not *this is too many* but *nothing provides this
            // much*.
            let of_kind = providers
                .keys()
                .next()
                .cloned()
                .unwrap_or_else(|| of_place.clone());
            let by_name: BTreeMap<&str, &str> = by_id.iter().map(|(a, b)| (*b, *a)).collect();
            let mut wanted = BTreeMap::new();
            wanted.insert(
                KIND.to_string(),
                by_name.get(of_kind.as_str()).unwrap_or(&"").to_string(),
            );
            wanted.insert(WHAT.to_string(), id.to_string());
            wanted.insert(QUANTITY.to_string(), taken.to_string());
            return Err(Malformed::Crowded {
                supply: named.to_string(),
                at,
                wanted: schema.write(&Row {
                    relation: PROVIDES.to_string(),
                    values: wanted,
                }),
                room: room.to_string(),
            });
        }
    }
    Ok(())
}

/// No more of a held thing anywhere than there is room for it.
///
/// **This is a reference with a number on it.** An ordinary reference asks whether the row it
/// points at exists; this asks whether it exists *and has room*, and the two are the same question
/// where the room is one. **Both halves come out of the same comparison**, because a row at
/// quantity zero is never written - so a deposit that is full and a deposit that is not there at
/// all differ only in what the number is.
///
/// **No rule says any of this.** `build-extractor` adds an extractor and nothing else; every rule
/// already refuses the world it would leave when that world does not fit, so a rule written
/// tomorrow is bound by this without knowing it exists.
fn held_within_what_holds_it(schema: &Schema, rows: &Store) -> Result<(), Malformed> {
    for (held, by) in &schema.limits {
        let (Some(holder), Some(holds)) = (schema.relation(held), schema.relation(by)) else {
            return Err(Malformed::CannotLimit {
                held: held.clone(),
                by: by.clone(),
            });
        };
        let (Some(counted), Some(room_in)) = (holder.quantity(), holds.quantity()) else {
            return Err(Malformed::CannotLimit {
                held: held.clone(),
                by: by.clone(),
            });
        };
        // **The container's key must be part of the held thing's.** A deposit is keyed by
        // `(where, what)` and an extractor by `(where, what, working)`, and the extractors of one
        // deposit are every row that agrees on the deposit's columns - so a subset rather than
        // equality, and a limit between relations that share no key at all still says so.
        let key = holds.key();
        if !key.iter().all(|column| holder.key().contains(column)) {
            return Err(Malformed::CannotLimit {
                held: held.clone(),
                by: by.clone(),
            });
        }

        // **The held rows are summed over the container's key.** They were matched key for key
        // until an extractor gained a `working` column: a deposit holds an extractor whatever
        // state it is in, so one spent and one fresh are two rows of the same deposit. **The sum
        // is what stops two groups each fitting while together they do not.**
        let mut taken: BTreeMap<Vec<String>, i64> = BTreeMap::new();
        for row in rows.rows().iter().filter(|it| it.relation == *held) {
            let Some(values) = key
                .iter()
                .map(|column| row.value(column).map(str::to_string))
                .collect::<Option<Vec<String>>>()
            else {
                continue;
            };
            *taken.entry(values).or_default() += row
                .value(counted)
                .and_then(|it| it.parse::<i64>().ok())
                .unwrap_or(0);
        }

        for (values, how_many) in taken {
            let there = rows
                .rows()
                .iter()
                .filter(|it| it.relation == *by)
                .find(|it| {
                    key.iter()
                        .zip(&values)
                        .all(|(column, value)| it.value(column) == Some(value.as_str()))
                });
            let room: i64 = there
                .and_then(|it| it.value(room_in))
                .and_then(|it| it.parse().ok())
                .unwrap_or(0);
            if how_many <= room {
                continue;
            }
            // **The refusal names the row that would have had to be there**, which is what a test
            // can state and what a reader can act on: not *this is too many* but *there is no
            // deposit with room for this many*.
            let mut wanted = BTreeMap::new();
            for (column, value) in key.iter().zip(&values) {
                wanted.insert(column.to_string(), value.clone());
            }
            wanted.insert(room_in.to_string(), how_many.to_string());
            return Err(Malformed::Overfull {
                held: held.clone(),
                by: by.clone(),
                wanted: schema.write(&Row {
                    relation: by.clone(),
                    values: wanted,
                }),
                room: room.to_string(),
            });
        }
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    /// **A tenth step comes after the ninth, and as text it came after the first.**
    ///
    /// This is the whole defect, and both halves are asserted: what the engine does now, and what
    /// it did until 2026-09-20. **Nothing in the data has ten of anything**, so no test of the game
    /// could have caught it - the order only starts to differ at ten, and `tree.txt` would have
    /// gone on agreeing with itself while the turn ran something else.
    #[test]
    fn a_tenth_step_sorts_after_the_ninth_and_not_after_the_first() {
        let mut by_number = vec!["10", "2", "1", "9"];
        by_number.sort_by_key(|it| super::ordinal(it));
        assert_eq!(by_number, vec!["1", "2", "9", "10"]);

        let mut as_text = vec!["10", "2", "1", "9"];
        as_text.sort();
        assert_eq!(
            as_text,
            vec!["1", "10", "2", "9"],
            "which is what every ordering in this engine did, and is the defect"
        );
    }

    use super::*;
    use crate::notation::read;

    /// **A relation whose columns are not in alphabetical order**, which the relations the data
    /// declares all happen to be - `tests/schema.rs` asserts that they are, so this exists for a
    /// reason a reader can check rather than one they have to take.
    const BACKWARDS: &str = "\
{relation name:pair}
{column id:pair.zero relation:pair seq:1 name:zero}
{column id:pair.also relation:pair seq:2 name:also}
";

    #[test]
    fn a_row_is_written_in_the_order_its_relation_declares() {
        let schema = Schema::of(&read(BACKWARDS).expect("a schema")).expect("read");
        let row = read("{pair also:2 zero:1}").expect("a row")[0].clone();

        assert_eq!(
            crate::notation::write(&row),
            "{pair also:2 zero:1}",
            "the notation on its own writes the keys in the order they sort"
        );
        assert_eq!(
            schema.write(&row),
            "{pair zero:1 also:2}",
            "and the schema writes them in the order the relation declares"
        );
    }

    /// Every way a schema can fail to be one, and the count so that none is untested.
    #[test]
    fn a_schema_that_is_not_one_says_what_about_it() {
        let refused = [
            (
                "{relation name:pair}\n{column id:a relation:other seq:1 name:a}\n",
                Malformed::ColumnOfNothing {
                    relation: "other".to_string(),
                    column: "a".to_string(),
                },
            ),
            (
                "{relation name:pair}\n",
                Malformed::NoColumns {
                    relation: "pair".to_string(),
                },
            ),
            (
                "{relation name:pair}\n{column id:a relation:pair seq:2 name:a}\n",
                Malformed::BadOrder {
                    relation: "pair".to_string(),
                    seq: vec!["2".to_string()],
                },
            ),
            (
                "{relation name:pair}\n{column id:a relation:pair seq:1 name:a}\n\
                 {reference column:a to:nowhere}\n",
                Malformed::ReferencesNothing {
                    column: "a".to_string(),
                    to: "nowhere".to_string(),
                },
            ),
            (
                "{relation name:pair}\n{column id:a relation:pair seq:1 name:a}\n\
                 {reference column:elsewhere to:pair}\n",
                Malformed::ReferenceOfNothing {
                    column: "elsewhere".to_string(),
                },
            ),
        ];
        for (stated, expected) in &refused {
            let why = Schema::of(&read(stated).expect(stated)).expect_err(stated);
            assert_eq!(&why, expected, "{stated}");
        }
        assert_eq!(refused.len(), 5, "five ways, and each is checked");
    }

    #[test]
    fn a_row_gives_exactly_the_columns_its_relation_declares() {
        let schema = Schema::of(&read(BACKWARDS).expect("a schema")).expect("read");
        schema
            .fits(&read("{pair zero:1 also:2}").expect("a row")[0])
            .expect("both columns, and no others");

        for wrong in [
            "{pair zero:1}",
            "{pair zero:1 also:2 spare:3}",
            "{other zero:1}",
        ] {
            let row = read(wrong).expect(wrong)[0].clone();
            schema.fits(&row).expect_err(wrong);
        }
    }
}
