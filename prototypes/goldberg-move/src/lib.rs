//! Moving a disk across a Goldberg planet, and nothing else.
//!
//! **The question.** Sean, 2026-09-21, asking for a prototype rather than a feature: can a
//! player lay out a move of any length across a sphere by clicking, without the interface ever
//! having to guess which way round an obstacle he meant? The answer this prototype offers is
//! that the interface never guesses, because **a destination is only clickable when exactly one
//! shortest route reaches it** - and where several do, he clicks an intermediate territory and
//! the question becomes two smaller ones, each with one answer.
//!
//! **No game mechanics.** There is no cost, no turn, no limit on distance and no rule about
//! what a disk is. What is being tried out is the gesture.
//!
//! The two modules here are the whole of it and neither knows Bevy exists: [`board`] answers
//! *which routes reach there*, [`plan`] answers *what does this click do*. `main.rs` is the
//! composition root that turns a cursor into a territory index and hands it over.

pub mod board;
pub mod plan;
