import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";

/**
 * Merge class names, with later Tailwind utilities winning over earlier ones of the
 * same kind. Component call sites pass overrides (`className`) and rely on this,
 * so do not replace it with a plain join.
 */
export function cn(...classes: ClassValue[]): string {
  return twMerge(clsx(classes));
}
