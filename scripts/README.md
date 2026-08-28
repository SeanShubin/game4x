# Scripts

[Documentation map](../docs/README.md) · [Root README](../README.md)

One script per thing you might want to run, so that running it never requires
remembering a cargo incantation.

| Script                    | Runs                                                           | Notes                                                                                                                                                        |
| ------------------------- | -------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `game4x.ps1` / `.sh`      | [the application](../crates/game4x/)                           | Drag, a finger or the arrows to turn; wheel or pinch to zoom; `R` resets; `1`-`5` start a new game at that size. The console is the terminal you ran it from |
| `web.ps1` / `.sh`         | the same application as WASM, on `localhost:8080`              | What GitHub Pages publishes, running locally. There the console is part of the page                                                                          |
| `planet-view.ps1` / `.sh` | [the planet view prototype](../docs/prototypes/planet-view.md) | Drag to turn the world, `P` to fold it into a globe, `Esc` to quit                                                                                           |
| `pad-tables.ps1` / `.sh`  | `tools/pad-tables`                                             | Aligns every markdown table in the repo. Not a prototype; see below                                                                                          |

| `shot.ps1` / `.sh` | [the application](../crates/game4x/), by remote control | One frame to a PNG, plus a text dump of what is behind it. `--help` lists the options |
```
scripts/planet-view.ps1                       # PowerShell
bash scripts/planet-view.sh                   # POSIX shell
scripts/planet-view.sh --regions 60 --seed 7  # arguments pass straight through
scripts/planet-view.sh --soccer               # the truncated icosahedron, as a reference
scripts/planet-view.ps1 --help                # every option the prototype takes
```

## The one script that is not a prototype

`pad-tables` runs a documentation tool rather than a prototype, and it is the exception to
the naming convention below. Its package lives in `tools/pad-tables` and is deliberately
**excluded from the workspace**, so it never appears in `cargo tree` or in
`cargo build --workspace`.

```
scripts/pad-tables.ps1              # every .md file in the repository
bash scripts/pad-tables.sh
scripts/pad-tables.sh spec docs     # only those directories
scripts/pad-tables.sh --check       # write nothing; exit 1 if anything is unpadded
scripts/pad-tables.sh --help
```

The padding rule itself is a tested library, `pad_tables(&str) -> String`, with idempotence
asserted. Because the package sits outside the workspace, **`cargo test --workspace` does
not run those tests**; run them directly:

```
cargo test --manifest-path tools/pad-tables/Cargo.toml
```

A pre-commit hook in [`hooks/`](../hooks/) does the same thing for staged files only.
It is not active until you opt in, once per clone:

```
git config core.hooksPath hooks
```

## Conventions

- **Two of each**, PowerShell and POSIX shell, doing the same thing. The project is
  developed on Windows but nothing about it is Windows-only, and a script that only runs
  on one machine is a trap for the next person.
- **Named after the prototype**, so the script list and the prototype list stay in step.
- **Arguments pass straight through**, so a script is never a reason to fall back to
  calling cargo by hand.
- **Runnable from any directory.** Each script finds the workspace from its own path and
  uses `--manifest-path`, rather than changing directory — that way a relative path in an
  argument, such as `--capture frame.png`, still resolves against wherever you ran it
  from.
- **Release mode always.** The prototypes rasterize every pixel, and a debug build is
  slow enough to give a false impression of the thing being prototyped.

On Unix the shell scripts may need `chmod +x` after a fresh clone, since the repository
is developed on Windows where git does not track the executable bit. `bash scripts/…`
works either way.

## Adding a prototype

Add both scripts, add a row to the table above, and add the prototype to
[the prototype index](../docs/prototypes/README.md). The scripts are thin enough to copy:
only the crate name changes.
