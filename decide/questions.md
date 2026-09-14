# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-507 - A description cannot say what a thing holds, so only an empty container can be named

**to** sean · **status** open · **raised** 2026-09-14 · **kind** measured · **shape** an instruction · **asks** a decision · **into** `spec/console.md` -> The language, and `P-501` before it is promoted

**You asked how `repeat:7` finds the empty transport, and then asked the question that breaks it.**
The first has an answer. The second does not, and `P-501` is held until it does.

## Why the first one works

`P-501` says a description used to name is **exact** - it leaves out no trait **and no content**. So
after

```
{stow into:{transport metal} kind:metal repeat:3}
```

that transport holds `{metal} -> 3` and **is no longer named by `{transport metal}`**, which states
no contents and therefore names things holding nothing. The second command has one candidate left.

## Why the second one does not

**One transport holding 1 metal, one holding 2, and you want 2 into the first and 5 into the
second.** Both are non-empty, so both must be named by their contents - and **the game notation has
no way to write contents inline.**

Containment is written by indentation, in a state file:

```
{transport resource:metal}
  {metal} -> 1
```

**A command is one line.** `P-212` lets a value be another command - `{a b:{c d:1}}` - but that is a
**named argument**, and contents are a map from a description to a quantity with no name to hang on.
**So there is no expression for *the transport holding one metal*.**

**`P-501` is therefore incomplete rather than wrong.** It works for exactly one case - the empty
container - and your example is the first that is not it.

## Three ways, and the choice is which

**`E1` - a command carries a tree by indentation, as a state does.**

```
{stow kind:metal repeat:2}
  into {transport resource:metal}
    {metal} -> 1
```

**One notation, and containment written the one way it is already written.** The cost: a command
stops being a line, and `scenario/commands/*.4x` is line-oriented today.

**`E2` - contents as a named argument.**

```
{stow into:{transport resource:metal holding:{metal}:1} kind:metal repeat:2}
```

Stays on one line. The cost: `holding` is a word the notation does not have, and a thing holding two
kinds needs it twice, which is a map wearing a field's clothes.

**`E3` - the entry form inline, in brackets.**

```
{stow into:{transport resource:metal [{metal} -> 1]} kind:metal repeat:2}
```

Stays on one line and reuses `->`. The cost: `[` and `]` are punctuation the notation does not have,
which is a second way of writing containment.

## `E4` - name the entry by its position, and write no contents at all

**Sean, 2026-09-14**: *for manual allocation we will likely be able to select individual transports,
which implies all we really need is a positional notation, to know what is in each transport, and to
be able to add/remove from a transport by position.*

```
{stow into:2 kind:metal repeat:2}
{stow into:3 kind:metal repeat:5}
```

**A position is an entry's index in the order the state already puts it in.** `spec/console.md`
already fixes that order - *entries are in the order their descriptions sort in* - so **nothing is
invented and no contents are written.** `P-502`, as corrected today, makes the order total where two
entries share a description.

**It is not a fourth identity, it is the second one indexed.** `id` names a thing for ever; a
description names a set; **a position names an entry in a container's own listing**, and it changes
when the state changes because the listing does.

## Why `E4` beats the other three, and it is a different kind of argument

**The other three invent a way to write contents. `E4` writes none.** `E1` makes a command
multi-line, `E2` adds a word, `E3` adds punctuation - and all three restate what a container already
displays. **A position points at it instead.**

**It is also the only one that does not grow with what a thing holds.** A transport holding four
kinds needs four contents clauses under `E2` or `E3`, and one number under `E4`.

## What Sean raised against it, and it is the real limit

**Stacking.** *How are we to have massive fleets if I have to make each one selectable by the user.*

**A position names an entry, not a thing**, so a million identical transports are **one entry at one
position**. Positions do not grow with the fleet. **They grow with the number of distinct
(description, contents) combinations**, which is the honest bound and is the one he named: *an
additional problem when we have more possible combinations of contents than can fit on a user
interface.*

**This lane has not bounded that number and does not know it.** What can be said is that it is the
number of distinct states, which `spec/logistics.md` already relies on being small - *a kind has few
states however many things of it there are.* **That sentence is load-bearing for `E4` and was
written before anything held cargo**, so it is a premise to check rather than a reassurance.

## And the simple algorithm he named needs no addressing at all

*Fill up each transport and move the ones that are full.* **That is `repeat` against a description
that names the not-yet-full ones**, and the set shrinks as they fill - `P-501`'s mechanism, no
position required. **So position is for manual allocation only**, which is the case it was proposed
for.

## What this lane would pick and why it is not confident

**`E4`, and this lane changed its mind on being given the position idea.** It argued for `E1` on the
grounds that *there is one notation* and a state writes containment by indentation. **`E4` is better
by that same rule**: it writes no containment at all, so there is nothing to write a second way.

`E1` remains the answer if a command should be able to name a thing that is **not** in a listing
anybody has - a hypothetical container, or one being created. **Nothing in the three scenarios needs
that.**

**The reason this is a decision and not an approval**: the cost of `E1` falls on files you derive by
hand this week, and whether a command may stop being one line is a judgement about your own reading
rather than about the notation.
*Nothing is open.*
