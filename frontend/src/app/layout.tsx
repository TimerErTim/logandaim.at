import type { Metadata } from "next";
import { Geist_Mono, Inter, Lora } from "next/font/google";
import "./globals.css";
import { AppChrome } from "@/components/layout/app-chrome";
import { Providers } from "./providers";

/**
 * Two faces, two jobs.
 *
 * Inter is the interface face: menus, tables, form controls, small figures. It is
 * deliberately plain, because the work has to be fast and the numbers have to line
 * up. Lora is the book face and is used for titles only — the logbook's hand,
 * carrying the historical character into the headings without touching the work.
 *
 * Every weight is listed explicitly because `next/font` only emits what is asked
 * for: Inter 400–700, Lora 500–700 plus its italic. Anything heavier than 700, or
 * a Lora weight not in this list, would be synthesised by the browser.
 */
const inter = Inter({
  variable: "--font-inter",
  subsets: ["latin"],
  weight: ["400", "500", "600", "700"],
});

const lora = Lora({
  variable: "--font-lora",
  subsets: ["latin"],
  weight: ["500", "600", "700"],
  style: ["normal", "italic"],
});

/** Code and machine values only: base URLs, token names, `globals.css`. */
const geistMono = Geist_Mono({
  variable: "--font-geist-mono",
  subsets: ["latin"],
});

/**
 * Icons and the web manifest live in `public/` and are linked explicitly, because the
 * favicon set is a single copy shared by every deployment shape.
 *
 * Next applies `basePath` to file-convention metadata it generates itself, but it
 * emits hand-written `metadata.icons` / `metadata.manifest` URLs verbatim. The prefix
 * therefore has to be added here, or the links 404 on the GitHub Pages project site.
 */
const basePath = process.env.NEXT_PUBLIC_LOGANDAIM_BASE_PATH ?? "";

const description =
  "Österreichweites Online-Logbuch für 3D-Bogenschießen: Runden Ziel für Ziel erfassen, den Schussabstand zwischen den Pflöcken festhalten und die eigene Treffsicherheit über die Zeit verfolgen.";

export const metadata: Metadata = {
  title: {
    default: "logandaim.at",
    template: "%s · logandaim.at",
  },
  description,
  applicationName: "logandaim.at",
  manifest: `${basePath}/site.webmanifest`,
  icons: {
    icon: [
      { url: `${basePath}/favicon.ico`, sizes: "any", type: "image/x-icon" },
      {
        url: `${basePath}/favicon-32x32.png`,
        sizes: "32x32",
        type: "image/png",
      },
      {
        url: `${basePath}/favicon-16x16.png`,
        sizes: "16x16",
        type: "image/png",
      },
    ],
    apple: {
      url: `${basePath}/apple-touch-icon.png`,
      sizes: "180x180",
      type: "image/png",
    },
  },
  openGraph: {
    type: "website",
    locale: "de_AT",
    siteName: "logandaim.at",
    title: "logandaim.at",
    description,
  },
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    // The font variables must live on `<html>`: `@theme` resolves `--font-sans` at
    // the root, so variables declared on `<body>` would not be visible there and the
    // whole app would fall back to the system stack.
    <html
      lang="de"
      className={`${inter.variable} ${lora.variable} ${geistMono.variable}`}
    >
      <body className="font-sans antialiased">
        <Providers>
          <AppChrome>{children}</AppChrome>
        </Providers>
      </body>
    </html>
  );
}
