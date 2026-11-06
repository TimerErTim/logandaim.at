/**
 * WCAG contrast helper for the `/dev` gallery.
 *
 * The gallery measures the real computed colours of rendered elements, so a palette
 * edit in `globals.css` turns into a pass or fail here without touching this file.
 * Nothing in production imports it.
 */

const NAMED_COLOURS: Record<string, [number, number, number]> = {
  white: [255, 255, 255],
  black: [0, 0, 0],
};

function parseColour(value: string): [number, number, number] | null {
  const trimmed = value.trim().toLowerCase();

  const named = NAMED_COLOURS[trimmed];
  if (named) {
    return named;
  }

  const hex = trimmed.match(/^#([0-9a-f]{3}|[0-9a-f]{6})$/);
  if (hex?.[1]) {
    const digits = hex[1];
    const expand = (part: string) => Number.parseInt(part, 16);
    return digits.length === 3
      ? [
          expand(digits[0] + digits[0]),
          expand(digits[1] + digits[1]),
          expand(digits[2] + digits[2]),
        ]
      : [
          expand(digits.slice(0, 2)),
          expand(digits.slice(2, 4)),
          expand(digits.slice(4, 6)),
        ];
  }

  const rgb = trimmed.match(/^rgba?\(\s*([\d.]+)[\s,]+([\d.]+)[\s,]+([\d.]+)/);
  if (rgb) {
    return [Number(rgb[1]), Number(rgb[2]), Number(rgb[3])];
  }

  return null;
}

function relativeLuminance([r, g, b]: [number, number, number]): number {
  const channel = (value: number) => {
    const normalised = value / 255;
    return normalised <= 0.04045
      ? normalised / 12.92
      : ((normalised + 0.055) / 1.055) ** 2.4;
  };
  return 0.2126 * channel(r) + 0.7152 * channel(g) + 0.0722 * channel(b);
}

/**
 * WCAG 2.x contrast ratio between two computed CSS colours.
 *
 * Returns `null` when either colour cannot be parsed, which is what happens for a
 * transparent background; the caller shows "unbekannt" instead of a wrong number.
 */
export function contrastRatio(
  foreground: string,
  background: string,
): number | null {
  const fg = parseColour(foreground);
  const bg = parseColour(background);
  if (!fg || !bg) {
    return null;
  }

  const ratios = [relativeLuminance(fg), relativeLuminance(bg)].sort(
    (a, b) => b - a,
  );
  const lighter = ratios[0] ?? 0;
  const darker = ratios[1] ?? 0;

  return (lighter + 0.05) / (darker + 0.05);
}

/** AA threshold for normal text; large text is allowed 3:1. */
export const AA_NORMAL_TEXT = 4.5;
