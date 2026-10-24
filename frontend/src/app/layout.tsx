import type { Metadata } from "next";
import { Geist_Mono, Nunito_Sans } from "next/font/google";
import "./globals.css";
import { AppChrome } from "@/components/layout/app-chrome";
import { Providers } from "./providers";

/**
 * Nunito Sans is the UI typeface. Softer and warmer than a geometric grotesque,
 * which suits an outdoor tool better than a software-product face.
 *
 * Every weight is listed explicitly because `next/font` only emits what is asked
 * for, and the interface uses all four (`font-bold` appears in the sidebar brand).
 */
const nunitoSans = Nunito_Sans({
  variable: "--font-nunito-sans",
  subsets: ["latin"],
  weight: ["400", "500", "600", "700"],
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
    <html lang="de" className={`${nunitoSans.variable} ${geistMono.variable}`}>
      <body className="font-sans antialiased">
        <Providers>
          <AppChrome>{children}</AppChrome>
        </Providers>
      </body>
    </html>
  );
}
