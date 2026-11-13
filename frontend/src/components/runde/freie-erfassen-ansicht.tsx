"use client";

import { useCallback, useRef, useState } from "react";
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
import { Input } from "@/components/ui/input";
import { Progress } from "@/components/ui/progress";
import { Separator } from "@/components/ui/separator";
import { cn } from "@/lib/cn";
import { anfangsPosition } from "@/lib/runden-position";
import { FreiePositionSteuerung } from "./freie-position-steuerung";
import { Rundenabschluss } from "./rundenabschluss";
import { SchussLedger } from "./schuss-ledger";
import { WertungAnzeige } from "./wertung-anzeige";
import { WertungszoneGruppe } from "./wertungszone-gruppe";
import { Zieluebersicht } from "./zieluebersicht";

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
  const [ungueltigePositionen, setUngueltigePositionen] = useState<
    Record<string, boolean>
  >({});
  const startRenaming = useCallback((ziel: Runde["ziele"][number]) => {
    cancelRename.current = false;
    setUmbenennenFehler(null);
    setEditingName(ziel.tiername);
    setEditingTargetId(ziel.zielId);
  }, []);
  const positionValidityChange = useCallback(
    (zielId: string, invalid: boolean) => {
      setUngueltigePositionen((current) => ({
        ...current,
        [zielId]: invalid,
      }));
    },
    [],
  );
  const [fehler, setFehler] = useState<string | null>(null);
  /**
   * A rename is its own result slot: its failure belongs next to the input that caused
   * it, not on the page-level `Alert` that reports shot writes (design rule 8).
   */
  const [umbenennenFehler, setUmbenennenFehler] = useState<string | null>(null);
  const [entfernen, setEntfernen] = useState<Runde["ziele"][number] | null>(
    null,
  );
  const [editingTargetId, setEditingTargetId] = useState<string | null>(null);
  const [editingName, setEditingName] = useState("");
  const cancelRename = useRef(false);
  const [index, setIndex] = useState(0);
  /** The Ziel that received the latest entry; its card carries the active ring. */
  const [aktivesZiel, setAktivesZiel] = useState<string | null>(null);
  const shots = runde.ziele.reduce(
    (count, ziel) => count + ziel.schüsse.length,
    0,
  );
  const usedColours = runde.ziele.flatMap((ziel) =>
    ziel.schüsse.flatMap((shot) =>
      shot.position && "Pflock" in shot.position
        ? [shot.position.Pflock.farbe]
        : [],
    ),
  );

  const addTarget = () => {
    meldung.verwerfen();
    setFehler(null);
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
    setFehler(null);
    freiesZielEntfernen.mutate(
      { rundeId: runde.id, zielId: entfernen.zielId },
      {
        onSuccess: () => {
          meldung.melden(`Ziel entfernt; ${deletedShots} Schüsse gelöscht.`);
          setEntfernen(null);
        },
        // No `onError` here: the dialog renders this mutation's failure itself, and one
        // failure gets one sentence in the place that caused it.
      },
    );
  };

  const saveTargetName = (ziel: Runde["ziele"][number]) => {
    const name = editingName.trim();
    if (name === "") {
      // An emptied name is a refused rename, not a cancel: the input stays open so the
      // typed text is not lost, and the reason sits beside it (AC-27).
      setUmbenennenFehler("Bitte geben Sie einen Namen für das Ziel ein.");
      return;
    }
    if (name === ziel.tiername) {
      setUmbenennenFehler(null);
      setEditingTargetId(null);
      setEditingName("");
      return;
    }
    meldung.verwerfen();
    setUmbenennenFehler(null);
    freiesZielUmbenennen.mutate(
      { rundeId: runde.id, zielId: ziel.zielId, name },
      {
        onSuccess: () => {
          meldung.melden("Ziel umbenannt.");
          setUmbenennenFehler(null);
          setEditingTargetId(null);
          setEditingName("");
        },
        onError: (error) =>
          setUmbenennenFehler(
            apiErrorMessage(error, {
              fallback: "Das Ziel konnte nicht umbenannt werden.",
            }),
          ),
      },
    );
  };

  const targetCard = (ziel: Runde["ziele"][number], order: number) => {
    const position = Object.hasOwn(positionen, ziel.zielId)
      ? positionen[ziel.zielId]
      : anfangsPosition(ziel, ziel.schüsse);
    return (
      <Card
        key={ziel.zielId}
        className={cn(
          "-mx-6 sm:mx-0",
          aktivesZiel === ziel.zielId && "ring-1 ring-primary",
        )}
      >
        <CardHeader className="pt-4 pb-2 [&>div:last-child]:mt-2">
          <div className="flex items-center justify-between gap-3">
            {runde.zustand === "InBearbeitung" &&
            editingTargetId === ziel.zielId ? (
              <div className="min-w-0 flex-1">
                <Input
                  autoFocus
                  aria-label={`Name für Ziel ${order}`}
                  value={editingName}
                  maxLength={120}
                  aria-invalid={editingName.trim() === ""}
                  onChange={(event) => setEditingName(event.target.value)}
                  onBlur={() => {
                    if (cancelRename.current) {
                      cancelRename.current = false;
                      setUmbenennenFehler(null);
                      setEditingTargetId(null);
                      setEditingName("");
                      return;
                    }
                    saveTargetName(ziel);
                  }}
                  onKeyDown={(event) => {
                    if (event.key === "Escape") {
                      cancelRename.current = true;
                      event.currentTarget.blur();
                    }
                    if (event.key === "Enter") event.currentTarget.blur();
                  }}
                  className="h-8 min-h-11 border-transparent bg-transparent px-1 font-serif text-[17px] font-semibold tracking-tight focus-visible:border-control"
                />
                {umbenennenFehler ? (
                  <p role="alert" className="text-xs text-impact">
                    {umbenennenFehler}
                  </p>
                ) : null}
              </div>
            ) : (
              <CardTitle className="min-w-0 flex-1 truncate">
                {runde.zustand === "InBearbeitung" ? (
                  <button
                    type="button"
                    aria-label={`${ziel.tiername}, Name bearbeiten`}
                    onFocus={() => startRenaming(ziel)}
                    onClick={() => startRenaming(ziel)}
                    className="min-h-11 w-full truncate rounded-sm text-left outline-offset-2 focus-visible:outline-2 focus-visible:outline-primary"
                  >
                    {ziel.tiername}
                  </button>
                ) : (
                  ziel.tiername
                )}
              </CardTitle>
            )}
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
                invalid={ungueltigePositionen[ziel.zielId] ?? false}
                onValidityChange={(invalid) =>
                  positionValidityChange(ziel.zielId, invalid)
                }
                onChange={(next) =>
                  setPositionen((current) => ({
                    ...current,
                    [ziel.zielId]: next,
                  }))
                }
              />
              <Separator />
              <WertungszoneGruppe
                disabled={
                  runde.zustand !== "InBearbeitung" ||
                  (ungueltigePositionen[ziel.zielId] ?? false)
                }
                busy={schussRegistrieren.isPending}
                onSelect={(zone: Wertungszone) => {
                  meldung.verwerfen();
                  setFehler(null);
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
                      onSuccess: () => {
                        setAktivesZiel(ziel.zielId);
                        meldung.melden("Schuss gespeichert.");
                      },
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
            editable={runde.zustand === "InBearbeitung"}
            positionArt="frei"
            farben={usedColours}
            onEdit={(schussId, wertungszone, shotPosition) => {
              setFehler(null);
              schussAendern.mutate(
                {
                  rundeId: runde.id,
                  schussId,
                  schuss: { wertungszone, position: shotPosition },
                },
                {
                  onError: (error) =>
                    setFehler(
                      apiErrorMessage(error, {
                        fallback: "Der Schuss konnte nicht geändert werden.",
                      }),
                    ),
                },
              );
            }}
            onDelete={(schussId) => {
              setFehler(null);
              schussLoeschen.mutate(
                { rundeId: runde.id, schussId },
                {
                  onError: (error) =>
                    setFehler(
                      apiErrorMessage(error, {
                        fallback: "Der Schuss konnte nicht gelöscht werden.",
                      }),
                    ),
                },
              );
            }}
          />
          {runde.zustand === "InBearbeitung" ? (
            <div className="flex flex-wrap gap-2">
              <Button
                size="field"
                variant="destructive"
                onClick={() => {
                  // A previous removal's failure must not greet the next one.
                  freiesZielEntfernen.reset();
                  setEntfernen(ziel);
                }}
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
  const leer = runde.ziele.length === 0;
  /**
   * A round with no target must not reach its summary while it is open: pressing the
   * finish action there would freeze an empty route, irreversibly. A round that is
   * already closed still shows its summary — that card only links to the annex.
   */
  const zeigeAbschluss = !leer || runde.zustand === "Abgeschlossen";
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
          <div className="flex items-center justify-self-end gap-1 text-sm tabular-nums text-secondary">
            <span>
              {done} / {runde.ziele.length} Ziele
            </span>
            <Zieluebersicht
              ziele={runde.ziele}
              currentIndex={index}
              onSelect={setIndex}
            />
          </div>
        </div>
        <Progress
          value={runde.ziele.length ? (done / runde.ziele.length) * 100 : 0}
          aria-label={`Zielfortschritt: ${done} von ${runde.ziele.length}`}
        />
      </div>
      {fehler ? <Alert variant="error">{fehler}</Alert> : null}
      <div className="space-y-4 py-6 lg:hidden pb-24">
        {visible ? targetCard(visible, index + 1) : null}
        {leer ? (
          <p className="text-sm text-secondary">
            Noch kein Ziel erfasst — legen Sie das erste an
          </p>
        ) : null}
        {index >= runde.ziele.length && zeigeAbschluss ? (
          <Rundenabschluss runde={runde} />
        ) : null}
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
          <div className="fixed inset-x-0 bottom-0 z-30 border-t border-subtle bg-surface p-3">
            <div className="mx-auto flex max-w-3xl justify-between gap-3">
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
                  setIndex((current) =>
                    Math.min(runde.ziele.length, current + 1),
                  )
                }
              >
                {index === runde.ziele.length - 1 ? "Zum Abschluss" : "Weiter"}
              </Button>
            </div>
          </div>
        ) : null}
      </div>
      <div className="hidden space-y-4 py-6 lg:block">
        {runde.ziele.map((ziel, order) => targetCard(ziel, order + 1))}
        {leer ? (
          <p className="text-sm text-secondary">
            Noch kein Ziel erfasst — legen Sie das erste an
          </p>
        ) : null}
        {runde.zustand === "InBearbeitung" ? (
          <Button
            variant="secondary"
            disabled={freiesZielHinzufuegen.isPending}
            onClick={addTarget}
          >
            Ziel hinzufügen
          </Button>
        ) : null}
        {/* The summary is withheld only while the round is still open: finishing it
            would freeze an empty route, and that cannot be undone. */}
        {zeigeAbschluss ? <Rundenabschluss runde={runde} /> : null}
      </div>
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
              onClick={() => {
                freiesZielEntfernen.reset();
                setEntfernen(null);
              }}
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
