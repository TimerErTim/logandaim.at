import type { NextConfig } from "next";

// GitHub Pages serves project sites under a path prefix. Set
// `NEXT_PUBLIC_LOGANDAIM_BASE_PATH` when the site is not at the domain root;
// `logandaim.at` as a custom domain serves at the root, so it stays empty.
const basePath = process.env.NEXT_PUBLIC_LOGANDAIM_BASE_PATH ?? "";

const nextConfig: NextConfig = {
  reactCompiler: true,
  reactStrictMode: true,
  output: "export",
  trailingSlash: true,
  basePath,
  assetPrefix: basePath || undefined,
  images: {
    unoptimized: true,
  },
};

export default nextConfig;
