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
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Field } from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import { Progress } from "@/components/ui/progress";
import { Separator } from "@/components/ui/separator";
import { anfangsPosition } from "@/lib/runden-position";
import { FreiePositionSteuerung } from "./freie-position-steuerung";
import { Rundenabschluss } from "./rundenabschluss";
import { SchussLedger } from "./schuss-ledger";
import { WertungAnzeige } from "./wertung-anzeige";
import { WertungszoneGruppe } from "./wertungszone-gruppe";

export function FreieErfassenAnsicht({ runde }: { runde: Runde }) {
  const {
    schussRegistrieren,
    schussAendern,
    schussLoeschen,
    freiesZielHinzufuegen,
    freiesZielUmbenennen,
    freiesZielEntfernen,
  } = useStammdatenMutations();
  const meldung = useErfolgsmeldung();
  const [positionen, setPositionen] = useState<
    Record<string, SchusspositionInput | null>
  >({});
  const [fehler, setFehler] = useState<string | null>(null);
  const [entfernen, setEntfernen] = useState<Runde["ziele"][number] | null>(
    null,
  );
  const [umbenennen, setUmbenennen] = useState<Runde["ziele"][number] | null>(
    null,
  );
  const [neuerName, setNeuerName] = useState("");
  const [index, setIndex] = useState(0);
  const shots = runde.ziele.reduce(
    (count, ziel) => count + ziel.schüsse.length,
    0,
  );
  const usedColours = useMemo(
    () => [
      ...new Set(
        runde.ziele.flatMap((ziel) =>
          ziel.schüsse.flatMap((shot) =>
            shot.position && "Pflock" in shot.position
              ? [shot.position.Pflock.farbe]
              : [],
          ),
        ),
      ),
    ],
    [runde.ziele],
  );

  const addTarget = () => {
    meldung.verwerfen();
    freiesZielHinzufuegen.mutate(
      { rundeId: runde.id },
      {
        onSuccess: () => meldung.melden("Ziel hinzugefügt."),
        onError: (error) =>
          setFehler(
            apiErrorMessage(error, {
              fallback: "Das Ziel konnte nicht hinzugefügt werden.",
            }),
          ),
      },
    );
  };

  const confirmRemove = () => {
    if (!entfernen) return;
    const deletedShots = entfernen.schüsse.length;
    freiesZielEntfernen.mutate(
      { rundeId: runde.id, zielId: entfernen.zielId },
      {
        onSuccess: () => {
          meldung.melden(`Ziel entfernt; ${deletedShots} Schüsse gelöscht.`);
          setEntfernen(null);
        },
        onError: (error) =>
          setFehler(
            apiErrorMessage(error, {
              fallback: "Das Ziel konnte nicht entfernt werden.",
            }),
          ),
      },
    );
  };

  const confirmRename = () => {
    if (!umbenennen) return;
    const name = neuerName.trim();
    if (!name) return;
    meldung.verwerfen();
    freiesZielUmbenennen.mutate(
      { rundeId: runde.id, zielId: umbenennen.zielId, name },
      {
        onSuccess: () => {
          meldung.melden("Ziel umbenannt.");
          setUmbenennen(null);
          setNeuerName("");
        },
        onError: (error) =>
          setFehler(
            apiErrorMessage(error, {
              fallback: "Das Ziel konnte nicht umbenannt werden.",
            }),
          ),
      },
    );
  };

  const targetCard = (ziel: Runde["ziele"][number], order: number) => {
    const position =
      positionen[ziel.zielId] ?? anfangsPosition(ziel, ziel.schüsse);
    return (
      <Card key={ziel.zielId}>
        <CardHeader className="pt-4 pb-2 [&>div:last-child]:mt-2">
          <div className="flex items-center justify-between gap-3">
            <CardTitle>{ziel.tiername}</CardTitle>
            <WertungAnzeige
              ersterTreffer={ziel.wertung?.ersterTreffer}
              mehrschuss={ziel.wertung?.mehrschuss}
              className="text-base font-semibold"
            />
          </div>
        </CardHeader>
        <CardContent className="space-y-4 pt-3">
          {runde.zustand === "InBearbeitung" ? (
            <>
              <FreiePositionSteuerung
                ziel={ziel}
                farben={usedColours}
                value={position}
                onChange={(next) =>
                  setPositionen((current) => ({
                    ...current,
                    [ziel.zielId]: next,
                  }))
                }
              />
              <Separator />
              <WertungszoneGruppe
                disabled={runde.zustand !== "InBearbeitung"}
                busy={schussRegistrieren.isPending}
                onSelect={(zone: Wertungszone) => {
                  meldung.verwerfen();
                  schussRegistrieren.mutate(
                    {
                      rundeId: runde.id,
                      schuss: {
                        zielId: ziel.zielId,
                        wertungszone: zone,
                        position,
                      },
                    },
                    {
                      onSuccess: () => meldung.melden("Schuss gespeichert."),
                      onError: (error) =>
                        setFehler(
                          apiErrorMessage(error, {
                            fallback:
                              "Der Schuss konnte nicht gespeichert werden.",
                          }),
                        ),
                    },
                  );
                }}
              />
            </>
          ) : null}
          <SchussLedger
            ziel={ziel}
            reihenfolge={order}
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
          {runde.zustand === "InBearbeitung" ? (
            <div className="flex flex-wrap gap-2">
              <Button
                size="field"
                variant="secondary"
                onClick={() => {
                  setNeuerName(ziel.tiername);
                  setUmbenennen(ziel);
                }}
              >
                Ziel umbenennen
              </Button>
              <Button
                size="field"
                variant="destructive"
                onClick={() => setEntfernen(ziel)}
              >
                Ziel entfernen
              </Button>
            </div>
          ) : null}
        </CardContent>
      </Card>
    );
  };

  const done = runde.ziele.filter((ziel) => ziel.schüsse.length > 0).length;
  const visible = runde.ziele[index];
  return (
    <div className="px-6 pb-24">
      <div className="sticky top-0 z-40 -mx-6 flex flex-col gap-2 border-y border-subtle bg-surface px-6 py-3">
        <div className="grid grid-cols-[minmax(0,1fr)_auto] items-center gap-x-4 gap-y-1">
          <p className="truncate font-medium text-ink font-serif">
            {runde.name}
          </p>
          <WertungAnzeige
            ersterTreffer={runde.gesamtErsterTreffer}
            mehrschuss={runde.gesamtMehrschuss}
            className="justify-self-end text-base font-semibold"
          />
          <p className="font-mono text-sm tabular-nums text-secondary">
            {shots} Schüsse
          </p>
          <span className="justify-self-end text-sm tabular-nums text-secondary">
            {done} / {runde.ziele.length} Ziele
          </span>
        </div>
        <Progress
          value={runde.ziele.length ? (done / runde.ziele.length) * 100 : 0}
          aria-label={`Zielfortschritt: ${done} von ${runde.ziele.length}`}
        />
      </div>
      {fehler ? <Alert variant="error">{fehler}</Alert> : null}
      <div className="space-y-4 py-6 lg:hidden">
        {visible ? targetCard(visible, index + 1) : null}
        {index >= runde.ziele.length ? <Rundenabschluss runde={runde} /> : null}
        {runde.zustand === "InBearbeitung" ? (
          <Button
            variant="secondary"
            disabled={freiesZielHinzufuegen.isPending}
            onClick={addTarget}
          >
            Ziel hinzufügen
          </Button>
        ) : null}
        {runde.ziele.length > 0 ? (
          <div className="flex gap-3">
            <Button
              variant="secondary"
              disabled={index === 0}
              onClick={() => setIndex((current) => Math.max(0, current - 1))}
            >
              Zurück
            </Button>
            <Button
              variant="secondary"
              onClick={() =>
                setIndex((current) => Math.min(runde.ziele.length, current + 1))
              }
            >
              Weiter
            </Button>
          </div>
        ) : null}
      </div>
      <div className="hidden space-y-4 py-6 lg:block">
        {runde.ziele.map((ziel, order) => targetCard(ziel, order + 1))}
        {runde.zustand === "InBearbeitung" ? (
          <Button
            variant="secondary"
            disabled={freiesZielHinzufuegen.isPending}
            onClick={addTarget}
          >
            Ziel hinzufügen
          </Button>
        ) : null}
        <Rundenabschluss runde={runde} />
      </div>
      <Dialog
        open={umbenennen !== null}
        onOpenChange={(open) => {
          if (!open) {
            setUmbenennen(null);
            setNeuerName("");
          }
        }}
      >
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Ziel umbenennen</DialogTitle>
            <DialogDescription>
              Ändere den Namen dieses Ziels.
            </DialogDescription>
          </DialogHeader>
          <Field label="Name">
            <Input
              autoFocus
              value={neuerName}
              maxLength={120}
              onChange={(event) => setNeuerName(event.target.value)}
              onKeyDown={(event) => {
                if (event.key === "Enter") confirmRename();
              }}
            />
          </Field>
          {freiesZielUmbenennen.isError ? (
            <Alert variant="error">
              {apiErrorMessage(freiesZielUmbenennen.error, {
                fallback: "Das Ziel konnte nicht umbenannt werden.",
              })}
            </Alert>
          ) : null}
          <DialogFooter>
            <Button
              variant="secondary"
              disabled={freiesZielUmbenennen.isPending}
              onClick={() => setUmbenennen(null)}
            >
              Abbrechen
            </Button>
            <Button
              disabled={!neuerName.trim() || freiesZielUmbenennen.isPending}
              onClick={confirmRename}
            >
              Speichern
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
      <Dialog
        open={entfernen !== null}
        onOpenChange={(open) => {
          if (!open) setEntfernen(null);
        }}
      >
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Ziel entfernen?</DialogTitle>
            <DialogDescription>
              {entfernen?.schüsse.length ?? 0} Schüsse werden mit diesem Ziel
              gelöscht. Diese Aktion kann nicht rückgängig gemacht werden.
            </DialogDescription>
          </DialogHeader>
          {freiesZielEntfernen.isError ? (
            <Alert variant="error">
              {apiErrorMessage(freiesZielEntfernen.error, {
                fallback: "Das Ziel konnte nicht entfernt werden.",
              })}
            </Alert>
          ) : null}
          <DialogFooter>
            <Button
              variant="secondary"
              disabled={freiesZielEntfernen.isPending}
              onClick={() => setEntfernen(null)}
            >
              Abbrechen
            </Button>
            <Button
              variant="destructive"
              disabled={freiesZielEntfernen.isPending}
              onClick={confirmRemove}
            >
              Ziel entfernen
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
}
