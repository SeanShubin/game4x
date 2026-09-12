/* The game's data, edited by choosing.
 *
 * **The rule Sean set is enforced by the schema, not by discipline.** Every column declares
 * how it is chosen, and only two of the declared types put a caret in front of a person:
 * `text`, which is a thing's name and is typed once when it is created, and `prose`, which is
 * free text and is drawn in the warning colour with a dashed border wherever it appears. Every
 * other type renders a select, a stepper or a toggle. So "what still has to be typed" is a
 * query over this file rather than a thing to remember, and the count is on the page.
 *
 * Vocabularies are FUNCTIONS of the live state, never lists written here. Add a kind and it is
 * immediately choosable everywhere a kind can be chosen - which is the property that makes an
 * editor a test of the data rather than a picture of it.
 */

"use strict";

const ORIGINAL = JSON.stringify(window.GAME);
let game = JSON.parse(ORIGINAL);
let section = "recipes";
let dirty = false;

/* ---------- vocabularies, derived ---------- */

const BLANK = "—"; // an em dash, meaning "this row has no such value"

const V = {
  kinds: () => game.kinds.map((k) => k.name),
  families: () => game.families.map((f) => f.name),
  kindOrFamily: () => [...V.kinds(), ...V.families()],
  resources: () => (game.families.find((f) => f.name === "resource") || { members: [] }).members,
  biomes: () => game.biomes.map((b) => b.name),
  traits: () => game.traits.map((t) => t.name),
  recipes: () => game.recipes.map((r) => r.name),
  things: () => game.things.map((t) => t.name),
  wheres: () => game.wheres,
  quantityExpressions: () => game.quantityExpressions,
  roles: () => ["require", "limit", "consume", "produce", "put"],
  owners: () => ["player", "world"],
  storages: () => ["stored", "derived", "of the kind"],
  comparators: () => ["is", "at least", "at most", "at its maximum", "one less"],
  crosses: () => ["border", "orbit border"],
  requires: () => ["a Yard"],
};

/* The vocabularies the release itself uses for the columns that read as prose. Offered as
 * choices, with "something else" always available and always marked - so editing what exists
 * is all selection, and needing a phrase the game has never used is visible rather than free. */
for (const [name, values] of Object.entries(game.vocabularies)) {
  V[name] = () => game.vocabularies[name];
}

const SOMETHING_ELSE = "something else…";

/* Which fields a person has asked to write freely in. Keyed by section, row and column. */
const writing = new Set();

/* A comparator that carries its own value needs no value chosen beside it. */
const SELF_VALUED = new Set(["at its maximum", "one less"]);

/* ---------- the schema ---------- */

