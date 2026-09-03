# Push, then wait until the deployment is actually live, and say what happened.
#
# Returns when the published page is serving this commit - not when the deploy job
# reported success. The pipeline writes `dist/build-info.json` carrying the commit it
# built, so the live site can be asked which build it is, and that is the only answer that
# means "I have a deployment".
#
#   scripts\push.ps1                 push, wait for the deployment, then for the checks
#   scripts\push.ps1 -DeployOnly     return as soon as the page is live
#   scripts\push.ps1 -NoGate         skip the local gate (it has already been run)
#
# Exit codes are three because this pipeline has three outcomes, not two:
#
#   0  deployed, and everything that ran afterwards passed
#   1  no deployment - the gate failed, the push failed, or deploy did not succeed
#   2  deployed, and a check that runs *after* the deploy failed
#
# Two is not a failure of the deployment. .github/workflows/pipeline.yml deploys as soon as
# the gate passes and runs the fuller verification afterwards as notify-only, so a red
# verify job never unpublishes a page that is already up.

[CmdletBinding()]
param(
    [switch] $NoGate,
    [switch] $DeployOnly
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$site = "https://seanshubin.github.io/game4x"
$deployJob = "Deploy to GitHub Pages"

Push-Location (Split-Path -Parent $PSScriptRoot)
try {
    if (-not (Get-Command gh -ErrorAction SilentlyContinue)) {
        Write-Host "gh is not installed"
        exit 1
    }

    $branch = (git rev-parse --abbrev-ref HEAD).Trim()
    $remote = "origin/$branch"

    # What is about to go, listed by author. Three Claude instances commit to this branch
    # and a push carries whatever they have committed locally too - which is the stated
    # reason pushing is done by hand, so the script shows it rather than assuming.
    git rev-parse --verify --quiet $remote *> $null
    $hasRemote = $LASTEXITCODE -eq 0
    $ahead = if ($hasRemote) { [int](git rev-list --count "$remote..HEAD") } else { [int](git rev-list --count HEAD) }
    if (-not $hasRemote) { Write-Host "note: $remote does not exist yet" }

    if ($ahead -eq 0) {
        Write-Host "nothing to push; $branch is up to date with $remote"
        exit 0
    }

    Write-Host "About to push $ahead commit(s) to ${remote}:"
    if ($hasRemote) { git log --format='  %h  %an  %s' "$remote..HEAD" }
    else { git log --format='  %h  %an  %s' -n $ahead }
    Write-Host ""

    $dirty = git status --porcelain
    if ($dirty) {
        Write-Host "note: the working tree is not clean. None of this is pushed, but another"
        Write-Host "      instance may be mid-edit:"
        $dirty | ForEach-Object { Write-Host "        $_" }
        Write-Host ""
    }

    # Run the gate here rather than leaving it to the hook, so a failure costs nothing and
    # so this works in a clone where core.hooksPath was never set. The hook is executed
    # rather than copied: one list of what the gate is, in the file that owns it.
    if (-not $NoGate) {
        # `sh` is Git's and lives in its install rather than on PowerShell's PATH, so calling
        # it by name works from Git Bash - which is why `scripts/push.sh` never saw this - and
        # fails here with "the term 'sh' is not recognized". Found by running the script,
        # 2026-09-02; the gate had never run from PowerShell.
        #
        # Deliberately not falling back to `bash`: the one on PATH is
        # C:\Windows\system32\bash.exe, which is WSL, and that is a different machine as far
        # as cargo and the workspace are concerned. Better no shell than the wrong one.
        $sh = $null
        $git = Get-Command git -ErrorAction SilentlyContinue
        $candidates = @()
        if ($git) {
            # ...\Git\cmd\git.exe and ...\Git\mingw64\bin\git.exe both sit two below the root.
            $root = Split-Path -Parent (Split-Path -Parent $git.Source)
            $candidates += (Join-Path $root "bin\sh.exe")
            $candidates += (Join-Path $root "usr\bin\sh.exe")
            $candidates += (Join-Path (Split-Path -Parent $root) "bin\sh.exe")
        }
        $candidates += "C:\Program Files\Git\bin\sh.exe"
        foreach ($candidate in $candidates) {
            if (Test-Path $candidate) { $sh = $candidate; break }
        }
        if (-not $sh) {
            Write-Host "cannot find Git's sh.exe to run hooks/pre-push; looked in:"
            $candidates | ForEach-Object { Write-Host "  $_" }
            Write-Host "run scripts/push.sh from Git Bash, or pass -NoGate having run the gate yourself"
            exit 1
        }

        Write-Host "==> Gate (hooks/pre-push)"
        & $sh hooks/pre-push
        if ($LASTEXITCODE -ne 0) {
            Write-Host ""
            Write-Host "gate failed; nothing pushed"
            exit 1
        }
        Write-Host ""
    }

    # Already gated above, so the hook is not run a second time. It takes minutes.
    git push --no-verify
    if ($LASTEXITCODE -ne 0) { Write-Host "push failed"; exit 1 }
    $sha = (git rev-parse HEAD).Trim()
    $short = $sha.Substring(0, 7)
    Write-Host ""

    # Runs appear one at a time, so a count taken too early is a count of some of them.
    # Wait until it stops growing.
    $runs = @()
    $previous = @()
    for ($i = 0; $i -lt 40; $i++) {
        $found = @(gh run list --limit 25 --json databaseId,headSha --jq ".[] | select(.headSha == ""$sha"") | .databaseId" 2>$null)
        if ($found.Count -gt 0 -and $found.Count -eq $previous.Count) {
            $runs = $found
            break
        }
        $previous = $found
        Start-Sleep -Seconds 3
    }

    if ($runs.Count -eq 0) {
        Write-Host "no workflow run appeared for $short after two minutes"
        Write-Host "the commit is pushed; check the Actions tab"
        exit 1
    }

    Write-Host "Watching $($runs.Count) run(s) for $short"
    foreach ($id in $runs) {
        # The exit code is deliberately ignored: gh run watch also fails on its own
        # transient errors while the run is still going, so it is the live view and never
        # the verdict.
        & gh run watch $id
    }

    $deployed = $false
    $failedAfter = $false
    $gateFailed = $false

    foreach ($id in $runs) {
        $status = ""
        for ($i = 0; $i -lt 90; $i++) {
            $status = (gh run view $id --json status --jq .status 2>$null)
            if ($status -eq "completed") { break }
            Start-Sleep -Seconds 10
        }
        $conclusion = (gh run view $id --json conclusion --jq .conclusion 2>$null)

        # A cancelled run almost always means a newer push took the concurrency slot.
        # "Failed" would be wrong, and silence would be worse, so name the newer commit.
        if ($conclusion -eq "cancelled") {
            $newer = (gh run list --limit 10 --branch $branch --json headSha,databaseId --jq "[.[] | select(.headSha != ""$sha"")] | .[0].headSha" 2>$null)
            Write-Host ""
            if ($newer -and $newer -ne "null") {
                Write-Host "run $id was CANCELLED, superseded by $($newer.Substring(0,7))"
                Write-Host "this pipeline cancels a run when a newer push arrives on the same branch"
            }
            else {
                Write-Host "run $id was CANCELLED"
            }
            $gateFailed = $true
            continue
        }

        Write-Host ""
        gh run view $id --json jobs --jq '.jobs[] | "  " + (.conclusion // .status) + "  " + .name' 2>$null

        $deploy = (gh run view $id --json jobs --jq ".jobs[] | select(.name == ""$deployJob"") | .conclusion" 2>$null)
        $thisRunDeployed = $deploy -eq "success"
        if ($thisRunDeployed) { $deployed = $true }

        if ($conclusion -ne "success") {
            # Which half of *this* run failed decides what its failure means. Asked per run
            # rather than of the tally, or a second run's red gate would be reported as a
            # failure after the first run's deploy.
            if ($thisRunDeployed) { $failedAfter = $true } else { $gateFailed = $true }
        }
    }

    if (-not $deployed) {
        Write-Host ""
        Write-Host "NOT DEPLOYED  $short"
        exit 1
    }

    # The deploy job going green means Pages accepted the artifact, which is not the same
    # as the page serving it: propagation lags and a cache can hand back the previous
    # bundle. The pipeline stamps the commit into build-info.json, so ask the live site
    # which build it is - the question actually being asked here.
    Write-Host ""
    Write-Host "Deploy job succeeded. Waiting for $site to serve $short"
    $live = ""
    for ($i = 0; $i -lt 60; $i++) {
        try {
            $info = Invoke-RestMethod -Uri "$site/build-info.json?cachebust=$(Get-Random)" -TimeoutSec 15
            $live = $info.commit
        }
        catch { $live = "" }
        if ($live -eq $sha) { break }
        Start-Sleep -Seconds 10
    }

    Write-Host ""
    if ($live -eq $sha) {
        Write-Host "DEPLOYED  $short is live at $site"
    }
    else {
        $serving = if ($live) { $live.Substring(0, [Math]::Min(7, $live.Length)) } else { "nothing readable" }
        Write-Host "DEPLOY REPORTED SUCCESS, but $site is still serving $serving"
        Write-Host "Pages can lag; check again in a minute, or hard-reload."
        exit 1
    }

    if ($DeployOnly) { exit 0 }
    if ($gateFailed) { exit 1 }
    if ($failedAfter) {
        Write-Host ""
        Write-Host "A check that runs after the deploy failed. The page above is up and is this"
        Write-Host "commit; the failure is a report about it, not an undeployment."
        exit 2
    }

    Write-Host "All checks passed."
    exit 0
}
finally {
    Pop-Location
}
