"use client";

import { Check, CircleAlert } from "lucide-react";
import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useRef,
  useState,
} from "react";
import { subscribeToGeneralErrors } from "@/api/fehlermeldung";
import { alertVariants } from "@/components/ui/alert";
import { cn } from "@/lib/cn";

type Tone = "success" | "error";
type Toast = { id: string; message: string; tone: Tone };
type ToastContextValue = {
  success: (id: string, message: string) => void;
  /** The failure note: one slot, replaced by the next failure. */
  error: (id: string, message: string) => void;
  dismiss: (id: string) => void;
};

const ToastContext = createContext<ToastContextValue | null>(null);
const TOAST_DURATION_MS = 5000;

export function ToastProvider({ children }: { children: React.ReactNode }) {
  const [toasts, setToasts] = useState<Toast[]>([]);
  const [ansage, setAnsage] = useState("");
  const timers = useRef(new Map<string, ReturnType<typeof setTimeout>>());

  const dismiss = useCallback((id: string) => {
    const timer = timers.current.get(id);
    if (timer) clearTimeout(timer);
    timers.current.delete(id);
    setToasts((current) => current.filter((toast) => toast.id !== id));
  }, []);

  const show = useCallback((toast: Toast) => {
    const timer = timers.current.get(toast.id);
    if (timer) clearTimeout(timer);
    setToasts((current) => [
      ...current.filter((entry) => entry.id !== toast.id),
      toast,
    ]);
    timers.current.set(
      toast.id,
      setTimeout(() => {
        timers.current.delete(toast.id);
        setToasts((current) =>
          current.filter((entry) => entry.id !== toast.id),
        );
      }, TOAST_DURATION_MS),
    );
  }, []);

  const success = useCallback(
    (id: string, message: string) => {
      show({ id, message, tone: "success" });
      setAnsage(message);
    },
    [show],
  );

  /**
   * A failure is announced by its own `role="alert"`, so it does not also go through the
   * polite `ansage` region: one failure, one announcement.
   */
  const error = useCallback(
    (id: string, message: string) => show({ id, message, tone: "error" }),
    [show],
  );

  useEffect(
    () => () => {
      for (const timer of timers.current.values()) clearTimeout(timer);
      timers.current.clear();
    },
    [],
  );

  // The `QueryClient` cannot reach this provider from outside React; it publishes
  // unreported mutation failures through `reportGeneralError`, and they land in the
  // same slot as a site-level failure note would.
  useEffect(
    () => subscribeToGeneralErrors((text) => error("allgemein", text)),
    [error],
  );

  const value = useMemo(
    () => ({ success, error, dismiss }),
    [success, error, dismiss],
  );

  return (
    <ToastContext.Provider value={value}>
      {children}
      <output aria-atomic="true" className="sr-only" aria-live="polite">
        {ansage}
      </output>
      <div className="pointer-events-none fixed inset-x-0 bottom-20 z-50 flex flex-col items-center gap-2 px-4 lg:bottom-6">
        {toasts.map((toast) => {
          const className = cn(
            alertVariants({
              variant: toast.tone === "error" ? "error" : "success",
            }),
            "paper-overlay pointer-events-auto w-fit max-w-full",
          );
          const content = (
            <>
              <span
                aria-hidden="true"
                className={cn(
                  "mt-0.5 shrink-0",
                  toast.tone === "error" ? "text-impact" : "text-success",
                )}
              >
                {toast.tone === "error" ? (
                  <CircleAlert className="size-4" />
                ) : (
                  <Check className="size-4" />
                )}
              </span>
              <span className="min-w-0">{toast.message}</span>
            </>
          );
          return toast.tone === "error" ? (
            <div key={toast.id} role="alert" className={className}>
              {content}
            </div>
          ) : (
            <output key={toast.id} aria-atomic="true" className={className}>
              {content}
            </output>
          );
        })}
      </div>
    </ToastContext.Provider>
  );
}

export function useToast() {
  const context = useContext(ToastContext);
  if (!context) {
    throw new Error("useToast must be used within ToastProvider");
  }
  return context;
}
