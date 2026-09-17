//! Two ways of reading a world, and neither is the game.
//!
//! **Sean, 2026-09-16**: *any generic utility logic not specific to the game should be elsewhere.*
//! Writing the rows out is not what the engine does; it is what a reader does with what the engine
//! left - so it is here rather than beside the loop.

use crate::engine::Game;

const RELATION: &str = "relation";
const STATE: &str = "state";

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
}
