#!/usr/bin/env bash
# Copy the thin-engine's tests and their approval records to where P-532 puts them.
#
# **Run this from the repository root, then read what it printed, then run
# `scripts/move-tests-remove.sh`.** Two scripts rather than one, so that nothing is ever
# missing from both places at the same time: this one only ever adds, and it refuses if the
# destinations already hold anything.
#
# **Sean runs these and no instance does.** `CLAUDE.md`: a record in `reviewed/` is added and
# removed only by the review application, acting as him. **A relocation is neither**, which is
# the gap `P-532` names and `M2` resolves - he is not an instance, so the move is his and only
# his. The scripts exist because he asked for something to review and run rather than a diff to
# read.
#
# **They are temporary.** Delete both once the move has landed and the paths have followed.

set -euo pipefail
cd "$(dirname "$0")/.."

tests_from="prototypes/thin-engine/data/friendly/tests"
records_from="prototypes/thin-engine/reviewed"
tests_to="spec/tests"
records_to="reviewed"

say() { printf '%s\n' "$*"; }
die() { printf 'REFUSED: %s\n' "$*" >&2; exit 1; }

# **What is expected before anything moves.** A count over nothing is the same failure with the
# sign flipped, so each of these is a floor and not a formality: an empty source would otherwise
# copy nothing and report success.
tests_before=$(find "$tests_from" -name '*.4x' | wc -l | tr -d ' ')
records_before=$(find "$records_from" -name '*.4x' | wc -l | tr -d ' ')
say "before: $tests_before tests in $tests_from"
say "before: $records_before records in $records_from"
[ "$tests_before" -gt 40 ] || die "only $tests_before tests found, which is not the suite"
[ "$records_before" -gt 40 ] || die "only $records_before records found, which is not the record"
[ "$records_before" -le "$tests_before" ] || die "more records than tests, so one answers to nothing"

# **The destinations must not exist.** This script only adds, and adding into something that is
# already there would merge two sets without saying so.
for at in "$tests_to" "$records_to"; do
    if [ -e "$at" ]; then
        die "$at already exists - this script only ever creates, so clear it or stop"
    fi
done

mkdir -p "$tests_to" "$records_to"
cp "$tests_from"/*.4x "$tests_to"/
cp "$records_from"/*.4x "$records_to"/

# **Byte for byte, every file, both ways.** A count alone would pass if a file were copied twice
# under one name, or if `cp` had written something short.
tests_after=$(find "$tests_to" -name '*.4x' | wc -l | tr -d ' ')
records_after=$(find "$records_to" -name '*.4x' | wc -l | tr -d ' ')
[ "$tests_after" -eq "$tests_before" ] || die "copied $tests_after tests of $tests_before"
[ "$records_after" -eq "$records_before" ] || die "copied $records_after records of $records_before"

for file in "$tests_from"/*.4x; do
    cmp -s "$file" "$tests_to/$(basename "$file")" || die "$(basename "$file") differs after copying"
done
for file in "$records_from"/*.4x; do
    cmp -s "$file" "$records_to/$(basename "$file")" || die "$(basename "$file") differs after copying"
done

# **Which tests have no record, named rather than counted.** A record arriving that should not
# have would keep the count right and the names wrong, and a name is what a false approval wears.
unread=""
for file in "$tests_to"/*.4x; do
    name=$(basename "$file")
    [ -f "$records_to/$name" ] || unread="$unread $name"
done
say "after:  $tests_after tests in $tests_to, byte-identical"
say "after:  $records_after records in $records_to, byte-identical"
say "after:  unread -$unread"

orphans=""
for file in "$records_to"/*.4x; do
    name=$(basename "$file")
    [ -f "$tests_to/$name" ] || orphans="$orphans $name"
done
[ -z "$orphans" ] || die "these records answer to no test:$orphans"

say ""
say "Copied. Nothing has been removed yet - read the numbers above, then run:"
say "  scripts/move-tests-remove.sh"
