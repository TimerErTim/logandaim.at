#!/usr/bin/env node
// release:notes — write the release body for a version to stdout or to a file.

import { writeFileSync } from "node:fs";
import { releaseBody, resolveRepo } from "./release-body.mjs";

const fail = (message) => {
  console.error(`release:notes: ${message}`);
  process.exit(1);
};

const version = process.env.usage_version ?? "";
const pullRequest = process.env.usage_pr ?? "";
const out = process.env.usage_out ?? "";

if (!version) fail("<version> is required");
if (!pullRequest) fail("--pr <number> is required");

let repo;
try {
  repo = resolveRepo();
} catch (error) {
  fail(error.message);
}

let body;
try {
  body = releaseBody(repo, version, pullRequest);
} catch (error) {
  fail(error.message);
}

if (out) {
  writeFileSync(out, body);
  console.error(`release:notes: wrote ${out}`);
} else {
  process.stdout.write(body);
}
