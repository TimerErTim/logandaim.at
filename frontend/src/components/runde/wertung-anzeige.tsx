import type { SVGProps } from "react";

function Einzelpfeil(props: SVGProps<SVGSVGElement>) {
  return (
    <svg
      aria-hidden="true"
      viewBox="0 0 32 16"
      fill="none"
      stroke="currentColor"
      strokeWidth="2"
      strokeLinecap="square"
      strokeLinejoin="miter"
      {...props}
    >
      <path d="M2 8h23" />
      <path d="m20 3 9 5-9 5" />
      <path d="M3 5v6m3-6v6" />
    </svg>
  );
}

function Mehrfachpfeil(props: SVGProps<SVGSVGElement>) {
  return (
    <svg aria-hidden="true" viewBox="0 0 48 32" fill="currentColor" {...props}>
      <path d="M1 3.5h2.5L5 1l1.5 2.5H38V1l9 4.5-9 4.5V7H6.5L5 9.5 3.5 7H1z" />
      <path d="M1 13.5h4L7 11l2 2.5h32V10l7 6-7 6v-3.5H9L7 21l-2-2.5H1z" />
      <path d="M1 24.5h2.5L5 22l1.5 2.5H38V21l9 4.5-9 4.5v-3H6.5L5 29.5 3.5 27H1z" />
    </svg>
  );
}

export function WertungAnzeige({
  ersterTreffer,
  mehrschuss,
  className = "",
}: {
  ersterTreffer: number | undefined;
  mehrschuss: number | undefined;
  className?: string;
}) {
  return (
    <span
      role="img"
      aria-label={
        ersterTreffer === undefined || mehrschuss === undefined
          ? "Noch keine Wertung"
          : `Erster Treffer: ${ersterTreffer} Punkte, Mehrschuss: ${mehrschuss} Punkte`
      }
      className={`inline-flex shrink-0 items-center gap-1 whitespace-nowrap font-mono tabular-nums text-ink ${className}`}
    >
      <span aria-hidden="true">{ersterTreffer ?? "—"}</span>
      <Einzelpfeil className="h-3 w-5" />
      <span aria-hidden="true">/</span>
      <span aria-hidden="true">{mehrschuss ?? "—"}</span>
      <Mehrfachpfeil className="h-4 w-6" />
    </span>
  );
}
