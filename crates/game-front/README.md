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

Neither `spec/` nor `releases/` names a binding for getting from one surface to another, so
this is the one thing here that is invented rather than followed. It is at least invented
once: the page's buttons and the terminal's `/game`, `/console`, `/browser` are the same
three names, from `Surface::called`.

The leading `/` is what keeps those names clear of the command language. `spec/console.md`
says a command is a verb followed by arguments, and no verb can begin with a slash, so
`/browser` can never be mistaken for one — now or after the language grows.

## Tests

- `the_generation_counts_changes_and_not_questions` — the counter the engine watches moves
  when the game moves and at no other time, so `show turn` does not rebuild the globe.
- `what_is_carried_is_what_is_on_disk` — the embedded command files are the files in
  [`commands/`](../../commands), byte for byte. That is what makes
  [`first_release.rs`](../game-console/tests/first_release.rs), which reads them off disk,
  a test of what actually ships.
- `the_shell_holds_exactly_one_console` — one test rather than four, because the console is
  a process-wide static and the runner runs tests in parallel threads of one process.
- `every_territory_is_listed_and_not_just_the_first_few` — `spec/interface.md` asks for
  *every* entity. The Bevy panel truncated to forty lines because glyphs on a canvas do not
  scroll; a page element and a terminal both do.
