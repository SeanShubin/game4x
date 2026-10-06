# Specification

**Authored.** Sean owns every idea here. Claude may rephrase and reorganize what is already
present, reporting every change; a new idea is entered by Sean himself, whether he types it
or pastes it from a [proposal](../docs/notes/proposals.md).

[Root README](../README.md) · [Documentation map](../docs/README.md) · [Notes](../docs/notes/README.md)

What the game **is**, stated normatively, **and the shape of the thing that runs it**. If a rule
is not written here, it is not decided, no matter how thoroughly it was discussed.

## The documents

| Document                     | What it specifies                                                |
| ---------------------------- | ---------------------------------------------------------------- |
| [Invariants](invariants.md)  | Statements that are always true; every other document obeys them |
| [Narrative](narrative.md)    | The fiction the rules implement                                  |
| [The planet](planet.md)      | The sphere, its territories, and what a territory carries        |
| [Resources](resources.md)    | The list of resources                                            |
| [Structures](structures.md)  | The list of structures and what each one does                    |
| [Units](units.md)            | What is true of every unit                                       |
| [Unit types](unit-types.md)  | Each particular unit, one section apiece                         |
| [Economy](economy.md)        | Extraction, structures and labor                                 |
| [Logistics](logistics.md)    | Where materials are, and moving them to where they are needed    |
| [Population](population.md)  | Citizens, how they grow, and the labor they provide              |
| [The turn](turn.md)          | The order in which a turn resolves                               |
| [Control](future/control.md) | **A future plan.** How a game is won and lost                    |
| [Force](future/force.md)     | **A future plan.** Force, garrisons, and holding ground          |
| [Interface](interface.md)    | What the player sees and can reach                               |
| [Console](console.md)        | The command language                                             |
| [Combat](future/combat.md)   | **A future plan.** Ranges, weapons, resolution                   |
| [Orbit](orbit.md)            | The orbital layer and what sits in it                            |
| [Scenarios](scenarios.md)    | The scenarios that demonstrate the game, and what each is for    |

Add a file when a topic firms up. Add its row here first.

## Rules for this directory