const SCHEMA = [
  {
    key: "recipes",
    title: "Recipes",
    about:
      "Every rule in the game. A row is a role, a quantity, a kind and what must be true of " +
      "it. Nothing here is typed but a new recipe's name.",
    render: renderRecipes,
    count: () => game.recipes.length,
  },
  {
    key: "kinds",
    title: "Kinds",
    about: "The things that exist. A kind's description is prose and is the one field here that is not a choice.",
    count: () => game.kinds.length,
    columns: [
      { key: "name", label: "Kind", type: "name" },
      { key: "what", label: "What it is", type: "prose" },
    ],
  },
  {
    key: "families",
    title: "Families",
    about: "A name for several kinds at once. Members are chosen from the kinds.",
    count: () => game.families.length,
    columns: [
      { key: "name", label: "Family", type: "name" },
      { key: "members", label: "Members", type: "picks", of: "kinds", when: (r) => !r.everyKind },
      { key: "everyKind", label: "Every kind", type: "bool" },
    ],
  },
  {
    key: "traits",
    title: "Traits",
    about:
      "What a thing can have. Its subject, its value shape and its derivation are each chosen " +
      "from the phrases the release already uses - or written, which marks them.",
    count: () => game.traits.length,
    columns: [
      { key: "name", label: "Trait", type: "name" },
      { key: "of", label: "Of", type: "known", of: "traitSubjects" },
      { key: "values", label: "Values", type: "known", of: "traitValues" },
      { key: "storage", label: "Stored or derived", type: "pick", of: "storages" },
      {
        key: "derivation",
        label: "Derived by",
        type: "known",
        of: "derivations",
        when: (r) => r.storage === "derived",
      },
    ],
  },
  {
    key: "things",
    title: "Units and structures",
    about: "The numbers each built thing carries. Every one of them is a stepper or a toggle.",
    count: () => game.things.length,
    columns: [
      { key: "name", label: "Thing", type: "name" },
      { key: "strength", label: "Strength", type: "number" },
      { key: "fuel", label: "Fuel", type: "number" },
      { key: "upkeep", label: "Upkeep", type: "amounts", of: "kinds" },
      { key: "costs", label: "Costs to produce", type: "amounts", of: "kinds" },
      { key: "binding", label: "Binding", type: "number" },
      { key: "crosses", label: "Crosses", type: "pick", of: "crosses", optional: true },
      { key: "requires", label: "Requires", type: "pick", of: "requires", optional: true },
      { key: "readies", label: "Readies", type: "bool" },
      { key: "movable", label: "Movable", type: "bool" },
    ],
  },
  {
    key: "territories",
    title: "Territory resources",
    about:
      "The twelve territories. Each resource is a total capacity and a density, both chosen; " +
      "what a territory exercises is a note to a reader rather than data the game runs on.",
    count: () => game.territories.length,
    columns: [
      { key: "id", label: "Territory", type: "number" },
      { key: "food", label: "Food", type: "capdens" },
      { key: "metal", label: "Metal", type: "capdens" },
      { key: "energy", label: "Energy", type: "capdens" },
      { key: "exercises", label: "What it exercises", type: "prose" },
    ],
  },
  {
    key: "biomes",
    title: "Biomes",
    about: "What a biome is like. The numbers guide; force of nature is the one that binds.",
    count: () => game.biomes.length,
    columns: [
      { key: "name", label: "Biome", type: "name" },
      { key: "food", label: "Food", type: "capdens" },
      { key: "metal", label: "Metal", type: "capdens" },
      { key: "energy", label: "Energy", type: "capdens" },
      { key: "nature", label: "Force of nature", type: "number" },
    ],
  },
  {
    key: "bounds",
    title: "What bounds a kind",
    about:
      "What stops there being more of a kind in a territory. The reason is chosen from the " +
      "reasons the game already gives.",
    count: () => game.bounds.length,
    columns: [
      { key: "kind", label: "Kind", type: "pick", of: "kindOrFamily" },
      { key: "boundedBy", label: "Bounded by", type: "known", of: "boundReasons" },
    ],
  },
  {
    key: "containers",
    title: "Where things are",
    about:
      "What holds what, and how much of it. Three columns, each chosen from what the release " +
      "already says.",
    count: () => game.containers.length,
    columns: [
      { key: "container", label: "Container", type: "known", of: "containerNames" },
      { key: "holds", label: "Holds", type: "known", of: "containerHolds" },
      { key: "upTo", label: "Up to", type: "known", of: "containerUpTo" },
    ],
  },
  {
    key: "worldOrder",
    title: "Firing order",
    about:
      "The order the world's recipes fire when a turn ends. Each is chosen from the recipes " +
      "that exist, so an order cannot name one that does not.",
    count: () => game.worldOrder.length,
    render: renderOrder,
  },
  {
    key: "loop",
    title: "The loop",
    about: "The game from no presence to a launched ark. Prose, and a statement of intent rather than data.",
    count: () => game.loop.length,
    render: renderLoop,
  },
];

/* ---------- small helpers ---------- */

function el(tag, props = {}, ...children) {
  const node = document.createElement(tag);
  for (const [key, value] of Object.entries(props)) {
    if (key === "class") node.className = value;
    else if (key === "text") node.textContent = value;
    else if (key.startsWith("on")) node.addEventListener(key.slice(2), value);
    else if (value === true) node.setAttribute(key, "");
    else if (value !== false && value != null) node.setAttribute(key, value);
  }
  for (const child of children.flat()) {
    if (child == null) continue;
    node.append(child.nodeType ? child : document.createTextNode(child));
  }
  return node;
}

