import { PflockSwatch } from "@/components/stammdaten/pflock-swatch";

/**
 * The swatch and its word, drawn over the meter field while the stored form is a colour.
 * `text` is omitted when the field already carries a typed value: the swatch then marks
 * the last stored form beside the reader's own digits, instead of covering them.
 */
export function PflockPlatzhalter({
  farbe,
  text = "Pflock",
}: {
  farbe: string;
  text?: string | null;
}) {
  return (
    <span
      aria-hidden="true"
      className="pointer-events-none absolute inset-0 z-10 flex items-center gap-2 overflow-hidden px-2 text-xs text-secondary"
    >
      <PflockSwatch farbe={farbe} />
      {text ? <span className="truncate">{text}</span> : null}
    </span>
  );
}
