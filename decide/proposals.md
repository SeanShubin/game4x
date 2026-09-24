# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-548 - One sentence refused during promotion, because it cites a rule that is below it

**to** sean · **status** open · **raised** 2026-09-24 · **kind** recovered · **shape** text · **asks** approval · **into** `CLAUDE.md` -> Perspectives

**Your first sentence landed.** `decide/` is the specification lane's, after the column table, in
`CLAUDE.md`. **The second one is refused and this is the correction.**

## Why it was refused

**It says *on the rule above* and that rule is 87 lines below it.** The production-support
paragraph ends at line 51 and the column table at 61; *a generated file has no owner... nobody
edits it* is at line 138.

**A promotion may change line wrapping and nothing else**, so this lane could not repair the
sentence while landing it. **Refusing one block and landing the other is the alternative the
protocol gives**, and the refusal is this lane's drafting error rather than anything you decided.

## What lands now, with the reference pointing the way it actually points

> **The pipeline and the local build belong to the code lane too.** `.gitignore` and
> `.gitattributes` are mechanical details of how things get implemented rather than production
> support proper, **and that is near enough** - the lane that implements owns how implementing
> works. **What a tool writes for itself is owned by nobody**, which is the generated-file rule
> below saying the same thing about a directory: `.git/`, `.idea/` and `target/` have no owner
> because no instance edits them.

**One clause changed**: *on the rule above* became *which is the generated-file rule below saying
the same thing about a directory*. Nothing else moved.

## The alternative, if you would rather not have a forward reference

**Land it after the generated-file paragraph instead**, at line 138, where *the rule above* is
true as originally written. **This lane recommends the version above**: the sentence is about who
owns what and belongs beside the columns, not eighty lines away beside a rule about generated
files.
