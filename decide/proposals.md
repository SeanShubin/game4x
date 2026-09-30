# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-593 - `decide/attention.md`: one generated file for everything waiting on you

**to** sean · **status** open · **raised** 2026-09-30 · **asks** approval · **kind** recovered · **shape** text · **into** `decide/README.md` -> The three things only you can do

**You asked for one place and `decide/README.md` already promises it** - *everything in this
directory is waiting on Sean, and nothing else is* - **and then points at `pending.md`**, which is at
the root, is about every lane, and misses two of the five things that wait on you.

**Offered as a replacement for the section *Why the release is a link rather than a file here*:**

> ## `attention.md` is the one file, and it is generated
>
> **Everything waiting on you is listed in [`attention.md`](attention.md)**, so that nothing has to
> be remembered or looked for. It is generated and rewritten on every commit, which is what makes it
> trustworthy rather than tidy: a thing that waits on me and is not in it is a defect in whatever
> writes it.
>
> **Four of the five kinds are derivable and one is not.** An open proposal, an open question and a
> capability marked `built` are items with an addressee, and a test awaiting a reading is a file
> comparison - present in `spec/tests/` and absent from `reviewed/`, or present in both and
> different. **A stale regression case is only knowable by generating**, so `attention.md` says the
> suite is what reports one rather than pretending to.
>
> **A capability stays in the release that specifies it** and is listed here by name. Moving it would
> separate a capability from the document the code lane builds from, and give that lane two places to
> read.

## What it costs, and who builds it

**The generator is `tools/outbox`'s**, which already writes `pending.md` from every outbox and is
production support - so the file is the code lane's to build and this promotion files it to them.
**`pending.md` stays**: it is the whole index across four perspectives, and `attention.md` is the one
column of it that is yours.

**The fifth kind is named rather than covered**, because a hook that regenerated the regression
suite to answer it would rewrite the tree under whoever was committing. **The suite already names
stale cases and prints the deletion**, which is where that one belongs.

## One thing this does not do

**It does not make reviewing possible from anywhere.** Marking a test read is a key on a local page,
and that is deliberate - a public form that wrote `reviewed/` would be the one artifact nothing
judged by it may touch, writable by anyone. **You said remote reviewing may come later**, and this is
the half that needs nothing decided about hosting.

