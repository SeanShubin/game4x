"""Extract every piece of the game's data from the release into `game.json`.

    python tools/research/editor/extract.py

**The editor edits a derived copy, never a hand-made one.** Four times in one week an
instrument in this repository consulted a copy under the same author's control as the thing
it was checking, and could not fail the way it was supposed to - `X-29`'s arc count, check
7's poison, this lane's `release_recipes`, the code lane's `tests/petri.rs`. A hand-typed
`game.json` would be the fifth and the largest. So it is generated, and every table asserts
how many rows it produced.

**What this does not do is write back.** Turning edited data into the release's markdown is a
promotion, which is Sean's, and the editor is a prototype answering whether the data can be
*edited by choosing at all*. The answer is the deliverable; the round trip is not.
"""

import io
import json
import pathlib
import re

ROOT = pathlib.Path(__file__).parents[3]
RELEASE = ROOT / "releases" / "first-release.md"
OUT = pathlib.Path(__file__).parent / "game.json"


class Table:
    """One markdown table, read by column NAME rather than by position.

    **Position is the bug class this whole file is about.** The first version indexed cells by
    number, and when the release dropped three columns from *Units and structures* it raised an
    IndexError - which was lucky. Had it dropped a column in the middle instead, every value
    after it would have shifted one place and been read into the wrong field, silently, with
    every count still looking plausible. A name cannot shift.
    """

    def __init__(self, heading, headers):
        self.heading = heading
        self.headers = [plain(h) for h in headers]
        self.rows = []

    def cell(self, row, name, otherwise=None):
        """One cell by its column's name. A missing column is an error, not a blank."""
        if name not in self.headers:
            if otherwise is not None:
                return otherwise
            raise AssertionError(
                f"{self.heading!r} has no column {name!r} - it has {self.headers}. "
                f"The release moved; this parser has to be told, not guessed at."
            )
        index = self.headers.index(name)
        return row[index] if index < len(row) else ""

    def each(self, *names):
        """Every row as a tuple of the named columns, in the order asked for."""
        for row in self.rows:
            yield tuple(self.cell(row, name) for name in names)


def tables(document):
    """Every markdown table in the document, as {heading: Table}.

    Parsed to stripped cells rather than matched as text, because the padder rewrites column
    widths and a pattern that matched yesterday stops matching after the next pad. The
    heading is the nearest `##`/`###` above the table, plus an index when a section holds
    more than one.
    """
    out, heading, table, seen = {}, None, None, {}
    for line in document.split("\n"):
        if line.startswith("#"):
            heading = line.lstrip("#").strip()
            table = None
            continue
        if line.startswith("|"):
            cells = [c.strip() for c in line.strip().strip("|").split("|")]
            if set("".join(cells)) <= set("- :"):
                continue  # the separator row
            if table is None:
                seen[heading] = seen.get(heading, 0) + 1
                key = heading if seen[heading] == 1 else f"{heading} {seen[heading]}"
                table = Table(key, cells)
                out[key] = table
                continue  # the header row
            table.rows.append(cells)
        else:
            table = None
    return out


def plain(cell):
    """A cell without its markdown emphasis or code ticks."""
    return cell.replace("**", "").replace("`", "").strip()


def number_or(cell, otherwise=None):
    """A cell that is a whole number, or `otherwise` when it is not one."""
    text = plain(cell)
    return int(text) if re.fullmatch(r"\d+", text) else otherwise


def capacity_density(cell):
    """`3 x 4` -> {capacity: 3, density: 4}. `none` and `-` -> None.

    **Split rather than kept as text, because the editor has to offer each half.** The
    release says what the shape means: *`5 x 6` is total capacity for five extractors, each
    yielding six.*
    """
    text = plain(cell).lower()
    if text in ("none", "-", ""):
        return None
    match = re.fullmatch(r"(\d+)\s*x\s*(\d+)", text)
    assert match, f"not a capacity x density: {cell!r}"
    return {"capacity": int(match.group(1)), "density": int(match.group(2))}


def amounts(cell):
    """`1 labor, 1 metal` -> [{qty: 1, kind: 'labor'}, {qty: 1, kind: 'metal'}]."""
    text = plain(cell)
    if not text:
        return []
    out = []
    for part in text.split(","):
        match = re.fullmatch(r"(\d+)\s+([\w ]+?)(?:\s+per turn)?", part.strip())
        assert match, f"not a quantity and a kind: {part!r} in {cell!r}"
        out.append({"qty": int(match.group(1)), "kind": match.group(2).strip()})
    return out


