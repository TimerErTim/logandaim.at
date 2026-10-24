// Assemble a release body. Shared by `release:notes`, `release:publish`, and the
// `check:release` gate, so the three cannot disagree about what gets published.

import { execFileSync } from "node:child_process";
import { contentLines, readChangelog, sectionContent } from "./changelog.mjs";
import { gh } from "./gh.mjs";

export const resolveRepo = () => {
  const explicit = process.env.LOGANDAIM_REPO || process.env.GITHUB_REPOSITORY;
  if (explicit) return explicit;

  // CI checks out `origin`; a jj workspace may name it `github`. Never guess a
  // repository: publishing to the wrong one is silent.
  const url = ["origin", "github"]
    .map((remote) => {
      try {
        return execFileSync("git", ["remote", "get-url", remote], { encoding: "utf8" }).trim();
      } catch {
        return "";
      }
    })
    .find(Boolean);

  const repo = (url ?? "")
    .replace(/^git@github\.com:/, "")
    .replace(/^https:\/\/github\.com\//, "")
    .replace(/\.git$/, "");

  if (!repo) throw new Error("cannot determine the GitHub repository; set LOGANDAIM_REPO");
  return repo;
};

/**
 * The `## Release description` section of the release pull request, not the whole
 * body: the template's checklists are for reviewers and must not open the release
 * page. Fails when the section is missing or holds only guidance comments.
 */
const releaseDescription = (repo, pullRequest) => {
  const body = gh("pr", "view", pullRequest, "--repo", repo, "--json", "body", "--jq", ".body");

  const lines = body.split("\n");
  const start = lines.findIndex((line) => /^## Release description/.test(line));
  if (start === -1) {
    throw new Error(`pull request #${pullRequest} has no '## Release description' section`);
  }

  const captured = [];
  for (const line of lines.slice(start + 1)) {
    if (line.startsWith("## ")) break;
    captured.push(line);
  }

  // Check after stripping: a section holding only the template's guidance comment
  // would publish an empty description.
  const description = contentLines(captured.join("\n")).join("\n");
  if (!description) {
    throw new Error(`pull request #${pullRequest} has no non-empty '## Release description' section`);
  }
  return description;
};

/** First-time contributors, from GitHub's generated notes. Empty when there are none. */
const newContributors = (repo, version) => {
  const args = ["api", `repos/${repo}/releases/generate-notes`, "-f", `tag_name=v${version}`];

  try {
    const previousTag = gh(
      "release",
      "list",
      "--repo",
      repo,
      "--limit",
      "1",
      "--json",
      "tagName",
      "--jq",
      '.[0].tagName // ""',
    );
    if (previousTag) args.push("-f", `previous_tag_name=${previousTag}`);
  } catch {
    // No previous release: generate notes for every commit.
  }

  const lines = gh(...args, "--jq", ".body").split("\n");
  const start = lines.findIndex((line) => /^## New Contributors/.test(line));
  if (start === -1) return "";

  const captured = [];
  for (const line of lines.slice(start + 1)) {
    if (line.startsWith("## ") && !line.startsWith("## New Contributors")) break;
    captured.push(line);
  }
  return contentLines(captured.join("\n")).join("\n");
};

/** The full release body: description, then the changelog section, then contributors. */
export const releaseBody = (repo, version, pullRequest) => {
  const changelog = sectionContent(readChangelog(), version);
  if (!changelog) throw new Error(`CHANGELOG.md has no entries under '## [${version}]'`);

  const parts = [releaseDescription(repo, pullRequest), `## Changelog\n\n${changelog}`];
  const contributors = newContributors(repo, version);
  if (contributors) parts.push(contributors);

  return `${parts.join("\n\n")}\n`;
};
