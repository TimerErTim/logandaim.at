"use client";

import Link from "next/link";
import { useRouter } from "next/navigation";
import { useState } from "react";
import { apiErrorMessage } from "@/api/errors";
import { useStammdatenMutations } from "@/api/hooks";
import type { Runde } from "@/api/logandaim-api";
import { Alert } from "@/components/ui/alert";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Progress } from "@/components/ui/progress";
import { paths } from "@/lib/navigation";

export function Rundenabschluss({ runde }: { runde: Runde }) {
  const { rundeAbschliessen } = useStammdatenMutations();
  const router = useRouter();
  const [error, setError] = useState<string | null>(null);
  const done = runde.ziele.filter((ziel) => ziel.schüsse.length > 0).length;
  const abgeschlossen = runde.zustand === "Abgeschlossen";
  return (
    <Card>
      <CardHeader>
        <CardTitle>Rundenabschluss</CardTitle>
      </CardHeader>
      <CardContent className="space-y-4">
        {error ? <Alert variant="error">{error}</Alert> : null}
        <div className="grid gap-4 sm:grid-cols-3">
          <p>
            Erster Treffer{" "}
            <strong className="tabular-nums">
              {runde.gesamtErsterTreffer}
            </strong>
          </p>
          <p>
            Mehrschuss{" "}
            <strong className="tabular-nums">{runde.gesamtMehrschuss}</strong>
          </p>
          <p>
            Zielfortschritt{" "}
            <strong className="tabular-nums">
              {done} / {runde.ziele.length}
            </strong>
          </p>
        </div>
        <Progress
          value={runde.ziele.length ? (done / runde.ziele.length) * 100 : 0}
          aria-label={`Zielfortschritt: ${done} von ${runde.ziele.length}`}
        />
        <p className="text-sm text-secondary">
          Noch nicht erfasst: {runde.ziele.length - done} Ziele
        </p>
        {abgeschlossen ? (
          <Button variant="secondary" asChild>
            <Link href={`${paths.runden}/detail?id=${runde.id}`}>
              Zur Zusammenfassung
            </Link>
          </Button>
        ) : (
          <Button
            disabled={rundeAbschliessen.isPending}
            onClick={() => {
              setError(null);
              rundeAbschliessen.mutate(runde.id, {
                onSuccess: () =>
                  router.push(`${paths.runden}/detail?id=${runde.id}`),
                onError: (cause) =>
                  setError(
                    apiErrorMessage(cause, {
                      fallback: "Die Runde konnte nicht abgeschlossen werden.",
                    }),
                  ),
              });
            }}
          >
            Runde abschließen
          </Button>
        )}
      </CardContent>
    </Card>
  );
}
