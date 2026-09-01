# Print the release's kinds and recipes, rendered from the Rust data that holds
# them. The tables it prints are the ones in releases/first-release.md, and a test keeps
# them that way; see prototypes/kinds/README.md.

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$root = Split-Path -Parent $PSScriptRoot
cargo run --quiet --manifest-path "$root/Cargo.toml" -p kinds -- @args
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
