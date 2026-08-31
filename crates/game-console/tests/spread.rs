//! How far the loop can actually be played. `R-6`.

use game_console::{Library, Outcome, Session};

struct Files;

impl Library for Files {
    fn fetch(&self, name: &str) -> Option<String> {
        std::fs::read_to_string(format!("../../commands/{name}.4x")).ok()
    }
    fn names(&self) -> Vec<String> {
        vec!["setup".into(), "play".into(), "spread".into()]
    }
}

fn run(session: &mut Session, line: &str) -> String {
    match session.run(line, &Files) {
        Ok(Outcome::Said(said)) => said,
        Ok(other) => format!("{other:?}"),
        Err(why) => format!("REFUSED: {why:?}"),
    }
}

#[test]
fn the_loop_is_played_as_far_as_the_rules_allow() {
    let mut session = Session::new();
    run(&mut session, "run setup");
    run(&mut session, "start");
    let played = run(&mut session, "run play");
    assert!(!played.contains("REFUSED"), "play.4x: {played}");
    let spread = run(&mut session, "run spread");
    assert!(!spread.contains("REFUSED"), "spread.4x: {spread}");

    for id in 1..=12u32 {
        println!("{}", run(&mut session, &format!("show territory {id}")));
    }
}
