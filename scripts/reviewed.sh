#!/usr/bin/env bash
# Commit the thin-engine tests you have marked reviewed.
#
# The review server writes a copy of each test into prototypes/thin-engine/reviewed/ as you
# approve it, and that copy is the marker: a later edit makes the two differ and the test shows
# as drifted until you read it again. The copies are only on your disk until they are committed.
#
# This regenerates the report so its tally matches, then stages the copies and the report and
# commits them. Pass a message to use instead of the generated one.
#
#   scripts/reviewed.sh
#   scripts/reviewed.sh "Read the berth tests after the rewrite"

set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
prototype="$root/prototypes/thin-engine"

# **Staging is publishing, and `git commit` commits the index rather than your changes.** So a
# file somebody else staged would be committed under this message. Refuse rather than carry it.
strays="$(git -C "$root" diff --cached --name-only \
  | grep -v '^prototypes/thin-engine/reviewed/' \
  | grep -v '^prototypes/thin-engine/report\.\(html\|txt\)$' || true)"
if [ -n "$strays" ]; then
  echo "something else is already staged, and a commit takes the whole index:"
  echo "$strays" | sed 's/^/  /'
  echo "commit or unstage it first."
  exit 1
fi

pending="$(git -C "$root" status --porcelain -- "prototypes/thin-engine/reviewed")"
if [ -z "$pending" ]; then
  echo "no review to record - every copy in reviewed/ is already committed."
  exit 0
fi

# A stamp whose test is gone records a reading of something that no longer exists.
for copy in "$prototype"/reviewed/*.4x; do
  [ -e "$copy" ] || continue
  name="$(basename "$copy")"
  if [ ! -f "$prototype/data/friendly/tests/$name" ]; then
    echo "note: reviewed/$name has no test any more"
  fi
done

( cd "$prototype" && cargo run --quiet --example report )

git -C "$root" add -- \
  "prototypes/thin-engine/reviewed" \
  "prototypes/thin-engine/report.html" \
  "prototypes/thin-engine/report.txt"

names="$(git -C "$root" diff --cached --name-only -- "prototypes/thin-engine/reviewed" \
  | sed 's|.*/||; s|\.4x$||')"
how_many="$(echo "$names" | grep -c . || true)"

if [ "$#" -gt 0 ]; then
  title="$1"
elif [ "$how_many" = "1" ]; then
  title="Reviewed $names"
else
  title="Reviewed $how_many tests"
fi

git -C "$root" commit -q -m "$title" -m "$names"
git -C "$root" --no-pager log --oneline -1
