//! The page itself, assembled from the thirty worlds.
//!
//! **Assembling the page is a function of the library so that a test can read what it
//! produced.** `X-35`: the page drew the first of twenty under a header that promised twenty,
//! and the other nineteen were ten keypresses away - so a correct build was reported as
//! missing. `tests/drawing.rs` now reads the page a person would open, which is where that
//! was invisible.

use crate::{both_families, draw, partner_of};
use graph_coloring::color_graph;

/// The whole page, as the bytes that go on disk.
pub fn page() -> String {
    let mut groups = String::new();
    let mut frames = Vec::new();
    let mut rows = String::new();
    // **Both families, folded first**, because which of them the game should use is Sean's and
    // the question he asked is answered by seeing them against each other. The colour count is
    // no longer asserted here: the axis-aligned family needs four unless three divides `C`,
    // which is a real difference between them and something the page should show.
    let worlds = both_families();
    for (at, torus) in worlds.iter().enumerate() {
        let coloured = color_graph(&torus.adjacency());
        groups.push_str(&draw::group(
            torus,
            &coloured.colors,
            partner_of(&worlds, at),
        ));
        let (x, y, w, h) = draw::extent(torus);
        frames.push(format!("[{x:.1},{y:.1},{w:.1},{h:.1}]"));
        rows.push_str(&format!(
            "<tr><td>{}</td><td>{}</td><td>{}</td><td>{:?}</td><td>{}</td></tr>\n",
            torus.family.name(),
            torus.cells(),
            draw::around(torus),
            coloured.method,
            // **The pairing is in the table too**, so a reader can see which three of the
            // thirty the toggle joins without pressing anything.
            match partner_of(&worlds, at) {
                Some(which) => format!("{} territories", worlds[which].cells()),
                None => "-".to_string(),
            }
        ));
    }
    TEMPLATE
        .replace("{{groups}}", &groups)
        .replace("{{frames}}", &frames.join(","))
        .replace("{{rows}}", &rows)
}

