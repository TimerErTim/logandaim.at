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
 * Progress of a multi-step flow.
 *
 * State is carried by the step number, the text and `aria-current`, not by colour
 * alone, so the sequence stays readable in daylight and to colour-blind users.
 */
export function WorkflowSteps({
  steps,
  currentStepId,
  optionalLabel = "optional",
}: WorkflowStepsProps) {
  return (
    <ol className="flex flex-wrap gap-2">
      {steps.map((step, index) => {
        const isCurrent = step.id === currentStepId;
        const isDone = step.done;

        return (
          <li
            key={step.id}
            aria-current={isCurrent ? "step" : undefined}
            className={cn(
              "flex items-center gap-2 rounded-md border px-3 py-1 text-xs font-medium",
              isCurrent
                ? "border-primary bg-primary/10 text-foreground"
                : isDone
                  ? "border-border text-muted-foreground"
                  : "border-border/60 text-muted-foreground/70",
            )}
          >
            <span
              className={cn(
                "flex size-5 items-center justify-center rounded-full text-[10px]",
                isCurrent
                  ? "bg-primary text-primary-foreground"
                  : isDone
                    ? "bg-border text-foreground"
                    : "bg-surface-muted text-muted-foreground",
              )}
            >
              {index + 1}
            </span>
            {step.label}
            {step.optional ? (
              <span className="text-muted-foreground">({optionalLabel})</span>
            ) : null}
          </li>
        );
      })}
    </ol>
  );
}
