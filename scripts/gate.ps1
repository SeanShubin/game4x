# The gate, run on its own - the same thing `hooks/pre-push` runs before a push.
#
# `Q-91`: *the gate is green* had no way to be said, so it got approximated as
# `cargo test --workspace`, which is narrower twice. No tool crate is a workspace member -
# each `tools/*` declares its own `[workspace]` on purpose - so that reaches neither
# `tools/outbox`'s promotion checks nor `tools/spec`'s citation checks. And it runs no
# `cargo fmt` and no clippy, which are two of the five things the gate does.
#
# It also measures whatever is in the tree, and several instances work in this checkout at
# once - so a run includes other lanes' uncommitted work, in both directions. Not fixable
# from here; printed rather than hidden.

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$root = Split-Path -Parent $PSScriptRoot
Set-Location $root

$dirty = git status --porcelain
if ($dirty) {
    Write-Output "==> The tree is not clean. This measures what is here, not what is committed:"
    $dirty | ForEach-Object { Write-Output "    $_" }
    Write-Output ""
}

Write-Output "==> Gate (hooks/pre-push)"
# Executed with `sh` rather than run as a hook, so it works in a clone where
# `core.hooksPath` was never set.
sh hooks/pre-push
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Write-Output ""
Write-Output "==> Green: fmt, clippy, the test suite, the tools, and the engine-facing crates."
if ($dirty) {
    Write-Output "    Over the working tree listed above, which is shared."
}
