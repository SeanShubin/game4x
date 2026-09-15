//! A thin engine that runs a game from data, with no game in it.
//!
//! **The question is `C-114`'s open half**: the main tree's engine has a struct per game noun and
//! a method per rule, and the data describes what the code already knows. Can the engine instead
//! know nothing, and the data say everything? `README.md` states the question and, when there is
//! one, the answer.
//!
//! Three modules and nothing else:
//!
//! - [`notation`] - a line is a row of a relation
//! - [`store`] - the rows that are true, and matching a pattern against them
//! - [`engine`] - bind a command, check what the rule needs, apply what it changes
//!
//! **No `main`, no file reading, no argument parsing.** The engine takes `&str` and rows; the
//! tests read `data/`. That is what makes *no file read outside its own directory* true by
//! construction rather than by discipline - there is no `std::fs` in `src/` at all, which
//! `tests/isolation.rs` asserts.

pub mod engine;
pub mod notation;
pub mod schema;
pub mod store;
