"use client";

import { useId } from "react";

import { cn } from "@/lib/cn";

/**
 * One specimen plate in the design gallery.
 *
 * The page is a logbook of the system itself, so every section is laid out like a
 * page of one: an index number in mono down the left, the title in the book face, a
 * one-line rationale, then a ruled rule before the specimens. The same content is
 * drawn twice — once on ivory paper, once on charcoal — so a palette change is
 * judged side by side instead of from memory.
 *
 * Only `page.dev.tsx` imports this. Nothing here ships.
 */
export function ComponentSection({
  title,
  description,
  id,
  index,
  children,
}: {
  title: string;
  description?: string;
  id?: string;
  /** Two-digit plate number, printed in the margin. */
  index?: string;
  children: React.ReactNode;
}) {
  const headingId = useId();

  return (
    <section id={id} aria-labelledby={headingId} className="scroll-mt-6">
      <header className="flex items-baseline gap-4 pb-3">
        {index ? (
          <span
            aria-hidden="true"
            className="font-mono text-xs text-secondary tabular-nums"
          >
            {index}
          </span>
        ) : null}
        <div className="min-w-0 flex-1 space-y-1">
          <h2
            id={headingId}
            className="font-serif text-lg font-semibold tracking-tight"
          >
            {title}
          </h2>
          {description ? (
            <p className="max-w-prose text-sm text-secondary">{description}</p>
          ) : null}
        </div>
      </header>

      <div aria-hidden="true" className="rule-ticks mb-4 h-px w-full" />

      <div className="grid gap-4 lg:grid-cols-2">
        <ThemePlate theme="light">{children}</ThemePlate>
        <ThemePlate theme="dark">{children}</ThemePlate>
      </div>
    </section>
  );
}

/**
 * A plate of paper with the specimens on it. The `.light` / `.dark` class is what
 * actually switches the tokens, so a frame is a complete theme scope rather than a
 * tint, and both are truthful.
 */
function ThemePlate({
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
        "overflow-hidden rounded-md border border-subtle bg-canvas text-ink",
      )}
    >
      <div className="flex items-center justify-between border-b border-subtle px-4 py-2">
        <span className="text-[10px] font-semibold tracking-[0.09em] text-secondary uppercase">
          {theme === "light" ? "Papier" : "Lampe"}
        </span>
        <code className="font-mono text-[10px] text-secondary">.{theme}</code>
      </div>
      <div className="space-y-5 p-4">{children}</div>
    </div>
  );
}

/**
 * A labelled row of specimens inside a plate. The caption is a field name on a
 * form — tracked caps in the secondary ink — so it labels the row without competing
 * with whatever is being shown.
 */
export function Example({
  caption,
  children,
  className,
}: {
  caption?: string;
  children: React.ReactNode;
  className?: string;
}) {
  const captionId = useId();

  return (
    <div className="space-y-2">
      {caption ? (
        <p
          id={captionId}
          className="text-[10px] font-semibold tracking-[0.09em] text-secondary uppercase"
        >
          {caption}
        </p>
      ) : null}
      <div className={cn("flex flex-wrap items-center gap-2", className)}>
        {children}
      </div>
    </div>
  );
}

/**
 * Debug aid: outlines every interactive element, so a target smaller than the
 * 44x44 minimum from the accessibility bar is visible at a glance. Dashed amber is
 * intentionally off-system: it is a measuring overlay, not part of the identity.
 */
export const touchTargetOutline = [
  "[&_a]:outline-2 [&_a]:outline-offset-2 [&_a]:outline-dashed [&_a]:outline-warning",
  "[&_button]:outline-2 [&_button]:outline-offset-2 [&_button]:outline-dashed [&_button]:outline-warning",
  "[&_input]:outline-2 [&_input]:outline-offset-2 [&_input]:outline-dashed [&_input]:outline-warning",
  "[&_select]:outline-2 [&_select]:outline-offset-2 [&_select]:outline-dashed [&_select]:outline-warning",
  "[&_textarea]:outline-2 [&_textarea]:outline-offset-2 [&_textarea]:outline-dashed [&_textarea]:outline-warning",
].join(" ");
