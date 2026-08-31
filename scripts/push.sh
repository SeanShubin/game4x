#!/usr/bin/env bash
# Push, then wait until the deployment is actually live, and say what happened.
#
# Returns when the published page is serving this commit - not when the deploy job
# reported success. The pipeline writes `dist/build-info.json` carrying the commit it
# built, so the live site can be asked which build it is, and that is the only answer that
# means "I have a deployment".
#
#   scripts/push.sh                 push, wait for the deployment, then for the checks
#   scripts/push.sh --deploy-only   return as soon as the page is live
#   scripts/push.sh --no-gate       skip the local gate (it has already been run)
#
# Exit codes are three because this pipeline has three outcomes, not two:
#
#   0  deployed, and everything that ran afterwards passed
#   1  no deployment - the gate failed, the push failed, or deploy did not succeed
#   2  deployed, and a check that runs *after* the deploy failed
#
# Two is not a failure of the deployment. `.github/workflows/pipeline.yml` deploys as soon
# as the gate passes and runs the fuller verification afterwards as notify-only, so a red
# verify job never unpublishes a page that is already up. Collapsing that into one exit
# code would report a live, working deployment as a failure.
set -uo pipefail
cd "$(dirname "$0")/.."

SITE="https://seanshubin.github.io/game4x"
DEPLOY_JOB="Deploy to GitHub Pages"
GATE=1
DEPLOY_ONLY=0

for argument in "$@"; do
    case "$argument" in
        --no-gate) GATE=0 ;;
        --deploy-only) DEPLOY_ONLY=1 ;;
        -h|--help) sed -n '2,25p' "$0" | sed 's|^# \{0,1\}||'; exit 0 ;;
        *) echo "unknown option $argument" >&2; exit 1 ;;
    esac
done

command -v gh >/dev/null || { echo "gh is not installed" >&2; exit 1; }

branch="$(git rev-parse --abbrev-ref HEAD)"
remote="origin/$branch"

# --- What is about to go ----------------------------------------------------------------
#
# Listed by author, because three Claude instances commit to this branch and a push carries
# whatever they have committed locally as well. That is the stated reason pushing is done
# by hand rather than by whoever finished last, so the script shows it rather than assuming
# the person pushing knows.
if git rev-parse --verify --quiet "$remote" >/dev/null; then
    ahead="$(git rev-list --count "$remote..HEAD")"
else
    ahead="$(git rev-list --count HEAD)"
    echo "note: $remote does not exist yet"
fi

if [ "$ahead" -eq 0 ]; then
    echo "nothing to push; $branch is up to date with $remote"
    exit 0
fi

echo "About to push $ahead commit(s) to $remote:"
git log --format='  %h  %an  %s' "$remote..HEAD" 2>/dev/null || git log --format='  %h  %an  %s' -n "$ahead"
echo

dirty="$(git status --porcelain)"
if [ -n "$dirty" ]; then
    echo "note: the working tree is not clean. None of this is pushed, but another"
    echo "      instance may be mid-edit:"
    echo "$dirty" | sed 's/^/        /'
    echo
fi

# --- The gate ---------------------------------------------------------------------------
#
# Run here rather than left to the hook, so a failure costs nothing and so this works in a
# clone where `core.hooksPath` was never set. `hooks/pre-push` is executed rather than
# copied: one list of what the gate is, in the file that already owns it.
if [ "$GATE" -eq 1 ]; then
    echo "==> Gate (hooks/pre-push)"
    if ! sh hooks/pre-push; then
        echo
        echo "gate failed; nothing pushed" >&2
        exit 1
    fi
    echo
fi

# Already gated above, so the hook is not run a second time. It takes minutes.
git push --no-verify || { echo "push failed" >&2; exit 1; }
sha="$(git rev-parse HEAD)"
short="${sha:0:7}"
echo

# --- Find the runs this push started -----------------------------------------------------
#
# Runs appear one at a time, so a count taken too early is a count of some of them. Wait
# until it stops growing, which is boardgame's trick and a good one.
runs=""
previous=""
for _ in $(seq 1 40); do
    found="$(gh run list --limit 25 --json databaseId,headSha \
        --jq ".[] | select(.headSha == \"$sha\") | .databaseId" 2>/dev/null)"
    if [ -n "$found" ] && [ "$found" = "$previous" ]; then
        runs="$found"
        break
    fi
    previous="$found"
    sleep 3
