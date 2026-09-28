# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-582 - `regression/` is in no column, and adding a case and removing one are different acts on one path

**to** sean · **status** open · **raised** 2026-09-27 · **asks** a decision · **kind** entailed · **from** `C-160`

**`hooks/pre-commit` puts every path in a column and `regression/` was in none** when the four
suites landed at the repository root - 126 new files, and the check went red. **The code lane
restored the column the cases had under `scenario/`**, which was its own, rather than choosing. So
nothing is blocked and nothing was decided by accident.

**Your own sentence rules out one of the three answers**, which is why this is short. `reviewed/*`
is a column of `sean`, and the hook says why: *a column of its own is what refuses the commit shape
the guarantee turns on - a record added or removed beside the code it judges.* **A deleted case is
that shape exactly.** But you also said you ask a lane to write cases, and a `sean` column would
refuse the commit that lands them.

## The choice

**Both gestures happen on the same path and mean opposite things.** A lane adding a case is
publishing what the test wrote; you removing one is an approval. **The hook is given a path and a
name, and cannot see which act it is looking at.**

- **The hook learns the difference** - a commit that removes a file under `regression/` is yours, a
  commit that adds one is anybody's. The only answer that matches what the two acts mean, and it
  costs a change to `hooks/`, which is the code lane's
- **`regression/` stays the code lane's** - the generator is theirs, so the directory is, and the
  rule that only you delete a case stays prose that nothing enforces. What is there today
- **`regression/` becomes yours** - and then no lane can commit a regenerated case, so the half you
  asked for stops working. Named so it is visibly considered rather than forgotten

**This lane would take the first** and will not write it without your word, because it is a
statement about who may write what and `CLAUDE.md` takes those from you directly. **A one-word
answer is enough.**
