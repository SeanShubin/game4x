#!/usr/bin/env bash
# Commit the engine tests you have marked reviewed.
#
# The review application writes a copy of each test into reviewed/ as you approve it, and that
# copy is the marker: a later edit makes the two differ and the test shows as drifted until you
# read it again. The copies are only on your disk until they are committed.
#
# This commits the copies and nothing else. Pass a message to use instead of the generated one.
#
# **It used to stage the report beside them and that was always wrong.** reviewed/ is Sean's
# column and the report is the code lane's, and hooks/pre-commit refuses a commit that spans two -
# so this failed on every run once that path became a column of its own. The report is regenerated
# here so its tally is right on disk, and left for the lane that owns it to commit.
#
#   scripts/reviewed.sh
#   scripts/reviewed.sh "Read the berth tests after the rewrite"

set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
prototype="$root/crates/game-model"

# **Staging is publishing, and `git commit` commits the index rather than your changes.** So a
# file somebody else staged would be committed under this message. Refuse rather than carry it.
strays="$(git -C "$root" diff --cached --name-only | grep -v '^reviewed/' || true)"
if [ -n "$strays" ]; then
  echo "something else is already staged, and a commit takes the whole index:"
  echo "$strays" | sed 's/^/  /'
  echo "commit or unstage it first."
  exit 1
fi

pending="$(git -C "$root" status --porcelain -- reviewed)"
if [ -z "$pending" ]; then
  echo "no review to record - every copy in reviewed/ is already committed."
  exit 0
fi

# A stamp whose test is gone records a reading of something that no longer exists.
for copy in "$root"/reviewed/*.4x; do
  [ -e "$copy" ] || continue
  name="$(basename "$copy")"
  if [ ! -f "$root/spec/tests/$name" ]; then
    echo "note: reviewed/$name has no test any more"
  fi
done

( cd "$prototype" && cargo run --quiet --example report )

git -C "$root" add -- reviewed

names="$(git -C "$root" diff --cached --name-only -- reviewed \
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

# **A reading does not bite until the form exists, and nothing else runs this.** `spec/tests/` is
# what Sean reads and `crates/game-model/data/foundation/tests/` is what the suite iterates, so a
# test he has just approved constrains nothing until the generator has run. On 2026-09-29 that
# window had swallowed two tests, and running it turned three reviewed tests red - a defect that
# had been in the tree since `P-587` and could not be seen. `S-222`.
#
# **A second commit rather than a second staging**, and that is the columns working rather than an
# inconvenience: `reviewed/` is Sean's and `crates/game-model/data/` is the code lane's, and
# `hooks/pre-commit` refuses a commit that spans two. So the record lands, then the form does.
echo
echo "generating the foundation form, so what you just read constrains the game:"
( cd "$prototype" && cargo run --quiet --example foundation )

form="$(git -C "$root" status --porcelain -- crates/game-model/data/foundation)"
if [ -z "$form" ]; then
  echo "the foundation form was already current."
  exit 0
fi

git -C "$root" add -- crates/game-model/data/foundation
git -C "$root" commit -q -m "Generate the foundation form for what was just reviewed" \
  -m "$names"
git -C "$root" --no-pager log --oneline -1

# **Say what it did rather than leaving it to the next run.** A form that is newly current may
# make a test that has been passing go red, which is the promotion biting rather than a break.
echo
echo "run the suite - a test that was green against a stale form may now have something to say."
