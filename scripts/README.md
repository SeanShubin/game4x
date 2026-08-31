# Scripts

[Documentation map](../docs/README.md) · [Root README](../README.md)

One script per thing you might want to run, so that running it never requires
remembering a cargo incantation.

| `goldberg-view.ps1` / `.sh` | [the goldberg view prototype](../prototypes/goldberg-view/README.md) | The ten smallest Goldberg solids in the abstract drawing. `[` and `]` step through them |
| `outbox.ps1` / `.sh` | `tools/outbox` | What is open and addressed to whom, across every outbox. Not a prototype; see below |
| Script                    | Runs                                                           | Notes                                                                                                                                                        |
| ------------------------- | -------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `game4x.ps1` / `.sh`      | [the application](../crates/game4x/)                           | Drag, a finger or the arrows to turn; wheel or pinch to zoom; `R` resets; `1`-`5` start a new game at that size. The console is the terminal you ran it from |
| `web.ps1` / `.sh`         | the same application as WASM, on `localhost:8080`              | What GitHub Pages publishes, running locally. There the console is part of the page                                                                          |
| `planet-view.ps1` / `.sh` | [the planet view prototype](../docs/prototypes/planet-view.md) | Drag to turn the world, `P` to fold it into a globe, `Esc` to quit                                                                                           |
| `pad-tables.ps1` / `.sh`  | `tools/pad-tables`                                             | Aligns every markdown table in the repo. Not a prototype; see below                                                                                          |

| `shot.ps1` / `.sh` | [the application](../crates/game4x/), by remote control | One frame to a PNG, plus a text dump of what is behind it. `--help` lists the options |
| `push.ps1` / `.sh` | the gate, then `git push`, then the pipeline | Returns when the published page is serving this commit. See below |
```
scripts/planet-view.ps1                       # PowerShell
bash scripts/planet-view.sh                   # POSIX shell
scripts/planet-view.sh --regions 60 --seed 7  # arguments pass straight through
scripts/planet-view.sh --soccer               # the truncated icosahedron, as a reference
scripts/planet-view.ps1 --help                # every option the prototype takes
```

## Pushing, and knowing you have a deployment

```
scripts/push.sh                 # gate, push, wait for the page to be live, then the checks
scripts/push.sh --deploy-only   # return as soon as the page is live
scripts/push.sh --no-gate       # the gate has already been run
```

**It returns when the published page is serving this commit**, which is not the same as the
deploy job going green: Pages can accept an artifact and still hand back the previous bundle
for a while. `.github/workflows/pipeline.yml` stamps the commit into `dist/build-info.json`,
so the live site can be asked which build it is, and that is the only answer to *do I have a
deployment*.

Three exit codes, because this pipeline has three outcomes:

| Code | Means                                                           |
| ---- | --------------------------------------------------------------- |
| 0    | Deployed, and everything that ran afterwards passed             |
| 1    | No deployment - the gate failed, the push failed, or deploy did |
| 2    | Deployed, and a check that runs *after* the deploy failed       |

Two is not a failed deployment. The pipeline deploys as soon as the gate passes and runs the
fuller verification afterwards as notify-only, so a red verify job never unpublishes a page
that is already up. One exit code for both would report a live, working page as a failure.

Three other things it does that a plain `git push` does not:

- **Lists what is about to go, by author.** Several Claude instances commit to this branch,
  and a push carries whatever they have committed locally too. That is the stated reason
  pushing is done by hand, so the script shows it rather than assuming the person pushing
  knows.
- **Names the commit that cancelled a run.** The pipeline cancels a run when a newer push
  arrives on the same branch, so a cancelled run is usually not a failure - it is a run that
  was overtaken, and reporting it as red would be wrong.
- **Runs `hooks/pre-push` rather than its own copy of the gate**, so there is one list of
  what the gate is, in the file that owns it. It pushes with `--no-verify` afterwards only
  because the gate has just run and takes minutes.

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
