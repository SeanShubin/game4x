//! What a click does, and nothing about how a click is detected.
//!
//! **A state machine over territory indices**, so the whole interaction can be driven from a
//! test with no window, no camera and no picking. What the Bevy layer does is turn a cursor
//! position into a territory and hand it here.

use crate::board::{Board, Reach, Where};

/// A coloured disk, and the territory it is standing on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Disk {
    pub standing: Where,
}

/// How far the player has got.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Plan {
    /// Nothing selected.
    Idle,
    /// A disk is selected and no destination has been chosen.
    Holding { disk: usize },
    /// A disk is selected and a route is being laid out. `route` is the territories stepped
    /// onto, in order; the disk's own territory is not in it, and the last entry is where the
    /// move has reached.
    Plotting { disk: usize, route: Vec<Where> },
}

/// What happened, in words the status line can show.
///
/// **A refusal says why and a success says what**, because the one rule a player has to learn
/// here is which destinations are allowed - and *nothing happened* teaches it to nobody.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Said {
    Nothing(&'static str),
    Selected(usize),
    Deselected,
    Reached { at: Where, steps: usize },
    Refused { ways: usize, steps: usize },
    Moved { disk: usize, to: Where },
    Cancelled,
}

impl Plan {
    /// Which disk is selected, if any.
    pub fn disk(&self) -> Option<usize> {
        match self {
            Plan::Idle => None,
            Plan::Holding { disk } | Plan::Plotting { disk, .. } => Some(*disk),
        }
    }

    /// The route so far, which is empty unless a destination has been chosen.
    pub fn route(&self) -> &[Where] {
        match self {
            Plan::Plotting { route, .. } => route,
            _ => &[],
        }
    }

    /// Where the move has reached: the end of the route, or the disk's own territory.
    pub fn head(&self, disks: &[Disk]) -> Option<Where> {
        match self {
            Plan::Idle => None,
            Plan::Holding { disk } => Some(disks[*disk].standing),
            Plan::Plotting { disk, route } => {
                Some(route.last().copied().unwrap_or(disks[*disk].standing))
            }
        }
    }

    /// One click on one territory.
    ///
    /// **Sean's four sentences, in order**: click a disk to select it, click the same disk to
    /// deselect it, click a territory to lay out a move and recentre on it, click that same
    /// territory again to finalise. Extending and recentring is the same arm as laying out,
    /// because *a different destination* is a destination like any other.
    pub fn clicked(&mut self, board: &Board, disks: &mut [Disk], face: Where) -> Said {
        match self.clone() {
            Plan::Idle => match disks.iter().position(|disk| disk.standing == face) {
                Some(disk) => {
                    *self = Plan::Holding { disk };
                    Said::Selected(disk)
                }
                None => Said::Nothing("no disk there - click a disk to pick it up"),
            },

            Plan::Holding { disk } if disks[disk].standing == face => {
                *self = Plan::Idle;
                Said::Deselected
            }

            // **Clicking the disk's own territory abandons the move**, which is the escape the
            // four sentences do not name and every prototype needs. It is the same click that
            // deselects before a route exists, so there is one thing to remember rather than
            // two.
            Plan::Plotting { disk, .. } if disks[disk].standing == face => {
                *self = Plan::Idle;
                Said::Cancelled
            }

            Plan::Holding { disk } | Plan::Plotting { disk, .. } => {
                let from = self.head(disks).expect("a disk is selected");
                if from == face {
                    // The head clicked a second time: the move is finalised.
                    let route = self.route().to_vec();
                    disks[disk].standing = face;
                    let steps = route.len();
                    *self = Plan::Idle;
                    return match steps {
                        0 => Said::Deselected,
                        _ => Said::Moved { disk, to: face },
                    };
                }
                match board.reach(from, face) {
                    Reach::One(more) => {
                        let mut route = self.route().to_vec();
                        route.extend(more);
                        let steps = route.len();
                        *self = Plan::Plotting { disk, route };
                        Said::Reached { at: face, steps }
                    }
                    Reach::Many { ways, steps } => Said::Refused { ways, steps },
                    Reach::Arrived => Said::Nothing("the move is already there"),
                    Reach::Nowhere => Said::Nothing("nothing reaches that territory"),
                }
            }
        }
    }
}