# **The comparators the release actually uses**, written as patterns so that a constraint can
# be offered as three selections - a trait, a comparator, and a value - instead of typed.
#
# **A trailing `null` value means the comparator carries its own**, so the editor offers two
# selections rather than three. *at its maximum* and *one less* are whole thoughts.
COMPARATORS = [
    (re.compile(r"^(\w+) at least (\d+)$"), "at least"),
    (re.compile(r"^(\w+) at most (\d+)$"), "at most"),
    (re.compile(r"^(\w+) at its maximum$"), "at its maximum"),
    (re.compile(r"^(\w+) one less$"), "one less"),
    (re.compile(r"^(\w+) (\d+)$"), "is"),
]


def constraint(text, trait_names, kind_names, resources):
    """One Traits cell as {trait, compare, value}, or None when nothing can offer it.

    **Returning None is the point of this function.** A cell it cannot decompose is a cell an
    editor would have to let a person type, and the count of those is what says whether the
    data is editable by choosing. It is reported, never repaired.
    """
    if not text:
        return None
    for pattern, compare in COMPARATORS:
        match = pattern.fullmatch(text)
        if match and match.group(1) in trait_names:
            value = match.group(2) if match.lastindex and match.lastindex > 1 else None
            return {
                "trait": match.group(1),
                "compare": compare,
                "value": int(value) if value is not None else None,
            }
    # A bare value of a closed-set trait: `food` and `metal` under an extractor are the
    # `resource` trait, written without naming it because there is only one it could be.
    if text in resources:
        return {"trait": "resource", "compare": "is", "value": text}
    # A parameter standing where a value goes. The editor offers the recipe's own parameters.
    if re.fullmatch(r"\$\w+", text):
        return {"trait": "resource", "compare": "is", "value": text}
    return None


def counters(cell):
    """`bearing 1, defending 1, laboring 1` -> [{trait: 'bearing', max: 1}, ...].

    **The `Readies` column stopped being a yes and became a list of counters** when the
    release split readiness into five traits. A yes/no reader would have seen a non-empty
    cell and gone on saying yes, which is true and says nothing about which counter.
    """
    text = plain(cell)
    if not text:
        return []
    out = []
    for part in text.split(","):
        match = re.fullmatch(r"([\w ]+?)\s+(\d+)", part.strip())
        assert match, f"not a counter and a maximum: {part!r} in {cell!r}"
        out.append({"trait": match.group(1).strip(), "max": int(match.group(2))})
    return out


def recipes_from(table):
    """The Recipes table, grouped. A blank first cell continues the recipe above it.

    **A recipe name may appear more than once** - `stow`, `discard` and `refresh` are each
    stated several times - and those are separate recipes sharing a name, not one recipe
    written in pieces. What joins rows is a blank name, never a matching one.
    """
    out, current = [], None
    # The first column has been called `Recipe` and `Owner` has been called `Auto`; ask for
    # what is there rather than assuming either.
    first = table.headers[0]
    owner_column = "Owner" if "Owner" in table.headers else "Auto"
    for cells in table.rows:
        name = plain(table.cell(cells, first))
        owner = plain(table.cell(cells, owner_column))
        role = plain(table.cell(cells, "Role"))
        qty = plain(table.cell(cells, "Qty"))
        kind = plain(table.cell(cells, "Kind"))
        traits = plain(table.cell(cells, "Traits"))
        where = plain(table.cell(cells, "Where"))
        if name:
            current = {"name": name, "owner": owner, "rows": []}
            out.append(current)
        assert current is not None, f"a row before any recipe: {cells}"
        if not role:
            continue
        current["rows"].append(
            {
                "role": role,
                "qty": qty,
                "kind": kind,
                "traits": traits,
                "where": where,
            }
        )
    return out


def world_order(document):
    """The order the world's recipes fire, from the sentence that states it."""
    match = re.search(r"The world's fire when the turn ends, in\s*\n?that order:(.+?)\.", document, re.S)
    assert match, "the firing order sentence is not where this expected it"
    return [plain(w) for w in re.findall(r"`([^`]+)`", match.group(1))]


def loop(document):
    """The numbered loop under `## The loop`."""
    section = document.split("## The loop", 1)[1].split("\n## ", 1)[0]
    return [m.group(1).strip() for m in re.finditer(r"^\d+\.\s+(.+)$", section, re.M)]


