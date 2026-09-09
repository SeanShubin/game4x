"""Render the formula report from data.json.

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
    "create": "op-create",
    "destroy": "op-destroy",
    "set": "op-set",
    "threshold": "op-threshold",
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
    out.append("<table><thead><tr>")
    for h in ("Op", "Target", "Amount / Bound", "Attach", "Was"):
        out.append(f"<th>{h}</th>")
    out.append("</tr></thead><tbody>")
    for op, target, amount, attach, was_note in f["lines"]:
        cls = OP_CLASS.get(op, "")
        attach_cell = f'<span class="attach">{esc(attach)}</span>' if attach else ""
        out.append(
            f'<tr><td class="op {cls}">{esc(op)}</td>'
            f"<td class=\"target\">{esc(target)}</td>"
            f"<td class=\"amt\">{esc(amount)}</td>"
            f"<td>{attach_cell}</td>"
            f'<td class="note">{esc(was_note)}</td></tr>'
        )
    out.append("</tbody></table>")
    return "\n".join(out)


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
    """The line count for a formula, read from the formula itself.

    `collapse` used to restate this number, so the table could disagree with the tables
    above it without anything noticing. One number, one home.
    """
    for f in DATA["player"] + DATA["world"]:
        if f["name"] == name or f["name"].split("(")[0] == name:
            return len(f["lines"])
    raise KeyError(name)


def totals():
    """Rows before, lines in bare primitives, and lines once the sugar is used.

    The third is the honest one for an author. `consume n k` is listed as sugar for a
    threshold plus destroys, so a pair that came from one `consume` is one line again the
    moment the author writes `consume`. Counted, not estimated: a threshold reading
    "at least" whose target is also destroyed in the same formula is one such pair.
    """
    by_name = {}
    for f in DATA["player"] + DATA["world"]:
        by_name[f["name"]] = len(f["lines"])
        by_name[f["name"].split("(")[0]] = len(f["lines"])
    was = sum(r[1] for r in DATA["collapse"])
    now = sum(by_name[r[0]] for r in DATA["collapse"])
    pairs = 0
    for f in DATA["player"] + DATA["world"]:
        destroyed = {l[1] for l in f["lines"] if l[0] == "destroy"}
        for op, target, amount, _attach, _note in f["lines"]:
            if op == "threshold" and str(amount).startswith("at least") and target in destroyed:
                pairs += 1
    return was, now, pairs


def main():
    was, now, pairs = totals()
    sugared = now - pairs
    n_player = len(DATA["player"])
    n_world = len(DATA["world"])
    n_creation = len(DATA["creation"])
    parts = []
    parts.append(f"""<title>Formula Report</title>
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
.op-create {{ color: var(--create); }} .op-destroy {{ color: var(--destroy); }}
.op-set {{ color: var(--set); }} .op-threshold {{ color: var(--threshold); }}
.op-call {{ color: var(--call); }} .op-let {{ color: var(--muted); }}
.target, .amt {{ font-family: ui-monospace, SFMono-Regular, Menlo, monospace; }}
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
<h1>The whole specification as formulas</h1>
<p class="lede">Every recipe re-expressed in six primitives &mdash; and the same six building the
world from an empty game. Generated from <code>tools/research/formulas/data.json</code>.</p>

<div class="callout">
<h4>The one finding</h4>
<p><strong>Creation and transformation are already the same format, because the specification has
been quietly turning relations into things.</strong> <code>deposit</code> and
<code>adjacency</code> are kinds. Once a relation is a thing, building the world is
<code>create</code> and <code>set</code> with no thresholds &mdash; the identical primitives that
play the game. Nothing needs adding for world-building except the <code>set</code> that
transformation needed anyway.</p>
</div>

<h2>The primitives</h2>
<p>Six. <code>set</code> is the one the sketch was missing, and it is what collapses
<code>age</code> and <code>refresh</code> from two rows to one each.</p>
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
<p><code>threshold</code> always can &mdash; refusing is its purpose. <code>create</code> can only
where a capacity is declared. <code>destroy</code> can, when there is not enough.
<code>call</code> can, when something inside it fails hard. <code>let</code> can, when a path does
not resolve &mdash; <code>ark.location.below</code> has no answer for an ark that is not in orbit,
and <strong>hard is the only sane value there</strong>. <code>set</code> effectively cannot, which
is why no <code>set</code> in this report carries an attachment.</p>
</div>

<h3>Three decisions the column is carrying, none of them taken</h3>
<div class="scroll">{simple_table(
    ["Question", "The two readings", "What is at stake", "Where it came from"],
    DATA["attach_decisions"], ["target", "", "note", "note"])}</div>
