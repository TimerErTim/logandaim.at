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
 * The way: a journey through a workflow drawn like a route on the map.
 *
 * Numbered marks sit on a dashed way; a passed step is a filled mark, the step
 * ahead is an outline, and the current position is a filled triangle. State is
 * carried by the mark shape, the step number, the text and `aria-current`, never
 * by colour alone, so the route stays readable in daylight and to colour-blind
 * users. The current position is named in text ("Schritt 2 von 3").
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
                "flex size-6 items-center justify-center rounded-full text-[11px] font-semibold",
                isCurrent
                  ? "bg-primary/10 text-foreground ring-2 ring-primary"
                  : isDone
                    ? "bg-primary text-primary-foreground"
                    : "border-2 border-border text-muted-foreground",
              )}
            >
              {/* The triangle marks the current position; other steps stay numbered. */}
              {isCurrent ? (
                <svg
                  aria-hidden="true"
                  width="10"
                  height="10"
                  viewBox="0 0 10 10"
                  className="fill-primary"
                >
                  <path d="M5 1 L9 8 L1 8 Z" />
                </svg>
              ) : (
                index + 1
              )}
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
              <span aria-hidden="true" className="weg-linie h-0.5 w-8" />
            )}
          </li>
        );
      })}
    </ol>
  );
}
