#!/usr/bin/env node
// Health-check an already running frontend.
//
// Independent of `verify:frontend`: point it at any deployment with
// LOGANDAIM_FRONTEND_URL, for example after deploying to the VPS.

const fail = (message) => {
  console.error(`smoke:frontend: ${message}`);
  process.exit(1);
};

const baseUrl = (
  process.env.LOGANDAIM_FRONTEND_URL ||
  `http://127.0.0.1:${process.env.LOGANDAIM_FRONTEND_PORT || 4173}`
).replace(/\/+$/, "");

const fetchText = async (path) => {
  try {
    const response = await fetch(`${baseUrl}${path}`, { redirect: "follow" });
    return { status: response.status, body: await response.text() };
  } catch (error) {
    fail(`${baseUrl}${path} is unreachable: ${error.message}`);
  }
};

const root = await fetchText("/");
if (root.status !== 200) fail(`${baseUrl}/ returned '${root.status}', expected 200`);
if (!root.body.includes("<html")) fail(`${baseUrl}/ is not an HTML document`);

// The export uses trailing slashes, so the server redirects `/404.html` to
// `/404`. Follow redirects: what matters is that a static 404 page is reachable.
const notFound = await fetchText("/404.html");
if (notFound.status !== 200 || !notFound.body.includes("<html")) {
  fail(`${baseUrl}/404.html is not reachable as HTML`);
}

console.log(`smoke:frontend: ok (${baseUrl})`);
