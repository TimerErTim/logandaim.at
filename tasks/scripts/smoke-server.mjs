#!/usr/bin/env node
// Health-check an already running backend.
//
// Independent of `verify:server`: point it at any deployment with
// LOGANDAIM_API_URL, for example after deploying to the VPS.

const baseUrl = (process.env.LOGANDAIM_API_URL || `http://127.0.0.1:${process.env.LOGANDAIM_API_PORT || 3311}`).replace(/\/+$/, "");

const fail = (message) => {
  console.error(`smoke:server: ${message}`);
  process.exit(1);
};

const request = async (path, redirect = "follow") => {
  try {
    const response = await fetch(`${baseUrl}${path}`, { redirect });
    return { status: response.status, body: await response.text() };
  } catch (error) {
    fail(`${path} is unreachable: ${error.message}`);
  }
};

// The endpoint returns the JSON string "OK", so drop the quotes before comparing.
const health = await request("/api/health");
const body = health.body.replaceAll('"', "").trim();
if (body !== "OK") fail(`/api/health returned '${body}', expected OK`);

const specJson = await request("/spec.json");
try {
  JSON.parse(specJson.body);
} catch {
  fail("/spec.json is not valid JSON");
}

for (const path of ["/spec.yaml", "/swagger"]) {
  const response = await request(path);
  if (response.status !== 200) fail(`${path} returned '${response.status}', expected 200`);
}

const missing = await request("/api/does-not-exist", "manual");
if (missing.status !== 404 && missing.status !== 405) {
  fail(`/api/does-not-exist returned '${missing.status}', expected 404 or 405`);
}

console.log(`smoke:server: ok (${baseUrl})`);