function touched() {
  dirty = true;
  document.getElementById("dirty").hidden = false;
}

/* ---------- field renderers ---------- */

/**
 * One of the phrases the release already uses, or something it has never used.
 *
 * **The second option is the measurement.** A person editing what is there never needs it; a
 * person inventing a subject, a value-shape or a reason the game has no word for does, and it
 * is drawn as typing because that is what it is. Counted on the Recipes page.
 */
function pickOrProse(value, options, onChange, key) {
  // **Held in a set rather than inferred from the value**, because an empty value is what
  // choosing "something else" starts from, and an empty value also means *nothing chosen*.
  // Inferring made the two indistinguishable and the field could never enter prose at all.
  const typing = writing.has(key) || (value && !options.includes(value));
  return el(
    "span",
    { class: "row" },
    pick(typing ? SOMETHING_ELSE : value, [...options, SOMETHING_ELSE], (chosen) => {
      if (chosen === SOMETHING_ELSE) {
        writing.add(key);
        onChange("");
      } else {
        writing.delete(key);
        onChange(chosen);
      }
    }, true),
    typing
      ? el("input", {
          class: "prose",
          type: "text",
          value: value || "",
          placeholder: "a phrase the release has never used",
          oninput: (event) => { onChange(event.target.value); touched(); },
        })
      : null
  );
}

/** A closed set, offered as a select. The only way a value ever changes here. */
function pick(value, options, onChange, optional) {
  const node = el("select", {
    onchange: (event) => {
      const chosen = event.target.value;
      onChange(chosen === BLANK ? "" : chosen);
      touched();
      draw();
    },
  });
  const all = optional ? [BLANK, ...options] : options;
  // A value already in the data that is not in the vocabulary is kept and shown, rather than
  // silently replaced by the first option - which is how an editor quietly rewrites data.
  if (value && !all.includes(value)) all.unshift(value);
  for (const option of all) {
    node.append(el("option", { value: option, selected: option === (value || BLANK) }, option));
  }
  return node;
}

/** Several from a closed set, as a row of toggles - a checkbox list without the checkboxes. */
function picks(values, options, onChange) {
  return el(
    "div",
    { class: "row" },
    options.map((option) => {
      const on = values.includes(option);
      return el(
        "button",
        {
          class: "toggle",
          "aria-pressed": String(on),
          style: on ? "border-color:var(--accent)" : "opacity:.55",
          onclick: () => {
            onChange(on ? values.filter((v) => v !== option) : [...values, option]);
            touched();
            draw();
          },
        },
        option
      );
    })
  );
}

/** A whole number, stepped rather than typed. Blank is a state, and is not zero. */
function number(value, onChange, { min = 0, allowBlank = true } = {}) {
  const has = value !== null && value !== undefined && value !== "";
  const step = (by) => {
    if (!has) return onChange(min);
    const next = Number(value) + by;
    if (next < min) return onChange(allowBlank ? null : min);
    onChange(next);
  };
  return el(
    "span",
    { class: "stepper" },
    el("button", { title: "less", onclick: () => { step(-1); touched(); draw(); } }, "−"),
    el("span", { class: has ? "value" : "value blank" }, has ? String(value) : BLANK),
    el("button", { title: "more", onclick: () => { step(1); touched(); draw(); } }, "+")
  );
}

/** Yes or no. */
function bool(value, onChange) {
  return el(
    "button",
    {
      class: "toggle",
      "aria-pressed": String(!!value),
      style: value ? "border-color:var(--accent)" : "opacity:.55",
      onclick: () => { onChange(!value); touched(); draw(); },
    },
    value ? "yes" : "no"
  );
}

