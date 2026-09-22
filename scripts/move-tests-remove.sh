#!/usr/bin/env bash
# Remove the originals, once `scripts/move-tests-copy.sh` has put copies where P-532 wants them.
#
# **Run this from the repository root, and only after reading what the copy script printed.**
# It refuses unless every file it is about to delete is already present at the destination and
# byte-identical to it - so there is no order of events in which a file is missing from both
# places.
#
# **Sean runs this and no instance does**, for the reason in the copy script.
#
# **It is temporary.** Delete both scripts once the move has landed and the paths have followed.

set -euo pipefail
cd "$(dirname "$0")/.."

tests_from="prototypes/thin-engine/data/friendly/tests"
records_from="prototypes/thin-engine/reviewed"
tests_to="spec/tests"
records_to="reviewed"

say() { printf '%s\n' "$*"; }
die() { printf 'REFUSED: %s\n' "$*" >&2; exit 1; }

for at in "$tests_to" "$records_to" "$tests_from" "$records_from"; do
    [ -d "$at" ] || die "$at is not there - run scripts/move-tests-copy.sh first"
done

# **Every original is checked against its copy before anything is deleted, and the loop that
# checks is not the loop that deletes.** Checking as it went would leave a half-deleted source
# behind the first mismatch.
checked=0
for file in "$tests_from"/*.4x "$records_from"/*.4x; do
    case "$file" in
        "$tests_from"/*) into="$tests_to" ;;
        *) into="$records_to" ;;
    esac
    at="$into/$(basename "$file")"
    [ -f "$at" ] || die "$at is missing, so $file is not safe to remove"
    cmp -s "$file" "$at" || die "$at differs from $file, so $file is not safe to remove"
    checked=$((checked + 1))
done

# **A count over nothing is the same failure with the sign flipped.** With no originals left the
# loop above would check nothing and this script would report a clean removal of them.
[ "$checked" -gt 80 ] || die "only $checked files were checked, which is not both sets"
say "checked: $checked originals, each present at its destination and byte-identical"

rm "$tests_from"/*.4x
rm "$records_from"/*.4x

# **The emptied directories are left where they are.** Git does not track an empty directory, so
# removing them changes nothing it can see - and `rmdir` here cost a rehearsal: `find` was asked
# about a directory that had just stopped existing, and under `pipefail` that failed the script
# *after* it had correctly deleted everything.
left=$(find "$tests_from" "$records_from" -name '*.4x' | wc -l | tr -d ' ')
[ "$left" -eq 0 ] || die "$left originals are still there"

tests_after=$(find "$tests_to" -name '*.4x' | wc -l | tr -d ' ')
records_after=$(find "$records_to" -name '*.4x' | wc -l | tr -d ' ')
say "after:   $tests_after tests in $tests_to"
say "after:   $records_after records in $records_to"
say ""
say 'Moved. data/foundation/tests is untouched and is generated from what just left;'
say "the prototype will not build until ten files follow these paths, which is the code"
say "lane's to do and cannot start until now."
