# Align every markdown table in the repository so columns line up in a monospace editor.
# Pass paths to limit it to those files or directories.

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$root = Split-Path -Parent $PSScriptRoot
$targets = if ($args.Count -eq 0) { @($root) } else { $args }
cargo run --quiet --manifest-path "$root/tools/pad-tables/Cargo.toml" -- @targets
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
