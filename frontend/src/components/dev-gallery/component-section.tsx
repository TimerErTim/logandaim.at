"use client";

import { useId } from "react";

import { cn } from "@/lib/cn";

/**
 * One gallery block: the same content rendered in a light and a dark frame, so both
 * themes are visible on one screen and a palette change is judged side by side.
 *
 * The frames set `.light` / `.dark` on themselves, which is exactly how the tokens
 * in `globals.css` switch. Nothing here is imported by a production route.
 */
export function ComponentSection({
  title,
  description,
  id,
  children,
}: {
  title: string;
  description?: string;
  id?: string;
  children: React.ReactNode;
}) {
  const headingId = useId();

  return (
    <section id={id} aria-labelledby={headingId} className="space-y-3">
      <header className="space-y-1">
        <h2
          id={headingId}
          className="font-serif text-lg font-semibold tracking-tight"
        >
          {title}
        </h2>
        {description ? (
          <p className="max-w-prose text-sm text-muted-foreground">
            {description}
          </p>
        ) : null}
      </header>
      <div className="grid gap-4 lg:grid-cols-2">
        <ThemeFrame theme="light">{children}</ThemeFrame>
        <ThemeFrame theme="dark">{children}</ThemeFrame>
      </div>
    </section>
  );
}

function ThemeFrame({
  theme,
  children,
}: {
  theme: "light" | "dark";
  children: React.ReactNode;
}) {
  return (
    <div
      className={cn(
        theme,
        "overflow-hidden rounded-md border border-border bg-background text-foreground",
      )}
    >
      <div className="flex items-center justify-between border-b border-border px-3 py-1.5 text-xs font-medium text-muted-foreground">
        <span>{theme === "light" ? "Hell" : "Dunkel"}</span>
        <code className="font-mono">.{theme}</code>
      </div>
      <div className="space-y-4 p-4">{children}</div>
    </div>
  );
}

/** Rows of examples inside a theme frame, each with a caption. */
export function Example({
  caption,
  children,
  className,
}: {
  caption?: string;
  children: React.ReactNode;
  className?: string;
}) {
  return (
    <div className="space-y-1.5">
      {caption ? (
        <p className="text-xs font-medium text-muted-foreground">{caption}</p>
      ) : null}
      <div className={cn("flex flex-wrap items-center gap-2", className)}>
        {children}
      </div>
    </div>
  );
}

/**
 * Debug aid: outlines every interactive element, so a target smaller than the
 * 44x44 minimum from the accessibility bar is visible at a glance. The wrapper
 * class lives here so the gallery page only owns the toggle state.
 */
export const touchTargetOutline =
  "[&_a]:outline-dashed [&_a]:outline-2 [&_a]:outline-offset-2 [&_a]:outline-warning [&_button]:outline-dashed [&_button]:outline-2 [&_button]:outline-offset-2 [&_button]:outline-warning [&_input]:outline-dashed [&_input]:outline-2 [&_input]:outline-offset-2 [&_input]:outline-warning [&_select]:outline-dashed [&_select]:outline-2 [&_select]:outline-offset-2 [&_select]:outline-warning";