def main():
    document = RELEASE.read_text(encoding="utf-8")
    found = tables(document)

    def need(name):
        assert name in found, f"no table under {name!r} - found {sorted(found)}"
        return found[name]

    game = {}

    game["kinds"] = [
        {"name": name, "what": what} for name, what in need("Kinds").each("Kind", "What it is")
    ]
    game["families"] = [
        {
            "name": name,
            "members": [m.strip() for m in members.split(",")],
            "everyKind": members == "every kind above",
        }
        for name, members in map(
            lambda p: (plain(p[0]), plain(p[1])), need("Families").each("Family", "Members")
        )
    ]
    game["containers"] = [
        {"container": plain(a), "holds": plain(b), "upTo": plain(c)}
        for a, b, c in need("Where things are").each("Container", "Holds", "Up to")
    ]
    # **The Stored-or-derived column says two things in one cell**: which of three a trait is,
    # and - for a derived one - the rule it is derived by. The first is a choice from a closed
    # set and the second is prose, so they are split, and the editor can offer the half that
    # is a choice.
    game["traits"] = [
        {
            "name": plain(name),
            "of": plain(of),
            "values": plain(values),
            "storage": plain(store).split(":", 1)[0].strip(),
            "derivation": plain(store).split(":", 1)[1].strip() if ":" in plain(store) else "",
        }
        for name, of, values, store in need("Traits").each(
            "Trait", "Of", "Values", "Stored or derived"
        )
    ]
    game["bounds"] = [
        {"kind": plain(kind), "boundedBy": plain(by)}
        for kind, by in need("What bounds a kind in a territory").each("Kind", "Bounded by")
    ]
    # **This table's columns come and go, so the editor's columns are derived from it.**
    # Between 2026-09-11 and 2026-09-12 the release dropped `Costs to produce`, `Binding` and
    # `Requires`, and turned `Readies` from a yes into a list of counters. A parser reading by
    # position would have put `Crosses` values into `Upkeep` and said nothing.
    #
    # **So what is declared here is a type per column name, and what is emitted is the
    # intersection with what the release actually has.** A column that goes disappears from
    # the editor; one that comes back reappears; and a column the release grows that is not
    # named here is an error rather than a silent omission.
    THING_COLUMNS = {
        "Thing": ("name", "name", lambda c: plain(c)),
        "Strength": ("strength", "number", number_or),
        "Fuel": ("fuel", "number", number_or),
        "Upkeep": ("upkeep", "amounts", amounts),
        "Costs to produce": ("costs", "amounts", amounts),
        "Binding": ("binding", "number", number_or),
        "Crosses": ("crosses", "crosses", lambda c: plain(c)),
        "Requires": ("requires", "requires", lambda c: plain(c)),
        "Readies": ("readies", "counters", counters),
        "Movable": ("movable", "bool", lambda c: bool(plain(c))),
    }
    units = need("Units and structures")
    unknown = [h for h in units.headers if h not in THING_COLUMNS]
    assert not unknown, (
        f"Units and structures grew columns this parser has no type for: {unknown}. "
        f"Add them to THING_COLUMNS rather than letting them be dropped."
    )
    game["thingColumns"] = [
        {"key": THING_COLUMNS[h][0], "label": h, "type": THING_COLUMNS[h][1]}
        for h in units.headers
    ]
    game["things"] = [
        {
            THING_COLUMNS[h][0]: THING_COLUMNS[h][2](units.cell(row, h))
            for h in units.headers
        }
        for row in units.rows
    ]
    game["recipes"] = recipes_from(need("Recipes"))
    game["biomes"] = [
        {
            "name": plain(name),
            "food": capacity_density(food),
            "metal": capacity_density(metal),
            "energy": capacity_density(energy),
            "nature": number_or(nature),
        }
        for name, food, metal, energy, nature in need("Biomes").each(
            "Biome", "Food", "Metal", "Energy", "Force of nature"
        )
    ]
    game["territories"] = [
        {
            "id": number_or(tid),
            "food": capacity_density(food),
            "metal": capacity_density(metal),
            "energy": capacity_density(energy),
            "exercises": plain(what),
        }
        for tid, food, metal, energy, what in need("Territory resources").each(
            "Territory", "Food", "Metal", "Energy", "What it exercises"
        )
    ]
    game["worldOrder"] = world_order(document)
    game["loop"] = loop(document)

    # **Every population is asserted non-empty, and the two that have a stated size are
    # asserted against it.** A silent drop and an empty table look identical from inside, and
    # a check over nothing passes for the wrong reason - the failure this repository has
    # written down four times.
    for name, rows in game.items():
        assert rows, f"{name} came out empty, so the parse is wrong rather than the release"
    assert len(game["territories"]) == 12, (
        f"the release says twelve territories, parsed {len(game['territories'])}"
    )
    assert len(game["loop"]) == 7, f"the loop is seven steps, parsed {len(game['loop'])}"

    # The firing order names recipes, and every one of them has to exist. This is the join
    # that would break silently if a recipe were renamed in one place and not the other.
    named = {r["name"] for r in game["recipes"]}
    unknown = [n for n in game["worldOrder"] if n not in named]
    assert not unknown, f"the firing order names recipes that do not exist: {unknown}"

    # **Every Traits cell decomposed into selections, and the ones that will not are counted
    # rather than smoothed over.** This is the measurement the editor exists to make: a cell
    # that cannot be offered as a choice is a cell a person has to type, and typing is what
    # Sean asked to be rid of everywhere but a thing's first name.
    trait_names = {t["name"] for t in game["traits"]}
    kind_names = {k["name"] for k in game["kinds"]}
    resources = next(f["members"] for f in game["families"] if f["name"] == "resource")
    typed = []
    cells = 0
    for recipe in game["recipes"]:
        for row in recipe["rows"]:
            if not row["traits"]:
                continue
            cells += 1
            row["constraint"] = constraint(row["traits"], trait_names, kind_names, resources)
            if row["constraint"] is None:
                typed.append((recipe["name"], row["traits"]))
    assert cells, "no Traits cell carried anything, so this measured nothing"
    game["typedCells"] = [{"recipe": r, "text": t} for r, t in typed]

    # The distinct Where cells, which the editor offers as a closed set. Derived from the
    # release rather than listed here, so a new one appears in the editor by arriving in the
    # release rather than by being remembered.
    game["wheres"] = sorted(
        {row["where"] for recipe in game["recipes"] for row in recipe["rows"] if row["where"]}
    )

    # **Vocabularies for the columns that read as prose but are not.** A trait's *Of* names a
    # subject, its *Values* names a shape, a bound names a reason - each is written as a
    # phrase, and each draws on a small set the release uses over and over.
    #
    # **Offering the set is not the same as claiming it is closed.** The editor lets a person
    # choose one of these or write something else, and marks the something-else. So editing
    # what exists is all selection, and inventing a subject or a value-shape the game has
    # never had is where a person meets the wall - which is the thing Sean asked to be able to
    # feel, rather than a thing to be smoothed away.
    def distinct(rows, key):
        return sorted({r[key] for r in rows if r[key]})

    game["vocabularies"] = {
        "traitSubjects": distinct(game["traits"], "of"),
        "traitValues": distinct(game["traits"], "values"),
        "boundReasons": distinct(game["bounds"], "boundedBy"),
        "containerNames": distinct(game["containers"], "container"),
        "containerHolds": distinct(game["containers"], "holds"),
        "containerUpTo": distinct(game["containers"], "upTo"),
        "derivations": distinct(game["traits"], "derivation"),
    }
    for name, values in game["vocabularies"].items():
        assert values, f"the {name} vocabulary came out empty, so the column moved"

    # A quantity is *a whole number, or an expression*. The whole numbers need no list; the
    # expressions do, and it is the release's own rather than one invented here.
    game["quantityExpressions"] = sorted(
        {
            row["qty"]
            for recipe in game["recipes"]
            for row in recipe["rows"]
            if row["qty"] and not re.fullmatch(r"\d+", row["qty"])
        }
    )

    body = json.dumps(game, indent=2, ensure_ascii=False)
    OUT.write_text(body + "\n", encoding="utf-8", newline="\n")
    # **Written as a script as well as as data, so the page opens from a file.** A browser
    # refuses `fetch` against `file://`, and requiring a server to look at a prototype is a
    # step between Sean and the thing he asked to see.
    OUT.with_suffix(".js").write_text(
        "// Generated by extract.py from releases/first-release.md. Do not edit.\n"
        f"window.GAME = {body};\n",
        encoding="utf-8",
        newline="\n",
    )
    print(f"wrote {OUT.relative_to(ROOT)} and game.js")
    for name, rows in game.items():
        print(f"  {len(rows):3}  {name}")


if __name__ == "__main__":
    main()