const TEMPLATE: &str = r##"<!doctype html>
<meta charset="utf-8">
<title>hex torus view</title>
<style>
 body {
   margin: 0; font: 14px system-ui, sans-serif; background: #f7f8fa; color: #22303f;
   height: 100vh; display: flex; flex-direction: column;
 }
 /* **The drawing gets the space and everything else is one line or folded away.** Sean,
    2026-09-12: the picture was wedged between a block of prose above it and a table below.
    Both are still here - the prose is what says what the page is for and the table is the
    thirty as numbers - but a `<details>` costs one line each until somebody wants them, and
    the stage takes the rest. **The picker does not fold**, because a control nobody can see
    is `X-35` again. */
 header {
   padding: 6px 12px 5px; border-bottom: 1px solid #dde2e8; background: #fff; flex: none;
 }
 h1 { font-size: 14px; font-weight: 600; margin: 0; }
 p { margin: 4px 0; max-width: 62em; }
 details { font-size: 12px; }
 details summary { cursor: pointer; color: #5a6a7a; padding: 1px 0; }
 details[open] summary { margin-bottom: 2px; }
 footer {
   flex: none; padding: 3px 12px 5px; border-top: 1px solid #dde2e8; background: #fff;
   max-height: 40vh; overflow: auto;
 }
 /* **The stage takes whatever the header and the table leave**, rather than a constant
    that was right for one header. It was `calc(100vh - 190px)`, and adding the picker
    made that number wrong - a height that has to be re-derived whenever the text above it
    changes is a defect waiting for the next edit. */
 #stage { flex: 1; min-height: 0; width: 100%; cursor: grab; display: block; }
 #stage.dragging { cursor: grabbing; }
 .size { display: none; }
 .size.showing { display: inline; }
 text.id { font: 4px system-ui, sans-serif; text-anchor: middle; }
 svg.no-ids text.id { display: none; }
 table { border-collapse: collapse; margin-top: 6px; font-size: 12px; }
 td, th { padding: 1px 10px 1px 0; text-align: right; }
 kbd { background: #eef1f5; border: 1px solid #cfd6de; border-radius: 3px; padding: 0 4px; }
 #where { font-weight: 600; }
 /* **All thirty are listed and clickable, because a control nobody can see is a control
    nobody uses.** `X-35`: the page drew the first of twenty under a header promising twenty,
    and the other nineteen were ten keypresses away behind a picture that reframes to look
    similar - so it read as *nothing changed* and a correct build was reported as missing. */
 #pick { margin-top: 4px; display: flex; gap: 4px; flex-wrap: wrap; align-items: baseline; }
 #pick b { font-weight: 600; margin: 0 4px 0 10px; }
 #pick b:first-child { margin-left: 0; }
 #pick button {
   font: 12px system-ui, sans-serif; padding: 2px 7px; cursor: pointer;
   border: 1px solid #cfd6de; background: #fff; border-radius: 3px; color: #22303f;
 }
 #pick button.here { background: #22303f; border-color: #22303f; color: #fff; }
 #pick button.pair { border-color: #d13b3b; color: #8f2626; }
 #pick button[disabled] { opacity: 0.4; cursor: default; }
 /* **Pointing at a hex lights all seven copies of it, and that is a distance instrument.**
    `X-36`, from Sean: on a torus every territory is drawn many times, so two that look far
    apart in the bright region may be adjacent through a wrap. The drawing shows the copy
    nearest the middle, which does not merely fail to show distance - it misleads about it.
    With every copy lit, the nearest one decides, and the wrap stops being something a person
    has to hold in their head. */
 polygon.lit { stroke: #d13b3b; stroke-width: 2; }
 text.id.lit { font-weight: 700; fill: #8f2626; }
 svg.no-ids text.id.lit { display: inline; }
 #reach { color: #8f2626; font-weight: 600; margin-left: 10px; }
</style>
<header>
 <h1>hex torus view &mdash; <span id="where"></span><span id="reach"></span></h1>
 <nav id="pick"></nav>
 <details id="about">
 <summary>what this shows, and how to drive it</summary>
 <p>One complete world is drawn bright and every other hex is an echo of it. <strong>Point at a
 hex and every copy of that territory lights up</strong> &mdash; the nearest lit copy to
 whatever you are measuring from is the one that decides the distance, because two territories
 that look far apart here may be adjacent through a wrap.</p>
 <p><strong>Thirty worlds are drawn, and the buttons above pick one</strong> &mdash; ten
 folded, ten axis-aligned and ten offset, labelled by how many territories each has.
 <kbd>T</kbd> jumps to the world with the same circumference in the other family, where there
 is one &mdash; same distance around, three times the territories, which is the folding itself.
 <kbd>[</kbd> <kbd>]</kbd> step &middot; <kbd>I</kbd> ids &middot; drag or
 <kbd>&larr;&uarr;&darr;&rarr;</kbd> pan &middot; wheel zooms &middot; <kbd>R</kbd> resets.
 <strong>This settles whether the wrapping is legible and nothing about whether a torus should
 be the world's shape.</strong></p>
 <p><strong>The offset family is not isotropic, and that is the point of it.</strong> It closes
 in <em>2W</em>, <em>H</em>, <em>2W</em> &mdash; one axis takes twice as long as another
 &mdash; and it is the wrap a hex map gets free from being stored as a 2D array. Sean measured
 it in Solium Infernum rather than reasoning about it: twelve up returns to the start, and so
 do twelve alternating rightward steps. <strong>A shipped game he finds perfectly legible does
 not have the equal circumference the other two families were built to satisfy</strong>, so
 what makes the pathing sensible is not isotropy &mdash; it is that the wrap happens in the
 coordinates a person thinks in. <code>offset 144</code> and <code>axis-aligned 144</code> are
 the two to hold against each other. <kbd>X-37</kbd>.</p>
 </details>
</header>
<svg id="stage">{{groups}}</svg>
<script>
const frames = [{{frames}}];
const stage = document.getElementById('stage');
const groups = [...stage.querySelectorAll('.size')];
let at = 0, zoom = 1, panX = 0, panY = 0, ids = true;

// **Every one of the twenty is a button, grouped by family and labelled by its size.** The
// keys still work; what they could not do is say that there is anything to step to.
const pick = document.getElementById('pick');
const buttons = groups.map((g, i) => {
  if (i === 0 || g.dataset.family !== groups[i - 1].dataset.family) {
    const name = document.createElement('b');
    name.textContent = g.dataset.family;
    pick.appendChild(name);
  }
  const button = document.createElement('button');
  button.textContent = g.dataset.cells;
  button.title = `${g.dataset.family}, ${g.dataset.cells} territories, circumference ${g.dataset.around}`;
  button.addEventListener('click', () => { at = i; reset(); });
  pick.appendChild(button);
  return button;
});

// **The toggle pairs by circumference, never by list position.** The two families are one
// lattice at two foldings, so one circumference and two worlds shows the folding: same
// distance around, three times the territories. Two adjacent list entries are unrelated worlds
// and teach nothing. Six of the thirty have a partner; the rest disable the control rather
// than silently doing nothing.
function partner() {
  const which = Number(groups[at].dataset.partner);
  return which >= 0 ? which : null;
}
const fold = document.createElement('button');
fold.id = 'fold';
fold.addEventListener('click', () => { const other = partner(); if (other !== null) { at = other; reset(); } });
pick.appendChild(document.createElement('b')).textContent = 'same circumference';
pick.appendChild(fold);

function show() {
  groups.forEach((g, i) => g.classList.toggle('showing', i === at));
  buttons.forEach((b, i) => b.classList.toggle('here', i === at));
  const other = partner();
  buttons.forEach((b, i) => b.classList.toggle('pair', i === other));
  fold.disabled = other === null;
  fold.className = other === null ? '' : 'pair';
  fold.textContent = other === null
    ? 'no partner at this circumference'
    : `${groups[other].dataset.family}, ${groups[other].dataset.cells} (T)`;
  light(null);
  const [x, y, w, h] = frames[at];
  const cx = x + w / 2, cy = y + h / 2;
  const vw = w / zoom, vh = h / zoom;
  stage.setAttribute('viewBox',
    `${cx - vw / 2 + panX} ${cy - vh / 2 + panY} ${vw} ${vh}`);
  stage.classList.toggle('no-ids', !ids);
  const g = groups[at];
  // **One line, because the drawing gets the space.** What this used to spell out - one bright
  // world and six echoes - is in the folded note below and does not change between worlds.
  document.getElementById('where').textContent =
    `${g.dataset.family}, ${g.dataset.cells} territories, ${g.dataset.around} around`;
}
function reset() { zoom = 1; panX = 0; panY = 0; show(); }

// **Lighting is per territory, not per hex**, which is the whole point: a territory is drawn
// seven times and the hover shows all seven at once. The resting fill travels on each element,
// so letting go puts a dimmed echo back to dim and the bright copy back to bright without the
// page remembering which was which.
let lit = [];
function light(cell) {
  for (const el of lit) {
    el.classList.remove('lit');
    if (el.dataset.fill) el.setAttribute('fill', el.dataset.fill);
  }
  lit = [];
  const reach = document.getElementById('reach');
  if (cell === null) { reach.textContent = ''; return; }
  lit = [...groups[at].querySelectorAll(`[data-cell="${cell}"]`)];
  for (const el of lit) {
    el.classList.add('lit');
    if (el.dataset.full) el.setAttribute('fill', el.dataset.full);
  }
  const places = lit.filter(el => el.tagName === 'polygon').length;
  reach.textContent = `— territory ${cell}, lit in ${places} places`;
}
stage.addEventListener('pointerover', e => {
  if (dragging) return;
  const cell = e.target.dataset && e.target.dataset.cell;
  if (cell !== undefined) light(cell);
});
stage.addEventListener('pointerleave', () => light(null));

addEventListener('keydown', e => {
  const step = frames[at][2] / 20;
  if (e.key === ']') at = Math.min(at + 1, groups.length - 1);
  else if (e.key === '[') at = Math.max(at - 1, 0);
  else if (e.key === 't' || e.key === 'T') { const other = partner(); if (other === null) return; at = other; e.preventDefault(); return reset(); }
  else if (e.key === 'i' || e.key === 'I') ids = !ids;
  else if (e.key === 'r' || e.key === 'R') return reset();
  else if (e.key === 'ArrowLeft') panX -= step;
  else if (e.key === 'ArrowRight') panX += step;
  else if (e.key === 'ArrowUp') panY -= step;
  else if (e.key === 'ArrowDown') panY += step;
  else return;
  e.preventDefault();
  show();
});
stage.addEventListener('wheel', e => {
  e.preventDefault();
  zoom = Math.min(8, Math.max(0.15, zoom * (e.deltaY < 0 ? 1.1 : 1 / 1.1)));
  show();
}, { passive: false });
let dragging = null;
stage.addEventListener('pointerdown', e => {
  dragging = { x: e.clientX, y: e.clientY };
  stage.classList.add('dragging');
  stage.setPointerCapture(e.pointerId);
});
stage.addEventListener('pointermove', e => {
  if (!dragging) return;
  const scale = frames[at][2] / zoom / stage.clientWidth;
  panX -= (e.clientX - dragging.x) * scale;
  panY -= (e.clientY - dragging.y) * scale;
  dragging = { x: e.clientX, y: e.clientY };
  show();
});
addEventListener('pointerup', () => { dragging = null; stage.classList.remove('dragging'); });
show();
</script>
<footer>
<details id="numbers">
<summary>the thirty as numbers &mdash; territories, circumference, colours, and what each pairs with</summary>
<table>
<tr><th>family</th><th>territories</th><th>circumference</th><th>colouring</th><th>same circumference</th></tr>
{{rows}}
</table>
</details>
</footer>
"##;
