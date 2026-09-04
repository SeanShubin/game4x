# Releases

**Authored.** Sean owns every idea here. Claude may rephrase and reorganize what is already
present, reporting every change; a new idea is entered by Sean himself, whether he types it
or pastes it from a [proposal](../docs/notes/proposals.md).

[Root README](../README.md) · [Specification](../spec/README.md) · [Documentation map](../docs/README.md)

The [specification](../spec/README.md) describes the **end state**: what the game is when it
is finished. It says nothing about when.

A release spec describes **one delivery**: which of that end state is being built now, what
is deliberately left out, and how we will know it works. Release specs are temporary. When
a release's capability has been vetted, its file is deleted.

## The rule that keeps them from contradicting each other

**The spec is the destination and always wins. A release spec only says what is true
today.** They never disagree about the goal, only about the schedule.

So a release spec never invents a rule. Every entry points at a line already in the spec,
and says one of three things about it:

| Entry       | Meaning                                                                                     |
| ----------- | ------------------------------------------------------------------------------------------- |
| **In**      | Delivered exactly as the spec states it                                                     |
| **Reduced** | Delivered in a narrowed form, and the narrowing is stated. The spec is still the target     |
| **Out**     | Deliberately not in this release. Listed so a reader knows it was considered, not forgotten |

If a release needs a rule the spec does not have, the rule goes in the spec first - by
[proposal](../docs/notes/proposals.md) if Claude drafted it - and then the release refers to
it. A release spec is never where a decision is made.

A release states scope, capabilities and what is deliberately left out, in prose. **It does not
contain the game's data.** Where a release needs to show data, it links to the generated view of
it, which is a file of its own.

## Vetting, and deletion

Every capability carries a **vetted when** line: the observable thing that has to be true.
"The planet renders" is not vetting. "A tiny planet renders, every region is clickable, and
the greatest distance between two regions reads 3" is.

This is the **feature** case of a general rule - work is verified by the thing that would be
false if it had not been done - and it is the case whose observer is a person. The other two are
in [CLAUDE.md](../CLAUDE.md), under *What done means*: a quality improvement is verified by a
test that would have failed before it, and research by a recorded answer to a stated question.
Only this one reaches Sean, which is why a capability changes hands at `built`.

When every capability in a release is vetted:

1. Check that anything learned along the way has reached the spec or
   [the notes](../docs/notes/README.md).
2. Delete the file.
3. Add its line to the log below.

Git keeps the file, so the log is a convenience, not the record.

## Shipped

| Release  | Delivered | Date |
| -------- | --------- | ---- |
| none yet |           |      |

## The shape of one

```markdown
# Release: <name>

**Authored.** ...
[Releases](README.md) · [Specification](../spec/README.md)

## Goal

One sentence. What someone can do at the end of this release that they could not before.

## Capabilities

### <capability>

- **In** - <spec rule, delivered as written>
- **Reduced** - <spec rule>, narrowed to <what exactly>
- **Out** - <spec rule>, because <reason>
- **Vetted when** - <the observable thing>

## Out of scope

Whole areas of the spec this release does not touch, so the omission reads as deliberate.

## Open questions
```