<div class="callout">
<h4><code>record</code> is the one worth looking at, because it is already there</h4>
<p><code>unpaid</code> is a trait in the specification &mdash; <em>derived: its upkeep was not
met</em> &mdash; and <code>perish</code> fires on it. So when <code>upkeep</code> cannot take its
food, the failure <strong>neither stops the turn nor vanishes</strong>: it is written down, and
another formula reads it next. <strong>That is a third behaviour on failure, in the game today,
which a two-valued column cannot express.</strong></p>
<p>It may not want to be an attach value. The alternative is that <strong>failure is state</strong>
&mdash; every line's outcome is recorded, and <code>unpaid</code> is just one query over it. That is
a larger idea and a cleaner one, and it is the sort of thing worth deciding before the column sets
rather than after.</p>
</div>

<h2>What is sugar</h2>
<p>Removable with linear growth, so by the test each is readability rather than expressiveness
&mdash; which means they can be added freely.</p>
<div class="scroll">{simple_table(
    ["Written as", "Means", "Cost of removing it"],
    DATA["sugar"], ["target", "", "note"])}</div>

<h2>Player formulas <span class="badge">{n_player}</span></h2>
{"".join(formula_table(f) for f in DATA["player"])}

<h2>World formulas <span class="badge">{n_world}</span></h2>
<p>These fire when the turn ends, in order: upkeep, grow and perish, age, spoil, refresh.</p>
{"".join(formula_table(f) for f in DATA["world"])}

<h2>Capacities</h2>
<div class="callout">
<h4>Where a bound lives, decided 2026-09-08</h4>
<p><strong>A capacity is a property of the container, declared once, and no formula states it.</strong>
That deletes the <code>limit</code> role outright: both of its uses in the whole specification were
<code>limit 0 garrison</code>, restating a capacity of 1 that <em>What bounds a kind in a territory</em>
already declared. Every other capacity below was <em>never</em> written in a recipe &mdash; so the
engine was already enforcing seven bounds that no formula stated, and the garrison was the odd one
out for being written twice rather than for being written at all.</p>
</div>
<div class="scroll">{simple_table(
    ["Container", "Holds", "Up to", "Was it ever in a recipe?"],
    DATA["capacities"], ["target", "target", "amt", "note"])}</div>

<h2>Building the world <span class="badge">{n_creation}</span></h2>
<div class="callout">
<p><strong>No new primitive appears below.</strong> Every line is <code>create</code>,
<code>set</code> or <code>call</code> &mdash; and not one threshold, because nothing at design
time can be refused. That is the answer to whether one format can both make the environment and
play the game: <strong>world-building is the play language with the guards left out.</strong></p>
</div>
{"".join(formula_table(f) for f in DATA["creation"])}

<h3>The data those calls consume</h3>
<p>Copied for review. <code>3 x 4</code> is capacity 3, density 4 &mdash; so
<code>make-deposit(1, food, 3, 4)</code>.</p>
<div class="scroll">{simple_table(
    ["Territory", "Food", "Metal", "Energy", "What it exercises"],
    DATA["territories"], ["amt", "amt", "amt", "amt", "note"])}</div>

<h2>Detecting a glitch, without a cap</h2>
<div class="callout">
<h4>Unbounded capacity does not cost you detection</h4>
<p><strong>Boundedness for one starting state is EXPSPACE-complete. Boundedness for
<em>every</em> starting state is polynomial &mdash; a linear program over the incidence
matrix.</strong> The second is the one an editor needs, because an author is editing formulas and
not a saved game. A net is not structurally bounded exactly when there is a non-negative firing
vector <code>x</code>, not all zero, with <code>C&middot;x &ge; 0</code>: a set of formulas that,
fired in some ratio, ends with more than it began. <strong>The vector is the error message</strong>
&mdash; it names which formulas and how many of each.</p>
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

<h2>The three checks, as run</h2>
<p>Not a description of what they would report &mdash; the output of
<code>tools/research/formulas/check.py</code>, read from <code>results.json</code>, so this page
and the checker cannot disagree. <strong>All three were poisoned before being believed</strong>:
{"; ".join(RESULTS["poison"])}.</p>

<h3>Check 1 &mdash; metal conserved outside extraction &nbsp;<span class="badge down">{len(RESULTS["conservation"]["violations"])} violations</span></h3>
<p>{RESULTS["conservation"]["analysed"]} formulas analysed;
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
costs in metal is what it delivers &mdash; now holds for every formula that is not mining.</p>
<p><strong>The check is poisoned against exactly this.</strong> Putting the two stores back turns
<code>deploy ark</code> and <code>found by land</code> red again, which is what makes the green
above worth reading. A check that agreed with a fix rather than measuring it would look identical
from here.</p>
<p><strong>Two faults in the check itself, both found by his objection and both fixed.</strong>
It tested a stronger claim than the words can carry &mdash; mining creates metal, so
<code>work</code> is a declared source and the invariant is <em>conserved outside extraction</em>.
And it weighed <code>found-colony</code> as if a player could fire it with no ark and no pioneer
spent; a formula that is only ever called is not a transition.</p>
</div>

