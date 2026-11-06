#!/usr/bin/env node
// check:version — the workspace, the frontend, and the newest changelog release
// must agree, and CHANGELOG.md must keep an `## [Unreleased]` section.

import { readFileSync } from "node:fs";

const fail = (message) => {
  console.error(`check:version: ${message}`);
  process.exit(1);
};

const read = (path) => readFileSync(path, "utf8");

const workspaceVersion = () => {
  const section = read("Cargo.toml").match(/^\[workspace\.package\]([\s\S]*?)^\[/m);
  return section?.[1]?.match(/^\s*version\s*=\s*"([^"]*)"/m)?.[1] ?? "";
};

const cargoVersion = workspaceVersion();
const packageVersion = JSON.parse(read("frontend/package.json")).version ?? "";
const changelog = read("CHANGELOG.md");
const changelogVersion = changelog.match(/^## \[(\d[^\]]*)\]/m)?.[1] ?? "";

if (!cargoVersion) fail("no version in [workspace.package] of Cargo.toml");
if (cargoVersion !== packageVersion) {
  fail(`Cargo.toml version '${cargoVersion}' != frontend/package.json version '${packageVersion}'`);
}
if (changelogVersion && cargoVersion !== changelogVersion) {
  fail(`Cargo.toml version '${cargoVersion}' != newest CHANGELOG.md release '${changelogVersion}'`);
}
if (!/^## \[Unreleased\]/m.test(changelog)) {
  fail("CHANGELOG.md has no ## [Unreleased] section");
}

console.log(`check:version: ${cargoVersion} ok`);
