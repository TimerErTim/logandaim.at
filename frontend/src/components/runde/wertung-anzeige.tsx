import type { SVGProps } from "react";

function Einzelpfeil(props: SVGProps<SVGSVGElement>) {
  return (
    <svg
      aria-hidden="true"
      viewBox="0 0 32 16"
      fill="none"
      stroke="currentColor"
      strokeWidth="1.8"
      strokeLinecap="round"
      strokeLinejoin="round"
      {...props}
    >
      <path d="M4 8h22" />
      <path d="m21 3 6 5-6 5" />
      <path d="M3 5 7 8l-4 3" />
      <path d="M5 6.5v3" />
    </svg>
  );
}

function Mehrfachpfeil(props: SVGProps<SVGSVGElement>) {
  return (
    <svg
      aria-hidden="true"
      viewBox="0 0 40 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="1.8"
      strokeLinecap="round"
      strokeLinejoin="round"
      {...props}
    >
      <path d="M4 4h24" />
      <path d="m24 1 5 3-5 3" />
      <path d="M2 3 5 4 2 5" />
      <path d="M4 12h30" />
      <path d="m29 8 7 4-7 4" />
      <path d="M2 10 5 12l-3 2" />
      <path d="M4 20h24" />
      <path d="m24 17 5 3-5 3" />
      <path d="M2 19 5 20l-3 1" />
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
      <Mehrfachpfeil className="h-4 w-7" />
    </span>
  );
}
