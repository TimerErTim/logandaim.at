"use client";

import { Check } from "lucide-react";
import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useRef,
  useState,
} from "react";
import { alertVariants } from "@/components/ui/alert";
import { cn } from "@/lib/cn";

type Toast = { id: string; message: string };
type ToastContextValue = {
  success: (id: string, message: string) => void;
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

  const success = useCallback((id: string, message: string) => {
    const timer = timers.current.get(id);
    if (timer) clearTimeout(timer);
    setToasts((current) => [
      ...current.filter((toast) => toast.id !== id),
      { id, message },
    ]);
    setAnsage(message);
    timers.current.set(
      id,
      setTimeout(() => {
        timers.current.delete(id);
        setToasts((current) => current.filter((toast) => toast.id !== id));
      }, TOAST_DURATION_MS),
    );
  }, []);

  useEffect(
    () => () => {
      for (const timer of timers.current.values()) clearTimeout(timer);
      timers.current.clear();
    },
    [],
  );
  const value = useMemo(() => ({ success, dismiss }), [success, dismiss]);

  return (
    <ToastContext.Provider value={value}>
      {children}
      <output aria-atomic="true" className="sr-only" aria-live="polite">
        {ansage}
      </output>
      <div className="pointer-events-none fixed inset-x-0 bottom-20 z-50 flex flex-col items-center gap-2 px-4 lg:bottom-6">
        {toasts.map((toast) => (
          <output
            key={toast.id}
            aria-atomic="true"
            className={cn(
              alertVariants({ variant: "success" }),
              "paper-overlay pointer-events-auto w-fit max-w-full",
            )}
          >
            <span aria-hidden="true" className="mt-0.5 shrink-0 text-success">
              <Check className="size-4" />
            </span>
            <span className="min-w-0">{toast.message}</span>
          </output>
        ))}
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
