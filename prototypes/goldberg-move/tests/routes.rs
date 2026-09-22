//! The rule of this prototype, checked without a window.
//!
//! **What is worth testing here is not that a route exists but that an ambiguous one is
//! refused**, because that is the whole of Sean's constraint: *I can only select destinations
//! that leave the path unambiguous, otherwise I have to click on the intermediate
//! destinations.* A test that only checked routes were found would pass for an interface that
//! picked one of six arbitrarily.

use goldberg_move::board::{Board, Reach};
use goldberg_move::plan::{Disk, Plan, Said};

/// The solid the prototype opens on: `GP(2, 0)`, the third smallest.
fn board() -> Board {
    Board::goldberg(2, 0)
}

#[test]
fn the_third_smallest_goldberg_solid_has_forty_two_territories() {
    // **Twelve, thirty-two, forty-two** - `GP(1,0)`, `GP(1,1)`, `GP(2,0)`. The three are
    // asserted together rather than the third alone, because *third smallest* is a claim about
    // the order and not about the number.
    let counts: Vec<usize> = [(1, 0), (1, 1), (2, 0)]
        .into_iter()
        .map(|(m, n)| Board::goldberg(m, n).territories())
        .collect();
    assert_eq!(counts, vec![12, 32, 42]);
}

#[test]
fn every_territory_is_reachable_from_every_other() {
    let board = board();
    let mut counted = 0;
    for from in 0..board.territories() as u32 {
        for to in 0..board.territories() as u32 {
            assert_ne!(
                board.reach(from, to),
                Reach::Nowhere,
                "{from} cannot reach {to}, so the board is in pieces"
            );
            counted += 1;
        }
    }
    // A count over nothing is the same failure with the sign flipped - `CLAUDE.md`.
    assert_eq!(counted, 42 * 42);
}

#[test]
fn a_neighbour_is_one_step_and_never_ambiguous() {
    let board = board();
    let mut counted = 0;
    for from in 0..board.territories() as u32 {
        for &to in &board.neighbours[from as usize] {
            assert_eq!(
                board.reach(from, to),
                Reach::One(vec![to]),
                "{from} touches {to}, so there is one way and it is one step"
            );
            counted += 1;
        }
    }
    // **A Goldberg solid's edge count, twice**, since each edge is walked from both ends.
    // `GP(2,0)` has 42 faces and 120 edges - Euler's formula over its 80 vertices.
    assert_eq!(counted, 240, "every edge, from both of its ends");
}

/// **Both answers happen, and this is what says so.**
///
/// A rule that refused everything would pass every test about refusals, and one that refused
/// nothing would pass every test about routes. The two counts together are the statement that
/// the board actually poses the question the interface exists to answer.
#[test]
fn some_destinations_are_unambiguous_and_some_are_not() {
    let board = board();
    let (mut one, mut many) = (0, 0);
    for from in 0..board.territories() as u32 {
        for to in 0..board.territories() as u32 {
            match board.reach(from, to) {
                Reach::One(route) => {
                    assert_eq!(route.last(), Some(&to), "a route ends at its destination");
                    one += 1;
                }
                Reach::Many { ways, steps } => {
                    assert!(ways > 1, "`Many` is more than one way");
                    assert!(steps > 1, "a neighbour is never ambiguous");
                    many += 1;
                }
                Reach::Arrived => assert_eq!(from, to),
                Reach::Nowhere => panic!("{from} cannot reach {to}"),
            }
        }
    }
    assert!(
        one > 0,
        "no destination is unambiguous, so nothing is ever clickable"
    );
    assert!(
        many > 0,
        "no destination is ambiguous, so the rule this prototype is about never fires"
    );
    assert_eq!(one + many + 42, 42 * 42, "every pair is one of the three");
}

/// A route the interface offers is a route you could have walked one territory at a time.
#[test]
fn every_offered_route_is_a_chain_of_neighbours() {
    let board = board();
    let mut checked = 0;
    for from in 0..board.territories() as u32 {
        for to in 0..board.territories() as u32 {
            let Reach::One(route) = board.reach(from, to) else {
                continue;
            };
            let mut here = from;
            for &step in &route {
                assert!(
                    board.neighbours[here as usize].contains(&step),
                    "{here} does not touch {step}"
                );
                here = step;
                checked += 1;
            }
        }
    }
    assert!(checked > 100, "only {checked} steps, so this proved little");
}

/// **An ambiguous destination becomes two unambiguous ones**, which is the escape the rule
/// leaves the player and the reason it is a constraint rather than a wall.
#[test]
fn clicking_a_territory_on_the_way_makes_the_rest_unambiguous() {
    let board = board();
    let mut opened = 0;
    for from in 0..board.territories() as u32 {
        for to in 0..board.territories() as u32 {
            let Reach::Many { .. } = board.reach(from, to) else {
                continue;
            };
            let helps = board.neighbours[from as usize].iter().any(|&through| {
                matches!(board.reach(from, through), Reach::One(_))
                    && matches!(board.reach(through, to), Reach::One(_))
            });
            if helps {
                opened += 1;
            }
        }
    }
    assert!(
        opened > 0,
        "no ambiguous destination is opened up by clicking a neighbour first"
    );
}

#[test]
fn a_disk_is_picked_up_and_put_down_by_the_same_click() {
    let board = board();
    let mut disks = [Disk { standing: 0 }];
    let mut plan = Plan::Idle;

    assert_eq!(plan.clicked(&board, &mut disks, 0), Said::Selected(0));
    assert_eq!(plan.disk(), Some(0));
    assert_eq!(plan.clicked(&board, &mut disks, 0), Said::Deselected);
    assert_eq!(plan, Plan::Idle);
}

