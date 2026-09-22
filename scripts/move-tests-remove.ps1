# Remove the originals, once `scripts/move-tests-copy.ps1` has put copies where P-532 wants them.
#
# Run this from the repository root, and only after reading what the copy script printed. It
# refuses unless every file it is about to delete is already present at the destination and
# byte-identical to it - so there is no order of events in which a file is missing from both
# places.
#
# Sean runs this and no instance does, for the reason in the copy script.
#
# It is temporary. Delete both pairs once the move has landed and the paths have followed.

$ErrorActionPreference = 'Stop'
Set-Location (Join-Path $PSScriptRoot '..')

$testsFrom = 'prototypes/thin-engine/data/friendly/tests'
$recordsFrom = 'prototypes/thin-engine/reviewed'
$testsTo = 'spec/tests'
$recordsTo = 'reviewed'

function Die($why) { Write-Error "REFUSED: $why"; exit 1 }
function Count($at) { @(Get-ChildItem -Path $at -Filter *.4x -File -ErrorAction SilentlyContinue).Count }
function Same($a, $b) {
    (Get-FileHash -Algorithm SHA256 $a).Hash -eq (Get-FileHash -Algorithm SHA256 $b).Hash
}

foreach ($at in @($testsTo, $recordsTo, $testsFrom, $recordsFrom)) {
    if (-not (Test-Path $at)) { Die "$at is not there - run scripts/move-tests-copy.ps1 first" }
}

# Every original is checked against its copy before anything is deleted, and the loop that checks
# is not the loop that deletes. Checking as it went would leave a half-deleted source behind the
# first mismatch.
$checked = 0
foreach ($pair in @(@($testsFrom, $testsTo), @($recordsFrom, $recordsTo))) {
    foreach ($file in Get-ChildItem -Path $pair[0] -Filter *.4x -File) {
        $at = Join-Path $pair[1] $file.Name
        if (-not (Test-Path $at)) { Die "$at is missing, so $($file.FullName) is not safe to remove" }
        if (-not (Same $file.FullName $at)) { Die "$at differs from $($file.FullName), so it is not safe to remove" }
        $checked++
    }
}

# A count over nothing is the same failure with the sign flipped. With no originals left the loop
# above would check nothing and this script would report a clean removal of them.
if ($checked -le 80) { Die "only $checked files were checked, which is not both sets" }
Write-Output "checked: $checked originals, each present at its destination and byte-identical"

Remove-Item -Path (Join-Path $testsFrom '*.4x')
Remove-Item -Path (Join-Path $recordsFrom '*.4x')

# The emptied directories are left where they are. Git does not track an empty directory, so
# removing them changes nothing it can see - and removing one cost a rehearsal in the shell twin,
# where a later count was asked about a directory that had just stopped existing.
$left = (Count $testsFrom) + (Count $recordsFrom)
if ($left -ne 0) { Die "$left originals are still there" }

Write-Output "after:   $(Count $testsTo) tests in $testsTo"
Write-Output "after:   $(Count $recordsTo) records in $recordsTo"
Write-Output ''
Write-Output 'Moved. data/foundation/tests is untouched and is generated from what just left;'
Write-Output 'the prototype will not build until ten files follow these paths, which is the code'
Write-Output "lane's to do and cannot start until now."
