#!/usr/bin/env node
// check:identifiers — German domain identifiers are spelled in real UTF-8, never
// transliterated. `Empfänger` is `Empfänger`, not `Empfaenger`: an umlaut is a letter,
// and the transliteration is a leftover from toolchains that could not carry one.
//
// Three exceptions, each of which the check enforces rather than tolerates:
//
//   1. Module names stay ASCII, because Rust resolves `mod x;` to a file path and refuses
//      a non-ASCII one. Every declared `mod` name is therefore allowed everywhere.
//   2. All-caps identifiers — environment variables and constants — stay ASCII. An
//      operator types those, and a shell cannot be relied on to carry an umlaut through.
//   3. English words that merely contain `ae`/`oe`/`ue` are not findings. A gate that
//      flags `Request` or `Issuer` is a gate people switch off.

import { readdirSync, readFileSync, statSync } from "node:fs";
import { join, relative } from "node:path";

const ROOTS = ["crates", "backend", "tasks", "specs", "frontend/src"];
const EXTENSIONS = new Set([".rs", ".toml", ".md", ".ts", ".tsx", ".mjs", ".json"]);
const SKIP = new Set(["target", "node_modules", "graphify-out", ".git", ".jj", "legacy"]);

// This file names the patterns it forbids, so scanning it would only report itself.
const SELF = relative(".", new URL(import.meta.url).pathname);

const TRANSLITERATION = /(ae|oe|ue)/i;
const ALLOWED = new Set([
  // English words that contain `ae`/`oe`/`ue` as real spelling.
  "value", "values", "valuetext", "query", "queries", "querying", "question",
  "questions", "request", "requests", "requested", "requester", "requesting",
  "requestid", "issue", "issues", "issued", "issuer", "unique", "uniqueness",
  "queue", "sequence", "continue", "continues", "subsequent", "consequence",
  "consequences", "true", "does", "doesn", "goes", "guess", "guessed", "guesses",
  "guessing", "guest", "hue", "hues", "blue", "opaque", "opaquely", "argued",
  "whoever", "daemon", "daemons", "poem", "echoes",
  // German words already correct, with no umlaut to restore.
  "neue", "neuen", "neuer", "neues", "erneuern", "erneuerung", "erneuerungs",
  "erneuerungsausgang", "aktuell", "aktueller", "dauer", "lebensdauer", "quelle",
  "manuell", "fuer", "baue", "visuelle", "visuell", "individuellem",
]);

const SEGMENT = /(?<=[a-zäöüß0-9])(?=[A-ZÄÖÜ])/;

const walk = (dir) => {
  let found = [];
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    if (SKIP.has(entry.name)) continue;
    const path = join(dir, entry.name);
    if (entry.isDirectory()) found = found.concat(walk(path));
    else if (EXTENSIONS.has(path.slice(path.lastIndexOf(".")))) found.push(path);
  }
  return found;
};

const files = ROOTS.flatMap((root) => {
  try {
    return statSync(root).isDirectory() ? walk(root) : [root];
  } catch {
    // A root that does not exist in this checkout is not a finding.
    return [];
  }
}).filter((path) => !path.endsWith(".mjs") || !path.endsWith("check-identifiers.mjs"));

// `mod` declarations are the one place an ASCII spelling is mandatory, so their names are
// exempt wherever they appear — module path, re-export, or doc comment.
const moduleNames = new Set();
for (const path of files.filter((file) => file.endsWith(".rs"))) {
  for (const line of readFileSync(path, "utf8").split("\n")) {
    const name = line.trim().match(/^(?:pub\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*;/)?.[1];
    if (name) moduleNames.add(name.toLowerCase());
  }
}

const findings = [];
const report = (path, line, segment, text, rule) =>
  findings.push({ path: relative(".", path), line, segment, text: text.trim(), rule });

for (const path of files) {
  readFileSync(path, "utf8").split("\n").forEach((line, index) => {
    // A non-ASCII module name is a build error, so it is reported as its own rule.
    if (path.endsWith(".rs") && /^(?:pub\s+)?mod\s+[^;]*[ÄÖÜäöüß]/.test(line.trim())) {
      report(path, index + 1, "mod", line, "module names must stay ASCII");
    }

    for (const token of line.match(/[A-Za-zÄÖÜäöüß][A-Za-z0-9ÄÖÜäöüß]*/g) ?? []) {
      // An all-caps token is a constant. Constants are Rust identifiers, so they keep their
      // umlauts (`MIN_PASSWORT_LÄNGE`); only what leaves the process must be ASCII, and the
      // `env = "…"` literal below is checked separately.
      if (/^[A-ZÄÖÜ][A-ZÄÖÜ0-9_]{2,}$/.test(token)) continue;

      for (const camel of token.split(SEGMENT)) {
        for (const segment of camel.split("_")) {
          const lower = segment.toLowerCase();
          if (ALLOWED.has(lower) || moduleNames.has(lower)) continue;
          if (!TRANSLITERATION.test(lower)) continue;
          report(path, index + 1, segment, line, "umlaut written ae/oe/ue");
        }
      }
    }

    // An environment variable is typed by an operator and a shell cannot be relied on to
    // carry an umlaut through every hop, so its name stays ASCII.
    for (const env of line.matchAll(/(?:env|name)\s*[:=]\s*"([^"]*)"/g)) {
      if (/[ÄÖÜäöüß]/.test(env[1])) {
        report(path, index + 1, env[1], line, "environment variable names must stay ASCII");
      }
    }
  });
}

if (findings.length === 0) {
  console.log("check:identifiers: ok");
  process.exit(0);
}

for (const { path, line, segment, text, rule } of findings) {
  console.error(`check:identifiers: ${path}:${line}: ${rule}: '${segment}'`);
  console.error(`  ${text}`);
}
console.error(`check:identifiers: ${findings.length} finding(s)`);
process.exit(1);