done

if [ -z "$runs" ]; then
    echo "no workflow run appeared for $short after two minutes" >&2
    echo "the commit is pushed; check $(gh repo view --json url --jq .url)/actions" >&2
    exit 1
fi

count="$(echo "$runs" | wc -l | tr -d ' ')"
echo "Watching $count run(s) for $short"
for id in $runs; do
    # The exit code is deliberately ignored. `gh run watch` also fails on its own transient
    # errors while the run is still going, so it is used for the live view and never for
    # the verdict.
    gh run watch "$id" || true
done

# --- The verdict, from the runs themselves ------------------------------------------------
deployed=0
failed_after=0
gate_failed=0

for id in $runs; do
    status=""
    for _ in $(seq 1 90); do
        status="$(gh run view "$id" --json status --jq .status 2>/dev/null)"
        [ "$status" = "completed" ] && break
        sleep 10
    done

    conclusion="$(gh run view "$id" --json conclusion --jq .conclusion 2>/dev/null)"

    # A cancelled run almost always means a newer push took the concurrency slot. Saying
    # "failed" would be wrong and saying nothing would be worse, so name the newer commit.
    if [ "$conclusion" = "cancelled" ]; then
        newer="$(gh run list --limit 10 --branch "$branch" --json headSha,databaseId \
            --jq "[.[] | select(.headSha != \"$sha\")] | .[0].headSha" 2>/dev/null)"
        echo
        if [ -n "$newer" ] && [ "$newer" != "null" ]; then
            echo "run $id was CANCELLED, superseded by ${newer:0:7}"
            echo "this pipeline cancels a run when a newer push arrives on the same branch"
        else
            echo "run $id was CANCELLED"
        fi
        gate_failed=1
        continue
    fi

    echo
    gh run view "$id" --json jobs \
        --jq '.jobs[] | "  " + (.conclusion // .status) + "  " + .name' 2>/dev/null

    deploy="$(gh run view "$id" --json jobs \
        --jq ".jobs[] | select(.name == \"$DEPLOY_JOB\") | .conclusion" 2>/dev/null)"
    this_run_deployed=0
    if [ "$deploy" = "success" ]; then
        this_run_deployed=1
        deployed=1
    fi

    if [ "$conclusion" != "success" ]; then
        # Which half of *this* run failed decides what its failure means. Asked per run
        # rather than of the tally, or a second run's red gate would read as a failure
        # after the first run's deploy.
        if [ "$this_run_deployed" -eq 1 ]; then
            failed_after=1
        else
            gate_failed=1
        fi
    fi
done

if [ "$deployed" -eq 0 ]; then
    echo
    echo "NOT DEPLOYED  $short"
    exit 1
fi

# --- Wait for the page to actually serve it ------------------------------------------------
#
# The deploy job going green means Pages accepted the artifact, which is not the same as the
# page serving it: propagation lags, and a cache can hand back the previous bundle for a
# while. The pipeline stamps the commit into `build-info.json`, so the live site can be
# asked which build it is - and that is the question actually being asked here.
echo
echo "Deploy job succeeded. Waiting for $SITE to serve $short"
live=""
for _ in $(seq 1 60); do
    live="$(curl -fsS --max-time 15 "$SITE/build-info.json?cachebust=$RANDOM" 2>/dev/null \
        | tr -d ' \n\r' | sed -n 's/.*"commit":"\([0-9a-f]*\)".*/\1/p')"
    [ "$live" = "$sha" ] && break
    sleep 10
done

echo
if [ "$live" = "$sha" ]; then
    echo "DEPLOYED  $short is live at $SITE"
else
    echo "DEPLOY REPORTED SUCCESS, but $SITE is still serving ${live:0:7}"
    echo "Pages can lag; check again in a minute, or hard-reload."
    exit 1
fi

if [ "$DEPLOY_ONLY" -eq 1 ]; then
    exit 0
fi

if [ "$gate_failed" -eq 1 ]; then
    exit 1
fi
if [ "$failed_after" -eq 1 ]; then
    echo
    echo "A check that runs after the deploy failed. The page above is up and is this"
    echo "commit; the failure is a report about it, not an undeployment."
    exit 2
fi

echo "All checks passed."
exit 0