/** A list of quantity-and-kind pairs: `1 labor, 1 metal`. */
function amounts(list, vocabulary, onChange) {
  return el(
    "div",
    { class: "rows" },
    list.map((item, index) =>
      el(
        "span",
        { class: "row" },
        number(item.qty, (v) => { list[index].qty = v; onChange(list); }, { min: 1, allowBlank: false }),
        pick(item.kind, vocabulary, (v) => { list[index].kind = v; onChange(list); }),
        el("button", {
          class: "drop",
          title: "remove",
          onclick: () => { list.splice(index, 1); onChange(list); touched(); draw(); },
        }, "×")
      )
    ),
    el("button", {
      class: "add",
      onclick: () => {
        list.push({ qty: 1, kind: vocabulary[0] });
        onChange(list);
        touched();
        draw();
      },
    }, "+ add")
  );
}

/** A total capacity and a density, or nothing at all. */
function capdens(value, onChange) {
  if (!value) {
    return el("button", {
      class: "add",
      onclick: () => { onChange({ capacity: 1, density: 1 }); touched(); draw(); },
    }, "none — give it some");
  }
  return el(
    "span",
    { class: "row" },
    number(value.capacity, (v) => { onChange({ ...value, capacity: v }); }, { min: 1, allowBlank: false }),
    el("span", { class: "note" }, "×"),
    number(value.density, (v) => { onChange({ ...value, density: v }); }, { min: 1, allowBlank: false }),
    el("button", { class: "drop", title: "none", onclick: () => { onChange(null); touched(); draw(); } }, "×")
  );
}

/**
 * What must be true of a thing: a trait, a comparator, and - unless the comparator carries
 * its own - a value. Three selections where the release writes one phrase.
 */
function constraint(row, onChange) {
  const current = row.constraint;
  if (!current) {
    return el(
      "span",
      { class: "row" },
      row.traits
        ? el("input", {
            class: "prose",
            type: "text",
            value: row.traits,
            title: "No selection can express this",
            oninput: (event) => { row.traits = event.target.value; touched(); },
          })
        : null,
      el("button", {
        class: "add",
        onclick: () => {
          row.constraint = { trait: V.traits()[0], compare: "at least", value: 1 };
          row.traits = "";
          touched();
          draw();
        },
      }, row.traits ? "replace with a choice" : "+ constrain")
    );
  }
  const valued = !SELF_VALUED.has(current.compare);
  const trait = game.traits.find((t) => t.name === current.trait);
  const closed = trait && V.resources().length && /one of the resources/.test(trait.values || "");
  return el(
    "span",
    { class: "row" },
    pick(current.trait, V.traits(), (v) => { current.trait = v; }),
    pick(current.compare, V.comparators(), (v) => {
      current.compare = v;
      current.value = SELF_VALUED.has(v) ? null : (current.value ?? 1);
    }),
    valued
      ? closed
        ? pick(String(current.value), V.resources(), (v) => { current.value = v; })
        : number(current.value, (v) => { current.value = v; }, { min: 0, allowBlank: false })
      : null,
    el("button", {
      class: "drop",
      title: "no constraint",
      onclick: () => { row.constraint = null; row.traits = ""; touched(); draw(); },
    }, "×")
  );
}

/** A quantity: a whole number, or one of the release's own expressions. */
function quantity(row) {
  const isExpression = row.qty && !/^\d+$/.test(row.qty);
  return el(
    "span",
    { class: "row" },
    pick(isExpression ? "an expression" : "a number", ["a number", "an expression"], (mode) => {
      row.qty = mode === "an expression" ? V.quantityExpressions()[0] : "1";
    }),
    isExpression
      ? pick(row.qty, V.quantityExpressions(), (v) => { row.qty = v; })
      : number(row.qty === "" ? null : Number(row.qty), (v) => {
          row.qty = v === null ? "" : String(v);
        }, { min: 0 })
  );
}

/* ---------- section renderers ---------- */

