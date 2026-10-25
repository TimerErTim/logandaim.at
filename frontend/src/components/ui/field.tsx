import {
  Children,
  cloneElement,
  type HTMLAttributes,
  isValidElement,
  type ReactElement,
  type ReactNode,
  useId,
} from "react";

import { cn } from "@/lib/cn";
import { Label } from "./label";

type FieldProps = HTMLAttributes<HTMLDivElement> & {
  label: string;
  /** Helper text shown under the control when there is no error. */
  hint?: string;
  /** Validation message; replaces the hint and is announced. */
  error?: string;
  /** Marks optional fields, as a printed form does. */
  optional?: boolean;
};

type LabelledProps = {
  id?: string;
  label?: string;
  "aria-label"?: string;
  "aria-labelledby"?: string;
};

/**
 * One labelled field on the form. The label sits above the control in small caps
 * with letter spacing, the way a printed form labels a line; the hint sits below in
 * the secondary ink and gives way to the error, which is announced.
 *
 * The label is *attached* to the control, not merely drawn above it: `Field` mints an
 * id, hands it to a single control child that has none, and points the real
 * `<label>` at it. Without that, a field looks labelled while an assistive technology
 * hears an unnamed input — the visual half of the identity passing and the accessible
 * half failing silently, which no contrast check would catch.
 *
 * A control that already names itself (`aria-label`, `aria-labelledby`, or a required
 * `label` prop such as `CheckpointSlider`'s) is left alone and the caption is drawn
 * as plain text, so no `htmlFor` ever points at nothing. The single child must be the
 * control itself, and it must forward `id` to the focusable element.
 */
export function Field({
  label,
  hint,
  error,
  optional = false,
  className,
  children,
  ...props
}: FieldProps) {
  const generatedId = useId();
  const single: ReactElement<LabelledProps> | null =
    Children.count(children) === 1 && isValidElement<LabelledProps>(children)
      ? children
      : null;

  const selfLabelled = Boolean(
    single?.props.label ??
      single?.props["aria-label"] ??
      single?.props["aria-labelledby"],
  );
  const controlId = selfLabelled
    ? undefined
    : (single?.props.id ?? generatedId);

  const control =
    single && controlId && !single.props.id
      ? cloneElement(single, { id: controlId })
      : (children as ReactNode);

  return (
    <div className={cn("space-y-1.5", className)} {...props}>
      <span className="flex items-baseline justify-between gap-2">
        <Label htmlFor={controlId}>{label}</Label>
        {optional ? (
          <span className="text-[11px] text-secondary">optional</span>
        ) : null}
      </span>
      {control}
      {error ? (
        <p role="alert" className="text-xs text-impact">
          {error}
        </p>
      ) : hint ? (
        <p className="text-xs text-secondary">{hint}</p>
      ) : null}
    </div>
  );
}