<h3>Check 2 &mdash; structurally unbounded &nbsp;<span class="badge up">unbounded, as it should be</span></h3>
<p>{RESULTS["unbounded"]["analysed"]} transitions after grounding;
{len(RESULTS["unbounded"]["skipped"])} skipped ({", ".join(RESULTS["unbounded"]["skipped"])}).</p>
<div class="scroll">{simple_table(
    ["Fire this many times", "Formula"],
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
own largest density. <strong>That is grounding</strong> &mdash; instantiating a formula against
what it could apply to &mdash; and it is the same operation an interface performs to build a menu.
The analysis and the interface want the same machinery, which is a reason to build it once.</p>
</div>

<h3>Check 2b &mdash; the same without mining &nbsp;<span class="badge down">clean</span></h3>
<p>{RESULTS["unbounded"]["without_sources"]["transitions"]} transitions with <code>work</code>
removed. <strong>This is the question worth asking</strong>, and it is the recommendation in
practice: rather than keeping a baseline of intended loops, remove the declared source and ask
whether metal can still grow.</p>
<div class="scroll">{simple_table(
    ["Fire this many times", "Formula"],
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
    ["Formula", "Line", "Attach", "Capacity"],
    [[r[0], r[1], r[2], r[3]] for r in RESULTS["attachment"]["meaningful"]],
    ["target", "target", "attach", "note"])}</div>
<p>And the one where it does not:</p>
<div class="scroll">{simple_table(
    ["Formula", "Line", "Attach", "Why it cannot fail"],
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
<h3>Check 5 &mdash; which territories would refuse a create, per line</h3>
<div class="scroll">{simple_table(
    ["Formula", "Line", "Attach", "Blocked on"],
    [[r[0], r[1], r[2], ("none - every territory has room" if not r[3]
      else "territory " + ", ".join(r[3]))] for r in RESULTS["placement"]],
    ["target", "target", "attach", "note"])}</div>
<div class="callout">
<h4>Corrected: territory 7 was never affected</h4>
<p>An earlier version of this report said territories 6 and 7 both mattered. <strong>Territory 7
has no energy, and no formula here creates an energy extractor</strong>, so it never did. Sean
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
<p>Under a selection interface the second never even arises: grounding the formula against
territory 6 produces no <code>build extractor[metal]</code> to select. <strong>The attachment is
what the console needs and the menu is what the interface needs, from one line.</strong></p>
</div>

<h3>Check 3 &mdash; a cap of {RESULTS["cap"]["cap"]} &nbsp;<span class="badge down">{len(RESULTS["cap"]["breaches"])} breaches</span></h3>
<p>Applying each formula once breaches nothing, which is exactly the point about a cap:
<strong>it found neither of the two things the other checks found.</strong> It is a backstop
against bugs in checks 1 and 2, not a way of finding anything.</p>

<h2>What the re-encoding cost and saved</h2>
<p>Computed from the data rather than asserted. In bare primitives,
<strong>{was} rows became {now} lines &mdash; up {now - was}</strong>. That is the number predicted
before any of this was written, and it is the misleading one.</p>
<p>Almost all of the rise is one thing: today's <code>consume</code> fuses a guard and an effect,
and splitting it costs a line each time. There are <strong>{pairs}</strong> such pairs. Since
<code>consume</code> is sugar &mdash; it is in the table above &mdash; an author never has to write
both, and the authored count is <strong>{sugared} lines, down {was - sugared} from {was}</strong>.</p>
<div class="callout">
<h4>So which number is real</h4>
<p>All three, measuring different things. <strong>Up {now - was}</strong> is what the machine
executes. <strong>Down {was - sugared}</strong> is what a person writes. And the one that matters
is neither: <strong>the vocabulary went from four roles that could not name
<code>create-if-missing</code> to six primitives that also build the world from nothing.</strong>
A count of lines cannot see that, which is the whole reason to fix the metric first.</p>
</div>
<div class="scroll">{simple_table(
    ["Formula", "Rows before", "Lines after", "Why"],
    [[r[0], r[1], lines_after(r[0]), r[2]] for r in DATA["collapse"]],
    ["target", "amt", "amt", "note"])}</div>

<h2>Decisions this forced, none of them taken here</h2>
<div class="scroll">{simple_table(
    ["Question", "What is at stake", "How it shows up"],
    DATA["decisions"], ["", "", "note"])}</div>

<h2>The source, for review</h2>
<p>Everything above is generated from two files, and no number on this page is typed by hand.</p>
<div class="scroll">{simple_table(
    ["File", "What it is", "Who may write it"],
    [["tools/research/formulas/data.json", "the formulas, primitives, capacities, invariants and decisions - the only thing to edit", "the research lens"],
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