function field(column, record, where) {
  const set = (value) => { record[column.key] = value; };
  switch (column.type) {
    case "name":
      return el("span", { class: "name" }, String(record[column.key] ?? ""));
    case "prose":
      return el("input", {
        class: "prose",
        type: "text",
        value: record[column.key] ?? "",
        oninput: (event) => { set(event.target.value); touched(); },
      });
    case "known":
      return pickOrProse(record[column.key], V[column.of](), set, where);
    case "pick":
      return pick(record[column.key], V[column.of](), set, column.optional);
    case "picks":
      return picks(record[column.key] || [], V[column.of](), set);
    case "number":
      return number(record[column.key], set);
    case "bool":
      return bool(record[column.key], set);
    case "amounts":
      return amounts(record[column.key] || [], V[column.of](), set);
    case "capdens":
      return capdens(record[column.key], set);
    default:
      throw new Error("no renderer for " + column.type);
  }
}

function renderTable(spec, host) {
  const rows = game[spec.key];
  const table = el("table");
  table.append(
    el("thead", {}, el("tr", {}, spec.columns.map((c) => el("th", {}, c.label))))
  );
  const body = el("tbody");
  rows.forEach((record, index) => {
    const tr = el("tr");
    for (const column of spec.columns) {
      const cell = el("td", { class: column.type === "name" ? "name" : "" });
      if (!column.when || column.when(record)) {
        cell.append(field(column, record, `${spec.key}/${index}/${column.key}`));
      } else {
        cell.append(el("span", { class: "note" }, BLANK));
      }
      tr.append(cell);
    }
    const last = tr.lastElementChild;
    last.append(
      " ",
      el("button", {
        class: "drop",
        title: "remove",
        onclick: () => { rows.splice(index, 1); touched(); draw(); },
      }, "×")
    );
    body.append(tr);
  });
  table.append(body);
  host.append(el("div", { class: "scroll" }, table));
  host.append(newRow(spec, rows));
}

/** The one place a name is typed, and it happens once. */
function newRow(spec, rows) {
  const nameColumn = spec.columns.find((c) => c.type === "name");
  const box = el("div", { class: "row add" });
  if (!nameColumn) {
    box.append(el("button", {
      onclick: () => { rows.push(blankRecord(spec)); touched(); draw(); },
    }, "+ add"));
    return box;
  }
  const input = el("input", { type: "text", placeholder: "a name", "aria-label": "a name" });
  const submit = () => {
    const name = input.value.trim();
    if (!name) return;
    rows.push({ ...blankRecord(spec), [nameColumn.key]: name });
    touched();
    draw();
  };
  input.addEventListener("keydown", (event) => { if (event.key === "Enter") submit(); });
  box.append(input, el("button", { onclick: submit }, "+ add"));
  box.append(el("span", { class: "note" }, "the name is typed; everything else is chosen"));
  return box;
}

function blankRecord(spec) {
  const record = {};
  for (const column of spec.columns || []) {
    if (column.type === "picks") record[column.key] = [];
    else if (column.type === "amounts") record[column.key] = [];
    else if (column.type === "bool") record[column.key] = false;
    else if (column.type === "number" || column.type === "capdens") record[column.key] = null;
    else if (column.type === "pick") record[column.key] = column.optional ? "" : V[column.of]()[0];
    else record[column.key] = "";
  }
  return record;
}

