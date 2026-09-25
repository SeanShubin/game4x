#!/usr/bin/env python3
"""Which of the release's kinds are named in `spec/`, and in which part of it.

`X-26` counted *kinds named in `spec/` by their own word* and got two gaps. The specification
lane counted four. **Neither is a correction of the other**: `spec/` holds two kinds of file, and
`CLAUDE.md` says so - *a rule is stated in prose, in `spec/`; the game's data is stated in a data
file*. A kind named only in `spec/data/` is declared as data and never defined as a rule, which is
a different claim from being absent.

So this reports both populations separately rather than picking one, and the item says which it
means. Run: python tools/research/kinds-in-spec.py
"""

import pathlib
import re

ROOT = pathlib.Path(__file__).resolve().parents[2]

# The kinds, read from the release rather than written here, so the list cannot drift from it.
RELEASE = ROOT / "releases" / "first-release.md"

# **Two questions, and the first version of this script asked one.**
#
# `X-26` counts kinds *named in `spec/` by their own word*. Where the word is absent it then asks
# a second question - is the **concept** there? - and that one decides whether the absence is a
# gap or a reification. `deposit` is the case: no prose says the word, and
# `spec/economy.md:14` says the thing.
#
# The first version folded the concept terms into the presence test, which counted `deposit` as
# named and returned **one** where the answer is **two**. That is a changed metric reported as a
# measurement - the same class this lane has been tracking all week, committed by it twice in one
# exchange. The two are separate columns now and neither can silently stand in for the other.
CONCEPT = {
    "store": ["storage", "silo", "warehouse", "granary", "stockpile"],
    "deposit": ["density", "what its ground offers"],
    "adjacency": ["adjacent", "neighbour", "next to"],
    "fertility": ["fertile"],
}


def kinds_from_release():
    text = RELEASE.read_text(encoding="utf-8")
    section = re.search(r"\n## Kinds\n(.*?)\n## ", text, re.S)
    assert section, "no Kinds section in the release"
    found = re.findall(r"^\| \*\*([a-z ]+)\*\*", section.group(1), re.M)
    kinds = [word.strip() for word in found]
    assert len(kinds) == len(set(kinds)), "a kind is listed twice"
    return kinds


def found_in(paths, terms):
    """Whether any of the terms appears as a word. Case-insensitive, deliberately.

    `## Pioneer` and `### Yard` are section headings, so a case-sensitive search returns zero for
    two kinds that are defined at length. Both lanes ran that search and both got four.
    """
    for path in paths:
        text = path.read_text(encoding="utf-8", errors="replace").lower()
        for term in terms:
            if re.search(rf"\b{re.escape(term)}\b", text.lower()):
                return True
    return False


def by_word(paths, kind):
    return found_in(paths, [kind])


def by_concept(paths, kind):
    return found_in(paths, CONCEPT.get(kind, []))


def main():
    kinds = kinds_from_release()

    prose = sorted((ROOT / "spec").glob("*.md"))
    data = sorted((ROOT / "spec" / "data").rglob("*")) if (ROOT / "spec" / "data").is_dir() else []
    tests = sorted((ROOT / "spec" / "tests").rglob("*")) if (ROOT / "spec" / "tests").is_dir() else []
    data = [p for p in data if p.is_file()]
    tests = [p for p in tests if p.is_file()]

    print(f"kinds in the release      {len(kinds)}")
    print(f"spec prose files          {len(prose)}   (spec/*.md, excluding spec/future/)")
    print(f"spec/data files           {len(data)}")
    print(f"spec/tests files          {len(tests)}")
    assert kinds and prose, "a count over nothing proves nothing"
    print()

    unnamed, gaps, reifications = [], [], []
    print(f"  {'kind':<12} {'word in prose':<14} {'concept':<10} {'elsewhere under spec/'}")
    for kind in kinds:
        word = by_word(prose, kind)
        concept = by_concept(prose, kind)
        elsewhere = by_word(data + tests, kind)
        if not word:
            unnamed.append(kind)
            (reifications if concept else gaps).append(kind)
        print(
            f"  {kind:<12} {'yes' if word else 'NO':<14} "
            f"{('yes' if concept else '-'):<10} {'yes' if elsewhere else '-'}"
        )

    print()
    print(f"not named by their own word   {len(unnamed)}: {', '.join(unnamed) or 'none'}")
    print(f"  of those, concept absent    {len(gaps)}: {', '.join(gaps) or 'none'}")
    print(f"  of those, concept present   {len(reifications)}: {', '.join(reifications) or 'none'}")
    print()
    print("The count is the first line. Whether an absence is a gap or a reification is the")
    print("second question, and folding it into the first is what made this script say one.")


if __name__ == "__main__":
    main()
