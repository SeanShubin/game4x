# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-587 - `bearing` becomes load-bearing: `breed` puts the parent back spent, the way `toil` does

**to** sean · **status** open · **raised** 2026-09-28 · **answered** 2026-09-28, spend it · **asks** approval · **kind** recovered · **shape** an instruction · **into** `spec/data/rules.4x`

**Your analogy is exact and the data already contains the pattern.** `toil` is the same shape with
`laboring`:

```
clause-31  require citizen  laboring:1     the citizen must have its labour
clause-32  remove citizen   laboring:1     take that row
clause-33  add    citizen   laboring:0     put it back spent
```

**`breed` does the first two and not the third.** `clause-25` removes one citizen at `hungry:0
bearing:1 quantity:1`, and `clause-27` adds **two** at `hungry:0 bearing:1 laboring:1 quantity:2` -
the parent and the child, both with a bearing to spend. **So the parent comes back refreshed and can
breed again the same turn**, bounded only by food.

## The instruction

**`clause-27` becomes two `add` clauses, because the parent and the child no longer agree.** One row
of quantity 2 cannot say that one of them is spent.

```
clause-27   add citizen  hungry:0  bearing:0  laboring:<the parent's>  quantity:1   the parent, spent
clause-NEW  add citizen  hungry:0  bearing:1  laboring:1              quantity:1   the child, whole
```

**`{refresh what:citizen trait:bearing}` then has work at every turn's end**, which is what makes the
trait a limiter rather than a decoration, and `repeats` goes back to meaning *fire for every citizen
that can* rather than being the cap itself.

## How to tell it was carried out

```
spec/data/rules.4x   breed has four clauses; exactly one add writes bearing:0 and one writes 1
the suite            a test that a citizen breeds once per turn and not twice - which does not
                     exist yet, so this instruction lands red until one is written and read
reports/nogain.html  a weighting still exists
```

## And the same shape is in the same rule, which this lane found by following your analogy

**`clause-27` asserts `laboring:1` rather than reading the parent's.** `clause-25` does not constrain
`laboring`, so **a citizen that has already toiled can breed and come back able to toil again** -
free labour, once per food. `upkeep` does this correctly: `{reading id:7 clause:clause-23 column:137
of:clause-28 takes:137}` carries the value through rather than stating it.

**This is not an infinite-resource glitch and `S-219` says so** - the solver found a weighting with
this in place, so nothing comes back round with more. **It is a behaviour nobody asserted**, and the
parent's row is the one place to fix both at once, which is why it is in this instruction rather than
a separate item. **If you would rather the parent kept its spent labour, that is the `laboring:<the
parent's>` above and it is a reading rather than a literal.**