function renderRecipes(host) {
  for (const [index, recipe] of game.recipes.entries()) {
    const card = el("details", { class: "recipe", open: index < 2 });
    card.append(
      el(
        "summary",
        {},
        recipe.name,
        el("span", { class: "owner" }, recipe.owner),
        el("span", { class: "n" }, `${recipe.rows.length} row${recipe.rows.length === 1 ? "" : "s"}`)
      )
    );
    const body = el("div", { class: "body" });
    body.append(
      el("div", { class: "row", style: "margin-bottom:8px" },
        el("span", { class: "note" }, "fires"),
        pick(recipe.owner, V.owners(), (v) => { recipe.owner = v; }),
        el("button", {
          class: "drop",
          title: "remove this recipe",
          onclick: () => { game.recipes.splice(index, 1); touched(); draw(); },
        }, "× remove")
      )
    );
    const table = el("table");
    table.append(el("thead", {}, el("tr", {},
      ["Role", "Qty", "Kind", "What must be true of it", "Where", ""].map((h) => el("th", {}, h))
    )));
    const rows = el("tbody");
    recipe.rows.forEach((row, at) => {
      // `P-421`: a put has no quantity, because nothing is made or taken. The editor does not
      // offer one rather than offering one and hoping nobody fills it in.
      const takesQuantity = row.role !== "put";
      rows.append(el("tr", {},
        el("td", {}, pick(row.role, V.roles(), (v) => {
          row.role = v;
          if (v === "put") row.qty = "";
          else if (!row.qty) row.qty = "1";
        })),
        el("td", {}, takesQuantity ? quantity(row) : el("span", { class: "note" }, "none")),
        el("td", {}, pick(row.kind, V.kindOrFamily(), (v) => { row.kind = v; })),
        el("td", {}, constraint(row, () => {})),
        el("td", {}, pick(row.where, V.wheres(), (v) => { row.where = v; }, true)),
        el("td", {}, el("button", {
          class: "drop",
          title: "remove",
          onclick: () => { recipe.rows.splice(at, 1); touched(); draw(); },
        }, "×"))
      ));
    });
    table.append(rows);
    body.append(el("div", { class: "scroll" }, table));
    body.append(el("button", {
      class: "add",
      onclick: () => {
        recipe.rows.push({ role: "produce", qty: "1", kind: V.kinds()[0], traits: "", where: "", constraint: null });
        touched();
        draw();
      },
    }, "+ add a row"));
    card.append(body);
    host.append(card);
  }
  const input = el("input", { type: "text", placeholder: "a name", "aria-label": "a recipe name" });
  const submit = () => {
    const name = input.value.trim();
    if (!name) return;
    game.recipes.push({ name, owner: "player", rows: [] });
    touched();
    draw();
  };
  input.addEventListener("keydown", (event) => { if (event.key === "Enter") submit(); });
  host.append(el("div", { class: "row add" }, input, el("button", { onclick: submit }, "+ add a recipe"),
    el("span", { class: "note" }, "the name is typed; everything else is chosen")));
}

function renderOrder(host) {
  const list = el("div", { class: "rows" });
  game.worldOrder.forEach((name, index) => {
    list.append(el("div", { class: "row" },
      el("span", { class: "note", style: "min-width:2em;text-align:right" }, String(index + 1)),
      pick(name, V.recipes(), (v) => { game.worldOrder[index] = v; }),
      el("button", { class: "drop", title: "earlier", onclick: () => { move(game.worldOrder, index, -1); } }, "↑"),
      el("button", { class: "drop", title: "later", onclick: () => { move(game.worldOrder, index, 1); } }, "↓"),
      el("button", { class: "drop", title: "remove", onclick: () => { game.worldOrder.splice(index, 1); touched(); draw(); } }, "×")
    ));
  });
  host.append(list);
  host.append(el("button", {
    class: "add",
    onclick: () => { game.worldOrder.push(V.recipes()[0]); touched(); draw(); },
  }, "+ add"));
}

function move(list, index, by) {
  const to = index + by;
  if (to < 0 || to >= list.length) return;
  [list[index], list[to]] = [list[to], list[index]];
  touched();
  draw();
}

function renderLoop(host) {
  const list = el("div", { class: "rows" });
  game.loop.forEach((step, index) => {
    list.append(el("div", { class: "row" },
      el("span", { class: "note", style: "min-width:2em;text-align:right" }, String(index + 1)),
      el("input", {
        class: "prose",
        type: "text",
        value: step,
        oninput: (event) => { game.loop[index] = event.target.value; touched(); },
      }),
      el("button", { class: "drop", title: "earlier", onclick: () => { move(game.loop, index, -1); } }, "↑"),
      el("button", { class: "drop", title: "later", onclick: () => { move(game.loop, index, 1); } }, "↓"),
      el("button", { class: "drop", title: "remove", onclick: () => { game.loop.splice(index, 1); touched(); draw(); } }, "×")
    ));
  });
  host.append(list);
}

