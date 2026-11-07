import type { SVGProps } from "react";

function Einzelpfeil(props: SVGProps<SVGSVGElement>) {
  return (
    <svg aria-hidden="true" viewBox="0 0 32 32" fill="none" {...props}>
      <path d="M4 28 26 6" />
      <path d="m20 6 6 0 0 6" />
      <path d="m4 28 1-6 5 5-6 1Z" />
      <path d="m8 24-4-1 1-4m8 1-4-1 1-4m8 1-4-1 1-4" />
      <path d="m4 28 5-5" />
    </svg>
  );
}

function Mehrfachpfeil(props: SVGProps<SVGSVGElement>) {
  return (
    <svg aria-hidden="true" viewBox="0 0 32 32" fill="none" {...props}>
      <path d="M4 28 26 6" />
      <path d="m20 6 6 0 0 6" />
      <path d="m4 28 1-6 5 5-6 1Z" />
      <path d="m8 24-4-1 1-4m8 1-4-1 1-4m8 1-4-1 1-4" />
      <path d="m4 28 5-5" />
      <path d="M8 28 28 8M13 29 29 13M3 23 23 3" />
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
