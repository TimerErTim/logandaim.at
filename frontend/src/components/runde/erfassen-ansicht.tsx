"use client";

import { useMemo, useState } from "react";
import { apiErrorMessage } from "@/api/errors";
import { useStammdatenMutations } from "@/api/hooks";
import { useErfolgsmeldung } from "@/api/hooks/erfolgsmeldung";
import type {
  Runde,
  SchusspositionInput,
  Wertungszone,
} from "@/api/logandaim-api";
import { Erfolgsmeldung } from "@/components/feedback/erfolgsmeldung";
import { Alert } from "@/components/ui/alert";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Progress } from "@/components/ui/progress";
import { Separator } from "@/components/ui/separator";
import { anfangsPosition } from "@/lib/runden-position";
import { PositionSteuerung } from "./position-steuerung";
import { Rundenabschluss } from "./rundenabschluss";
import { SchussLedger } from "./schuss-ledger";
import { WertungszoneGruppe } from "./wertungszone-gruppe";
import { Zieluebersicht } from "./zieluebersicht";

export function ErfassenAnsicht({ runde }: { runde: Runde }) {
  const { schussRegistrieren, schussAendern, schussLoeschen } =
    useStammdatenMutations();
  const meldung = useErfolgsmeldung();
  const [zielIndex, setZielIndex] = useState(0);
  const ziel = runde.ziele[zielIndex];
  const [positionen, setPositionen] = useState<
    Record<string, SchusspositionInput | null>
  >({});
  const [fehler, setFehler] = useState<string | null>(null);
  const busy =
    schussRegistrieren.isPending ||
    schussAendern.isPending ||
    schussLoeschen.isPending;

  const zielCards = useMemo(
    () => runde.ziele.map((eintrag, index) => ({ eintrag, index })),
    [runde.ziele],
  );
  const erfassen = (
    selected: (typeof runde.ziele)[number],
    zone: Wertungszone,
  ) => {
    const position =
      positionen[selected.zielId] ??
      anfangsPosition(selected, selected.schüsse);
    meldung.verwerfen();
    setFehler(null);
    schussRegistrieren.mutate(
      {
        rundeId: runde.id,
        schuss: {
          zielId: selected.zielId,
          wertungszone: zone,
          position,
        },
      },
      {
        onSuccess: () => meldung.melden("Schuss gespeichert."),
        onError: (error) =>
          setFehler(
            apiErrorMessage(error, {
              fallback: "Der Schuss konnte nicht gespeichert werden.",
            }),
          ),
      },
    );
  };
  const inhalt = (selected: (typeof runde.ziele)[number], index: number) => (
    <Card
      key={selected.zielId}
      className={index === zielIndex ? "ring-1 ring-primary" : ""}
    >
      <CardHeader>
        <CardTitle>
          Ziel {index + 1} · {selected.tiername}
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-5">
        <div className="grid gap-5 xl:grid-cols-[minmax(0,1fr)_minmax(16rem,0.8fr)] xl:items-start">
          <PositionSteuerung
            key={selected.zielId}
            ziel={selected}
            pflöcke={runde.pflöcke}
            value={
              positionen[selected.zielId] ??
              anfangsPosition(selected, selected.schüsse)
            }
            onChange={(next) =>
              setPositionen((current) => ({
                ...current,
                [selected.zielId]: next,
              }))
            }
          />
          <WertungszoneGruppe
            disabled={runde.zustand !== "InBearbeitung"}
            busy={busy}
            onSelect={(zone) => erfassen(selected, zone)}
          />
        </div>
        <Separator />
        <SchussLedger
          ziel={selected}
          reihenfolge={index + 1}
          editable={
            runde.zustand === "InBearbeitung" ||
            runde.zustand === "Abgeschlossen"
          }
          onEdit={(schussId, wertungszone, shotPosition) =>
            schussAendern.mutate({
              rundeId: runde.id,
              schussId,
              schuss: { wertungszone, position: shotPosition },
            })
          }
          onDelete={(schussId) =>
            schussLoeschen.mutate({ rundeId: runde.id, schussId })
          }
        />
        {index === zielIndex ? (
          <div className="flex justify-between gap-3 lg:hidden">
            <Button
              variant="secondary"
              disabled={index === 0}
              onClick={() => setZielIndex(index - 1)}
            >
              Zurück
            </Button>
            <Button variant="secondary" onClick={() => setZielIndex(index + 1)}>
              Weiter
            </Button>
          </div>
        ) : null}
      </CardContent>
    </Card>
  );

  return (
    <div className="space-y-6 p-6">
      <div className="sticky top-0 z-20 -mx-6 flex flex-col gap-2 border-y border-subtle bg-surface px-6 py-3">
        <div className="flex items-center justify-between gap-4">
          <div className="min-w-0">
            <p className="truncate font-medium text-ink">{runde.parkourName}</p>
            <p className="text-sm text-secondary">
              {new Date(`${runde.datum}T00:00:00`).toLocaleDateString("de-AT", {
                dateStyle: "long",
              })}
            </p>
          </div>
          <span className="shrink-0 text-sm tabular-nums text-secondary">
            {runde.ziele.filter((eintrag) => eintrag.schüsse.length > 0).length}{" "}
            / {runde.ziele.length} Ziele
          </span>
        </div>
        <Progress
          value={
            runde.ziele.length
              ? (runde.ziele.filter((eintrag) => eintrag.schüsse.length > 0)
                  .length /
                  runde.ziele.length) *
                100
              : 0
          }
          aria-label={`Zielfortschritt: ${runde.ziele.filter((eintrag) => eintrag.schüsse.length > 0).length} von ${runde.ziele.length}`}
        />
      </div>
      {fehler ? <Alert variant="error">{fehler}</Alert> : null}
      <Erfolgsmeldung meldung={meldung.meldung} />
      <div className="lg:hidden">
        <Zieluebersicht
          ziele={runde.ziele}
          currentIndex={zielIndex}
          onSelect={setZielIndex}
        />
        <div className="mt-4 space-y-4">
          {ziel ? inhalt(ziel, zielIndex) : null}
          {zielIndex >= runde.ziele.length ? (
            <Rundenabschluss runde={runde} />
          ) : null}
        </div>
      </div>
      <div className="hidden space-y-4 lg:block">
        {zielCards.map(({ eintrag, index }) => inhalt(eintrag, index))}
        <Rundenabschluss runde={runde} />
      </div>
    </div>
  );
}
