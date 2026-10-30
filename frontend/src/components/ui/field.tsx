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
 * The label row is a fixed 18px high, so a field with an `optional` marker and one
 * without start their control at the same y. Neighbours in a row therefore line up
 * without a form having to align anything by hand: the label row plus its gap is the
 * offset that `FormActions` repeats.
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
      <span className="flex min-h-[18px] items-center justify-between gap-2">
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

type FormRowProps = HTMLAttributes<HTMLDivElement> & {
  /** Buttons that belong to this row; placed in an aligned trailing cell. */
  actions?: ReactNode;
};

/**
 * A row-shaped form: fields side by side with their actions on the same line.
 *
 * The row is `items-start`, never `items-end`. Bottom alignment looks right only
 * while every field is exactly as tall as its neighbour; the moment one field shows
 * a hint or an error, `items-end` pushes its *control* up and the row stops reading
 * as one line of a form. Starting at the top and reserving the label row instead
 * keeps every control at the same y, whatever the helper text below it does.
 *
 * The column count is the caller's `className` (`sm:grid-cols-2`,
 * `sm:grid-cols-[1fr_auto]`, …); the actions cell is one more grid cell.
 */
export function FormRow({
  actions,
  className,
  children,
  ...props
}: FormRowProps) {
  return (
    <div className={cn("grid items-start gap-4", className)} {...props}>
      {children}
      {actions ? <FormActions>{actions}</FormActions> : null}
    </div>
  );
}

/**
 * The actions cell of a `FormRow`. It reserves the field's label row (18px) plus the
 * `space-y-1.5` gap (6px) as top padding, so a `Button size="field"` starts exactly
 * where the adjacent control starts and ends where it ends — both are 44px tall.
 */
export function FormActions({
  className,
  children,
  ...props
}: HTMLAttributes<HTMLDivElement>) {
  return (
    <div
      className={cn("flex items-center gap-2 self-start pt-6", className)}
      {...props}
    >
      {children}
    </div>
  );
}
