//! Writes `drawing/index.html` - ten worlds to step through, and the numbers beside them.
//!
//! **A page rather than a window**, which is `gap-view`'s shape and not `goldberg-view`'s. The
//! deliverable is the answer to a question, and a file Sean can open without a build reaches
//! him more reliably than a binary he has to run. The controls `X-32` asks for are all here;
//! they are key handlers in a page instead of key handlers in `bevy`.
//!
//! Pass a directory to write somewhere else.

use graph_coloring::color_graph;
use hex_torus_view::{draw, sizes};

fn main() {
    let mut arguments = std::env::args().skip(1);
    let into = arguments
        .next()
        .unwrap_or_else(|| format!("{}/drawing", env!("CARGO_MANIFEST_DIR")));
    std::fs::create_dir_all(&into).unwrap_or_else(|why| panic!("cannot make {into}: {why}"));

    let mut groups = String::new();
    let mut frames = Vec::new();
    let mut rows = String::new();
    for torus in sizes() {
        let coloured = color_graph(&torus.adjacency());
        assert_eq!(
            coloured.color_count, 3,
            "k = {}: {} colours, and this page says three",
            torus.k, coloured.color_count
        );
        groups.push_str(&draw::group(&torus, &coloured.colors));
        let (x, y, w, h) = draw::extent(&torus);
        frames.push(format!("[{x:.1},{y:.1},{w:.1},{h:.1}]"));
        rows.push_str(&format!(
            "<tr><td>{}</td><td>{}</td><td>{}</td><td>{:?}</td></tr>\n",
            torus.k,
            torus.cells(),
            torus.circumference(),
            coloured.method
        ));
    }

    let page = PAGE
        .replace("{{groups}}", &groups)
        .replace("{{frames}}", &frames.join(","))
        .replace("{{rows}}", &rows);
    let at = format!("{into}/index.html");
    std::fs::write(&at, page).unwrap_or_else(|why| panic!("cannot write {at}: {why}"));
    println!("wrote {at}");
    println!();
    println!("Open it. At each size, zoomed out: exactly N hexes are undimmed, and every");
    println!("dimmed hex shows the id of an undimmed one. That is what `X-32` asks you to see.");
    println!();
    println!("[ and ] step the sizes, I toggles ids, drag or the arrows pan, the wheel zooms,");
    println!("R resets.");
}

const PAGE: &str = r##"<!doctype html>
<meta charset="utf-8">
<title>hex torus view</title>
<style>
 body { margin: 0; font: 14px system-ui, sans-serif; background: #f7f8fa; color: #22303f; }
 header { padding: 12px 16px; border-bottom: 1px solid #dde2e8; background: #fff; }
 h1 { font-size: 16px; margin: 0 0 4px; }
 p { margin: 4px 0; max-width: 62em; }
 #stage { width: 100vw; height: calc(100vh - 190px); cursor: grab; display: block; }
 #stage.dragging { cursor: grabbing; }
 .size { display: none; }
 .size.showing { display: inline; }
 text.id { font: 4px system-ui, sans-serif; text-anchor: middle; }
 svg.no-ids text.id { display: none; }
 table { border-collapse: collapse; margin-top: 6px; font-size: 12px; }
 td, th { padding: 1px 10px 1px 0; text-align: right; }
 kbd { background: #eef1f5; border: 1px solid #cfd6de; border-radius: 3px; padding: 0 4px; }
 #where { font-weight: 600; }
</style>
<header>
 <h1>hex torus view &mdash; <span id="where"></span></h1>
 <p>One complete world is drawn bright and every other hex is an echo of it. <strong>Turn the
 ids on and watch the same number appear in every direction</strong> &mdash; that is the same
 territory again, not more world.</p>
 <p><kbd>[</kbd> <kbd>]</kbd> step the ten sizes &middot; <kbd>I</kbd> ids &middot; drag or
 <kbd>&larr;&uarr;&darr;&rarr;</kbd> pan &middot; wheel zooms &middot; <kbd>R</kbd> resets.
 <strong>This settles whether the wrapping is legible and nothing about whether a torus should
 be the world's shape.</strong></p>
</header>
<svg id="stage">{{groups}}</svg>
<script>
const frames = [{{frames}}];
const stage = document.getElementById('stage');
const groups = [...stage.querySelectorAll('.size')];
let at = 0, zoom = 1, panX = 0, panY = 0, ids = true;

function show() {
  groups.forEach((g, i) => g.classList.toggle('showing', i === at));
  const [x, y, w, h] = frames[at];
  const cx = x + w / 2, cy = y + h / 2;
  const vw = w / zoom, vh = h / zoom;
  stage.setAttribute('viewBox',
    `${cx - vw / 2 + panX} ${cy - vh / 2 + panY} ${vw} ${vh}`);
  stage.classList.toggle('no-ids', !ids);
  const g = groups[at];
  document.getElementById('where').textContent =
    `${g.dataset.cells} territories, k = ${g.dataset.k}, ${g.dataset.cells / 1} bright and six echoes`;
}
function reset() { zoom = 1; panX = 0; panY = 0; show(); }

addEventListener('keydown', e => {
  const step = frames[at][2] / 20;
  if (e.key === ']') at = Math.min(at + 1, groups.length - 1);
  else if (e.key === '[') at = Math.max(at - 1, 0);
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
<div style="padding: 8px 16px">
<table>
<tr><th>k</th><th>territories</th><th>circumference</th><th>colouring</th></tr>
{{rows}}
</table>
</div>
"##;
