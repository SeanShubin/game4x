# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-551 - `game state` and `local state` are chosen, and a log is a thing `spec/` does not have

**to** sean · **status** open · **raised** 2026-09-24 · **kind** recovered · **shape** an instruction · **asks** a decision · **into** `spec/invariants.md`, `spec/console.md` -> Commands

**Two answers taken.** *`game state` and `local state` is fine* - so the mechanics side keeps the
name your invariants already give it, and the other side takes the term
deterministic-lockstep RTS uses for exactly this. **And**: *no need to record interface commands in
the history but they do need to be logged.*

**Three decisions are left, and the second is the one your answer created.**

## `L1` - what a typed interface command is called

**There is no settled term**, because most games never reify one - they handle input and move on.
**`command` and `local command` is the closest to standard**, and it falls out of the split you just
chose: a command changes game state, a local command changes local state.

**This lane recommends `local command`**, and the cost is that *local* then does two jobs - the
state and the command - which is an argument for it rather than against: one word, one boundary.

## `L2` - what a log is, since `spec/` has none

**Measured: the word does not appear in `spec/*.md`.** There is exactly one record today -
**history** - and `spec/console.md` says what it is: `history` lists every command executed so far,
`/save` writes it to a file, and `run` executes that file. **A `/` line is in none of it.**

**So *logged but not in history* asks for a second artifact**, and it needs three things your
sentence does not say:

```
what reads it      a command like `history`, or a file only, or the review application
what replays it    nothing, presumably - that is what makes it not history
what it is for     the answer that decides the other two
```

**This lane's reading of *what it is for*, offered as the thing to confirm or correct**: an
interface test is a file of commands, and a test that begins *select the ark* has to say so
somewhere. **The log is what an interface test is made of**, which is why it earns its place and
why nothing needs to replay it - the test is the replay.

## `L3` - do interface commands differ from `/` lines, and how

**This is the sharpest one, and your answer is what sharpens it.** `spec/console.md:241`: a `/`
line *directs the front end rather than the game... none of these is a command and none is a
transition: history does not record them.*

**An interface command is also not a transition and also not in history.** So until your answer
they were indistinguishable. **Now they differ in one way: an interface command is logged and a
`/` line is not** - and that is either the whole of the distinction or a consequence of a deeper
one.

**Two readings:**

```
one kind    `/game`, `/save` and `{select ...}` are all front-end lines, and the log is
            simply where the ones about the game's own objects go
two kinds   a `/` line directs the application and a local command acts on the game's
            objects without changing them - different things that share a history rule
```

**This lane recommends *two kinds*, on your own sentence**: *the game mechanics are abstract rules
while the user interface is how the user interacts with those rules.* **`/console` is not an
interaction with a rule and `{select ark}` is.** And `/` lines are already excluded from `help`,
where a local command a player can type ought to be findable.

## What lands once these are answered, so you can see the size

**One sentence in `spec/invariants.md`** naming the two kinds of state and saying the mechanics know
nothing of the interface. **One paragraph in `spec/console.md` -> Commands** saying what a local
command is, that it is logged and not in history, and that `help` lists it. **Nothing in
`spec/units.md` or the release.**

### P-554 - Nothing in the first release shows you a win, and the loop you described is the winning one

**to** sean · **status** open · **raised** 2026-09-24 · **kind** measured · **shape** a decision · **asks** a decision · **into** `releases/first-release.md` -> `R-6`, or a twelfth capability

**Your two descriptions of the loop are different loops**, and only one of them wins.

```
2026-09-20   start with an ark -> develop a planet -> launch an ark
2026-09-24   start with an ark, deploy to the surface, develop that territory,
             expand to an adjacent one, develop it, launch an ark from there
```

**The second wins and the first does not**, because winning takes a launch from a territory other
than the one the Ark deployed to.

## What the release vets today, measured

**Eleven capabilities and none of them is a win.** `R-6` is *the loop can be played through*, and
its *vetted when* says plainly: **it does not win, and that is the win condition working** - *this
scenario deploys to territory 1 and launches from territory 1.*

**And the scenario stops one step short of your six.** In `scenario/commands/play.4x`:

```
:19    {deploy-ark territory:1}          your step 2
:149   {build-yard territory:1}          your step 3, in the first territory
:154   {found-by-land territory:2}       your step 4
:164   {launch-ark territory:1}          from the FIRST territory, so no win
       territory 2 gets one {create-labor} and one {work resource:food}
```

**Five of your six steps.** The missing one is developing the second territory far enough to build
a Yard there.

## The two answers

**`V1` - the release shows you a win.** `R-6`'s *vetted when* changes, or a twelfth capability is
added, and the scenario develops territory 2 to a Yard and launches from it. **What it costs**: the
scenario grows by the turns that takes, and `scenario/expected/play.4x` is reseeded - which the
code lane can predict and has done twice before.

**`V2` - it does not, and `R-6` keeps saying why.** The release vets the mechanics and the win is
vetted when you play it yourself. **What it costs**: the win condition is promoted, the model
latches it as of today, and **nothing you can look at demonstrates it** - which is the state
`CLAUDE.md` says a feature is not in until a person observes it.

## What this lane would say

**`V1`, and the reason is your own words rather than this lane's taste.** You said the tests are
how you keep executive control and that a feature is done when you have observed it. **A win
condition nobody has watched happen is the one rule in `spec/control.md` with no observation behind
it.**

**And it is cheap now in a way it was not.** `S-151` built the latch today and the code lane
already reseeded `play.4x` once for the Ark in orbit, so the machinery and the habit both exist.

**`V2` is defensible** if you would rather see the win by playing than by reading a scenario - and
if you pick it, that is worth one sentence in `R-6` so the absence reads as chosen.