/* ---------- what still has to be typed ---------- */

/**
 * Counted from the schema rather than remembered, so it cannot drift from what the page does.
 * This is the measurement the prototype exists to make.
 */
function typing() {
  const prose = [];
  const offColour = [];
  for (const spec of SCHEMA) {
    for (const column of spec.columns || []) {
      if (column.type === "prose") {
        prose.push(`${spec.title} → ${column.label}`);
        continue;
      }
      // **A `known` column is a choice until somebody needs a phrase the game has never
      // used.** Counted from the data rather than from the column's type, because the type
      // says what is offered and only the data says what was taken - which is the difference
      // this repository has now got wrong four times in a week.
      if (column.type !== "known") continue;
      const vocabulary = V[column.of]();
      for (const record of game[spec.key]) {
        const value = record[column.key];
        if (value && !vocabulary.includes(value)) {
          offColour.push({ where: `${spec.title} → ${column.label}`, text: value });
        }
      }
    }
  }
  const cells = game.recipes.flatMap((r) => r.rows).filter((r) => r.traits && !r.constraint);
  return { prose, cells, offColour };
}

function renderFlag(host) {
  const { prose, cells, offColour } = typing();
  const rows = game.recipes.flatMap((r) => r.rows);
  const constrained = rows.filter((r) => r.traits || r.constraint).length;
  const flag = el("div", { class: "flag" });
  flag.append(el("h3", {}, "What still has to be typed"));
  flag.append(el("p", {},
    `${cells.length} of the ${constrained} conditions in the recipes cannot be offered as a ` +
    `choice. Every other value in every recipe is a selection.`));
  if (cells.length) {
    flag.append(el("ul", {}, cells.map((c) => el("li", {}, el("code", {}, c.traits)))));
  }
  flag.append(el("p", { style: "margin-top:8px" },
    `Elsewhere, ${prose.length} columns are prose because what they hold is written for a ` +
    `person rather than run by the game — `,
    el("span", { class: "note" }, prose.join(", ")),
    `, and the steps of the loop.`));
  flag.append(el("p", { style: "margin-top:8px" },
    offColour.length
      ? `${offColour.length} ${offColour.length === 1 ? "value has" : "values have"} been ` +
        `written rather than chosen:`
      : `Everything else is chosen from what the release already says. Ask for a subject, a ` +
        `value shape or a reason the game has never used and it appears here.`));
  if (offColour.length) {
    flag.append(el("ul", {}, offColour.map((o) =>
      el("li", {}, el("code", {}, o.text), " ", el("span", { class: "note" }, `in ${o.where}`)))));
  }
  host.append(flag);
}

/* ---------- shell ---------- */

function draw() {
  const nav = document.getElementById("nav");
  nav.replaceChildren();
  for (const spec of SCHEMA) {
    nav.append(el("button", {
      "aria-current": String(spec.key === section),
      onclick: () => { section = spec.key; draw(); window.scrollTo(0, 0); },
    }, spec.title, el("span", { class: "count" }, String(spec.count()))));
  }

  const main = document.getElementById("main");
  main.replaceChildren();
  const spec = SCHEMA.find((s) => s.key === section);
  main.append(el("h2", {}, spec.title));
  main.append(el("p", { class: "about" }, spec.about));
  if (spec.key === "recipes") renderFlag(main);
  if (spec.render) spec.render(main);
  else renderTable(spec, main);
}

document.getElementById("export").addEventListener("click", () => {
  const text = JSON.stringify(game, null, 2);
  const blob = new Blob([text], { type: "application/json" });
  const url = URL.createObjectURL(blob);
  const link = el("a", { href: url, download: "game.json" });
  document.body.append(link);
  link.click();
  link.remove();
  URL.revokeObjectURL(url);
});

document.getElementById("revert").addEventListener("click", () => {
  game = JSON.parse(ORIGINAL);
  dirty = false;
  document.getElementById("dirty").hidden = true;
  draw();
});

draw();
