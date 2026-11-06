// Shared CHANGELOG.md helpers for the release tasks.
//
// `release` markers are parsed with the same rules everywhere, so the gate and
// the release body cannot disagree about what a section contains.

import { readFileSync } from "node:fs";

/** Drop HTML comments, which templates use as guidance for the author. */
export const stripComments = (text) => text.replace(/<!--[\s\S]*?-->/g, "");

/** Keep only the lines that carry content. */
export const contentLines = (text) => stripComments(text).split("\n").filter((line) => line.trim());

/**
 * Body of `## [title]`, up to the next `## ` heading or the link-reference block
 * at the bottom of the file. Returns null when the section does not exist.
 */
export const section = (changelog, title) => {
  const lines = changelog.split("\n");
  const start = lines.findIndex((line) => line === `## [${title}]` || line.startsWith(`## [${title}] `));
  if (start === -1) return null;

  const body = [];
  for (const line of lines.slice(start + 1)) {
    if (line.startsWith("## ") || (line.startsWith("[") && line.includes("]: "))) break;
    body.push(line);
  }
  return body.join("\n");
};

/** Release notes for a version, trimmed, with the guidance comments removed. */
export const sectionContent = (changelog, title) => {
  const body = section(changelog, title);
  return body === null ? null : contentLines(body).join("\n");
};

export const readChangelog = (path = "CHANGELOG.md") => readFileSync(path, "utf8");
