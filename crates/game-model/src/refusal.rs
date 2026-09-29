//! Why a command did not happen, said in terms of the data rather than of the engine.
//!
//! **Its own file because it is output and not logic.** Sean, 2026-09-16: *I need to be able to
//! understand the core input->processing->output loop, so I will need the input and output data
//! structures, but any generic utility logic not specific to the game should be elsewhere.* This
//! is one of the two output types - the other is `Effect` - and thirty-odd lines of it are the
//! words a refusal is written in, which is not the loop.

use crate::notation::Row;
use crate::schema::Malformed;

/// Why a command did not happen, said in terms of the data rather than of the engine.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Refused {
    /// Nothing states a `{command id:...}` with that id.
    NoSuchCommand { id: String },
    /// The command's rule declares an input the command gives no argument for.
    Missing { rule: String, input: String },
    /// An argument's value is not a key of the relation its input is typed as.
    ///
    /// **This is *no such place*, and it arrives from the structure rather than from a rule.**
    WrongType {
        rule: String,
        input: String,
        value: String,
        of: String,
    },
    /// A clause names a role that is not `require`, `remove` or `add`.
    NoSuchRole {
        rule: String,
        clause: String,
        role: String,
    },
    /// A clause has a column bound to no input, so the row it wants cannot be built.
    Unbound {
        rule: String,
        clause: String,
        column: String,
    },
    /// Everything was bound and the world does not agree.
    NotSo { rule: String, wanted: Row },
    /// A clause reads a value out of the row an earlier clause matched, and that clause did not
    /// match exactly one.
    ///
    /// **Not *no row* and not *a row*.** Several rows matching is the case this exists for: the
    /// engine would otherwise read whichever was encountered first - Sean, 2026-09-15: *We should
    /// never have non-determinism from what row happens to be encountered first.*
    NotOne {
        rule: String,
        clause: String,
        found: usize,
    },
    /// The rule removes something several rows match, so it did not say which.
    ///
    /// **The taking side of `NotOne`.** A reading from a clause that matched several rows is
    /// already refused rather than guessed at; taking still found the first. Sean, 2026-09-19: *I
    /// would like it to be an error condition to ever return one row when you could have returned
    /// another. That is nondeterminism and it should be possible to structure the code to make
    /// nondeterminism impossible by raising an error instead.*
    ///
    /// **A clause reaches this by not naming a trait.** Two citizens differing in `hungry` are two
    /// rows, and *remove one citizen* does not say which - so the answer is the clause saying more,
    /// not the engine choosing.
    NotOneToTake {
        rule: String,
        wanted: Row,
        found: usize,
    },
    /// The rule removes something no row matches, so the rule contradicts itself.
    NothingToRemove { rule: String, wanted: Row },
    /// A `{minted}` names a clause that matches rather than makes.
    ///
    /// **Minting an id to look for asks the world for a row nobody has made.** `require` and
    /// `remove` match a pattern and `add` builds a whole row, and only the second has an id to
    /// choose - so this is the data saying something that cannot mean anything, refused rather than
    /// answered with a number.
    MintedWhereNothingIsMade {
        rule: String,
        clause: String,
        column: String,
    },
    /// A `put` would assign a trait to a kind that does not carry one.
    ///
    /// **`refresh` is one rule over every trait, so the pairing is checked when it fires.** An
    /// input's type is fixed and which kinds carry a trait is not, so `{refresh what:extractor
    /// trait:moving}` cannot be refused by the type - `extractor` is a relation and `moving` is a
    /// trait, and only the two together are wrong.
    DoesNotCarry {
        rule: String,
        relation: String,
        carried: String,
    },
    /// The rule left a world that does not fit the structure.
    ///
    /// **Boxed, because this is the one refusal that carries another error.** `Malformed` grew a
    /// variant naming a row and a number, and a `Result` whose error is that large is paid for on
    /// every call that succeeds.
    Broke { rule: String, why: Box<Malformed> },
}

impl Refused {
    /// The same words, with every row it names written by `say`.
    ///
    /// **A refusal is read by a person and the engine cannot name anything.** Three of these
    /// carry a row, and a row in the form the engine holds says `{deposit where:4 what:50}` -
    /// true, and of no use to somebody reading `scenario/main.4x`, where the same row is
    /// `{deposit where:place-4 what:energy}`. **The names live in `friendly-notation`**, which
    /// is a dev-dependency here and must stay one: `tests/isolation.rs` holds that the engine
    /// depends on no crate.
    ///
    /// **So the caller brings the writer.** `Display` passes the plain one and a boundary that
    /// has a `Names` passes that, which is one set of words rendered two ways rather than two
    /// sets that can drift.
    pub fn told(&self, say: &dyn Fn(&Row) -> String) -> String {
        match self {
            Refused::NotSo { rule, wanted } => {
                format!("`{rule}` needs {} and it is not", say(wanted))
            }
            Refused::NotOneToTake {
                rule,
                wanted,
                found,
            } => format!(
                "`{rule}` removes {} and {found} rows match, so it has not said which",
                say(wanted)
            ),
            Refused::NothingToRemove { rule, wanted } => {
                format!("`{rule}` removes {} and nothing matched", say(wanted))
            }
            other => format!("{other}"),
        }
    }
}

impl std::fmt::Display for Refused {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Refused::NoSuchCommand { id } => write!(out, "no command is stated with id `{id}`"),
            Refused::Missing { rule, input } => write!(out, "`{rule}` wants `{input}`"),
            Refused::WrongType {
                rule,
                input,
                value,
                of,
            } => {
                write!(
                    out,
                    "`{rule}`.`{input}` is `{value}`, and no `{of}` has that key"
                )
            }
            Refused::NoSuchRole { rule, clause, role } => {
                write!(
                    out,
                    "`{rule}`.`{clause}` has the role `{role}`, which is not one"
                )
            }
            Refused::Unbound {
                rule,
                clause,
                column,
            } => {
                write!(out, "`{rule}`.`{clause}` binds nothing to `{column}`")
            }
            Refused::NotSo { rule, wanted } => write!(
                out,
                "`{rule}` needs {} and it is not",
                crate::notation::write(wanted)
            ),
            Refused::NotOne {
                rule,
                clause,
                found,
            } => write!(
                out,
                "`{rule}`.`{clause}` is read from and matched {found} rows, not one"
            ),
            Refused::NotOneToTake {
                rule,
                wanted,
                found,
            } => write!(
                out,
                "`{rule}` removes {} and {found} rows match, so it has not said which",
                crate::notation::write(wanted)
            ),
            Refused::NothingToRemove { rule, wanted } => {
                write!(
                    out,
                    "`{rule}` removes {} and nothing matched",
                    crate::notation::write(wanted)
                )
            }
            Refused::DoesNotCarry {
                rule,
                relation,
                carried,
            } => write!(
                out,
                "`{rule}` refreshes `{carried}`, and no `{relation}` carries one"
            ),
            Refused::Broke { rule, why } => write!(out, "`{rule}` would leave a world where {why}"),
            Refused::MintedWhereNothingIsMade {
                rule,
                clause,
                column,
            } => write!(
                out,
                "`{rule}` mints `{column}` in clause `{clause}`, which matches rather than makes"
            ),
        }
    }
}