1. **Present tense, normative.** "A missile has a range." Not "a missile could have."
2. **One topic per file.** If a file grows its own table of contents, split it.
3. **If it is not here, it is not decided.** Discussion is not decision.

   **A test is the primary statement.** What the game does is decided by a test that runs, read
   and approved one at a time, and a rule the tests assert is not written in prose as well.
   **Prose says what a test cannot** - what a thing is for, why a rule is the shape it is, and
   anything with no observable behaviour to assert. **The game's data is decided in its data
   file**, reviewed by hand and locked by the scenario test. None of the three is decided in a
   discussion, in a note, or in a rendering of any of them.

   **Where prose and a test disagree, the test is right and the prose is a defect.** Prose is
   the one of the three that can drift without anything noticing.

   **A test is stated in the friendly form, and the foundation form is a rendering of it.** The
   rendering is generated from `reviewed/` and never from `spec/tests/`, so that what the engine runs
   is derived from what has been read rather than compared with it.

   **A test is written in `spec/tests/`, and `reviewed/` holds what I thought of it.** A record
   names its verdict and carries the behaviour that verdict is about - the rows, canonical, without
   the prose. **There are three states and no others, for a test and for a case alike.** I have not looked at
   it; I have looked and approved it; I have looked and know it is wrong. **No record and no row are
   the first**, `approved` is the second and means the code is bound where a test is concerned, and
   `denied` is the third.

   **Denied is where I say what I want instead**, when I have words for it. That is the same state
   whether I have said it or not: *this is wrong* and *this needs changing* are one thing, and the
   words are an annotation rather than a state of their own.

   **A test whose rows have changed since I read it is in the first state**, because somebody edited
   it and my approval was of what it said. **A case whose rows have changed is not** - the game
   changed rather than the file, and *I looked at this and said it was wrong* is worth most at
   exactly the moment it changes again. **The suite reports a stale case; it does not clear my
   verdict on one.**

   **The review application writes a record and removes one, acting as me; nothing else puts a file
   there.** A denied test stays in `spec/tests/` - the verdict is a fact about my response, not
   about where the test lives.

   **What the engine runs is what the verdicts approve.** The foundation form is generated from the
   approved records, so a test I have not read constrains nothing and a test I have denied
   constrains nothing either. **Presence used to mean both *I read this* and *this binds*, and
   those are now two different facts.**

   **A case's verdict is a row in `reviewed/cases.4x` and pins no behaviour.** A test's record
   carries the rows it approves, because a test is written by hand and can be reworded under me. **A
   case is generated**, so it cannot change without the generator changing it, and the suite already
   says which cases no longer match. **There is nothing to pin and nothing to compare.**

   **An authorization is a different relation from a verdict because it is consumed.** A verdict
   stands until I change it; `{regenerate}` is spent by the regeneration it asks for and is gone
   afterwards. **A row that outlives being acted on and a row that does not are different kinds of
   fact**, and one file holding both with a field to tell them apart would hide that.

   **No suite is privileged.** A case in `types/` takes a verdict the same way one in `scenario/`
   does, whether or not I have ever looked at that suite.

   **There are two sets of unit tests and they state different kinds of thing.**
   [`spec/tests/rule/`](tests/rule/) states what the game does - rows in, one command, rows out.
   [`spec/tests/interface/`](tests/interface/) states what the interface shows: which items are
   displayed, whether each is active, and which one has my attention.

   **Only the first decides what the game does.** The second decides what a player sees, which is a
   separate concern and is why it is a separate directory rather than more files in the same one.

   **What has my attention is a property of the interface and names at most one item.** It is not a
   flag on an item, because two items could then hold it.

   **My approval is about what a test says, not where it is or how it is written.** The behaviour is
   every `{...}` row, including a `{load}`, and nothing else - a comment explains and does not decide,
   and `{test name:}` identifies rather than states. **So two tests are the same test when their rows
   say the same thing**, however the text differs: entries coalesced to one per description, **and the columns in a row normalized**, because the
   order they are written in is not significant; whitespace not significant either, because the
   braces say where a row begins and ends. **One function does that normalizing and everything
   that compares delegates to it** - so no comparison can disagree with another about whether two
   tests say the same thing.

   **An unapproved test is text in the friendly form, and it is the shape of an intention.** The code
   is bound to the tests I have approved. **So approving a test the code does not implement creates a
   divergence, which I expect** - and from there the code lane is compelled to resolve it. **Relations
   do not have to exist for a test I have not approved**, and a test whose relations do not exist yet
   is the normal way a thing I want becomes a thing that is built.

   **The order that function puts columns in must not depend on anything editable.** `id` first,
   then every other trait alphabetically, then `occupied`, `free` and `capacity` last - **the order
   this specification already states for an entry**, which depends on names. **An order taken from
   the schema's `seq:` would mean renumbering those cleared every approval I have given**, and
   renumbering is a tidy-up nobody thinks twice about.

   **An approval survives a change that does not change the behaviour** - a reworded comment, a
   reordered state, a repadded row. **It is cleared by any change that does**, including a `{load}`
   naming a different file. A cleared approval costs me one reading; a stale one costs me nothing I
   would notice, which is why they are not treated alike.

   **An approval is never inferred for a test that has not been given one.** Matching a test to an
   approval it was not given - because it was only renamed, or because it looks like one that was
   approved - is the one failure that is silent, and no convenience is worth it.

   **A change to what a test loads, or to the rules it runs against, clears nothing.** If it changed
   what a test means, the test is red and the suite names which ones; if it did not, nothing happened.
   **Clearing is for changes nothing else can see**, and the suite can see all of these.

   **A test states one part, and a part is small enough to hold in my head.** The point of the
   directory is that every piece of the game is checked independently, so a test that needs a
   narrative to follow has stopped being one - **and an arc of play belongs to the scenario instead.**
   The scenario is broken into a case per command for exactly this reason: I cannot validate a whole
   playthrough at once, and I can validate any step of one.

   **That is a shape rather than a limit.** The vast majority of tests fire one command, a small
   minority fire two, and three is rare. **More than three is imaginable and has one justification**:
   a single thing being checked that needs something simple done repeatedly, because the outcome only
   appears under that repetition. **Anything else with a long `when` is an arc** - and where a rule
   needs a world that took several commands to build, those commands belong in the `given` as state
   rather than as a story.

   **And a test never turns on an absent row.** What is missing cannot be read, so a test whose point
   is that something *could not* happen names what was lacking rather than leaving me to notice which
   of twelve rows is not there.

4. **A document says what the game is, or it says what the game will be, and it says which.**
   What is built and asserted by a test is the specification. **What is wanted and unbuilt is a
   future plan** - kept, linked and findable, and not mistaken for a rule anything obeys today.
   **The two are told apart by where a document sits**, not by a reader remembering which is
   which.
5. **Reasoning lives in [notes](../docs/notes/README.md).** State the rule here; link down
   for why. Keep this directory short enough to hold in your head.
6. **Open questions go at the bottom of the file**, under that heading, never scattered.
7. **Record what was rejected** when the rejection is load-bearing.
8. **Relationships in prose, data in data files, and both are in this directory.** State that a
   predator has more force than a scavenger; **state the game's data in several files in a
   directory of their own**, in the notation rather than in a table. **What the specification
   states is the default.** Tuning happens in the editor and does not touch the specification, and
   a tuned value becomes the default only when I say it does.
9. **A document says what the game is, or how the thing that runs it is shaped.** Rule 4's two
   kinds are about the game; an architecture document says what is true of the artifact. **A
   boundary stated here is one the build keeps**, and the check that fails when it stops being
   kept is part of stating it.

   **A check on the artifact's shape lives outside the column it constrains.** A constraint the
   constrained lane may weaken is a constraint nobody is holding, so an architecture check is the
   specification's and not the code's - and it runs in the same gate, because a check the
   constrained lane never runs is no better.
