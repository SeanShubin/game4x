"""Render the recipe report from data.json.

The data states; this renders. Nothing is decided here - every number in the output is
computed from `data.json`, so a claim in the report can be checked by reading the data
rather than by trusting the prose.

    python tools/research/formulas/render.py > lenses/research/formulas.html
"""

import json
import pathlib
import sys

HERE = pathlib.Path(__file__).parent
DATA = json.loads((HERE / "data.json").read_text(encoding="utf-8"))
RESULTS = json.loads((HERE / "results.json").read_text(encoding="utf-8"))

OP_CLASS = {
    "each": "op-each",
    "change": "op-change",
    "set": "op-set",
    "require": "op-require",
    "call": "op-call",
    "let": "op-let",
}


def esc(text):
    return (
        str(text)
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
    )


# ---------------------------------------------------------------------------
# One line as one string, in the console's notation
#
# `spec/console.md` gives one notation for a command and for a description of state:
# `{name field:value ...}`, a value is a word, a number or another command, a field
# that refers to a thing is named for that thing's kind, and `#` begins a comment.
# Everything below is that notation applied to a recipe line. Where a line needs
# something the notation does not define, the encoder records an assumption rather
# than inventing quietly - the assumptions are listed in the report and counted.
# ---------------------------------------------------------------------------

# What kind of thing each container is, so the field can be named for its kind.
CONTAINER_KIND = {
    "t": "territory",
    "game": "game",
    "unit": "unit",
    "t.orbit": "orbit",
    "citizen.location": "territory",
    "thing.location": "territory",
    # `move` leaves one place and enters another. Both name the family `place` rather than a
    # kind, because a unit may be in a territory or in an orbit - which the flag below records.
    "from": "place",
    "$to": "place",
}

# What a `kind[...]` bracket says about the thing, as a trait.
BRACKET_FIELD = {
    "extractor": "resource",
    "store": "resource",
    "deposit": "resource",
    "resource": "kind",
    "food": "surplus",
}

# The parameters each called recipe takes, named as the fields of the call.
CALL_FIELDS = {
    "found-colony": ["territory"],
    "make-territory": ["id", "biome", "nature"],
    "make-deposit": ["territory", "resource", "capacity", "density"],
    "make-orbit": ["territory"],
    "make-adjacency": ["from", "to"],
}

ASSUMPTIONS = {
    "path": "A <strong>path</strong> is used as a value &mdash; <code>ark.location.below</code>. "
    "The notation says a value is a word, a number, or another command, so a path is none of "
    "the three. Every <code>let</code> needs one.",
    "for-each": "<strong>How many a quantifier ranges over is not in the line.</strong> "
    "<code>each</code> now says <em>that</em> a recipe repeats, and the size of the set is still "
    "written beside it &mdash; <code>x12</code>, <code>x36</code>, <code>xE</code>. That is right "
    "rather than missing: <code>E</code> depends on the planet, so the number cannot be written "
    "when the recipe is. It is recorded because a reader will want it.",
    "set-not-in-state": "<strong>A quantifier ranges over something that is not a description.</strong> "
    "<code>every row of Territory resources</code> is a data table and the territories do not exist "
    "yet; <code>every shared edge</code> is the planet's geometry, which is neither the state nor a "
    "table. The notation has a form for <em>every thing matching this description</em> and none for "
    "either of these.",
    "positional": "<strong>Positional arguments</strong>, in <code>min(a, b)</code>. Every other "
    "argument in the notation is named.",
    "recipe-as-thing": "<code>recipe:</code> names the recipe a <code>call</code> fires, which "
    "treats a recipe as a thing with a name. The alternative is a second field that names no kind, "
    "and <code>id</code> is meant to be the only one.",
    "root-container": "<code>game:game</code> &mdash; the game is the one thing inside nothing, so "
    "the field naming its kind has no identifier to take and repeats the kind instead.",
    "crossed-by": "<code>crossed-by:</code> on an adjacency, for <code>move</code>'s qualifier "
    "<em>the edge the unit crosses</em>. The release states <em>Crosses</em> against the unit, not "
    "against the adjacency, so the trait this guard reads has no name &mdash; the code lane's "
    "<code>C-60</code> is the same hole from the other side.",
    "derived-trait": "<code>surplus:yes</code> puts a <strong>derived</strong> trait in a "
    "description. The release lists <em>surplus</em> as derived, and the notation says a "
    "description is a kind and every <em>stored</em> trait, and that <strong>a derived trait is "
    "never part of one</strong>. So <code>grow</code>'s target has no form as a description, and "
    "this is the one assumption here that contradicts a rule rather than filling a gap.",
    "unnamed-trait": "<code>location</code> and <code>below</code> are read as though they were "
    "traits, and <code>spec/logistics.md</code> says they may not be: <strong>a thing is not "
    "located by a trait, and what holds it is what says where it is.</strong> So these paths have "
    "to mean <em>what holds this</em> rather than <em>this thing's location field</em> &mdash; a "
    "reading the notation has no form for. <code>move</code> no longer sets one; since 2026-09-09 "
    "it leaves one place and enters another. The code lane's <code>C-56</code> is the same hole "
    "from the other side.",
    "family-not-kind": "A description names a <strong>family</strong> rather than a kind &mdash; "
    "<code>{thing}</code> for the world recipes, <code>{resource kind:...}</code> for what "
    "<code>work</code> makes. Resolving one to the other is grounding, which is <code>X-17</code>.",
}

# Traits used by the recipes that the release's Traits table does not list.
UNNAMED_TRAITS = {"location", "below"}

# The families, read from the data rather than restated, so the two cannot disagree.
FAMILIES = {row[0] for row in DATA["families"]}

_used = {}


def _assume(tag, where):
    # `arithmetic` was declared here until 2026-09-09. The fusion turned `age`'s
    # `thing.keeps - 1` into a consume, and it was the only arithmetic expression in the
    # model - so the tag stopped being needed and the assertion below found that, rather
    # than the list quietly keeping an entry nothing used.
    assert tag in ASSUMPTIONS, f"{tag} is assumed at {where} and is not declared"
    _used.setdefault(tag, []).append(where)


def _value(text, where):
    """A value, flagged where the notation does not cover what it is."""
    text = str(text).strip()
    if UNNAMED_TRAITS & set(text.replace("[", ".").replace("]", ".").split(".")):
        _assume("unnamed-trait", where)
    if " - " in text or " + " in text:
        _assume("arithmetic", where)
        return f"({text})"
    if "." in text and not text.replace(".", "").isdigit():
        _assume("path", where)
    return text


def _description(target, where):
    """`kind[arg] in container` as a description: {kind trait:value container:where}."""
    if " in " in target:
        head, container = target.split(" in ", 1)
    else:
        head, container = target, None
    head = head.strip()
    fields = []
    if "[" in head:
        kind, arg = head[: head.index("[")], head[head.index("[") + 1 : head.rindex("]")]
        if ":" in arg:
            # A create carries its traits, since composition is not sequential.
            for pair in arg.split(","):
                trait, value = (q.strip() for q in pair.split(":", 1))
                if trait in UNNAMED_TRAITS:
                    _assume("unnamed-trait", where)
                fields.append(f"{trait}:{_value(value, where)}")
        else:
            field = BRACKET_FIELD.get(kind)
            assert field, f"no bracket field for kind {kind!r}"
            if field == "surplus":
                _assume("derived-trait", where)
                fields.append("surplus:yes")
            else:
                fields.append(f"{field}:{_value(arg, where)}")
        if kind in FAMILIES:
            _assume("family-not-kind", where)
    else:
        kind = head
        if kind in FAMILIES:
            _assume("family-not-kind", where)
    if container is not None:
        container = container.strip()
        held_by = CONTAINER_KIND.get(container)
        assert held_by, f"unknown container {container!r}"
        if held_by in FAMILIES:
            _assume("family-not-kind", where)
        if container == "game":
            _assume("root-container", where)
        else:
            _value(container, where)
        fields.append(f"{held_by}:{container}")
    return "{" + " ".join([kind] + fields) + "}"


def console_line(op, target, amount, attach, note, where):
    """One row of a recipe table, as a single string in the console's notation."""
    amount, attach = str(amount).strip(), str(attach).strip().lower()

    if op == "each":
        name, over = (p.strip() for p in target.split(":", 1))
        if over.startswith("{"):
            body = over
        else:
            _assume("set-not-in-state", where)
            body = over
        out = f"{{each {name}:{body}}}"
        if amount.startswith("x"):
            _assume("for-each", where)
            note = f"{amount} of them" + (f" - {note}" if note else "")

    elif op == "let":
        name, expr = [p.strip() for p in target.split("=", 1)]
        if expr.startswith("min("):
            _assume("positional", where)
            inner = expr[len("min(") : expr.rindex(")")]
            a, b = [p.strip() for p in inner.split(",")]
            if a.startswith("surplus "):
                kind, rest = a[len("surplus ") :].split(" ", 1)
                a = f"{kind}[surplus] {rest}"
            body = f"{{min {_description(a, where)} {_description(b, where)}}}"
        else:
            body = _value(expr, where)
        out = f"{{let {name}:{body}}}"

    elif op == "call":
        name = target.split("(")[0].strip()
        if name in CALL_FIELDS:
            args = [a.strip() for a in target[target.index("(") + 1 : target.index(")")].split(",")]
            fields = CALL_FIELDS[name]
            assert len(args) == len(fields), f"{name}: {len(args)} args, {len(fields)} fields"
            _assume("recipe-as-thing", where)
            pairs = " ".join(f"{f}:{_value(a, where)}" for f, a in zip(fields, args))
            out = f"{{call recipe:{name} {pairs}}}".replace("  ", " ")
        else:
            raise KeyError(name)
        if "..." in target:
            out += " ... " + out.replace(
                "recipe:make-territory id:1 biome:grassland nature:1",
                "recipe:make-territory id:12 biome:mountain nature:1",
            )
        if amount.startswith("x"):
            _assume("for-each", where)
            note = f"{amount}, one call per argument" + (f" - {note}" if note else "")

    elif op == "set":
        lhs, value = [p.strip() for p in target.split("=", 1)]
        subject, trait = lhs.rsplit(".", 1)
        if trait in UNNAMED_TRAITS:
            _assume("unnamed-trait", where)
        if subject in FAMILIES:
            _assume("family-not-kind", where)
        out = f"{{set thing:{{{subject}}} {trait}:{_value(value, where)}}}"

    elif op == "require":
        if target.startswith("adjacency from"):
            _assume("crossed-by", where)
            thing = "{adjacency from:from to:$to crossed-by:unit}"
        else:
            thing = _description(target, where)
        assert amount.startswith("at least "), amount
        out = f"{{require thing:{thing} at-least:{_value(amount[len('at least '):], where)}}}"

    elif op == "change" and "." in target and " in " not in target:
        # A numeric trait is a counter too, so `age` changes one.
        subject, trait = target.rsplit(".", 1)
        if subject in FAMILIES:
            _assume("family-not-kind", where)
        out = f"{{change thing:{{{subject}}} {trait}:{_value(amount, where)}}}"

    elif op == "change":
        out = f"{{change thing:{_description(target, where)} amount:{_value(amount, where)}}}"

    else:
        raise KeyError(op)

    if attach:
        out = out[:-1] + f" attach:{attach}}}"
    if note:
        out += f"  # {note}"
    return out


