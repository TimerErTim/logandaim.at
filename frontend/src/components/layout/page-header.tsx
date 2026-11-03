"use client";

import type { ReactNode } from "react";
import { RegisterLine, SheetNumber } from "@/components/ui/paper-rule";
import { cn } from "@/lib/cn";

type PageHeaderProps = {
  /**
   * The running head, e.g. `["Logbuch", "Anlagen"]`. Words, not links: it orients the
   * reader the way a ledger prints its register line across every spread.
   */
  register: readonly string[];
  title: ReactNode;
  /** The printed plate number in the margin; omit where the page has none. */
  plate?: string;
  description?: ReactNode;
  /**
   * The page's single primary action, plus at most quiet alternatives. Rendered in a
   * row that wraps under the title, so a narrow screen stacks rather than squeezes.
   */
  actions?: ReactNode;
  className?: string;
};

/**
 * The head of a page of the logbook: register line, serif title, plate number, a ruled
 * hairline, and one action row.
 *
 * Every route assembles its head the same way, so a page cannot drift into a
 * different running head than its navigation entry — the mismatch between "Logbuch"
 * in the sidebar and "Öffentlich · Parkour · Anlagen" on the page was a hand-built
 * header, not a decision.
 *
 * The hairline is `rule-ticks`: the printed form's ruling, hairline plus ticks, and it
 * closes the title block exactly as `CardHeader` does for a card.
 */
export function PageHeader({
  register,
  title,
  plate,
  description,
  actions,
  className,
}: PageHeaderProps) {
  return (
    <header className={cn("space-y-3", className)}>
      <RegisterLine items={register} />

      <div className="flex flex-wrap items-center gap-3">
        {plate ? <SheetNumber>{plate}</SheetNumber> : null}
        <h1 className="font-serif text-2xl font-semibold tracking-tight text-ink">
          {title}
        </h1>
      </div>

      {description ? (
        <p className="max-w-2xl text-sm text-secondary">{description}</p>
      ) : null}

      <div aria-hidden="true" className="rule-ticks h-px w-full" />

      {actions ? (
        <div className="flex flex-wrap items-center gap-3">{actions}</div>
      ) : null}
    </header>
  );
}
