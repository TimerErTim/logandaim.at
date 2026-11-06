#!/usr/bin/env node
// release:publish — create the source-only GitHub release for the current commit.
//
// `check:release` already validated everything this needs, including that the body
// can be assembled, so this only publishes what the gate proved.

import { execFileSync } from "node:child_process";
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { gh, tryGh } from "./gh.mjs";
import { releaseBody, resolveRepo } from "./release-body.mjs";

const fail = (message) => {
  console.error(`release:publish: ${message}`);
  process.exit(1);
};

const workspaceVersion = () => {
  const section = readFileSync("Cargo.toml", "utf8").match(/^\[workspace\.package\]([\s\S]*?)^\[/m);
  return section?.[1]?.match(/^\s*version\s*=\s*"([^"]*)"/m)?.[1] ?? "";
};

const version = process.env.usage_version || workspaceVersion();
if (!version) fail("no version in Cargo.toml");

const sha =
  process.env.GITHUB_SHA || execFileSync("git", ["rev-parse", "HEAD"], { encoding: "utf8" }).trim();

let repo;
try {
  repo = resolveRepo();
} catch (error) {
  fail(error.message);
}

if (tryGh("release", "view", `v${version}`, "--repo", repo).ok) {
  fail(`release v${version} already exists; bump the version`);
}

let pullRequest = process.env.usage_pr ?? "";
if (!pullRequest) {
  pullRequest = gh("api", `repos/${repo}/commits/${sha}/pulls`, "--jq", '.[0].number // ""');
}
if (!pullRequest) fail(`no pull request found for ${sha}`);

const dir = mkdtempSync(join(tmpdir(), "logandaim-release-"));
const notesFile = join(dir, "notes.md");

try {
  writeFileSync(notesFile, releaseBody(repo, version, pullRequest));
  gh(
    "release",
    "create",
    `v${version}`,
    "--repo",
    repo,
    "--target",
    sha,
    "--title",
    `v${version}`,
    "--notes-file",
    notesFile,
  );
} finally {
  rmSync(dir, { recursive: true, force: true });
}

console.log(`release:publish: created v${version} from ${sha}`);