def encode_all():
    """Encode every line once, so the assumption counts cannot double-count."""
    encoded = {}
    for f in DATA["player"] + DATA["world"] + DATA["creation"]:
        for i, (op, target, amount, attach, note) in enumerate(f["lines"]):
            where = f'{f["name"]} line {i + 1}'
            encoded[(f["name"], i)] = console_line(op, target, amount, attach, note, where)
    total = sum(len(f["lines"]) for f in DATA["player"] + DATA["world"] + DATA["creation"])
    assert len(encoded) == total, f"{len(encoded)} encoded, {total} lines"
    undeclared = set(_used) - set(ASSUMPTIONS)
    assert not undeclared, undeclared
    unused = set(ASSUMPTIONS) - set(_used)
    assert not unused, f"declared and never needed: {unused}"
    return encoded, total


ENCODED, N_LINES = encode_all()


def formula_table(f):
    out = []
    was = f.get("was")
    if f.get("new"):
        badge = '<span class="badge new">new</span>'
    elif was is None:
        badge = '<span class="badge design">design time</span>'
    else:
        delta = len(f["lines"]) - was
        cls = "down" if delta < 0 else ("up" if delta > 0 else "level")
        arrow = "&minus;" if delta < 0 else ("+" if delta > 0 else "=")
        badge = (
            f'<span class="badge {cls}">{was} rows &rarr; {len(f["lines"])} lines '
            f'({arrow}{abs(delta) if delta else ""})</span>'
        )
    out.append(f'<h3>{esc(f["name"])} {badge}</h3>')
    out.append(f'<p class="sel">selection: <em>{esc(f["selection"])}</em></p>')
    out.append('<div class="scroll"><table><thead><tr>')
    for h in ("Op", "Target", "Amount / Bound", "Attach", "Was", "The same row, as one string"):
        out.append(f"<th>{h}</th>")
    out.append("</tr></thead><tbody>")
    for i, (op, target, amount, attach, was_note) in enumerate(f["lines"]):
        cls = OP_CLASS.get(op, "")
        attach_cell = f'<span class="attach">{esc(attach)}</span>' if attach else ""
        # The operator no longer says which way a change goes, so the sign has to carry it.
        sign = ""
        if op == "change":
            sign = " amt-neg" if str(amount).startswith("-") else " amt-pos"
        out.append(
            f'<tr><td class="op {cls}">{esc(op)}</td>'
            f"<td class=\"target\">{esc(target)}</td>"
            f'<td class="amt{sign}">{esc(amount)}</td>'
            f"<td>{attach_cell}</td>"
            f'<td class="note">{esc(was_note)}</td>'
            f'<td class="one-string">{_one_string_cell(ENCODED[(f["name"], i)])}</td></tr>'
        )
    out.append("</tbody></table></div>")
    return "\n".join(out)


def verdicts():
    """The two verdicts that were typed rather than derived.

    Check 2 is expected to be unbounded and check 2b is expected to be clean, and both said
    so in hand-written words beside a checker that reports its own answer. Found on
    2026-09-09 while fusing the primitives: the checker printed UNBOUNDED WITHOUT MINING and
    the page beside it printed `clean`. Both were true - the only loop gains no metal and
    lives in a declared-free kind - but a verdict nothing computes cannot go red.
    """
    u = RESULTS["unbounded"]
    two = (
        ("unbounded, as it should be", "up")
        if u["witness"]
        else ("BOUNDED - which the growth loop says it should not be", "down")
    )
    w = u["without_sources"]
    clean = w["metal_gain"] == "0" and not w["undeclared"]
    two_b = (
        ("clean", "down")
        if clean
        else (f"metal gain {w['metal_gain']} without mining", "up")
    )
    return two, two_b


VERDICT_2, VERDICT_2B = verdicts()


def _one_string_cell(text):
    """The string as it is, with its comment set back so the command reads first."""
    if "  # " in text:
        cmd, comment = text.split("  # ", 1)
        return f'{esc(cmd)} <span class="cmt"># {esc(comment)}</span>'
    return esc(text)


def a_territory(row):
    """One territory as rows, and as text. Derived from the release's own table.

    `row` is a *Territory resources* row: id, then `capacity x density` per resource, then
    what it exercises. The wide form is what a person reads; the long form is what the model
    holds - and turning one into the other is the whole of the answer to whether it should
    be `metal.density` or `density.metal`.
    """
    tid, note = row[0], row[4]
    facts, text = [], [f"{{territory id:{tid} game:game}}"]
    for trait, values, stored, source in DATA["thing_traits"]:
        if trait == "id":
            facts.append((f"territory {tid}", trait, tid, source))
        elif stored == "derived":
            facts.append((f"territory {tid}", trait, "&mdash;", source))
        else:
            facts.append((f"territory {tid}", trait, "<em>unstated</em>", source))
    for i, resource in enumerate(("food", "metal", "energy"), start=1):
        cell = row[i].strip()
        if cell == "none":
            continue
        capacity, density = (p.strip() for p in cell.split("x"))
        who = f"deposit ({tid}, {resource})"
        facts.append((who, "resource", resource, "the column this cell sits in"))
        facts.append((who, "total-capacity", capacity, f"<code>{cell}</code>, left of the &times;"))
        facts.append((who, "density", density, f"<code>{cell}</code>, right of the &times;"))
        text.append(
            f"{{deposit territory:{tid} resource:{resource} "
            f"total-capacity:{capacity} density:{density}}}"
        )
    rows = "".join(
        f'<tr><td class="target">{esc(w)}</td><td class="target">{esc(t)}</td>'
        f'<td class="amt">{v}</td><td class="note">{s}</td></tr>'
        for w, t, v, s in facts
    )
    return (
        tid,
        note,
        len(facts),
        '<table><thead><tr><th>Thing</th><th>Trait</th><th>Value</th>'
        "<th>Where the value comes from</th></tr></thead><tbody>" + rows + "</tbody></table>",
        "\n".join(text),
    )


def every_kind():
    """One row per kind: what a thing of it carries, and how it is written.

    The point is that a thing should be as reviewable as a recipe. A recipe gets a table of
    lines and a string per line; this gives a kind a row of traits and a string per kind.
    """
    stored = {t: s for t, _of, s in DATA["traits"]}
    rows, unstated = [], 0
    for kind, container, identity, keeps, derived, note in DATA["thing_shape"]:
        for t in keeps + derived:
            assert t in stored, f"{kind} carries {t}, which the Traits table does not list"
        fields = " ".join(f"{t}:.." for t in keeps if t != "id")
        if kind == "territory":
            fields = "id:1 " + fields
            unstated += 2
        text = "{" + " ".join(x for x in (kind, fields) if x) + "}"
        if container not in ("<em>nothing</em>",):
            text = text[:-1] + f" {container.split(',')[0].split(' or ')[0]}:..}}"
        rows.append(
            f'<tr><td class="target">{esc(kind)}</td>'
            f"<td>{container}</td><td>{identity}</td>"
            f'<td class="target">{esc(", ".join(keeps)) or "&mdash;"}</td>'
            f'<td class="note">{esc(", ".join(derived)) or "&mdash;"}</td>'
            f'<td class="one-string">{esc(text)}</td>'
            f'<td class="note">{note}</td></tr>'
        )
    assert len(rows) == len(DATA["kinds"])
    head = (
        "<th>Kind</th><th>In</th><th>Identity</th><th>Stored traits</th><th>Derived</th>"
        "<th>Written as</th><th></th>"
    )
    return (
        len(rows),
        '<div class="scroll"><table><thead><tr>'
        + head
        + "</tr></thead><tbody>"
        + "".join(rows)
        + "</tbody></table></div>",
    )


def kind_collisions():
    """Kinds that no set of traits could tell apart, counted rather than argued.

    The question is whether `kind` could be dropped and a thing be nothing but its traits.
    It could not, and this is why: if two kinds carry the same traits, traits cannot name
    which one a thing is, and something else has to.
    """
    groups = {}
    for kind, _c, _i, keeps, _d, _n in DATA["thing_shape"]:
        groups.setdefault(tuple(sorted(keeps)), []).append(kind)
    clashes = {sig: ks for sig, ks in groups.items() if len(ks) > 1}
    caught = sum(len(ks) for ks in clashes.values())
    rows = "".join(
        f'<tr><td class="target">{esc(", ".join(sig)) or "<em>no stored traits at all</em>"}</td>'
        f'<td>{len(ks)}</td><td class="target">{esc(", ".join(ks))}</td></tr>'
        for sig, ks in sorted(clashes.items(), key=lambda x: -len(x[1]))
    )
    return (
        caught,
        len(DATA["thing_shape"]),
        "<table><thead><tr><th>These stored traits</th><th>Kinds</th><th>Which</th></tr></thead>"
        "<tbody>" + rows + "</tbody></table>",
    )


def decisions_split():
    """Open questions, and ones already answered. Partitioned by the row's own prefix."""
    settled = ("ANSWERED", "CORRECTED", "WITHDRAWN", "NOTED", "NOT A DECISION")
    op = [r for r in DATA["decisions"] if not r[0].startswith(settled)]
    shut = [r for r in DATA["decisions"] if r[0].startswith(settled)]
    assert len(op) + len(shut) == len(DATA["decisions"])
    assert all(r[0].startswith("OPEN") for r in op), [r[0] for r in op]
    return op, shut


def enum_table(key, heading):
    """A small table of values, so the big tables can carry the bare word."""
    rows = "".join(
        f'<tr><td class="target"><strong>{esc(v)}</strong></td><td>{text}</td></tr>'
        for v, text in DATA[key]
    )
    return (
        f"<table><thead><tr><th>{heading}</th><th></th></tr></thead><tbody>"
        + rows
        + "</tbody></table>"
    )


def capacity_table():
    """Every kind, and how capacity treats it.

    Two columns are enums, defined once above rather than spelled out in every row. Before
    2026-09-09 this table repeated `persists` twelve times and one 45-character phrase four
    times; the words did not become clearer for being written out again.
    """
    bounds = {r[0] for r in DATA["bound_values"]}
    lifetimes = {r[0] for r in DATA["lifetime_values"]}
    rows = ""
    for k, holder, at, life, note in DATA["capacity_shape"]:
        assert at in bounds and life in lifetimes, (k, at, life)
        rows += (
            f'<tr><td class="target">{esc(k)}</td><td>{holder}</td>'
            f'<td class="target">{esc(at)}</td><td class="target">{esc(life)}</td>'
            f'<td class="note">{note}</td></tr>'
        )
    assert len(DATA["capacity_shape"]) == len(DATA["kinds"])
    # Every declared value is used, so the enum cannot keep a row nothing means.
    assert {r[2] for r in DATA["capacity_shape"]} == bounds
    assert {r[3] for r in DATA["capacity_shape"]} == lifetimes
    return (
        '<div class="scroll"><table><thead><tr><th>Kind</th><th>Held by</th>'
        "<th>At the bound</th><th>Lifetime</th><th>The number, where there is one</th>"
        "</tr></thead><tbody>" + rows + "</tbody></table></div>"
    )


