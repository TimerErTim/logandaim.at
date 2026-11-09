"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";
import { useEffect, useState } from "react";
import { useAuth } from "@/api/auth-provider";
import { useRundenQuery } from "@/api/hooks";
import { useRundeQuery } from "@/api/hooks/queries";
import { Button } from "@/components/ui/button";
import { Progress } from "@/components/ui/progress";
import { isExactPath, paths } from "@/lib/navigation";

export function RundenBand() {
  const phase = useAuth();
  const pathname = usePathname();
  const runden = useRundenQuery();
  const offen = runden.data?.find((runde) => runde.zustand === "InBearbeitung");
  const detail = useRundeQuery(offen?.id ?? null);
  const [now, setNow] = useState(Date.now());
  useEffect(() => {
    if (!offen) return;
    const timer = window.setInterval(() => setNow(Date.now()), 1000);
    return () => window.clearInterval(timer);
  }, [offen]);
  if (
    phase.status !== "authenticated" ||
    !offen ||
    isExactPath(pathname, paths.rundenErfassen) ||
    isExactPath(pathname, paths.rundenAbschluss)
  )
    return null;
  const minutes = Math.max(
    0,
    Math.floor((now - Date.parse(offen.begonnenAm)) / 60000),
  );
  const done = detail.data?.zieleMitWertung ?? offen.anzahlErfassteZiele;
  const total = detail.data?.ziele.length ?? offen.anzahlZiele;
  return (
    <aside
      aria-label="Offene Runde"
      className="flex flex-wrap items-center gap-x-6 gap-y-3 border-t border-subtle bg-surface px-6 py-4"
    >
      <div className="min-w-0 flex-1">
        <p className="font-medium text-ink">{offen.parkourName}</p>
        <p aria-live="off" className="text-xs tabular-nums text-secondary">
          Seit {minutes} Min.
        </p>
      </div>
      <div className="min-w-48 flex-[2] space-y-1.5">
        <div className="flex items-center justify-between text-xs tabular-nums">
          <span className="font-medium text-ink">Zielfortschritt</span>
          <span className="text-secondary">
            {done} / {total}
          </span>
        </div>
        <Progress
          value={total ? (done / total) * 100 : 0}
          aria-label={`Zielfortschritt: ${done} von ${total}`}
          className="h-3"
        />
      </div>
      <Button size="sm" asChild>
        <Link href={`${paths.rundenErfassen}?id=${offen.id}`}>Fortsetzen</Link>
      </Button>
    </aside>
  );
}
