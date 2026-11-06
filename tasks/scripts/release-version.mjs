#!/usr/bin/env node
// release:version — bump the workspace and frontend versions for a release PR.
//
// `Cargo.lock` pins every workspace crate version, so it must agree with the
// manifest in the same commit. `cargo metadata` rewrites the lock without building.

import { execFileSync } from "node:child_process";
import { readFileSync, writeFileSync } from "node:fs";

const fail = (message) => {
  console.error(`release:version: ${message}`);
  process.exit(1);
};

const workspaceVersion = () => {
  const section = readFileSync("Cargo.toml", "utf8").match(/^\[workspace\.package\]([\s\S]*?)^\[/m);
  return section?.[1].match(/^\s*version\s*=\s*"([^"]*)"/m)?.[1] ?? "";
};

const current = workspaceVersion();
if (!current) fail("no version in [workspace.package] of Cargo.toml");

const kind = process.env.usage_kind ?? "";
const [major, minor, patch] = current.split(".").map(Number);

const next =
  {
    patch: `${major}.${minor}.${patch + 1}`,
    minor: `${major}.${minor + 1}.0`,
    major: `${major + 1}.0.0`,
  }[kind] ?? kind;

if (!/^\d+\.\d+\.\d+$/.test(next)) fail(`'${next}' is not a semantic version`);

// Rewrite only the first `version = "..."`, which is the `[workspace.package]` one.
const manifest = readFileSync("Cargo.toml", "utf8").replace(
  /^version\s*=\s*".*"/m,
  `version = "${next}"`,
);
writeFileSync("Cargo.toml", manifest);

const packageJson = JSON.parse(readFileSync("frontend/package.json", "utf8"));
packageJson.version = next;
writeFileSync("frontend/package.json", `${JSON.stringify(packageJson, null, 2)}\n`);

execFileSync("cargo", ["metadata", "--format-version", "1"], { stdio: "ignore" });

const today = new Date().toISOString().slice(0, 10);
console.log(`release:version: ${current} -> ${next}`);
console.log(`Now move the Unreleased changelog entries into '## [${next}] - ${today}'.`);