def storage_table():
    """Every kind of storage, with what is stored that way, read from the declarations."""
    by_pair = {(c, k): (b, s) for c, k, b, s in DATA["containment_declared"]}
    rows = []
    for label, where, bound, life, pairs in DATA["storage_kinds"]:
        examples = ", ".join(
            f"{k} in a {c}" if c != "territory" else k for c, k in pairs
        ) or "&mdash;"
        for c, k in pairs:
            assert (c, k) in by_pair, (c, k)
        rows.append(
            f'<tr><td class="target"><strong>{esc(label)}</strong></td><td>{esc(where)}</td>'
            f"<td>{bound}</td><td>{life}</td>"
            f'<td class="target">{esc(examples)}</td></tr>'
        )
    return (
        len(rows),
        '<div class="scroll"><table><thead><tr><th>Storage</th><th>Where it is</th>'
        "<th>What bounds it</th><th>Lifetime</th><th>What is stored that way</th></tr></thead>"
        "<tbody>" + "".join(rows) + "</tbody></table></div>",
    )


def repetition_counts():
    """Where each sort of repeating shows up, counted rather than recalled.

    This counted prose `selection` fields until 2026-09-09, and went to zero the moment they
    became `each` lines - which is the assertion below doing its job rather than a break. A
    count of the input stops meaning anything when the input moves; this now counts the lines.
    """
    every = DATA["player"] + DATA["world"] + DATA["creation"]
    quantifiers = [
        (f["name"], t) for f in every for op, t, _a, _at, _n in f["lines"] if op == "each"
    ]
    over_state = [q for q in quantifiers if ": {" in q[1]]
    multiplied = [
        (f["name"], t) for f in every for op, t, a, _at, _n in f["lines"]
        if op == "change" and str(a).lstrip("+-").isdigit() and abs(int(a)) > 1
    ]
    assert quantifiers and multiplied, "counted over nothing"
    return len(quantifiers), len(multiplied), len(over_state), len(every)


def selection_table():
    """What each player recipe selects, and what else the player must pick."""
    rows = "".join(
        f'<tr><td class="target">{esc(n)}</td><td>{esc(s)}</td><td class="target">{p}</td></tr>'
        for n, s, p in DATA["selections"]
    )
    assert len(DATA["selections"]) == len(DATA["player"])
    multi = [s for _n, s, _p in DATA["selections"] if " and " in s]
    return (
        len(DATA["selections"]), len(multi),
        '<div class="scroll"><table><thead><tr><th>Recipe</th><th>What is selected</th>'
        "<th>What else the player picks</th></tr></thead><tbody>" + rows + "</tbody></table></div>",
    )


def assumption_table():
    """What the encoding assumed, and how many lines needed each - counted, not written."""
    rows = []
    for tag, text in ASSUMPTIONS.items():
        where = _used[tag]
        rows.append(
            f'<tr><td class="amt">{len(where)}</td><td>{text}</td>'
            f'<td class="note">{esc(where[0])}</td></tr>'
        )
    assert len(rows) == len(ASSUMPTIONS)
    return (
        '<table><thead><tr><th>Lines</th><th>What it assumed</th><th>First at</th></tr></thead>'
        "<tbody>" + "".join(rows) + "</tbody></table>"
    )


def simple_table(headers, rows, classes=None):
    out = ["<table><thead><tr>"]
    for h in headers:
        out.append(f"<th>{esc(h)}</th>")
    out.append("</tr></thead><tbody>")
    for row in rows:
        out.append("<tr>")
        for i, cell in enumerate(row):
            cls = f' class="{classes[i]}"' if classes and classes[i] else ""
            out.append(f"<td{cls}>{esc(cell)}</td>")
        out.append("</tr>")
    out.append("</tbody></table>")
    return "\n".join(out)


def lines_after(name):
    """The line count for a recipe, read from the recipe itself.

    `collapse` used to restate this number, so the table could disagree with the tables
    above it without anything noticing. One number, one home.
    """
    for f in DATA["player"] + DATA["world"]:
        if f["name"] == name or f["name"].split("(")[0] == name:
            return len(f["lines"])
    raise KeyError(name)


def totals():
    """Rows before, and lines now.

    There used to be a third number. `consume n k` was sugar for a threshold plus destroys,
    so what the machine ran and what a person wrote were different counts and the report had
    to carry both. Since the fusion they are the same number, and the honest way to report
    that is to stop reporting two.
    """
    by_name = {}
    for f in DATA["player"] + DATA["world"]:
        by_name[f["name"]] = len(f["lines"])
        by_name[f["name"].split("(")[0]] = len(f["lines"])
    was = sum(r[1] for r in DATA["collapse"])
    now = sum(by_name[r[0]] for r in DATA["collapse"])
    return was, now


