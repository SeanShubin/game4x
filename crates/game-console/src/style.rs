//! Two stylesheets, shared by every page - `R-9` and `S-64`.
//!
//! **Sean pointed at a working model rather than describing one**:
//! `vote/generated/code-structure/browse/` is `reset.css`, one stylesheet, and a `<link>` to
//! each from every page. That shape is what this is. The rules are ours - a browsing surface
//! for a 4X game's state has nothing in common with one for a call graph - and the split is
//! his: a reset that says what a browser should stop doing, and one file that says what these
//! pages look like.
//!
//! **Why it is worth doing at all**, since three inline `<style>` blocks rendered the same
//! pixels. They were three copies with two of them subsets of the first, and a rule added to
//! one page silently did not reach the other two - `.quiet a` was in two of the three, and
//! `.empty` meant *an empty cell* in one and *an empty container* in another. **A shared
//! stylesheet makes that collision visible instead of letting it sit in two files.**
//!
//! **Generated like everything else here**, marker and all, so
//! `tests/dumps_are_current.rs` finds them by the same discovery it uses for the pages and a
//! stale stylesheet fails the same way a stale report does. A CSS comment carries the marker
//! because CSS has no doctype.

/// The marker, as a CSS comment, on the first line.
const MARKER: &str = "/* Generated. Do not edit. */\n";

/// What a browser should stop doing.
///
/// **Deliberately small.** A reset that normalises everything is a dependency in disguise -
/// something to maintain against browsers we do not test on. This turns off the four defaults
/// that fight a document made of headings, tables and lists.
const RESET: &str = "\
*, *::before, *::after { box-sizing: border-box }
body, h1, h2, h3, p, ul, ol, li, figure, table { margin: 0; padding: 0 }
ul, ol { list-style: none }
table { border-collapse: collapse }
img, svg { max-width: 100%; display: block }
";

/// What these pages look like.
///
/// **One file for all of them**, so the page-specific rules sit beside the shared ones rather
/// than in a block only that page can see. The tree's rules and the index's rules are here
/// under comments naming which page uses them; a rule nothing uses is then findable, which is
/// the point of them sharing a file.
const REPORT: &str = "\
:root { color-scheme: light dark }

body {
  font: 15px/1.5 ui-monospace, SFMono-Regular, Menlo, monospace;
  margin: 2rem auto;
  max-width: 70rem;
  padding: 0 1rem;
}

h1 { font-size: 1.3rem; margin-bottom: .5rem }
h2 { margin: 2rem 0 .25rem; font-size: 1.1rem }
h3 { margin: 1.25rem 0 .25rem; font-size: 1rem; opacity: .85 }
p { margin: .5rem 0 }

table { margin: .5rem 0 }
th, td { border: 1px solid currentColor; padding: .15rem .5rem; text-align: left }
th { font-weight: 600 }

pre { background: rgba(127,127,127,.12); padding: .6rem .8rem; overflow-x: auto }
ul.bullets { list-style: disc; padding-left: 1.2rem }
ul.bullets li { margin: .1rem 0 }

/* A cell with nothing in it, and a container holding nothing: different sentences, and
   they were two different `.empty` rules in two stylesheets that never met. */
.blank { opacity: .7; font-style: italic }
.count, .note { opacity: .7; font-size: .85rem }
.quiet { opacity: .55; font-size: .85rem }
.quiet a { font-weight: 400 }

/* Every reference is a link - `R-9` - so there are a great many of them, and a link that
   shouts is unreadable at that density. Underlined on hover, coloured always. */
a { color: inherit; text-decoration: underline; text-decoration-style: dotted }
a:hover { text-decoration-style: solid }

/* index.html */
.reports { padding: 0 }
.reports li { margin: .5rem 0 }
.reports a { font-weight: 600; text-decoration: none }
.reports a:hover { text-decoration: underline }
.what { opacity: .75 }
.view {
  font-size: .8rem;
  padding: 0 .35rem;
  border-radius: .2rem;
  background: rgba(127,127,127,.18);
}

/* containment.html */
ul.tree, ul.tree ul { padding-left: 1.1rem; margin: .1rem 0 }
ul.tree > li { margin: .1rem 0 }
summary { cursor: pointer }
summary::marker { opacity: .5 }
li.leaf { opacity: .9 }
.many { font-weight: 600 }
.bounds { font-size: .82rem; opacity: .8 }
.bound { display: inline-block; padding: 0 .3rem; border-radius: .2rem; background: rgba(127,127,127,.14) }
.bound.full { background: rgba(200,120,0,.28) }
.nothing { margin: .1rem 0 .1rem 1.1rem; opacity: .55; font-size: .85rem }

