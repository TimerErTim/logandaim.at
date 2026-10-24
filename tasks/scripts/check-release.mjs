#!/usr/bin/env node
// check:release — this tree must be a valid release for the version it carries.
//
// Version agreement comes from `check:version`, declared as a task dependency.
// Everything else that publishing needs is validated here, before `deploy/pages`
// runs, so a release cannot deploy and then fail to publish.
//
// The remote checks need network and a `gh` authentication. A developer machine
// without them skips those parts; in CI the same failure fails the gate.

import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { contentLines, readChangelog, section } from "./changelog.mjs";
import { isUnauthenticated, tryGh } from "./gh.mjs";
import { releaseBody, resolveRepo } from "./release-body.mjs";

const fail = (message) => {
  console.error(`check:release: ${message}`);
  process.exit(1);
};

const run = (command, args) => {
  try {
    return {
      ok: true,
      stdout: execFileSync(command, args, { encoding: "utf8", stdio: ["ignore", "pipe", "pipe"] }),
    };
  } catch (error) {
    return { ok: false, stdout: error.stdout ?? "", stderr: error.stderr ?? "" };
  }
};

const workspaceVersion = (manifest = readFileSync("Cargo.toml", "utf8")) => {
  const section = manifest.match(/^\[workspace\.package\]([\s\S]*?)^\[/m);
  return section?.[1]?.match(/^\s*version\s*=\s*"([^"]*)"/m)?.[1] ?? "";
};

const isCi = Boolean(process.env.GITHUB_ACTIONS || process.env.CI);
const version = workspaceVersion();
const pullRequestArg = process.env.usage_pr ?? "";

if (!version) fail("no version in [workspace.package] of Cargo.toml");

const changelog = readChangelog();

// 1. A dated section for this exact version.
const escaped = version.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
if (!new RegExp(`^## \\[${escaped}\\] - \\d{4}-\\d{2}-\\d{2}\\s*$`, "m").test(changelog)) {
  fail(`CHANGELOG.md has no dated '## [${version}] - YYYY-MM-DD' section`);
}

// 2. The section must say something: a heading with no entries would publish an
// empty changelog part.
const dated = section(changelog, version);
if (dated === null || contentLines(dated).length === 0) {
  fail(`CHANGELOG.md has no entries under '## [${version}]'`);
}

// 3. A release pull request moves every entry into the dated section, so
// `Unreleased` must be empty. Empty is only acceptable for pull requests into
// `dev`, where the next release has not been decided yet.
const unreleased = section(changelog, "Unreleased");
if (unreleased !== null && contentLines(unreleased).length > 0) {
  fail(`CHANGELOG.md has entries under '## [Unreleased]'; move them into '## [${version}]'`);
}

// 4. Compare against the most recent release tag rather than `main`, so the check
// behaves the same in a release pull request and in the push-to-main run that
// follows it. Tags are created by the `main` workflow only.
const tags = run("git", ["tag", "--list", "v*", "--sort=-v:refname"]);
const previousTag = tags.ok ? (tags.stdout.split("\n")[0] ?? "") : "";
if (previousTag) {
  const released = run("git", ["show", `${previousTag}:Cargo.toml`]);
  if (released.ok && workspaceVersion(released.stdout) === version) {
    fail(
      `version '${version}' was already released as ${previousTag}; bump it before opening the release pull request`,
    );
  }
}

// 5. The release body must be assemblable. `release:publish` re-derives it through
// the same function, so a body the gate cannot build is a release that deploys and
// then fails to publish.
//
// A developer without a `gh` login cannot run the remote part; skip it there, so
// `mise run check:release` stays usable offline. CI always has a token, so the same
// state is a broken pipeline and must fail before the deploy.
const auth = tryGh("auth", "status");
if (!auth.ok && isUnauthenticated(auth)) {
  if (isCi) fail("gh is not authenticated; the release body cannot be assembled");
  console.error("check:release: gh is not authenticated; skipped the release body check");
  console.log(`check:release: ${version} ok`);
  process.exit(0);
}
if (!auth.ok) fail(`gh auth status failed: ${auth.stderr.trim() || `exit ${auth.status}`}`);

let repo;
try {
  repo = resolveRepo();
} catch (error) {
  fail(error.message);
}

if (tryGh("release", "view", `v${version}`, "--repo", repo).ok) {
  fail(`release v${version} already exists on ${repo}; bump the version`);
}

let pullRequest = pullRequestArg;
if (pullRequest && !/^\d+$/.test(pullRequest)) fail(`--pr '${pullRequest}' is not a pull request number`);

if (!pullRequest) {
  const head = run("git", ["rev-parse", "HEAD"]);
  if (!head.ok) fail("cannot resolve HEAD; pass --pr");
  const sha = head.stdout.trim();
  const merged = tryGh(
    "api",
    `repos/${repo}/commits/${sha}/pulls`,
    "--jq",
    'if type == "array" and length > 0 then (.[0].number | tostring) else "" end',
  );

  if (merged.ok) {
    pullRequest = merged.stdout.trim();
    if (!pullRequest) fail("no pull request is associated with HEAD; the release has no description");
  } else {
    // A fixture commit that GitHub does not know (no PR) also lands here, so report
    // the actual failure instead of assuming the network is down.
    const reason = merged.stderr.trim().split("\n")[0] || `exit ${merged.status}`;
    if (isCi) fail(`cannot look up the pull request for ${sha.slice(0, 7)}: ${reason}`);
    console.error(`check:release: cannot look up the pull request for ${sha.slice(0, 7)}: ${reason}`);
    console.error("check:release: skipped the release body check");
  }
}

if (pullRequest) {
  try {
    releaseBody(repo, version, pullRequest);
  } catch (error) {
    fail(`the release body cannot be generated from pull request #${pullRequest}: ${error.message}`);
  }
}

console.log(`check:release: ${version} ok`);
