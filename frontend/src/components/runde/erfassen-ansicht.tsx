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
import { Alert } from "@/components/ui/alert";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Progress } from "@/components/ui/progress";
import { Separator } from "@/components/ui/separator";
import { anfangsPosition } from "@/lib/runden-position";
import { PositionSteuerung } from "./position-steuerung";
import { Rundenabschluss } from "./rundenabschluss";
import { SchussLedger } from "./schuss-ledger";
import { WertungAnzeige } from "./wertung-anzeige";
import { WertungszoneGruppe } from "./wertungszone-gruppe";
import { Zieluebersicht } from "./zieluebersicht";

export function ErfassenAnsicht({ runde }: { runde: Runde }) {
  const { schussRegistrieren, schussAendern, schussLoeschen } =
    useStammdatenMutations();
  const meldung = useErfolgsmeldung();
  const [zielIndex, setZielIndex] = useState(0);
  const ziel = runde.ziele[zielIndex];
  const schussAnzahl = runde.ziele.reduce(
    (gesamt, eintrag) => gesamt + eintrag.schüsse.length,
    0,
  );
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
      <CardHeader className="pt-4 pb-2 [&>div:last-child]:mt-2">
        <div className="flex min-w-0 items-center justify-between gap-3">
          <CardTitle className="min-w-0 flex-1 truncate">
            {index + 1}. {selected.tiername}
          </CardTitle>
          <WertungAnzeige
            ersterTreffer={selected.wertung?.ersterTreffer}
            mehrschuss={selected.wertung?.mehrschuss}
            className="text-base font-semibold"
          />
        </div>
      </CardHeader>
      <CardContent className="space-y-3 pt-3">
        <div className="space-y-3">
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
          <Separator />
          <fieldset className="min-w-0 space-y-1.5">
            <legend className="min-h-[18px] text-xs font-semibold tracking-[0.09em] text-secondary uppercase">
              Schüsse
            </legend>
            <WertungszoneGruppe
              disabled={runde.zustand !== "InBearbeitung"}
              busy={busy}
              onSelect={(zone) => erfassen(selected, zone)}
            />
          </fieldset>
        </div>
        <div className="pt-1">
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
        </div>
      </CardContent>
    </Card>
  );

  return (
    <div className="px-6 pb-24">
      <div className="sticky top-0 z-40 -mx-6 flex flex-col gap-2 border-y border-subtle bg-surface px-6 py-3">
        <div className="flex items-center justify-between gap-4">
          <div className="min-w-0 flex-1">
            <p className="truncate font-medium text-ink">{runde.parkourName}</p>
            <p className="font-mono text-sm tabular-nums text-secondary">
              {schussAnzahl} Schüsse
            </p>
          </div>
          <div className="flex shrink-0 flex-col items-end gap-1">
            <div className="flex items-center gap-2">
              <WertungAnzeige
                ersterTreffer={runde.gesamtErsterTreffer}
                mehrschuss={runde.gesamtMehrschuss}
                className="text-base font-semibold"
              />
            </div>
            <div className="flex items-center gap-1 text-sm tabular-nums text-secondary">
              <span>
                {
                  runde.ziele.filter((eintrag) => eintrag.schüsse.length > 0)
                    .length
                }{" "}
                / {runde.ziele.length} Ziele
              </span>
              <Zieluebersicht
                ziele={runde.ziele}
                currentIndex={zielIndex}
                onSelect={setZielIndex}
              />
            </div>
          </div>
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
      <div className="py-6">
        <div className="space-y-4 lg:hidden">
          {ziel ? inhalt(ziel, zielIndex) : null}
          {zielIndex >= runde.ziele.length ? (
            <Rundenabschluss runde={runde} />
          ) : null}
        </div>
        <div className="hidden space-y-4 lg:block">
          {zielCards.map(({ eintrag, index }) => inhalt(eintrag, index))}
          <Rundenabschluss runde={runde} />
        </div>
      </div>
      <div className="fixed inset-x-0 bottom-0 z-30 border-t border-subtle bg-surface p-3 lg:hidden">
        <div className="mx-auto flex max-w-3xl justify-between gap-3">
          <Button
            variant="secondary"
            disabled={zielIndex === 0}
            onClick={() => setZielIndex((index) => Math.max(0, index - 1))}
          >
            Zurück
          </Button>
          <Button
            variant="secondary"
            onClick={() =>
              setZielIndex((index) => Math.min(runde.ziele.length, index + 1))
            }
          >
            Weiter
          </Button>
        </div>
      </div>
    </div>
  );
}
