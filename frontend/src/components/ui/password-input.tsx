"use client";

import { Eye, EyeOff } from "lucide-react";
import { type InputHTMLAttributes, useState } from "react";

import { cn } from "@/lib/cn";
import { Input } from "./input";

type PasswordInputProps = Omit<InputHTMLAttributes<HTMLInputElement>, "type">;

/**
 * A password field with a visibility toggle.
 *
 * The toggle lives *inside* the field's trailing area instead of beside it: a field
 * is a replaced element drawn at 44px, so a button next to it would either grow the
 * row or break the control line of a `FormRow`. The field therefore reserves
 * `pr-12` of trailing padding and the button occupies the whole 44×44 corner, which
 * satisfies the touch minimum without changing the drawn height of anything.
 *
 * The control is a real toggle, not an icon: it carries `aria-pressed`, a German name
 * that states what the next press does, and a focus ring drawn *inside* the field so
 * it stays visible against the control contour. It reuses `Input`, so the contour,
 * hover, focus, invalid, and disabled states are the field's own.
 *
 * `id` is forwarded to the `<input>` untouched, so `Field`'s `htmlFor` keeps pointing
 * at the element that actually receives focus.
 */
export function PasswordInput({
  className,
  disabled,
  ...props
}: PasswordInputProps) {
  const [visible, setVisible] = useState(false);

  return (
    <div className="relative">
      <Input
        {...props}
        className={cn("pr-12", className)}
        disabled={disabled}
        type={visible ? "text" : "password"}
      />
      <button
        type="button"
        aria-label={visible ? "Passwort verbergen" : "Passwort anzeigen"}
        aria-pressed={visible}
        disabled={disabled}
        onClick={() => setVisible((current) => !current)}
        className={cn(
          "absolute inset-y-0 right-0 flex w-11 items-center justify-center rounded-sm",
          "text-secondary transition-colors hover:text-ink",
          "focus-visible:outline-2 focus-visible:-outline-offset-2 focus-visible:outline-primary",
          "disabled:cursor-not-allowed disabled:text-muted",
        )}
      >
        {visible ? (
          <EyeOff aria-hidden="true" className="size-4" />
        ) : (
          <Eye aria-hidden="true" className="size-4" />
        )}
      </button>
    </div>
  );
}
