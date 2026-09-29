#!/usr/bin/env pwsh
# Commit the engine tests you have marked reviewed.
#
# The review application writes a copy of each test into reviewed/ as you approve it, and that
# copy is the marker: a later edit makes the two differ and the test shows as drifted until you
# read it again. The copies are only on your disk until they are committed.
#
# This commits the copies, then generates the foundation form so that what you have just read
# constrains the game. Pass a message to use instead of the generated one.
#
# **It used to stage the report beside them and that was always wrong.** reviewed/ is Sean's
# column and the report is the code lane's, and hooks/pre-commit refuses a commit that spans two -
# so this failed on every run once that path became a column of its own. The report is regenerated
# here so its tally is right on disk, and left for the lane that owns it to commit.
#
# **And it watched the wrong directory for eight days.** `P-532` moved the records to reviewed/ at
# the root on 2026-09-21 and this file went on asking about crates/game-model/reviewed, which does
# not exist - so `$pending` was always empty and it did nothing at all, quietly. `S-222`. The
# shell twin was corrected at the time and this one was not, which is the pair disagreeing in the
# direction nobody checks.
#
#   scripts/reviewed.ps1
#   scripts/reviewed.ps1 "Read the berth tests after the rewrite"

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$root = Split-Path -Parent $PSScriptRoot
$prototype = Join-Path $root "crates/game-model"

# **Staging is publishing, and `git commit` commits the index rather than your changes.** So a
# file somebody else staged would be committed under this message. Refuse rather than carry it.
$strays = @(git -C $root diff --cached --name-only | Where-Object { -not $_.StartsWith("reviewed/") })
if ($strays.Count -gt 0) {
  Write-Host "something else is already staged, and a commit takes the whole index:"
  $strays | ForEach-Object { Write-Host "  $_" }
  Write-Host "commit or unstage it first."
  exit 1
}

$pending = @(git -C $root status --porcelain -- reviewed)
if ($pending.Count -eq 0) {
  Write-Host "no review to record - every copy in reviewed/ is already committed."
  exit 0
}

# A stamp whose test is gone records a reading of something that no longer exists.
Get-ChildItem -Path (Join-Path $root "reviewed") -Filter *.4x -ErrorAction SilentlyContinue |
  ForEach-Object {
    if (-not (Test-Path (Join-Path $root "spec/tests/$($_.Name)"))) {
      Write-Host "note: reviewed/$($_.Name) has no test any more"
    }
  }

Push-Location $prototype
try {
  cargo run --quiet --example report
  if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
}
finally { Pop-Location }

git -C $root add -- reviewed

$names = @(git -C $root diff --cached --name-only -- reviewed |
    ForEach-Object { [IO.Path]::GetFileNameWithoutExtension($_) })

$title = if ($args.Count -gt 0) { $args[0] }
elseif ($names.Count -eq 1) { "Reviewed $($names[0])" }
else { "Reviewed $($names.Count) tests" }

git -C $root commit -q -m $title -m ($names -join "`n")
git -C $root --no-pager log --oneline -1

# **A reading does not bite until the form exists, and nothing else runs this.** spec/tests/ is
# what Sean reads and crates/game-model/data/foundation/tests/ is what the suite iterates, so a
# test he has just approved constrains nothing until the generator has run. On 2026-09-29 that
# window had swallowed two tests, and running it turned three reviewed tests red - a defect that
# had been in the tree since `P-587` and could not be seen. `S-222`.
#
# **A second commit rather than a second staging**, and that is the columns working rather than an
# inconvenience: reviewed/ is Sean's and crates/game-model/data/ is the code lane's, and
# hooks/pre-commit refuses a commit that spans two. So the record lands, then the form does.
Write-Host ""
Write-Host "generating the foundation form, so what you just read constrains the game:"
Push-Location $prototype
try {
  cargo run --quiet --example foundation
  if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
}
finally { Pop-Location }

$form = @(git -C $root status --porcelain -- crates/game-model/data/foundation)
if ($form.Count -eq 0) {
  Write-Host "the foundation form was already current."
  exit 0
}

git -C $root add -- crates/game-model/data/foundation
git -C $root commit -q -m "Generate the foundation form for what was just reviewed" -m ($names -join "`n")
git -C $root --no-pager log --oneline -1

# **Say what it did rather than leaving it to the next run.** A form that is newly current may
# make a test that has been passing go red, which is the promotion biting rather than a break.
Write-Host ""
Write-Host "run the suite - a test that was green against a stale form may now have something to say."
