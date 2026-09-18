#!/usr/bin/env pwsh
# Commit the thin-engine tests you have marked reviewed.
#
# The review server writes a copy of each test into prototypes/thin-engine/reviewed/ as you
# approve it, and that copy is the marker: a later edit makes the two differ and the test shows
# as drifted until you read it again. The copies are only on your disk until they are committed.
#
# This regenerates the report so its tally matches, then stages the copies and the report and
# commits them. Pass a message to use instead of the generated one.
#
#   scripts/reviewed.ps1
#   scripts/reviewed.ps1 "Read the berth tests after the rewrite"

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$root = Split-Path -Parent $PSScriptRoot
$prototype = Join-Path $root "prototypes/thin-engine"

# **Staging is publishing, and `git commit` commits the index rather than your changes.** So a
# file somebody else staged would be committed under this message. Refuse rather than carry it.
$mine = @("prototypes/thin-engine/reviewed/", "prototypes/thin-engine/report.html", "prototypes/thin-engine/report.txt")
$strays = @(git -C $root diff --cached --name-only | Where-Object {
    $path = $_
    -not ($mine | Where-Object { $path.StartsWith($_) })
  })
if ($strays.Count -gt 0) {
  Write-Host "something else is already staged, and a commit takes the whole index:"
  $strays | ForEach-Object { Write-Host "  $_" }
  Write-Host "commit or unstage it first."
  exit 1
}

$pending = @(git -C $root status --porcelain -- "prototypes/thin-engine/reviewed")
if ($pending.Count -eq 0) {
  Write-Host "no review to record - every copy in reviewed/ is already committed."
  exit 0
}

# A stamp whose test is gone records a reading of something that no longer exists.
Get-ChildItem -Path (Join-Path $prototype "reviewed") -Filter *.4x -ErrorAction SilentlyContinue |
  ForEach-Object {
    if (-not (Test-Path (Join-Path $prototype "data/friendly/tests/$($_.Name)"))) {
      Write-Host "note: reviewed/$($_.Name) has no test any more"
    }
  }

Push-Location $prototype
try {
  cargo run --quiet --example report
  if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
}
finally { Pop-Location }

git -C $root add -- `
  "prototypes/thin-engine/reviewed" `
  "prototypes/thin-engine/report.html" `
  "prototypes/thin-engine/report.txt"

$names = @(git -C $root diff --cached --name-only -- "prototypes/thin-engine/reviewed" |
    ForEach-Object { [IO.Path]::GetFileNameWithoutExtension($_) })

$title = if ($args.Count -gt 0) { $args[0] }
elseif ($names.Count -eq 1) { "Reviewed $($names[0])" }
else { "Reviewed $($names.Count) tests" }

git -C $root commit -q -m $title -m ($names -join "`n")
git -C $root --no-pager log --oneline -1
