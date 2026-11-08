import { PflockSwatch } from "@/components/stammdaten/pflock-swatch";

export function PflockPlatzhalter({ farbe }: { farbe: string }) {
  return (
    <span
      aria-hidden="true"
      className="pointer-events-none absolute inset-0 z-10 flex items-center gap-2 overflow-hidden px-2 text-xs text-secondary"
    >
      <PflockSwatch farbe={farbe} />
      <span className="truncate">Pflock</span>
    </span>
  );
}
