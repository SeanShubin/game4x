#!/usr/bin/env pwsh
# What is open, and addressed to whom, across every outbox in the repository.
#
#   scripts/outbox.ps1                 every open item, grouped by addressee
#   scripts/outbox.ps1 --to code       one addressee's inbox
#   scripts/outbox.ps1 --check         exit 1 if anything is open and addressed
#   scripts/outbox.ps1 --count         the aggregate, against the limit
#
# Like pad-tables, the package sits outside the workspace, so it never appears in
# `cargo tree` or `cargo build --workspace`.

$ErrorActionPreference = 'Stop'
$manifest = Join-Path (Split-Path -Parent $PSScriptRoot) 'tools/outbox/Cargo.toml'
cargo run --quiet --manifest-path $manifest -- @args
exit $LASTEXITCODE
