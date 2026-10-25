import { cn } from "@/lib/cn";

type Step = {
  id: string;
  label: string;
  done: boolean;
  optional?: boolean;
};

type WorkflowStepsProps = {
  steps: Step[];
  currentStepId: string;
  /** Iteration labels come from the locale source, never from this component. */
  optionalLabel?: string;
};

/**
 * A workflow as a ruled sequence: numbered marks joined by a short stitched line,
 * like the entries of a logbook. A passed step is a filled mark, the step ahead an
 * outline, and the current step is confirmed by a stamp-like ring.
 *
 * State is carried by the mark shape, the step number, the text and `aria-current`,
 * never by colour alone, so the sequence stays readable in daylight and to
 * colour-blind users. The current step is named in text ("Schritt 2 von 3").
 */
export function WorkflowSteps({
  steps,
  currentStepId,
  optionalLabel = "optional",
}: WorkflowStepsProps) {
  const aktuell = steps.findIndex((step) => step.id === currentStepId);

  return (
    <ol
      aria-label={
        aktuell >= 0
          ? `Schritt ${aktuell + 1} von ${steps.length}: ${steps[aktuell]?.label}`
          : undefined
      }
      className="flex flex-wrap items-center gap-x-2 gap-y-3"
    >
      {steps.map((step, index) => {
        const isCurrent = step.id === currentStepId;
        const isDone = step.done;
        const isLast = index === steps.length - 1;

        return (
          <li
            key={step.id}
            aria-current={isCurrent ? "step" : undefined}
            className="flex items-center gap-2"
          >
            <span
              className={cn(
                "flex size-6 items-center justify-center rounded-sm text-[11px] font-semibold tabular-nums",
                isCurrent
                  ? "border-2 border-primary bg-surface text-foreground"
                  : isDone
                    ? "bg-primary text-primary-foreground"
                    : "border border-border text-muted-foreground",
              )}
            >
              {index + 1}
            </span>
            <span
              className={cn(
                "text-sm",
                isCurrent
                  ? "font-semibold text-foreground"
                  : isDone
                    ? "text-foreground"
                    : "text-muted-foreground",
              )}
            >
              {step.label}
              {isCurrent ? (
                <span className="ml-1.5 font-mono text-xs text-muted-foreground">
                  Schritt {index + 1} von {steps.length}
                </span>
              ) : null}
            </span>
            {step.optional ? (
              <span className="text-xs text-muted-foreground">
                ({optionalLabel})
              </span>
            ) : null}
            {isLast ? null : (
              <span
                aria-hidden="true"
                className="h-px w-8 border-t border-dashed border-border"
              />
            )}
          </li>
        );
      })}
    </ol>
  );
}
