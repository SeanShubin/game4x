#!/usr/bin/env sh
# The gate, run on its own - the same thing `hooks/pre-push` runs before a push.
#
#   scripts/gate.sh            run it
#   scripts/gate.sh --help     this
#
# **`Q-91`: *the gate is green* had no way to be said and so it got approximated.** The
# approximation everybody reached for is `cargo test --workspace`, and it is narrower than the
# gate in two directions at once:
#
#   - **No tool crate is a workspace member.** Each `tools/*` declares its own `[workspace]`
#     deliberately, to stay out of `cargo tree` and `cargo build --workspace` - so
#     `--workspace` reaches neither `tools/outbox`'s promotion checks nor `tools/spec`'s
#     citation checks. `hooks/pre-push` loops over `tools/*/Cargo.toml` by finding them, which
#     is `Q-46`: a name list goes stale the moment a fourth exists, and one had.
#   - **It runs no `cargo fmt` and no `clippy`.** Two of the five things the gate does.
#
# So a lane saying *green* from `--workspace` is reporting a narrower population than the words
# claim, which is the failure this repository keeps finding under other names.
#
# **And it measures whatever is in the tree, which is not only yours.** Several instances work
# in this checkout at once, so a run includes every other lane's uncommitted work - a red that
# is not yours, or a green that depends on something nobody has committed. That is not fixable
# from here and is worth knowing rather than hiding, so this prints what is dirty before it
# starts and says whose columns those paths are in.
#
# `scripts/push.sh` has run the gate correctly all along. It is on the path that pushes rather
# than the path that asks, so anybody wanting the answer without the push had nothing to call.
set -eu
cd "$(dirname "$0")/.."

case "${1:-}" in
    -h|--help) sed -n '2,28p' "$0" | sed 's|^# \{0,1\}||'; exit 0 ;;
    "") ;;
    *) echo "unknown option $1" >&2; exit 1 ;;
esac

# **Named before the run rather than after**, so that a failure is read against a tree somebody
# already knows is shared. `--porcelain` is stable output; `git status` prose is not.
dirty=$(git status --porcelain)
if [ -n "$dirty" ]; then
    echo "==> The tree is not clean. This measures what is here, not what is committed:"
    echo "$dirty" | sed 's|^|    |'
    echo
fi

echo "==> Gate (hooks/pre-push)"
# **Executed with `sh` rather than run as a hook**, for the reason `push.sh` gives: it works in
# a clone where `core.hooksPath` was never set.
sh hooks/pre-push
echo
echo "==> Green: fmt, clippy, the test suite, the tools, and the engine-facing crates."
if [ -n "$dirty" ]; then
    echo "    Over the working tree listed above, which is shared."
fi