#[test]
fn clicking_where_no_disk_stands_does_nothing() {
    let board = board();
    let mut disks = [Disk { standing: 0 }];
    let mut plan = Plan::Idle;
    assert!(matches!(
        plan.clicked(&board, &mut disks, 5),
        Said::Nothing(_)
    ));
    assert_eq!(plan, Plan::Idle);
}

/// The four sentences in order: pick up, lay out, extend, finalise.
#[test]
fn a_move_is_laid_out_then_finalised_by_clicking_the_destination_twice() {
    let board = board();
    let start = 0;
    let mut disks = [Disk { standing: start }];
    let mut plan = Plan::Idle;

    assert_eq!(plan.clicked(&board, &mut disks, start), Said::Selected(0));

    // Somewhere one step away, which is never ambiguous.
    let first = board.neighbours[start as usize][0];
    assert_eq!(
        plan.clicked(&board, &mut disks, first),
        Said::Reached {
            at: first,
            steps: 1
        }
    );
    assert_eq!(plan.route(), [first]);
    assert_eq!(disks[0].standing, start, "nothing has moved yet");

    // A second step, from the head rather than from the start.
    let second = *board.neighbours[first as usize]
        .iter()
        .find(|&&other| other != start && !board.neighbours[start as usize].contains(&other))
        .expect("a neighbour of the first step that is not next to the start");
    assert_eq!(
        plan.clicked(&board, &mut disks, second),
        Said::Reached {
            at: second,
            steps: 2
        }
    );
    assert_eq!(plan.route(), [first, second]);

    // The same destination again finalises it.
    assert_eq!(
        plan.clicked(&board, &mut disks, second),
        Said::Moved {
            disk: 0,
            to: second
        }
    );
    assert_eq!(disks[0].standing, second);
    assert_eq!(plan, Plan::Idle);
}

#[test]
fn an_ambiguous_destination_is_refused_and_changes_nothing() {
    let board = board();
    let start = 0;
    let (to, ways, steps) = (0..board.territories() as u32)
        .find_map(|to| match board.reach(start, to) {
            Reach::Many { ways, steps } => Some((to, ways, steps)),
            _ => None,
        })
        .expect("some destination is ambiguous from territory 0");

    let mut disks = [Disk { standing: start }];
    let mut plan = Plan::Idle;
    plan.clicked(&board, &mut disks, start);
    let was = plan.clone();

    assert_eq!(
        plan.clicked(&board, &mut disks, to),
        Said::Refused { ways, steps }
    );
    assert_eq!(plan, was, "a refusal leaves the move exactly where it was");
    assert_eq!(disks[0].standing, start);
}

#[test]
fn clicking_the_disks_own_territory_abandons_the_move() {
    let board = board();
    let start = 0;
    let mut disks = [Disk { standing: start }];
    let mut plan = Plan::Idle;

    plan.clicked(&board, &mut disks, start);
    let step = board.neighbours[start as usize][0];
    plan.clicked(&board, &mut disks, step);
    assert_eq!(plan.route().len(), 1);

    assert_eq!(plan.clicked(&board, &mut disks, start), Said::Cancelled);
    assert_eq!(plan, Plan::Idle);
    assert_eq!(disks[0].standing, start, "an abandoned move moves nothing");
}

/// **No limit on how far**, which Sean asked for in those words - so a route across the whole
/// solid is laid out one unambiguous click at a time and finalised like any other.
#[test]
fn a_move_can_cross_the_whole_planet() {
    let board = board();
    let start = 0;
    // The territory furthest from the start, whichever it is.
    let far = (0..board.territories() as u32)
        .max_by_key(|&to| match board.reach(start, to) {
            Reach::One(route) => route.len(),
            Reach::Many { steps, .. } => steps,
            _ => 0,
        })
        .expect("a furthest territory");

    let mut disks = [Disk { standing: start }];
    let mut plan = Plan::Idle;
    plan.clicked(&board, &mut disks, start);

    // Walk there a neighbour at a time, which is always allowed however far it is.
    let mut here = start;
    let mut steps = 0;
    while here != far {
        let Reach::One(route) = board.reach(here, far) else {
            // Ambiguous from here, so take one step along any shortest way.
            let next = *board.neighbours[here as usize]
                .iter()
                .find(|&&other| shorter(&board, other, far, here))
                .expect("a neighbour nearer the destination");
            plan.clicked(&board, &mut disks, next);
            here = next;
            steps += 1;
            continue;
        };
        for step in route {
            plan.clicked(&board, &mut disks, step);
            here = step;
            steps += 1;
        }
    }
    assert!(
        steps > 3,
        "the furthest territory is only {steps} steps away"
    );
    assert_eq!(plan.route().len(), steps);
    assert_eq!(
        plan.clicked(&board, &mut disks, far),
        Said::Moved { disk: 0, to: far }
    );
    assert_eq!(disks[0].standing, far);
}

/// Whether `other` is nearer `to` than `here` is.
fn shorter(board: &Board, other: u32, to: u32, here: u32) -> bool {
    let far = |from: u32| match board.reach(from, to) {
        Reach::Arrived => 0,
        Reach::One(route) => route.len(),
        Reach::Many { steps, .. } => steps,
        Reach::Nowhere => usize::MAX,
    };
    far(other) < far(here)
}
