#!/usr/bin/env bash
# Who quotes a file of the specification, and where.
#
#   scripts/quotes.sh                       every document, with a count of what quotes it
#   scripts/quotes.sh spec/logistics.md     who quotes that one
#   scripts/quotes.sh logistics.md          the same, by bare name
#
# Run this before rewording a sentence of the specification. CLAUDE.md asks for a grep at that
# moment, and a rule stated without its tool is a rule whose tool nobody reaches for - so this is
# the tool. P-568 reworded three containment bullets on 2026-09-26 and stranded four quotations in
# crates/game-console/src/tree.rs and containment.rs; this lists them.
#
# It is a question and not a gate. Nothing here fails. The gate is
# crates/game-console/tests/quotations.rs, which holds every quotation to what the document says
# now, and this asks that same checker what it already knows - before an edit rather than after it.
#
# See crates/game-console/examples/quotes.rs, which is where the reasoning is.

set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cargo run --quiet --manifest-path "$root/Cargo.toml" -p game-console --example quotes -- "$@"