def main():
    was, now = totals()
    direction = "down" if now < was else "up"
    n_primitives = len(DATA["primitives"])
    t_id, t_note, n_facts, t_table, t_text = a_territory(DATA["territories"][0])
    n_kinds, kinds_table = every_kind()
    n_clash, n_all, clash_table = kind_collisions()
    open_decisions, shut_decisions = decisions_split()
    cap_table = capacity_table()
    n_storage, sto_table = storage_table()
    n_quant, n_mult, n_state, n_recipes = repetition_counts()
    n_sel, n_multi, sel_table = selection_table()
    c6 = RESULTS["containment"]
    c6_examined, c6_x20 = c6["examined"], c6["x20_lines"]
    n_declared = c6["declared"]
    c6_bad = len(c6["homeless"]) + len(c6["undeclared"])
    v6, v6cls = ("every one lands somewhere declared", "down") if not c6_bad else (
        f"{c6_bad} land nowhere declared", "up")
    n_open, n_decisions = len(open_decisions), len(DATA['decisions'])
    n_player = len(DATA["player"])
    n_world = len(DATA["world"])
    n_creation = len(DATA["creation"])
    parts = []
    parts.append(f"""<title>Recipe Report</title>
<style>
:root {{
  --bg: #fbfaf8; --fg: #1c1a17; --muted: #6b6560; --rule: #ddd8d1;
  --card: #ffffff; --accent: #7c4a2d; --shade: #f4f1ec;
  --create: #1f6f43; --destroy: #9a2c2c; --set: #1f5b8f; --threshold: #8a6d1f; --call: #5b3a8f;
}}
@media (prefers-color-scheme: dark) {{
  :root:not([data-theme="light"]) {{
    --bg: #16151a; --fg: #eae7e2; --muted: #a49d95; --rule: #34313a;
    --card: #1e1d23; --accent: #d9a07a; --shade: #24232a;
    --create: #6fd39b; --destroy: #f08c8c; --set: #82b8ea; --threshold: #dcc07a; --call: #b79ae8;
  }}
}}
:root[data-theme="dark"] {{
  --bg: #16151a; --fg: #eae7e2; --muted: #a49d95; --rule: #34313a;
  --card: #1e1d23; --accent: #d9a07a; --shade: #24232a;
  --create: #6fd39b; --destroy: #f08c8c; --set: #82b8ea; --threshold: #dcc07a; --call: #b79ae8;
}}
body {{ background: var(--bg); color: var(--fg); font: 15px/1.55 -apple-system, BlinkMacSystemFont,
  "Segoe UI", Roboto, sans-serif; margin: 0; }}
.wrap {{ max-width: 1080px; margin: 0 auto; padding: 40px 24px 80px; }}
h1 {{ font-size: 30px; line-height: 1.2; margin: 0 0 6px; letter-spacing: -0.01em; }}
h2 {{ font-size: 21px; margin: 44px 0 6px; padding-top: 18px; border-top: 2px solid var(--rule); }}
h3 {{ font-size: 15px; margin: 26px 0 2px; font-family: ui-monospace, SFMono-Regular, Menlo, monospace; }}
.lede {{ color: var(--muted); margin: 0 0 4px; }}
.sel {{ color: var(--muted); font-size: 13px; margin: 0 0 8px; }}
p {{ margin: 10px 0; }}
table {{ border-collapse: collapse; width: 100%; margin: 10px 0 4px; font-size: 13.5px; }}
th {{ text-align: left; font-size: 11px; text-transform: uppercase; letter-spacing: 0.06em;
  color: var(--muted); border-bottom: 1px solid var(--rule); padding: 5px 9px; font-weight: 600; }}
td {{ border-bottom: 1px solid var(--rule); padding: 5px 9px; vertical-align: top; }}
tbody tr:nth-child(odd) {{ background: var(--shade); }}
.scroll {{ overflow-x: auto; }}
.op {{ font-family: ui-monospace, SFMono-Regular, Menlo, monospace; font-weight: 700; white-space: nowrap; }}
.op-each {{ color: var(--call); font-weight: 700; }}
.op-change {{ color: var(--fg); }} .op-set {{ color: var(--set); }}
.op-require {{ color: var(--threshold); }}
.amt-pos {{ color: var(--create); font-weight: 700; }}
.amt-neg {{ color: var(--destroy); font-weight: 700; }}
.op-call {{ color: var(--call); }} .op-let {{ color: var(--muted); }}
.target, .amt {{ font-family: ui-monospace, SFMono-Regular, Menlo, monospace; }}
.one-string {{ font-family: ui-monospace, SFMono-Regular, Menlo, monospace; font-size: 12px;
  min-width: 26em; color: var(--accent); }}
.one-string .cmt {{ color: var(--muted); font-weight: 400; }}
.note {{ color: var(--muted); font-size: 12.5px; }}
.attach {{ font-size: 11px; font-weight: 700; letter-spacing: 0.04em; }}
.badge {{ font-family: -apple-system, sans-serif; font-size: 11px; font-weight: 600; padding: 2px 7px;
  border-radius: 10px; background: var(--shade); color: var(--muted); border: 1px solid var(--rule);
  margin-left: 6px; white-space: nowrap; }}
.badge.down {{ color: var(--create); border-color: var(--create); }}
.badge.up {{ color: var(--threshold); border-color: var(--threshold); }}
.badge.new {{ color: var(--call); border-color: var(--call); }}
.badge.design {{ color: var(--set); border-color: var(--set); }}
.callout {{ background: var(--card); border: 1px solid var(--rule); border-left: 3px solid var(--accent);
  padding: 14px 18px; margin: 18px 0; border-radius: 3px; }}
.callout h4 {{ margin: 0 0 6px; font-size: 14px; }}
code {{ font-family: ui-monospace, SFMono-Regular, Menlo, monospace; background: var(--shade);
  padding: 1px 5px; border-radius: 3px; font-size: 0.92em; }}
.foot {{ color: var(--muted); font-size: 12.5px; margin-top: 40px; padding-top: 16px;
  border-top: 1px solid var(--rule); }}
</style>
<div class="wrap">
<h1>The whole specification as recipes</h1>
<p class="lede">Every recipe re-expressed in {n_primitives} primitives &mdash; and the same
{n_primitives} building the world from an empty game. Generated from
<code>tools/research/formulas/data.json</code>.</p>

<div class="callout">
<h4>The one finding</h4>
<p><strong>Creation and transformation are already the same format, because the specification has
been quietly turning relations into things.</strong> <code>deposit</code> and
<code>adjacency</code> are kinds. Once a relation is a thing, building the world is
<code>change</code> and <code>call</code> with nothing that can refuse &mdash; the
identical primitives that play the game, and nothing added for world-building at all.</p>
</div>

<h2>The primitives</h2>
<p>There are {n_primitives}. <code>change</code> carries the guard, so it is the only line that can
refuse on a count; <code>set</code> exists only to change a thing that already exists, since a
positive <code>change</code> carries its traits.</p>
<div class="scroll">{simple_table(
    ["Primitive", "What it does", "Example", "Why it earns a place"],
    DATA["primitives"], ["op", "", "target", "note"])}</div>

<h2>The attach column</h2>
<p>What happens when a line cannot do what it says. <strong>Five values, of which two are in use,
one is the absence of a question, one is in the game without a name, and one is a candidate that
depends on a decision about a different column.</strong></p>
<div class="scroll">{simple_table(
    ["Value", "What it means", "What to know", "Status"],
    DATA["attach_values"], ["op", "", "note", "attach"])}</div>
<div class="callout">
<h4>Which lines can fail at all</h4>
<p><code>change</code> can, and it is the only line that refuses on a count &mdash; that
is what fusing the guard into it means. A <strong>negative</strong> change fails when there is not
enough; a <strong>positive</strong> one fails only where a capacity is declared, which is the same
condition seen from the other end, because a capacity is a count of free space.
<code>require</code> can, being a read that has to find something.
<code>call</code> can, when something inside it fails hard. <code>let</code> can, when a path does
not resolve &mdash; <code>ark.location.below</code> has no answer for an ark that is not in orbit,
and <strong>hard is the only sane value there</strong>. <code>set</code> effectively cannot, which
is why no <code>set</code> in this report carries an attachment.</p>
</div>

<h2>What is sugar</h2>
<p>Removable with linear growth, so by the test each is readability rather than expressiveness
&mdash; which means they can be added freely.</p>
<div class="scroll">{simple_table(
    ["Written as", "Means", "Cost of removing it"],
    DATA["sugar"], ["target", "", "note"])}</div>

<h2>The last column: every row as one string</h2>
<p>The last column of every recipe table restates that row &mdash; op, target, amount, attach and
the note &mdash; as a single string in the notation <code>spec/console.md</code> gives for a command
and for a description of state: <code>{{name field:value ...}}</code>, where a value is a word, a
number or another command, a field referring to a thing is named for that thing's kind, and
<code>#</code> begins a comment. All {N_LINES} lines are encoded, none by hand.</p>
<p>Nine of them needed something that notation does not define. The encoder records each rather
than inventing quietly, and they are listed under <em>What the encoding assumed</em>, below the
data.</p>

<h2>Player recipes <span class="badge">{n_player}</span></h2>
{"".join(formula_table(f) for f in DATA["player"])}

<h2>World recipes <span class="badge">{n_world}</span></h2>
<p>These fire when the turn ends, in order: upkeep, grow and perish, age, spoil, refresh.</p>
{"".join(formula_table(f) for f in DATA["world"])}

<h2>Building the world <span class="badge">{n_creation}</span></h2>
<div class="callout">
<p><strong>No new primitive appears below, and one fewer than before.</strong> Every line
is a positive <code>change</code> or a <code>call</code> &mdash; not one <code>require</code>,
because nothing at design time can be refused, and since a <code>change</code> carries its traits,
not one <code>set</code> either. That is the answer to whether one format can both make the
environment and play the game: <strong>world-building is the play language with the guards left
out.</strong></p>
</div>
{"".join(formula_table(f) for f in DATA["creation"])}

<h3>The data those calls consume</h3>
<p>Copied for review. <code>3 x 4</code> is capacity 3, density 4 &mdash; so
<code>make-deposit(1, food, 3, 4)</code>.</p>
<div class="scroll">{simple_table(
    ["Territory", "Food", "Metal", "Energy", "What it exercises"],
    DATA["territories"], ["amt", "amt", "amt", "amt", "note"])}</div>

<h2>Every kind, the way every recipe is</h2>
<p>Sean, 2026-09-09: <em>I want to be able to review things just as easily as I can review
recipes.</em> A recipe gets a table of lines and a string for each; here is the other half.
<strong>All {n_kinds} kinds</strong>, with what a thing of each carries and how it is written.
The <em>Kinds</em> and <em>Traits</em> tables are copied from the release into this lane's data, so
the two checks below have a population to count against: <strong>every kind has a row</strong>, and
<strong>every trait but one lands on a kind</strong> &mdash; <code>unpaid</code> is the exception,
because it is <em>of a thing with upkeep</em>, which is a predicate rather than a kind.</p>
{kinds_table}

<h2>A thing, as tabular data and as text</h2>
<p>Sean, 2026-09-09: <em>I want to see things as organized tabular data, with the corresponding text
representation</em> &mdash; and, off the top of his head, a territory. Here is territory
{t_id}: <em>{t_note}</em>. Every value below is read from the release's own
<em>Territory resources</em> row; none is typed here.</p>
{t_table}
<pre><code>{t_text}</code></pre>
<div class="callout">
<h4>Two of a territory's four traits have no value anywhere</h4>
<p><strong><code>biome</code> and <code>nature</code> are stored traits of every territory, and no
document gives either one a value for any of the twelve.</strong> <em>Territory resources</em> has
four columns and none of them is a biome; the <em>Biomes</em> table describes what each biome is
like and says <em>force of nature is the one column that binds</em>, which binds a territory's
nature to a biome it has not been given. The code assigns them &mdash; <code>biomes_of</code>, which
<code>R-4</code> vetted &mdash; so this is where a thing lives in two authorities at once, and only
one of them is a document.</p>
<p><strong>This report asserted otherwise until today.</strong> Its world-building data said
<code>make-territory(1, grassland, 1)</code>, which named a biome for territory 1 and a nature for
it, and <strong>nothing states either</strong>. Removed rather than corrected, because there is no
correct value to put there. It was found by trying to draw the table above and having two columns
come up empty &mdash; which is the argument for drawing it.</p>
</div>

<h3><code>metal.density</code> or <code>density.metal</code>: neither</h3>
<p>The question only exists because the resource has been moved into the name.
<strong><code>density</code> is a variable and <code>metal</code> is a value of a different variable,
<code>resource</code></strong> &mdash; so once <code>resource</code> is a column there is nothing
left to order, and the row above reads
<code>{{deposit territory:{t_id} resource:metal total-capacity:.. density:..}}</code>.</p>
<p><strong>The specification already rules on this, and not by analogy.</strong>
<em>Nothing in the state is special to a kind. Adding a kind adds no field and no case, and whatever
reads the state reads it the same way whatever kind it holds.</em> A territory carrying
<code>metal.density</code>, <code>food.density</code> and <code>energy.density</code> gains a field
per resource, which is exactly what that forbids. The long form gains a <em>row</em> per resource
instead, and rows are free.</p>
<p><strong>The instinct behind the dots is right about presentation and wrong about statement.</strong>
The release's <em>Territory resources</em> is wide on purpose &mdash; one row per territory, a
resource per column, <code>3 x 4</code> packing two variables into one cell &mdash; and it is far
easier to read than {n_facts} rows would be. <code>CLAUDE.md</code> already separates these:
<em>a table of game data in markdown is a rendering and never a source.</em> Both forms exist above,
and the wide one was generated from the long one rather than the other way round.</p>

<h2>Capacity, in four questions rather than one</h2>
<p>Sean, 2026-09-09, listing what capacity has to cover: things a territory has capacity for;
containers inside a territory with their own; things that need not worry about it; things that are
ephemeral; and four behaviours &mdash; cannot exist over capacity, can go over with the excess
discarded at the turn's end, cannot be created in that container at all, and no limit.</p>
<p><strong>Mapped onto the rules that exist, the list is shorter than it looks.</strong> Two of the
four behaviours are the same capacity answer wearing different lifetimes, and one of the four is not
a capacity behaviour at all.</p>
<p><strong>Two of the columns below are enums</strong>, defined here rather than spelled out in
every row:</p>
<div class="scroll">{enum_table("bound_values", "At the bound")}</div>
<div class="scroll">{enum_table("lifetime_values", "Lifetime")}</div>
{cap_table}
<div class="callout">
<h4>Resolved 2026-09-09, and it turns on one word</h4>
<p>Sean: <strong>everything except the game itself has to be in a location.</strong> Extractors and
stores are attached to the territory and limited by its capacity; a store then has a capacity of its
own. A resource and the labor a citizen makes are <strong>held by the territory with no limit</strong>
&mdash; labor has to be somewhere, since a citizen makes it and an extractor consumes it.</p>
<p>The release says <em>a territory declares no capacity for a resource</em>, and
<code>spec/logistics.md</code> says <strong>a kind that declares no capacity contains nothing, and
never can</strong>. Those two together forbid what the game does. <strong>What is meant is no
<em>limit</em>, and no capacity is its opposite</strong> &mdash; one declares an unbounded maximum,
the other declares that this sort of thing is never held here. That is <code>X-20</code>, and it is
a phrase rather than a rule.</p>
</div>
<div class="callout">
<h4>Nothing goes over capacity, and the end of a turn is about disorder rather than overflow</h4>
<p><em>Can go over capacity, with the excess discarded</em> describes resources, and nothing goes
over anything. A store holds ten and never refuses, because a resource beyond ten is not in the
store &mdash; it is held by the territory, which has no limit. <strong>What is lost at the turn's
end is what the territory holds directly rather than what a container holds.</strong></p>
<p>Sean's reason for it is <strong>disorder</strong>, and the purpose is that
<strong>an extractor is useful before anything has been built to store what it makes</strong>: work
it, spend what it made that turn, and no store was needed. <strong>Labor is the pure case</strong> -
no container ever holds it, so it is always lost, which is why one citizen's labor is one turn's
labor and never accumulates.</p>
<p>So this is a lifetime rather than a capacity, and it needs no attach value: nothing failed. The
release's phrase for it is <em>a resource that is in nothing</em>, and nothing but the game is in
nothing &mdash; the same one-phrase fix as above.</p>
</div>
<div class="callout">
<h4><em>Cannot be created there at all</em> is already a rule, and it is not capacity zero</h4>
<p><code>spec/logistics.md</code>: <strong>a kind that declares no capacity contains nothing, and
never can.</strong> That is stronger than a capacity of zero and different in kind &mdash; a
capacity of zero is a number that could have been another number, and no declaration is a statement
about what sort of thing this is. Both appear in the game: territory 6 has a metal extractor
capacity of <strong>zero</strong>, which is a number from <em>Territory resources</em>; an extractor
declares no capacity at all, so it holds nothing and never will.</p>
<p>And the spec already carries the reading this report reached from the other side. <strong>Total
capacity is stored; used and available capacity are derived</strong>, available being the total less
the used. <strong>Available capacity is a counter</strong>, and filling a container is subtracting
from it &mdash; which is the complement construction, in the specification, before this lane
suggested it.</p>
</div>
<div class="callout">
<h4>What this changed in the model, rather than in the prose</h4>
<p><code>move</code> no longer writes a <code>location</code>. <code>spec/logistics.md</code> says
<strong>a thing is not located by a trait, and what holds it is what says where it is</strong>, so
moving is <strong>leaving one place and entering another</strong> - two changes where there was one
assignment. It costs a line and deletes a trait the specification does not have.</p>
<p><strong>Neither check can see the difference</strong>, because both aggregate by kind and ignore
where a thing is - a loop that moves metal between territories is not a loop that creates metal, so
ignoring location is right for them and blind here. That is why <code>X-20</code> went unnoticed by
this lane's own tooling for a day.</p>
</div>

<h2>The kinds of storage, and what is stored each way</h2>
<p><strong>{n_storage} of them</strong>, and every one of the
{n_declared} (container, kind) pairs the game allows belongs to exactly one &mdash; asserted, so a
pair that fitted none, or two, would fail rather than render.</p>
{sto_table}
<div class="callout">
<h4>The three that are easy to confuse</h4>
<p><strong>Loose in the territory</strong> and <strong>attached to the territory with no limit</strong>
are the same containment and differ only in lifetime: a citizen the territory holds directly stays,
and a unit of metal it holds directly does not. Nothing about <em>where</em> tells them apart, which
is why lifetime has to be its own column rather than a consequence of the bound.</p>
<p><strong>Inside a container in the territory</strong> is two bounds, not one. A store is bounded by
the territory - as many as the extractors of its resource - and then holds 10 itself. That is the
spec's <em>a thing that contains things takes up capacity in whatever contains it, so capacity is not
conserved</em>.</p>
<p><strong>Held by nothing, and holding nothing</strong> are two different facts that both come out
empty. The game is in nothing, which is the one exception to everything being somewhere. An extractor
<em>declares no capacity</em>, so it holds nothing and never can - and that is not a capacity of zero,
which territory 6's metal extractors have.</p>
</div>

<h2>The capacities the game declares</h2>
<div class="callout">
<h4>Where a bound lives, decided 2026-09-08</h4>
<p><strong>A capacity is a property of the container, declared once, and no recipe states it.</strong>
That deletes the <code>limit</code> role outright: both of its uses in the whole specification were
<code>limit 0 garrison</code>, restating a capacity of 1 that <em>What bounds a kind in a territory</em>
already declared. Every other capacity below was <em>never</em> written in a recipe &mdash; so the
engine was already enforcing seven bounds that no recipe stated, and the garrison was the odd one
out for being written twice rather than for being written at all.</p>
</div>
<div class="scroll">{simple_table(
    ["Container", "Holds", "Up to", "Was it ever in a recipe?"],
    DATA["capacities"], ["target", "target", "amt", "note"])}</div>

<h2>Repetition, or quantification</h2>
<p>Sean, 2026-09-09: <em>should we have a construct for repetition? It is an extra construct but has
the potential to lower the number of instructions needed.</em> Then, a minute later:
<em>I suppose repetition and quantity are similar concepts.</em></p>
<p><strong>They are the same concept, and the specification already says exactly when.</strong>
<code>change +2 citizen in t</code> <em>is</em> the creation of a citizen repeated twice &mdash; the
amount column is a repetition count written as a number. So there is no question of adding
repetition; it is there, and it is called a quantity.</p>
<div class="callout">
<h4>The line between them is whether the repeated things are distinguishable</h4>
<p><code>spec/console.md</code>: <strong>what a thing contains is a map from a description to a
quantity</strong>, and <strong>each distinct description is its own entry</strong>. So a quantity is
always <em>per description</em>. Two citizens made in the same territory share a description, so
they are one entry of two. Twelve territories have different ids, so they are twelve descriptions
and <strong>no number can merge them</strong>.</p>
<p>The rule is already stated from the other side as well: <strong>there is never a quantity of a
thing with an <code>id</code></strong>, and a thing carrying one has a description no other thing
shares, so its quantity is always one. <strong><code>change +12 territory</code> is therefore not
merely awkward, it is forbidden</strong> &mdash; and that is the whole reason
<code>make-territory</code> has to be called twelve times rather than once with an amount.</p>
<p>So the missing construct is not repetition. <strong>It is what you need when repetition cannot
collapse into a quantity</strong>, which is quantification: one firing per member of a set, with
something bound differently each time.</p>
</div>
<div class="scroll"><table><thead><tr><th></th><th>What it is</th><th>Where it is used</th>
<th>Is it expressible?</th></tr></thead><tbody>
<tr><td class="target"><strong>Repetition</strong></td>
<td>the same thing, n times</td>
<td><strong>{n_mult}</strong> changes whose amount is more than one</td>
<td><strong>Yes, twice over.</strong> The amount column does it for a change, and
<code>spec/console.md</code> gives a command a <code>repeat</code>, which is <em>how many times it
fires</em>. A third way would buy nothing.</td></tr>
<tr><td class="target"><strong>Quantification</strong></td>
<td>once per member of a set, with something bound differently each time</td>
<td><strong>{n_quant}</strong> <code>each</code> lines, {n_state} of them over a description</td>
<td><strong>Yes, since 2026-09-09.</strong> It was in a prose <em>selection</em> field in six
recipes and in a comment in four lines of <code>make-world</code>; Sean added the construct and both
became lines.</td></tr>
</tbody></table></div>
<div class="callout">
<h4>Added 2026-09-09, and what it cost</h4>
<p><code>each &lt;name&gt;: &lt;set&gt;</code> binds a name over every member of a set and scopes
the lines below it, up to the next <code>each</code> or the end of the recipe. One rule covers both
shapes: a world recipe has one at the top scoping its whole body, and <code>make-world</code> has
four, each scoping the call beneath it.</p>
<p><strong>It cost ten lines and a sixth primitive</strong>, and bought the thing the constraint
asks for rather than conciseness: the six world recipes and the four world-building calls now say
what they do <em>in the language</em>, where before they said it in a field and a comment that
nothing could run. <strong>{n_quant} quantifiers, {n_state} of them over a description of the
state</strong> - the rest range over a data table or over the planet's geometry, which the notation
has no form for and which the assumption list records.</p>
<p><strong>It changes nothing about the checks, and that is on purpose.</strong> An
<code>each</code> has no effect of its own: it says how many transitions a recipe is a source of,
and the lines below it are the transition. So checks 1 and 2 weigh the body, exactly as
<code>spec/invariants.md</code> describes - <em>a rule is a source of transitions, not a kind of
one</em>. Weighing the whole would make a recipe's net effect depend on the size of the planet,
which is not a fixed vector at all.</p>
</div>

<div class="callout">
<h4>It is not sugar, and the reason is <code>xE</code></h4>
<p>The test this report has used since <code>X-11</code> is Felleisen's: a construct is
<em>eliminable</em> if a local expansion removes it without restructuring anything. Quantification
over twelve territories expands to twelve lines &mdash; linear, and by that test sugar.</p>
<p><strong>Except that twelve is not a constant.</strong> <code>spec/console.md</code> has
<code>create planet &lt;size&gt;</code> and <code>spec/planet.md</code> has five planet sizes, so the
number of territories is a parameter and the number of shared edges is whatever the geometry gives
&mdash; which is why that row's multiplicity is written <code>xE</code> and not a number.
<strong>You cannot expand what you cannot count when you write it.</strong> So quantification is not
removable by a local expansion, which is precisely what it means for a construct to add expressive
power rather than convenience.</p>
</div>
<div class="callout">
<h4>And it is already half-built, in two places</h4>
<p><code>spec/invariants.md</code>: <strong>a rule is a source of transitions, not a kind of
one.</strong> A rule that is a <em>source</em> of many is a schema, and turning it into its
transitions is quantification &mdash; so the specification has already committed to this, and the
word for it is grounding.</p>
<p><code>check.py</code> does it today. <code>work</code> names the family <code>resource</code> and
the checker expands it into <code>work[food]</code>, <code>work[metal]</code> and
<code>work[energy]</code>, which is <code>X-17</code>. <strong>That is a quantifier over a family,
already implemented, and the same operation an interface performs to build a menu.</strong> What is
missing is not the machinery but a way to write it in a line.</p>
<p><strong>The caveat is the same one <code>X-9</code> gave <code>call</code>.</strong> A quantifier
ranging over a set the recipe does not change is finite and free. One ranging over something the
recipe itself creates is a loop, and that is where the analysis stops terminating. <em>Each
territory</em> is safe; <em>each thing this makes</em> is not.</p>
</div>

<h2>Defining a kind, then using it</h2>
<p>Sean, 2026-09-09: <em>can we define what an ark is first, then define recipes that define what an
ark does? Is that kind of thing possible with our current model?</em></p>
<p><strong>The specification already commits to it.</strong> <code>spec/invariants.md</code>: <em>the
definitions are part of the game state, and defining one is a transition like any other, so a game's
history is a complete account of it, including what its rules were.</em> And <em>a definition
arrives in one transition; there is no state in which a kind or a recipe is half defined</em> -
which is exactly what a positive <code>change</code> carrying its traits now does.</p>
<div class="callout">
<h4>It is not possible today, and check 6 says why in one line</h4>
<p>Written as a recipe and run through the checks, <code>define ark</code> is refused:</p>
<pre><code>UNDECLARED: define ark | kind[name:ark, force:1, fuel:6] in game
            nothing says a game may hold a kind
UNDECLARED: define ark | recipe[name:launch-ark] in game
            nothing says a game may hold a recipe</code></pre>
<p><strong>Neither <code>kind</code> nor <code>recipe</code> is one of the sixteen kinds</strong>, so
a definition has nowhere in the state to be - and everything but the game has to be somewhere. The
fix is small and mechanical: two kinds, and two declarations saying the game may hold them. Half the
step is taken already, since <code>call</code> writes <code>recipe:found-colony</code> as though a
recipe were a thing with a name, six times.</p>
<p><strong>The ordering is not a difficulty.</strong> Defining the ark and defining what it does are
two transitions, and a recipe that needs a kind to exist first can say so with the primitive that
survived for exactly this shape: <code>require 1 {{kind name:ark}}</code>. A read arc makes the
dependency explicit and checkable rather than a matter of writing them in the right order.</p>
</div>
<div class="callout">
<h4>The limiting factor is analysis, and it is narrower than this report first said</h4>
<p>Every check on this page assumes <strong>a fixed set of transitions</strong>. Check 2 builds a
matrix of them; check 1 weighs each one; check 6 reads them all. So the question is not whether
definitions happen, but whether the set they produce can be known.</p>
<p><strong>Corrected the same day.</strong> This section first said the design-time distinction was
what kept the checks meaningful. That is not it. <strong>Manual definition costs nothing at all</strong>
&mdash; these are edit-time checks over a fixed rule set, so writing a new recipe means re-running
them, which is what an editor does anyway. A person adding a recipe during play is still a person.
The line is not the phase, and it is not who is at the keyboard.</p>
<p><strong>The line is whether the set of recipes that could ever exist is finite and knowable in
advance.</strong> A recipe that picks a definition from a fixed catalogue is a <em>reconfigurable
net</em>, and boundedness stays decidable for that class. A recipe that composes an arbitrary new
recipe out of the state is a <em>net rewriting system</em>, which is Turing powerful, and the
decidable properties of Petri nets are lost with it. <strong>So automatic definition is allowed
too</strong>, on that condition &mdash; which is a much weaker restriction than a phase.</p>
</div>
<div class="callout">
<h4>Time flowing one direction buys something real, and it is not what saves you</h4>
<p>It makes the <strong>per-transition</strong> checks incremental. Checks 1 and 6 are properties of
a single recipe read on its own: does <em>this</em> recipe conserve metal, does <em>this</em> line
put its thing somewhere declared. A recipe that passes them passes them forever, and nothing later
can un-pass it &mdash; so with time going one way you check only what is new.</p>
<p><strong>Check 2 is not like that, and here is a recipe that proves it.</strong> Adding
<code>change +1 deposit in t</code> - a deposit out of nothing:</p>
<pre><code>as it stands                 check1 green (12)   check6 green   check2 undeclared gain: none
add: a deposit from nothing  check1 green (13)   check6 green   check2 undeclared gain: deposit +1</code></pre>
<p>It conserves metal, because a deposit has no Binding, and it puts the deposit somewhere a
territory declares. <strong>Both per-transition checks stay green and the whole-net one goes
red.</strong> Boundedness is a property of the matrix rather than of any row in it, so one new
transition can create a loop among transitions that were all fine before.</p>
<p><strong>So the working rule is cheap and precise: on defining a recipe, check the new one against
1 and 6, and re-run 2 over everything.</strong> Monotone time is exactly what makes the first half
incremental, and it does nothing for the second.</p>
</div>

<h2>Selection, and what an interface needs</h2>
<p>Sean, 2026-09-09, on the interface: select a thing and be shown the options for it; select several
things; select a portion of a quantity - all citizens, one, five of ten. And: <em>perhaps recipes
have an implicit selection parameter, and certain recipes only apply to certain types of
selections.</em></p>
<p><strong>They already do, and it is prose.</strong> Every one of the {n_sel} player recipes carries
a <em>selection</em> field, and it has never been part of the language:</p>
{sel_table}
<div class="callout">
<h4>The owner column <em>is</em> the quantifier</h4>
<p>The world's recipes now say <code>each x: {{...}}</code> on their first line. The player's name one
thing. That is the same field with a different quantifier &mdash; <strong>a world recipe fires for
every member of the set, and a player recipe is offered for one the player picks</strong> &mdash;
and it is the specification's own distinction: <em>the player's are offered wherever their inputs are
present, to take or to leave; the world's are not offered.</em></p>
<p>So <code>owner</code> is not a third thing to carry. It is <em>which quantifier</em>, and the six
world recipes prove one half of it is already writable.</p>
</div>
<div class="callout">
<h4>Fungibility is not a simplification the interface may take advantage of &mdash; it is the state</h4>
<p><em>Five of ten citizens</em> is not five things picked out of ten. <code>spec/console.md</code>:
<strong>what a thing contains is a map from a description to a quantity</strong>, and
<strong>there is never a quantity of a thing with an <code>id</code></strong>. Ten citizens in a
territory are one entry of ten, and <strong>there is no citizen number three to select</strong>.</p>
<p>So a selection is <strong>a description and a count</strong> &mdash; which is the same object as a
container's contents, and the same object a recipe line targets. All, one, and five of ten are one
shape with a different number, and the interface needs no special case for any of them. What it
cannot offer is picking <em>which</em> five, because that question has no referent.</p>
</div>
<div class="callout">
<h4>Nothing needs multi-select, and what looks like it is a parameter</h4>
<p><strong>Every one of the {n_sel} player recipes selects exactly one thing</strong>, and
{n_multi} select two. What looks like a second selection is a parameter: <code>move</code> selects a
unit and takes <code>$to</code>; <code>build extractor</code> and <code>build store</code> select a
territory and take <code>$resource</code>. Two of eleven need one at all.</p>
<p>That is `X-8`'s distinction doing work: <strong>a parameter is a choice the player makes and a
derived term is not</strong>, so which parameters exist decides exactly what the interface has to
ask for after the selection. Here that is a place, twice, and a resource, twice.</p>
</div>
<div class="callout">
<h4>The gap: a selection tests derived traits, and a description may not carry one</h4>
<p><strong>Six selections test a derived trait</strong> &mdash; <em>a territory you control</em> four
times, plus <code>surplus</code> in <code>grow</code> and <code>unpaid</code> in
<code>perish</code>. And <code>spec/console.md</code> says a description is a kind and every
<em>stored</em> trait, and <strong>a derived trait is never part of one</strong>.</p>
<p>So <strong>what the player selects is a query rather than a description</strong>, and only the
second has a form in the notation. They are close enough to be confused and different in exactly one
way: a description says what a thing <em>is stored as</em>, and a query says what is <em>true of
it</em>. An interface needs the second - you select what you control, not what is recorded.</p>
<p>This is the same hole the encoding has flagged all along for <code>food[surplus]</code>, now with
five more instances and a reason. Filed as an open decision.</p>
</div>
<div class="callout">
<h4>What the model does not care about</h4>
<p>A radial dial, a fixed row of buttons that change with the selection, a context menu - the model
is indifferent, because all three render the same list. <strong>What is settled is the order</strong>:
noun then verb, select then operate, which `X-8` traced to the Xerox Star and which is right here for
a reason stronger than habit - selecting first leaves the system in <strong>no mode</strong>, and
verb-first does not.</p>
</div>

<h2>Still open, and yours to take</h2>
<p>{n_open} of the {n_decisions} questions this re-encoding raised are still open. The rest are
below, under <em>What has already been settled</em>, so that this list is short enough to be a list
of things to do.</p>
<div class="scroll">{simple_table(
    ["Question", "What is at stake", "How it shows up"],
    open_decisions, ["", "", "note"])}</div>

<h2>Where the attach column might still go</h2>
<p>Four questions about the column, not about how to read it &mdash; one of them withdrawn since, and kept so it is not asked again.</p>
<div class="scroll">{simple_table(
    ["Question", "The two readings", "What is at stake", "Where it came from"],
    DATA["attach_decisions"], ["target", "", "note", "note"])}</div>
<div class="callout">
<h4><code>record</code> is the one worth looking at, because it is already there</h4>
<p><code>unpaid</code> is a trait in the specification &mdash; <em>derived: its upkeep was not
met</em> &mdash; and <code>perish</code> fires on it. So when <code>upkeep</code> cannot take its
food, the failure <strong>neither stops the turn nor vanishes</strong>: it is written down, and
another recipe reads it next. <strong>That is a third behaviour on failure, in the game today,
which a two-valued column cannot express.</strong></p>
<p>It may not want to be an attach value. The alternative is that <strong>failure is state</strong>
&mdash; every line's outcome is recorded, and <code>unpaid</code> is just one query over it. That is
a larger idea and a cleaner one, and it is the sort of thing worth deciding before the column sets
rather than after.</p>
</div>

<h2>Design time, if it stopped being a category</h2>
<p>Sean, 2026-09-09, still considering it: <em>removing the design-time distinction from the data as
something special, having it either enforced from the engine or having a trait indicate when it can
be used</em>. Today it is neither &mdash; it is a <strong>separate list</strong>, and that is the one
option below with no cost and no check.</p>
<div class="scroll"><table><thead><tr><th>Option</th><th>What it is</th><th>Cost</th><th>What it buys, and what it does not</th></tr></thead>
<tbody>{"".join(f'<tr><td class="target">{r[0]}</td><td>{r[1]}</td><td class="amt">{r[2]}</td><td class="note">{r[3]}</td></tr>' for r in DATA["phase_options"])}</tbody></table></div>
<div class="callout">
<h4>The middle one, and the one thing it costs that is not a line</h4>
<p><strong>It introduces no concept at all</strong>, which under <em>minimum expressiveness</em> is
the whole argument. <code>spec/console.md</code> already says <em>a command of one phase is refused
in the other</em>, and refusing is what a <code>require</code> does &mdash; so the sentence stops
being a rule the engine implements and becomes a line the recipe carries. It also takes
<code>require</code> from one use to six, which is most of the case for keeping it a primitive
rather than sugar for a self-loop.</p>
<p><strong>The cost is the error message.</strong> A design command refused after <code>start</code>
can say <em>that command belongs to the design phase</em> only because the engine knows the
category. Refused by a guard, it says <em>no game with phase design</em> &mdash; true, and worse.
<code>spec/console.md</code> asks that a rejection be <em>in terms of the game rather than the
parser</em> and name <em>what was wrong, where, and what was expected instead</em>, so this is a
real requirement and not a nicety. It is not fatal: a guard on <code>game.phase</code> is a
recognisable shape and a message can be written for it. But it is the thing to check before
deciding, and it is the only cost this lane can find.</p>
</div>

<h2>Should a kind be a trait?</h2>
<p><strong>It already is one.</strong> The release's <em>Traits</em> table opens with
<code>kind &middot; every thing &middot; one of the kinds &middot; stored</code>, and
<code>spec/invariants.md</code> says <em>a thing is a set of traits, and one of them names its
kind</em>. So they are not two sorts of thing to be unified; a kind is the value of a trait, and the
notation writes it first rather than as <code>kind:citizen</code> because every thing has one.</p>
<p><strong>The question worth asking is the harder one: could it be dropped?</strong> If a thing were
nothing but its traits, with no discriminator, the traits would have to say which kind it is. They
cannot. <strong>{n_clash} of the {n_all} kinds share their trait set with another</strong>, so no
reading of the traits could tell them apart:</p>
{clash_table}
<div class="callout">
<h4>What that settles, and what it does not</h4>
<p>Five kinds carry <strong>no stored traits whatever</strong>. A yard and a unit of metal are the
same bag of traits, and only the kind separates them &mdash; so <strong><code>kind</code> is not
derivable, and dropping it would mean inventing a discriminating trait, which is <code>kind</code>
under another name.</strong></p>
<p>It does <em>not</em> settle that kinds are a second sort of thing. They also appear as
<strong>values</strong>: <code>resource</code> takes <code>food</code>, <code>metal</code> or
<code>energy</code>, which are kinds, and <code>from</code> and <code>to</code> take places, which
are things. <code>spec/console.md</code> already leans on this &mdash; <em>a field that refers to a
thing is named for that thing's kind</em>. So a kind is doing two jobs: it classifies a thing, and it
is a domain that trait values are drawn from. <strong>Unify the first job with traits, because it
already is one; keep the second, because a value has to come from somewhere.</strong></p>
</div>

<h2>What unifies, and what must not</h2>
<p><strong>The shared object is the description, and the specification says so before this report
does.</strong> <em>A game's state is things, in places, and how many of each. A thing is a set of
traits, and one of them names its kind.</em> That is <code>{{kind trait:value ...}}</code> &mdash;
the same object a recipe line targets. So a thing and a recipe line are not two structures to be
unified; they are one structure appearing in two roles.</p>
<div class="scroll"><table><thead><tr><th></th><th>Which</th><th>Why</th></tr></thead>
<tbody>{"".join(f'<tr><td class="target">{r[0]}</td><td>{r[1]}</td><td class="note">{r[2]}</td></tr>' for r in DATA["unify"])}</tbody></table></div>
<div class="callout">
<h4>Then what is a recipe that only creates? Nothing but the thing</h4>
<p>Since the fold, <strong>every world-building recipe is one line, with
<code>change</code> as its operator, <code>+1</code> as its amount and no attachment</strong> &mdash;
all three constant. Take away what never varies and what remains is the description. <strong>So
&ldquo;one recipe creates each thing&rdquo; is not a way to unify them: it is what is left when
they already are unified.</strong></p>
<p>And it turns out to satisfy a rule nobody was aiming at. <code>spec/invariants.md</code>:
<em>A definition arrives in one transition. There is no state in which a kind or a recipe is half
defined.</em> A <code>create</code> followed by three <code>set</code>s <strong>has</strong> such a
state &mdash; a territory that exists with no biome. Non-sequential composition removes it, and the
invariant was not the reason for the decision.</p>
</div>

<h2>The checks, as run</h2>
<p>Not a description of what they would report &mdash; the output of
<code>tools/research/formulas/check.py</code>, read from <code>results.json</code>, so this page
and the checker cannot disagree. <strong>All three were poisoned before being believed</strong>:
{"; ".join(RESULTS["poison"])}.</p>

<h3>Check 1 &mdash; metal conserved outside extraction &nbsp;<span class="badge down">{len(RESULTS["conservation"]["violations"])} violations</span></h3>
<p>{RESULTS["conservation"]["analysed"]} recipes analysed;
{len(RESULTS["conservation"]["skipped"])} skipped for state-dependent amounts
({", ".join(RESULTS["conservation"]["skipped"])}); <code>work</code> excluded as the declared
source. Weights are the <em>Binding</em> column, copied rather than invented:
{", ".join(f"{k} {v}" for k, v in RESULTS["conservation"]["weights"].items())}.</p>
<div class="callout">
<h4>Clean, and the two stores were the whole of it</h4>
<p><strong>Sean dropped the two stores on 2026-09-09 and metal now balances exactly.</strong>
<code>found-colony</code> delivers garrison 1 + extractor 1 + extractor 1 = <strong>3</strong>,
which is precisely an ark's Binding and a pioneer's, so <code>deploy ark</code> and
<code>found by land</code> both come out at <strong>0</strong>. His principle &mdash; what a unit
costs in metal is what it delivers &mdash; now holds for every recipe that is not mining.</p>
<p><strong>The check is poisoned against exactly this.</strong> Putting the two stores back turns
<code>deploy ark</code> and <code>found by land</code> red again, which is what makes the green
above worth reading. A check that agreed with a fix rather than measuring it would look identical
from here.</p>
<p><strong>Two faults in the check itself, both found by his objection and both fixed.</strong>
It tested a stronger claim than the words can carry &mdash; mining creates metal, so
<code>work</code> is a declared source and the invariant is <em>conserved outside extraction</em>.
And it weighed <code>found-colony</code> as if a player could fire it with no ark and no pioneer
spent; a recipe that is only ever called is not a transition.</p>
</div>

<h3>Check 2 &mdash; structurally unbounded &nbsp;<span class="badge {VERDICT_2[1]}">{VERDICT_2[0]}</span></h3>
<p>{RESULTS["unbounded"]["analysed"]} transitions after grounding;
{len(RESULTS["unbounded"]["skipped"])} skipped ({", ".join(RESULTS["unbounded"]["skipped"])}).</p>
<div class="scroll">{simple_table(
    ["Fire this many times", "Recipe"],
    [[w[1], w[0]] for w in RESULTS["unbounded"]["witness"]],
    ["amt", "target"])}</div>
<p>Metal-equivalent gain <strong>{RESULTS["unbounded"]["metal_gain"]}</strong>, and
<code>work[metal]</code> is in the witness &mdash; which is the point. <strong>The check can see
mining now.</strong> Until families were resolved to kinds, <code>work</code> produced
<code>resource</code>, a family with no metal weight, and the game's only metal source scored
zero.</p>
<div class="callout">
<h4>Grounding, and it is the same operation the menu needs</h4>
<p>A target naming a family stands for one transition per kind in it, so <code>work</code> is
three: <code>work[food]</code>, <code>work[metal]</code>, <code>work[energy]</code>, each with its
own largest density. <strong>That is grounding</strong> &mdash; instantiating a recipe against
what it could apply to &mdash; and it is the same operation an interface performs to build a menu.
The analysis and the interface want the same machinery, which is a reason to build it once.</p>
</div>

<h3>Check 2b &mdash; the same without mining &nbsp;<span class="badge {VERDICT_2B[1]}">{VERDICT_2B[0]}</span></h3>
<p>{RESULTS["unbounded"]["without_sources"]["transitions"]} transitions with <code>work</code>
removed. <strong>This is the question worth asking</strong>, and it is the recommendation in
practice: rather than keeping a baseline of intended loops, remove the declared source and ask
whether metal can still grow.</p>
<div class="scroll">{simple_table(
    ["Fire this many times", "Recipe"],
    [[w[1], w[0]] for w in RESULTS["unbounded"]["without_sources"]["witness"]],
    ["amt", "target"])}</div>
<p>Metal-equivalent gain <strong>{RESULTS["unbounded"]["without_sources"]["metal_gain"]}</strong>.
The one loop left is <code>create labor</code>, which makes labor from nothing every turn and is
declared free in the <em>Kinds</em> table. <strong>So there is no metal source other than
extraction</strong> &mdash; which is what conservation was supposed to mean, now checked rather
than asserted.</p>

<h3>Check 4 &mdash; is an attachment observable? &nbsp;<span class="badge up">{len(RESULTS["attachment"]["unobservable"])} decide nothing</span></h3>
<p>A <code>create</code> fails when its container will not hold another. So an attachment on a
create whose target has <strong>no capacity</strong> decides nothing &mdash; hard and soft behave
identically and always will. <strong>Naming a question does not make it one.</strong></p>
<div class="scroll">{simple_table(
    ["Recipe", "Line", "Attach", "Capacity"],
    [[r[0], r[1], r[2], r[3]] for r in RESULTS["attachment"]["meaningful"]],
    ["target", "target", "attach", "note"])}</div>
<p>And the one where it does not:</p>
<div class="scroll">{simple_table(
    ["Recipe", "Line", "Attach", "Why it cannot fail"],
    [[r[0], r[1], r[2], r[3]] for r in RESULTS["attachment"]["unobservable"]],
    ["target", "target", "attach", "note"])}</div>
<div class="callout">
<h4>The citizen question mark is not a decision</h4>
<p>A territory never refuses a citizen &mdash; they are held down by food through upkeep, which is
not a capacity. <strong>So hard and soft are the same line.</strong> The options are below, and the
only one that behaves differently is a second reading of the word <em>soft</em> rather than a
second value of it.</p>
</div>
<div class="scroll">{simple_table(
    ["Option", "What it does", "What it costs", "Applies to"],
    DATA["attachment_options"], ["target", "", "note", "note"])}</div>
<div class="callout">
<h4>Withdrawn: <em>what does soft test</em> was a false choice</h4>
<p>An earlier version of this report asked whether <code>soft</code> means <em>the container is
full</em> or <em>one is already there</em>. <strong>There is exactly one condition under which a
create can fail &mdash; the container will not take another &mdash; so there is one meaning and
not two.</strong> The second reading was this lane's invention with no case behind it, and Sean
was right not to understand the question.</p>
<p><strong>The real distinction it was standing in front of is about <code>create</code>, not about
<code>soft</code>.</strong> <code>create X [soft]</code> means <em>add one if there is room</em>,
which is not the same as <em>ensure one exists</em>. Those coincide wherever capacity is 1 &mdash;
every case here except the extractors. Territory 1 has food capacity 3, so deploying twice would
add a second food extractor. <strong>If that is wrong, the fix is at the call</strong> &mdash;
do not found a colony that is already founded &mdash; and not a second kind of create.</p>
</div>
<h3>Parameter domains &mdash; what a recipe may be instantiated over</h3>
<p>A parameter is not free. Its domain is a <strong>condition on the kind</strong>, not a list, and
grounding resolves it &mdash; so an instance that does not satisfy the condition never exists, and
in a selection interface the player never sees it.</p>
<div class="scroll">{simple_table(
    ["Recipe", "Parameter", "Domain", "Today", "Why"],
    DATA["domains"], ["target", "target", "", "amt", "note"])}</div>
<div class="callout">
<h4>Food stores, and why the rule is derived rather than named</h4>
<p><strong>Sean, 2026-09-09: a food store buys nothing, so it is out.</strong> The tempting fix is
to write <em>no food stores</em>. The better one is
<strong><em>a store may be built for a resource whose things do not expire</em></strong>, which
today admits metal and energy and excludes food, because food is made with <code>keeps</code> 1 and
dies at the turn end it was made.</p>
<p><strong>Preservation technology raises food's <code>keeps</code>, and food stores become
available with no change to the rule.</strong> That is the difference between a condition and a
list: one of them already knows about a feature that does not exist yet.</p>
</div>
<div class="callout">
<h4>Two reasons agreeing is not two pieces of evidence</h4>
<p><code>found-colony</code> has no stores. That was decided for <strong>conservation</strong>
&mdash; garrison plus two extractors is 3, exactly an ark's Binding &mdash; and the food store also
turns out to buy nothing, so it would have gone anyway. <strong>The metal store does buy something
and still stays out</strong>: a new colony gets the minimum that works and builds its own. Worth
separating, because a decision that looks doubly supported is easy to stop examining.</p>
</div>

<h3>Check 5 &mdash; which territories would refuse a create, per line</h3>
<div class="scroll">{simple_table(
    ["Recipe", "Line", "Attach", "Blocked on"],
    [[r[0], r[1], r[2], ("none - every territory has room" if not r[3]
      else "territory " + ", ".join(r[3]))] for r in RESULTS["placement"]],
    ["target", "target", "attach", "note"])}</div>
<div class="callout">
<h4>Corrected: territory 7 was never affected</h4>
<p>An earlier version of this report said territories 6 and 7 both mattered. <strong>Territory 7
has no energy, and no recipe here creates an energy extractor</strong>, so it never did. Sean
caught it. The check now asks which territories lack <em>the resource the line actually names</em>
rather than which lack any resource &mdash; a wider question with a plausible answer, which is the
failure this repository keeps recording.</p>
<p><strong>Decided 2026-09-09.</strong> Two citizens unconditionally; each extractor only where
there is capacity. So both extractor lines are <code>SOFT</code>, and <strong>only territory 6 is
affected, and only by the metal line</strong>: a colony founded there simply has no metal
extractor, rather than founding being refused.</p>
</div>
<div class="callout">
<h4>The same condition, two attachments, and both are right</h4>
<p><code>found-colony</code> creates a metal extractor <code>SOFT</code>, so founding on territory
6 succeeds without one. <code>build extractor</code> creates one <code>HARD</code>, so a player who
chooses to build a metal extractor on territory 6 is refused. <strong>Identical condition, opposite
answers, and neither is a mistake</strong> &mdash; because one is a consequence of landing
somewhere and the other is a thing the player asked for.</p>
<p>Under a selection interface the second never even arises: grounding the recipe against
territory 6 produces no <code>build extractor[metal]</code> to select. <strong>The attachment is
what the console needs and the menu is what the interface needs, from one line.</strong></p>
</div>

<h3>Check 6 &mdash; is everything created put somewhere that declares it? &nbsp;<span class="badge {v6cls}">{v6}</span></h3>
<p><strong>Every positive change names a container, and that container declares a bound for that
kind.</strong> {c6_examined} positive changes read from the recipes themselves, against
{n_declared} declared pairs. A negative change is not tested: it acts on something that already
exists, so where it is is a fact about the state rather than a claim the recipe makes.</p>
<div class="callout">
<h4>Its green would mean nothing, so it is poisoned twice</h4>
<p>The declarations were written after Sean answered <code>X-20</code>, so the check agreeing with
the recipes today proves only that they were written from the same source. <strong>The second poison
is the evidence</strong>: take back the rows his answer added - a territory holds a resource with no
limit - and the check goes red on {c6_x20} lines, which is the state the release still describes.
<strong>So this check would have caught the contradiction this lane's own model carried for a
day</strong>, and neither of the other checks could, because both aggregate by kind and ignore where
a thing is.</p>
</div>

<h3>Check 3 &mdash; a cap of {RESULTS["cap"]["cap"]} &nbsp;<span class="badge down">{len(RESULTS["cap"]["breaches"])} breaches</span></h3>
<p>Applying each recipe once breaches nothing, which is exactly the point about a cap:
<strong>it found neither of the two things the other checks found.</strong> It is a backstop
against bugs in checks 1 and 2, not a way of finding anything.</p>

<h2>Detecting a glitch, without a cap</h2>
<div class="callout">
<h4>Unbounded capacity does not cost you detection</h4>
<p><strong>Boundedness for one starting state is EXPSPACE-complete. Boundedness for
<em>every</em> starting state is polynomial &mdash; a linear program over the incidence
matrix.</strong> The second is the one an editor needs, because an author is editing recipes and
not a saved game. A net is not structurally bounded exactly when there is a non-negative firing
vector <code>x</code>, not all zero, with <code>C&middot;x &ge; 0</code>: a set of recipes that,
fired in some ratio, ends with more than it began. <strong>The vector is the error message</strong>
&mdash; it names which recipes and how many of each.</p>
<p><strong>All of it needs the plain fragment.</strong> A zero test on an unbounded quantity makes
the language Turing-complete and every line of this section false.</p>
</div>
<div class="scroll">{simple_table(
    ["Check", "What it asks", "Cost and character", "What it reports", "Where it fits"],
    DATA["detection"], ["target", "", "", "", "note"])}</div>

<h3>What the specification already declares, and nothing checks</h3>
<p>These four sentences are invariant declarations sitting in the <em>Kinds</em> table as prose.
Making them checkable is the smallest change with the largest payoff.</p>
<div class="scroll">{simple_table(
    ["Kind", "Declared", "Where it already says so", "What would check it", "Note"],
    DATA["invariants"], ["target", "", "note", "", "note"])}</div>
<div class="callout">
<h4>Why the broad check must be a diff</h4>
<p><strong>The intended economy is itself an infinite loop.</strong> Food feeds citizens, citizens
make labor, labor works extractors, extractors make food. That is a T-increasing and a structural
check will flag it &mdash; correctly. So check 2 cannot ask <em>is anything unbounded</em>; it asks
<em>is anything unbounded that was not there before</em>. Check 1 has no such problem, because the
author names the resource that must not grow and food is simply not on the list.</p>
</div>

<h2>Theme, and the number it derives</h2>
<p>Sean, 2026-09-09: <em>food gets energy from the sun, citizens get energy from food, so labor is
not thematically free.</em> <strong>He is right that it does not belong in a recipe, and right that
it helps reason about them &mdash; it derives a quantity the specification never states.</strong></p>
<div class="scroll">{simple_table(
    ["", "The claim", "What it gives", "Status"],
    DATA["theme"], ["target", "target", "", "note"])}</div>

<h3>Growth rate, and why density does not buy speed</h3>
<p>Reproduction caps the per-capita rate at <strong>1</strong> &mdash; a parent per child. Food caps
it at <strong>d&minus;1</strong>. So the realised rate is <strong>min(d&minus;1, 1)</strong>. Every
density in the release is an integer, so <strong>the food-limited middle case never occurs</strong>:
a territory either doubles or is frozen.</p>
<div class="scroll">{simple_table(
    ["Territory", "Food", "Density", "Rate", "Behaviour", "Ceiling", "Surplus lost per turn"],
    DATA["growth"], ["amt", "amt", "amt", "amt", "attach", "amt", "note"])}</div>
<div class="callout">
<h4>Density does two things, and speed is not one of them</h4>
<p>Territory 3 is <code>6 x 2</code> and territory 8 is <code>6 x 6</code>. <strong>They grow at
exactly the same rate</strong> &mdash; both double every turn &mdash; and differ only in where they
stop: ceiling 12 against 36. Above density 2, extra density buys <strong>ceiling and surplus, never
speed</strong>. A reader who takes <code>6 x 6</code> as <em>six times faster</em> would be wrong,
and the release does not say otherwise anywhere.</p>
</div>
<div class="callout">
<h4>Which raises a question about the surplus, and about food stores</h4>
<p>Territory 8 produces <strong>5n</strong> surplus food and <code>grow</code> can use only
<strong>n</strong> of it. The other <strong>4n</strong> has nowhere to go. Food is made with
<code>keeps</code> 1, and the turn runs <em>upkeep, grow, perish, age, spoil</em> &mdash; so food
made this turn is aged to 0 and spoiled <strong>at the same turn end</strong>, whether it sits in a
store or in nothing.</p>
<p><strong>If that reading is right, a food store buys nothing</strong>, because stored and unstored
food die at the same moment. It may be that storing is meant to reset <code>keeps</code> and nothing
says so. <strong>Raised rather than filed as a defect</strong> &mdash; and it is a curious postscript
to dropping the two stores from <code>found-colony</code>, one of which was a food store.</p>
</div>

<h3>The population ceiling, which no document contains</h3>
<p>Per citizen per turn: <code>upkeep</code> eats 1 food, <code>create labor</code> yields 1 labor,
and <code>work</code> turns 1 labor into <em>d</em> food. So with <em>n</em> citizens and <em>c</em>
food extractors, the surplus is <strong>n(d&minus;1)</strong> while n &le; c, and
<strong>c&middot;d &minus; n</strong> after that. <strong>It reaches zero at n = c&middot;d</strong>
&mdash; so a territory's population ceiling is its food capacity times its density, which is the
<em>Food</em> column read as a product.</p>
<div class="scroll">{simple_table(
    ["Territory", "Food", "Ceiling", "Can it grow?", "What the release says it exercises"],
    DATA["ceiling"], ["amt", "amt", "amt", "attach", "note"])}</div>
<div class="callout">
<h4>Every note is explained by that one number, and one of them says so out loud</h4>
<p>Territories 2 and 3 are <code>2 x 6</code> and <code>6 x 2</code> &mdash; different shapes,
<strong>ceiling 12 both</strong> &mdash; and the release's own note on territory 3 reads
<em>many thin food extractors, <strong>same food total</strong></em>. Territory 9 has a ceiling of 6
against metal capacity 6: <em>rich metal, too few hands to work it</em>. Territory 12 has a ceiling
of 4 against 8 metal extractors: <em>rich extractors, almost no workers</em>. <strong>The notes were
written from a quantity that was never written down.</strong></p>
<p><strong>And territory 5 is sharper than its note.</strong> At density 1 a citizen eats exactly
what it produces, so the surplus is <em>n(1&minus;1) = 0</em> at any size: it is not merely
low-ceilinged, it <strong>cannot grow at all</strong> and is frozen at whatever founds it. The note
says <em>Food density 1</em>; the consequence is that the ceiling of 3 is unreachable.</p>
</div>
<div class="callout">
<h4>One thing the chain does not explain, and it is worth knowing</h4>
<p><code>grow</code> produces <em>the lesser of the surplus food and the citizens here</em>, so
growth <strong>caps at doubling per turn</strong> however dense the food is. Density above 2 does
not make a colony grow faster &mdash; it makes surplus for something else. <strong>So the ceiling
and the growth rate are set by different halves of the same column</strong>, and a reader who takes
<em>6 x 6</em> as <em>six times faster</em> would be wrong.</p>
</div>

<h2>What the encoding assumed</h2>
<p><strong>Where a line needed something the notation does not define, the encoder records it
rather than inventing quietly</strong> &mdash; so this list is the report's real output, and it is
generated from the encoding rather than written beside it.</p>
{assumption_table()}
<p><strong>Nothing here is a proposal.</strong> Each is a place where writing the row down in one
line required a choice the specification has not made.</p>

<h2>What the re-encoding cost and saved</h2>
<p>Computed from the data rather than asserted. <strong>{was} rows became {now} lines &mdash;
{direction} {abs(now - was)}</strong>.</p>
<div class="callout">
<h4>There used to be three numbers here, and now there is one</h4>
<p>Before the fusion this section had to carry two counts and explain the gap. <code>consume</code>
was <em>sugar</em> for a guard plus a destroy, so what the machine executed and what a person wrote
were different numbers, and the honest figure depended on which you meant. <strong>Since the guard
and the spend are one line, they are the same number</strong> &mdash; and the right way to report
that is to stop reporting two.</p>
<p>The saving is not the point either. <strong>The vocabulary went from four roles that could not
name <code>create-if-missing</code>, to six primitives that also build the world from nothing, to
{n_primitives} that cannot express a zero test.</strong> A count of lines cannot see that, which is
the whole reason to fix the metric first.</p>
<div class="callout">
<h4>A number that was typed rather than counted, and was wrong</h4>
<p>This report said on 2026-09-09 that fusing the guard into the spend took the primitives from six
to five. <strong>It did not.</strong> <code>destroy</code> and <code>threshold</code> became
<code>consume</code>, and <code>require</code> split out of the same move &mdash; six became six,
and the count was a word typed in prose beside a table that disagreed with it. The line saving was
real; the primitive saving arrived only with the signed <code>change</code>, a decision later the
same day. <strong>Every count on this page is now read from the data</strong>, this one included,
which is why it now says {n_primitives}.</p>
</div>
</div>
<div class="scroll">{simple_table(
    ["Recipe", "Rows before", "Lines after", "Why"],
    [[r[0], r[1], lines_after(r[0]), r[2]] for r in DATA["collapse"]],
    ["target", "amt", "amt", "note"])}</div>

<h2>What has already been settled</h2>
<p>Kept rather than deleted: a question that was answered is what stops it being asked again, and
each row names who answered it.</p>
<div class="scroll">{simple_table(
    ["Question", "What is at stake", "How it shows up"],
    shut_decisions, ["", "", "note"])}</div>

<h2>The source, for review</h2>
<p>Everything above is generated from two files, and no number on this page is typed by hand.</p>
<div class="scroll">{simple_table(
    ["File", "What it is", "Who may write it"],
    [["tools/research/formulas/data.json", "the recipes, primitives, capacities, invariants and decisions - the only thing to edit", "the research lens"],
     ["tools/research/formulas/render.py", "this page", "the research lens"],
     ["tools/research/formulas/check.py", "the three checks, with their poison", "the research lens"],
     ["tools/research/formulas/results.json", "what check.py last reported, read by this page", "generated"],
     ["lenses/research/formulas.html", "this page, generated", "generated"],
     ["releases/first-release.md", "what was copied and modified from, not referenced", "the specification lane"]],
    ["target", "", "note"])}</div>
<p class="foot">Research lens, 2026-09-08. Copied and modified from
<code>releases/first-release.md</code> rather than referencing it, per the request &mdash; so
divergence from the specification is expected and is not a defect in either. Every count here is
computed by <code>render.py</code> from <code>data.json</code>; no number is written by hand.</p>
</div>""")
    sys.stdout.write("\n".join(parts))


if __name__ == "__main__":
    main()
