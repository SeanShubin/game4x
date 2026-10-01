# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-608 - A case's verdict is a row and pins nothing, and an authorization is consumed rather than kept

**to** sean · **status** open · **raised** 2026-10-01 · **asks** approval · **kind** recovered · **shape** text · **into** `spec/README.md` -> rule 3, after *presence used to mean both* · **source** `P-607` settling what a case's verdict means without saying where it goes

**The layout first.** 165 cases, and no record carries a copy of one.

```
reviewed/cases.4x    {verdict case:scenario/01/04-toil state:denied}
                     {verdict case:types/17-adjacency state:approved}
                     {regenerate case:scenario/03/02-toil}
```

**Offered as a block:**

>    **A case's verdict is a row in `reviewed/cases.4x` and pins no behaviour.** A test's record
>    carries the rows it approves, because a test is written by hand and can be reworded under me. **A
>    case is generated**, so it cannot change without the generator changing it, and the suite already
>    says which cases no longer match. **There is nothing to pin and nothing to compare.**
>
>    **An authorization is a different relation from a verdict because it is consumed.** A verdict
>    stands until I change it; `{regenerate}` is spent by the regeneration it asks for and is gone
>    afterwards. **A row that outlives being acted on and a row that does not are different kinds of
>    fact**, and one file holding both with a field to tell them apart would hide that.
>
>    **No suite is privileged.** A case in `types/` takes a verdict the same way one in `scenario/`
>    does, whether or not I have ever looked at that suite.

## Why one file rather than 165

**Because a verdict about a case is small and a case is not.** A file per case would be 165 files
holding one row each, and the directory would read as though every case had been looked at. **A row
exists only when I have said something**, which is the same as a test's record being absent until I
read it - said in rows because there is no copy to make.

## What it leaves, and it is small

**A denial can outlive the case it is about.** If a lane deletes a case under the suspension and the
suite writes a new one, my denial is now about bytes that are gone. **Nothing is bound by it**, so the
cost is a misleading reminder rather than a wrong build - and the suite's staleness report is what
tells me the case moved.

**Not worth a mechanism**, and recorded so that the next reader does not build one.

