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

export const metadata: Metadata = {
  title: "logandaim.at",
  description: "Yet Another Management Software",
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
