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

# `store` is searched by concept as well as by word, for the reason `X-26` gives: a
# case-sensitive search for one word returned a plausible zero once already.
ALSO = {
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


def named_in(paths, word):
    """Whether the word appears, by itself or by one of its concept terms."""
    terms = [word] + ALSO.get(word, [])
    for path in paths:
        text = path.read_text(encoding="utf-8", errors="replace").lower()
        for term in terms:
            if re.search(rf"\b{re.escape(term)}\b", text):
                return True
    return False


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

    missing_prose, missing_anywhere = [], []
    for kind in kinds:
        in_prose = named_in(prose, kind)
        in_other = named_in(data + tests, kind)
        if not in_prose:
            missing_prose.append(kind)
        if not in_prose and not in_other:
            missing_anywhere.append(kind)
        mark = "prose" if in_prose else ("data/tests only" if in_other else "NOWHERE")
        print(f"  {kind:<12} {mark}")

    print()
    print(f"not named in spec PROSE          {len(missing_prose)}: {', '.join(missing_prose) or 'none'}")
    print(f"not named ANYWHERE under spec/   {len(missing_anywhere)}: {', '.join(missing_anywhere) or 'none'}")
    print()
    print("These are two different questions. A kind in `spec/data/` is declared and not defined.")


if __name__ == "__main__":
    main()
