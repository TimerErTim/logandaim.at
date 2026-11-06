import type { NextConfig } from "next";
import { PHASE_DEVELOPMENT_SERVER } from "next/constants";

// GitHub Pages serves project sites under a path prefix. Set
// `NEXT_PUBLIC_LOGANDAIM_BASE_PATH` when the site is not at the domain root;
// `logandaim.at` as a custom domain serves at the root, so it stays empty.
const basePath = process.env.NEXT_PUBLIC_LOGANDAIM_BASE_PATH ?? "";

// Production keeps exactly the extensions the App Router shipped with. Dev adds
// `.dev.tsx` / `.dev.ts` in front, so a route written as `page.dev.tsx` exists only
// under `next dev` and is never part of the static export.
const PROD_PAGE_EXTENSIONS = ["tsx", "ts"];
const DEV_PAGE_EXTENSIONS = ["dev.tsx", "dev.ts", ...PROD_PAGE_EXTENSIONS];

/**
 * Keyed on the Next phase, not `NODE_ENV`: the phase is what Next itself passes when
 * it resolves routes, so the gate cannot drift from how the route tree is built.
 */
export default function config(phase: string): NextConfig {
  const isDevServer = phase === PHASE_DEVELOPMENT_SERVER;

  return {
    reactCompiler: true,
    reactStrictMode: true,
    output: "export",
    trailingSlash: true,
    pageExtensions: isDevServer ? DEV_PAGE_EXTENSIONS : PROD_PAGE_EXTENSIONS,
    basePath,
    assetPrefix: basePath || undefined,
    images: {
      unoptimized: true,
    },
  };
}
