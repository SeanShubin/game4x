# Runs the hex-torus-view prototype: a flat hex world that wraps in all six directions, at the
# ten sizes that could ship.
#
# It writes a drawing rather than opening a window -
# `prototypes/hex-torus-view/drawing/index.html`, ten worlds to step through and the numbers
# beside them. Open that file to see the answer.
#
# `[` and `]` step the sizes, `I` toggles the ids, drag or the arrows pan, the wheel zooms and
# `R` resets. **The ids start on**, unlike goldberg-view, because an echo is only legible as
# the same territory when it carries the same number.
#
# Pass a directory to write somewhere else. Other arguments pass straight through.

$ErrorActionPreference = 'Stop'
cargo run --release -p hex-torus-view -- @args
