# Copy the thin-engine's tests and their approval records to where P-532 puts them.
#
# Run this from the repository root, read what it printed, then run
# `scripts/move-tests-remove.ps1`. Two scripts rather than one, so that nothing is ever missing
# from both places at the same time: this one only ever adds, and it refuses if the destinations
# already hold anything.
#
# Sean runs these and no instance does. CLAUDE.md: a record in `reviewed/` is added and removed
# only by the review application, acting as him. A relocation is neither, which is the gap P-532
# names and M2 resolves - he is not an instance, so the move is his and only his.
#
# They are temporary. Delete both pairs once the move has landed and the paths have followed.

$ErrorActionPreference = 'Stop'
Set-Location (Join-Path $PSScriptRoot '..')

$testsFrom = 'prototypes/thin-engine/data/friendly/tests'
$recordsFrom = 'prototypes/thin-engine/reviewed'
$testsTo = 'spec/tests'
$recordsTo = 'reviewed'

function Die($why) { Write-Error "REFUSED: $why"; exit 1 }
function Count($at) { @(Get-ChildItem -Path $at -Filter *.4x -File -ErrorAction SilentlyContinue).Count }

# What is expected before anything moves. A count over nothing is the same failure with the sign
# flipped, so each of these is a floor and not a formality.
$testsBefore = Count $testsFrom
$recordsBefore = Count $recordsFrom
Write-Output "before: $testsBefore tests in $testsFrom"
Write-Output "before: $recordsBefore records in $recordsFrom"
if ($testsBefore -le 40) { Die "only $testsBefore tests found, which is not the suite" }
if ($recordsBefore -le 40) { Die "only $recordsBefore records found, which is not the record" }
if ($recordsBefore -gt $testsBefore) { Die 'more records than tests, so one answers to nothing' }

# The destinations must not exist. This script only adds, and adding into something already there
# would merge two sets without saying so.
foreach ($at in @($testsTo, $recordsTo)) {
    if (Test-Path $at) { Die "$at already exists - this script only ever creates, so clear it or stop" }
}

New-Item -ItemType Directory -Path $testsTo -Force | Out-Null
New-Item -ItemType Directory -Path $recordsTo -Force | Out-Null
Copy-Item -Path (Join-Path $testsFrom '*.4x') -Destination $testsTo
Copy-Item -Path (Join-Path $recordsFrom '*.4x') -Destination $recordsTo

# Byte for byte, every file. A count alone would pass if a file were copied twice under one name,
# or if the copy had written something short.
$testsAfter = Count $testsTo
$recordsAfter = Count $recordsTo
if ($testsAfter -ne $testsBefore) { Die "copied $testsAfter tests of $testsBefore" }
if ($recordsAfter -ne $recordsBefore) { Die "copied $recordsAfter records of $recordsBefore" }

function Same($a, $b) {
    (Get-FileHash -Algorithm SHA256 $a).Hash -eq (Get-FileHash -Algorithm SHA256 $b).Hash
}
foreach ($file in Get-ChildItem -Path $testsFrom -Filter *.4x -File) {
    if (-not (Same $file.FullName (Join-Path $testsTo $file.Name))) { Die "$($file.Name) differs after copying" }
}
foreach ($file in Get-ChildItem -Path $recordsFrom -Filter *.4x -File) {
    if (-not (Same $file.FullName (Join-Path $recordsTo $file.Name))) { Die "$($file.Name) differs after copying" }
}

# Which tests have no record, named rather than counted. A record arriving that should not have
# would keep the count right and the names wrong, and a name is what a false approval wears.
$unread = @()
foreach ($file in Get-ChildItem -Path $testsTo -Filter *.4x -File) {
    if (-not (Test-Path (Join-Path $recordsTo $file.Name))) { $unread += $file.Name }
}
Write-Output "after:  $testsAfter tests in $testsTo, byte-identical"
Write-Output "after:  $recordsAfter records in $recordsTo, byte-identical"
Write-Output "after:  unread - $($unread -join ' ')"

$orphans = @()
foreach ($file in Get-ChildItem -Path $recordsTo -Filter *.4x -File) {
    if (-not (Test-Path (Join-Path $testsTo $file.Name))) { $orphans += $file.Name }
}
if ($orphans.Count -gt 0) { Die "these records answer to no test: $($orphans -join ' ')" }

Write-Output ''
Write-Output 'Copied. Nothing has been removed yet - read the numbers above, then run:'
Write-Output '  scripts/move-tests-remove.ps1'
