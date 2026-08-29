# game-front

[Architecture](../../docs/architecture.md) · [Layers](../../docs/layers.md) · [Root README](../../README.md)

The front end, with **no engine in it**.

`spec/interface.md` asks for three surfaces — the game itself, the console, and the data
browser — all reachable from the front end, in every build, and adds that *how a thing is
presented, and how the user acts on it, may follow the platform it runs on* while what the
user can do stays the same.

That is the seam this crate is cut along. What the console *does* lives here, once. How it
is reached is a shell, and there is one per platform.

| Platform | Console and browser are                                   |
| -------- | --------------------------------------------------------- |
| Web      | elements on the page, driven from JavaScript through wasm |
| Desktop  | stdin and stdout                                          |

## Why they are not drawn by the engine

They were, as Bevy `Text` nodes on the canvas, and that one decision broke four things at
once:

| Broken            | Why                                                                |
| ----------------- | ------------------------------------------------------------------ |
| Clipboard         | Glyphs on a canvas are not text; there is nothing to copy          |
| iOS soft keyboard | A canvas is not a text field, so iOS will not raise one for it     |
| Text selection    | Same reason — there is no text to select                           |
| Tapping a tab     | A Bevy `Text` node has no picking backend; a touch reached nothing |

None of those is a bug to be fixed inside a canvas. A text field, a scroll region and a
button already exist on both platforms, do all of this correctly, and are what the user's
own accessibility settings apply to.

## One Session, one door

There is exactly one `Console` in a running program, held by `shell`, and `Session::run` is
still the only way game state moves. Nothing else in the codebase holds a `Console`, which
is what makes *one Session outside the engine* a fact rather than an intention.

The engine never reaches it. It watches `Console::generation` — a count of how many times
the state has moved — and rebuilds when the number it last saw is not the number it sees
now. A counter rather than a callback, deliberately: on the web the change happens on the
page's call stack, not the engine's, and there is nothing safe to call back into from
there.

## Modules

| Module            | What it is                                                        |
| ----------------- | ----------------------------------------------------------------- |
| `console`         | The `Console`: session, transcript, generation. No platform in it |
| `browser`         | Every entity and its components, as text. Reads; never writes     |
| `library`         | The `commands/*.4x` files, carried in the binary                  |
| `shell`           | The one `Console`, and `with` — the only way to touch it          |
| `shell::web`      | `#[wasm_bindgen]` doorway. Compiled for `wasm32` only             |
| `shell::terminal` | Lines on stdin, answers on stdout. Compiled for everything else   |

Neither shell is chosen at run time. A build is for one target, so the other is not
compiled — which is also why `wasm-bindgen` is a dependency of one target rather than of
the crate.

## The sharing differs because the platforms do

- **Web.** One thread, and the page calls in. A `thread_local` is enough, and a `Mutex`
  would be a lie about what is happening.
- **Desktop.** The engine owns the main thread from the moment its event loop starts, so
  reading stdin needs a thread of its own and the two share a lock.

## Reaching a surface

`spec/console.md`: *a line beginning with `/` directs the front end rather than the game.
`/game`, `/console` and `/browser` choose a surface; `/new <size>` abandons the current game
and starts one on a planet of that size. `/save <file>` writes the history of the current
game to a file, which `run` can then execute. None of these is a command and none is a
transition: `history` does not record them, and `help` does not list them. A game's history
begins when the game does.*

That rule is **required, not a convenience.** `spec/interface.md` asks for all three
surfaces reachable in every build, and that reaching one *never requires a gesture or a key
the platform may lack*. A terminal has neither a button to point at nor an F-key to press,
so on the desktop typing is the only way two of the three surfaces can be reached at all.

The rule lives in `console`, not in a shell. It is a fact about a line typed at the
console, and the console is one thing however a platform presents it. It sat in the
terminal shell for one commit, which quietly made `/browser` a parse error on the page —
exactly the divergence this crate exists to prevent.

What *going there* means is the shell's business, and the two differ legitimately: a
terminal prints the surface, because it has no panels to switch between and nothing to
switch back from; the page shows a panel. `Console::submit` returns `Said::Reach(surface)`
and decides neither.

### Why a slash can never collide

`spec/console.md` says a command is a verb followed by arguments, and no verb begins with a
slash — so `/browser` matches no form, now or after the language grows. That was true by
inspection and is now true by construction:
`game-console`'s `no_command_can_begin_with_a_slash` asserts every form opens with a fixed
word and that no such word starts with `/`. The whole separation rests on it.

### Starting over is not a transition

`/new <size>` throws the game away and begins another. That sounds like it should trouble
`spec/invariants.md`'s *a game state and a transition yield a new game state*, and it does
not: abandoning produces no new state from an old one. It begins a **second fold**, whose
history starts empty. Nothing has to bend to allow it, and the same idea covers saving,
loading and restarting — all of them are choosing which fold you are in.

Note what is *not* claimed. `/new` plainly changes game state — it replaces every
observable of it. The claim that holds, and the one that connects to the invariant, is that
it is not a **transition**.

Two things follow, and both are tested:

- **A game's history begins when the game does.** The new fold's history is exactly what
  built it, and replays on its own to the same game. One spanning two folds could not be
  replayed into either.
- **The new fold is built to completion before the old one is let go.** `/new enormous`
  leaves the game in progress exactly where it was, which matters because there is nothing
  to undo with — an abandoned fold is simply gone.

It is built by running commands, like any game, and it runs the same
[`world.4x`](../../commands/world.4x) the release opens with. That file is `setup.4x`
without the line that decides which planet, so `/new tiny` and the world this console opened
on are the same world rather than two descriptions of it.

### Finding out it exists

`help` must not list what a slash directs, and the greeting scrolls away, so discovery is a
chain rather than one announcement:

| Type       | And you learn                                          |
| ---------- | ------------------------------------------------------ |
| `help`     | that a line beginning with `/` directs the front end   |
| `/`        | what it can direct — the surfaces, and `new <size>`    |
| `browser`  | that you wanted `/browser` — the near miss suggests it |
| `new tiny` | that you wanted `/new tiny`, for the same reason       |

The third is the error requirement in `spec/console.md` doing its job: the parser can only
ever expect commands, because it has never heard of a surface, so the word most likely to
have been meant is added by the layer that has.

## Tests

- `the_generation_counts_changes_and_not_questions` — the counter the engine watches moves
  when the game moves and at no other time, so `show turn` does not rebuild the globe.
- `what_is_carried_is_what_is_on_disk` — the embedded command files are the files in
  [`commands/`](../../commands), byte for byte. That is what makes
  [`first_release.rs`](../game-console/tests/first_release.rs), which reads them off disk,
  a test of what actually ships.
- `a_slash_line_names_a_surface_on_every_platform` — the rule is asserted once, against the
  shared `Console`, so the desktop and the page cannot answer it differently.
- `the_shell_holds_exactly_one_console` — every way in reaches the same console. It and the
  terminal's tests take `shell::exclusively`, because the console is a process-wide static
  and the runner runs tests in parallel threads of one process; without it they race over
  the transcript and over what the last line reached.
- `every_territory_is_listed_and_not_just_the_first_few` — `spec/interface.md` asks for
  *every* entity. The Bevy panel truncated to forty lines because glyphs on a canvas do not
  scroll; a page element and a terminal both do.
