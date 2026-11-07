import type { SVGProps } from "react";

function Einzelpfeil(props: SVGProps<SVGSVGElement>) {
  return (
    <svg aria-hidden="true" viewBox="0 0 24 24" fill="none" {...props}>
      <path d="M4 20 18 6M18 6H9M18 6v9" />
    </svg>
  );
}

function Mehrfachpfeil(props: SVGProps<SVGSVGElement>) {
  return (
    <svg aria-hidden="true" viewBox="0 0 24 24" fill="none" {...props}>
      <path d="m3 18 8-8m0 0H5m6 0v6M9 20l8-8m0 0h-6m6 0v6m-2-10 5-5m0 0h-5m5 0v5" />
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
      <Einzelpfeil
        aria-hidden="true"
        className="size-4 stroke-current stroke-[1.7]"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
      <span aria-hidden="true">/</span>
      <span aria-hidden="true">{mehrschuss ?? "—"}</span>
      <Mehrfachpfeil
        aria-hidden="true"
        className="size-4 stroke-current stroke-[1.7]"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
    </span>
  );
}
