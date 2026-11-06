"use client";

import { useMemo, useState } from "react";
import { useStammdatenMutations } from "@/api/hooks";
import { useErfolgsmeldung } from "@/api/hooks/erfolgsmeldung";
import type {
  Runde,
  SchusspositionInput,
  Wertungszone,
} from "@/api/logandaim-api";
import { Erfolgsmeldung } from "@/components/feedback/erfolgsmeldung";
import { PageHeader } from "@/components/layout/page-header";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
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
  const position = ziel
    ? (positionen[ziel.zielId] ?? anfangsPosition(ziel, ziel.schüsse))
    : null;
  const busy =
    schussRegistrieren.isPending ||
    schussAendern.isPending ||
    schussLoeschen.isPending;

  const zielCards = useMemo(
    () => runde.ziele.map((eintrag, index) => ({ eintrag, index })),
    [runde.ziele],
  );
  const erfassen = (zielId: string, zone: Wertungszone) => {
    schussRegistrieren.mutate(
      { rundeId: runde.id, schuss: { zielId, wertungszone: zone, position } },
      {
        onSuccess: () => meldung.melden("Schuss gespeichert."),
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
          Ziel {selected.zielnummer} · {selected.tiername}
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-5">
        <PositionSteuerung
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
        <WertungszoneGruppe
          disabled={runde.zustand !== "InBearbeitung"}
          busy={busy}
          onSelect={(zone) => erfassen(selected.zielId, zone)}
        />
        <SchussLedger
          ziel={selected}
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
          <div className="flex justify-between gap-3">
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
      <PageHeader
        register={["Logbuch", "Historie"]}
        title={runde.parkourName}
        description={`Runde vom ${runde.datum}`}
      />
      <Erfolgsmeldung meldung={meldung.meldung} />
      <Zieluebersicht
        ziele={runde.ziele}
        currentIndex={zielIndex}
        onSelect={setZielIndex}
      />
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
  );
}