/* territory-N.html */
.traits td:first-child { opacity: .8 }
.neighbours { display: flex; flex-wrap: wrap; gap: .4rem }
.neighbours a { padding: 0 .4rem; border: 1px solid currentColor; border-radius: .2rem }
";

/// The `<link>` elements every page carries, in the order they cascade.
pub fn links() -> String {
    String::from(
        "<link rel=\"stylesheet\" href=\"reset.css\">\n\
         <link rel=\"stylesheet\" href=\"report.css\">\n",
    )
}

/// The stylesheets, as generated files.
pub fn sheets() -> Vec<(&'static str, String)> {
    vec![
        ("reset.css", format!("{MARKER}{RESET}")),
        ("report.css", format!("{MARKER}{REPORT}")),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};

    struct Files(PathBuf);

    impl crate::Library for Files {
        fn fetch(&self, name: &str) -> Option<String> {
            std::fs::read_to_string(self.0.join(format!("{name}.4x"))).ok()
        }
    }

    fn pages() -> String {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        crate::dump::generated(&Files(root.join("scenario/commands")))
            .iter()
            .filter(|(name, _)| name.ends_with(".html"))
            .map(|(_, text)| text.clone())
            .collect()
    }

    /// Every class the stylesheets define, from their selectors.
    fn defined() -> Vec<String> {
        let sheets: String = sheets().iter().map(|(_, text)| text.clone()).collect();
        // **Comments out first.** They talk *about* the rules - `/* index.html */`, and a
        // paragraph naming `.empty` - so scraping them reports classes that no selector
        // declares and no page can use. A scraper that reads prose as code finds work that
        // is not there, which wastes exactly as much attention as missing work that is.
        let mut sheets = sheets;
        while let Some(at) = sheets.find("/*") {
            match sheets[at..].find("*/") {
                Some(end) => sheets.replace_range(at..at + end + 2, ""),
                None => break,
            }
        }
        let mut out: Vec<String> = Vec::new();
        for (at, _) in sheets.match_indices('.') {
            let name: String = sheets[at + 1..]
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '-')
                .collect();
            // A decimal - `.85rem` - is not a class, and neither is a bare dot.
            if name.is_empty() || name.starts_with(|c: char| c.is_ascii_digit()) {
                continue;
            }
            out.push(name);
        }
        out.sort();
        out.dedup();
        out
    }

    /// Every class the pages use, read out of their `class` attributes.
    ///
    /// **Attributes rather than substrings.** `class="bound full"` carries two, and asking
    /// whether the text contains `class="bound"` finds neither - which would report a rule
    /// that is used as unused, and is the shape of wrongness this whole check is about.
    fn used(html: &str) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        for (at, _) in html.match_indices("class=\"") {
            let rest = &html[at + 7..];
            if let Some(end) = rest.find('"') {
                out.extend(rest[..end].split_whitespace().map(String::from));
            }
        }
        out.sort();
        out.dedup();
        out
    }

    /// Every class the stylesheets define is used by some page, and the other way round.
    ///
    /// **This is the check the three inline blocks could not have.** A rule that nothing uses
    /// and a class that nothing styles are the two halves of the drift that made them worth
    /// merging, and neither is visible while the rules live in the file that uses them. Both
    /// directions, over every class, with both populations asserted - two empty sets compare
    /// equal and mean nothing.
    #[test]
    fn every_class_is_both_defined_and_used() {
        let defined = defined();
        let used = used(&pages());
        assert!(
            defined.len() > 5 && used.len() > 5,
            "{} classes defined and {} used, which is too few for the comparison to mean \
             anything",
            defined.len(),
            used.len()
        );

        let unstyled: Vec<&String> = used.iter().filter(|it| !defined.contains(it)).collect();
        assert!(
            unstyled.is_empty(),
            "these classes are used by a page and no stylesheet defines them: {unstyled:?}"
        );
        let unused: Vec<&String> = defined.iter().filter(|it| !used.contains(it)).collect();
        assert!(
            unused.is_empty(),
            "these classes are defined and no page uses them: {unused:?}"
        );
    }
}
