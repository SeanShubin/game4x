# Tests

[The specification](../README.md) · [Root README](../../README.md)

**A test is the primary statement** - `spec/README.md` rule 3. What the game does is decided by a
test that runs, read and approved one at a time.

**Two sets, and only one of them exists yet.**

| Directory                  | What it states                                                        |
| -------------------------- | --------------------------------------------------------------------- |
| [`rule/`](rule/)           | what the game does - rows in, one command, rows out                   |
| [`interface/`](interface/) | what the interface shows - displayed, active, and which has attention |

**A copy of a test in [`reviewed/`](../../reviewed/) is the record that Sean has read it**, and the
suite runs the copies. So a test nobody has read constrains nothing.

**Sean, 2026-10-01**: *it was always the case that my approval was about what the tests said, not
where the tests were.* **So moving a test does not unapprove it**, and a record moves with the test
it records.

**What is not here is `regression/`.** A regression case observes rather than decides -
`docs/process.md`: *a unit test decides and a regression case observes*, and *a regression case is
never where a behaviour is decided.* `spec/` is where behaviour is decided, so the cases live at
[`../../regression/`](../../regression/) and are generated, not written.
