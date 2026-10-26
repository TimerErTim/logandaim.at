#!/usr/bin/env node
// Design-system gate.
//
// Three rules, all mechanical:
//   1. every literal token in `globals.css` is named in the design rule
//   2. no raw Tailwind palette class appears in a component
//   3. no retired utility name reappears, so the palette keeps one name per colour
//
// Prose nuance (why a token exists, which role it may carry) cannot be checked here;
// that half is the "design rule updated" review checklist item.

import { readFileSync, readdirSync, statSync } from "node:fs";
import { join, relative } from "node:path";
import { fileURLToPath } from "node:url";

const ROOT = fileURLToPath(new URL("../..", import.meta.url));
const CSS = join(ROOT, "frontend/src/app/globals.css");
const RULE = join(ROOT, ".cursor/rules/design-system.mdc");
const SRC = join(ROOT, "frontend/src");

/** Files the rules apply to. */
const SCANNED = [".tsx", ".ts", ".css"];
const SKIP_DIRS = new Set(["node_modules", ".next", "out", "api"]);

/** Raw palette utilities, e.g. `bg-stone-100`, `text-emerald-600`. */
const PALETTE =
  /\b(?:bg|text|border|from|to|via|ring|outline|divide|placeholder|fill|stroke|accent|caret)-(?:zinc|gray|slate|neutral|stone|red|orange|amber|yellow|lime|green|emerald|teal|cyan|sky|blue|indigo|violet|purple|fuchsia|pink|rose)-\d{2,3}\b/;

/**
 * The shadcn-era names this system replaced. `text-primary` is deliberately absent:
 * it is the brand colour now, and ink is `text-ink`.
 */
const LEGACY =
  /\b(?:bg|text|border|from|to|via|ring|outline|divide|placeholder|fill|stroke|accent|caret|decoration)-(?:foreground|background|muted-foreground|destructive|surface-elevated|placeholder|border-strong|contour|border|ring)\b|\bborder-border\b/;

const failures = [];

function walk(dir) {
  const out = [];
  for (const entry of readdirSync(dir)) {
    if (SKIP_DIRS.has(entry)) {
      continue;
    }
    const full = join(dir, entry);
    if (statSync(full).isDirectory()) {
      out.push(...walk(full));
      continue;
    }
    if (SCANNED.some((ext) => entry.endsWith(ext))) {
      out.push(full);
    }
  }
  return out;
}

// 1. Every literal token is documented.
const css = readFileSync(CSS, "utf8");
const rule = readFileSync(RULE, "utf8");

// Tokens are literal `--name: <hex|rem>` declarations. The `@theme inline` block
// only re-exports them as `var(...)`/`calc(...)` and is skipped by that shape.
const tokens = [
  ...new Set(
    [...css.matchAll(/^\s+(--[a-z0-9-]+):\s*(?:#[0-9a-fA-F]{3,8}|[\d.]+rem)/gm)].map(
      (match) => match[1],
    ),
  ),
];

for (const token of tokens) {
  if (!rule.includes(token)) {
    failures.push(`token not documented in the design rule: ${token}`);
  }
}

// 2 and 3. Components use semantic utilities only, spelled one way.
for (const file of walk(SRC)) {
  if (file === CSS) {
    continue;
  }
  const text = readFileSync(file, "utf8");
  text.split("\n").forEach((line, index) => {
    const where = `${relative(ROOT, file)}:${index + 1}`;
    if (PALETTE.test(line)) {
      failures.push(`raw palette class: ${where}: ${line.trim()}`);
    } else if (LEGACY.test(line)) {
      failures.push(`retired utility name: ${where}: ${line.trim()}`);
    }
  });
}

if (failures.length > 0) {
  console.error("check:design: failed");
  for (const failure of failures) {
    console.error(`  ${failure}`);
  }
  console.error(
    "\n  use: bg-canvas, bg-surface, bg-elevated, border-subtle, border-strong,",
  );
  console.error(
    "       text-ink, text-secondary, text-muted, text-impact, bg-primary",
  );
  process.exit(1);
}

console.log(`check:design: ok (${tokens.length} tokens documented)`);
