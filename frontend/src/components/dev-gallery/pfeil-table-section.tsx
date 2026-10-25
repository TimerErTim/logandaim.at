"use client";

import {
  closestCenter,
  DndContext,
  KeyboardSensor,
  PointerSensor,
  useSensor,
  useSensors,
} from "@dnd-kit/core";
import {
  arrayMove,
  SortableContext,
  sortableKeyboardCoordinates,
  useSortable,
  verticalListSortingStrategy,
} from "@dnd-kit/sortable";
import { CSS } from "@dnd-kit/utilities";
import { GripVertical, Plus, Ruler, Trash2 } from "lucide-react";
import { Fragment, useId, useState } from "react";

import { Wertungszone } from "@/api/logandaim-api";
import {
  ComponentSection,
  Example,
} from "@/components/dev-gallery/component-section";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Checkbox } from "@/components/ui/checkbox";
import {
  type Checkpoint,
  CheckpointSlider,
} from "@/components/ui/checkpoint-slider";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { ToggleGroup, ToggleGroupItem } from "@/components/ui/toggle-group";
import { cn } from "@/lib/cn";

/**
 * Prototype of the Pfeil capture table, built to stress the design rules: a dense
 * table, a per-row segmented control, a per-row drawer holding a continuous slider,
 * a draft row, and drag-and-drop reordering.
 *
 * It is a gallery prototype, not the logging screen. Logging a Runde has no
 * PO-approved spec yet (issue #11), so nothing here talks to the API and nothing
 * ships: the module is reachable only from `page.dev.tsx`.
 */

const ZONEN = [
  Wertungszone.Spot,
  Wertungszone.Kill,
  Wertungszone.Body,
  Wertungszone.Miss,
] as const;

/**
 * Sample Pflöcke. Real ones come from the Parcours/Ziel master data, which owns the
 * colours; the slider takes them as a prop and never holds a palette itself.
 */
const PFLOCKE: Checkpoint[] = [
  { id: "weiss", label: "Weiß", wert: 12, farbe: "#e6e3d8" },
  { id: "blau", label: "Blau", wert: 24, farbe: "#3f6b8a" },
  { id: "rot", label: "Rot", wert: 55, farbe: "#9c3b32" },
  { id: "gelb", label: "Gelb", wert: 62, farbe: "#c9a227" },
];

const DISTANZ_MIN = 5;
const DISTANZ_MAX = 80;

/** Points per Pfeil position, from `specs/product/wertung.md`. */
const PUNKTE_NACH_PFEIL: Record<Wertungszone, number>[] = [
  {
    [Wertungszone.Spot]: 20,
    [Wertungszone.Kill]: 18,
    [Wertungszone.Body]: 16,
    [Wertungszone.Miss]: 0,
  },
  {
    [Wertungszone.Spot]: 14,
    [Wertungszone.Kill]: 12,
    [Wertungszone.Body]: 10,
    [Wertungszone.Miss]: 0,
  },
  {
    [Wertungszone.Spot]: 8,
    [Wertungszone.Kill]: 6,
    [Wertungszone.Body]: 4,
    [Wertungszone.Miss]: 0,
  },
];

/** Positions 4 and later score 0 in both Wertungen. */
function punkteFuer(pfeilPosition: number, zone: Wertungszone): number {
  return PUNKTE_NACH_PFEIL[pfeilPosition - 1]?.[zone] ?? 0;
}

type Pfeil = {
  id: string;
  zone: Wertungszone;
  /** An arrow that was not shot, for rounds where fewer than three were taken. */
  geschossen: boolean;
  /** Distance to the Pflock in Schritte, as master data would provide it. */
  distanz: number;
};

const START_PFEILE: Pfeil[] = [
  { id: "p1", zone: Wertungszone.Kill, geschossen: true, distanz: 42 },
  { id: "p2", zone: Wertungszone.Miss, geschossen: true, distanz: 42 },
  { id: "p3", zone: Wertungszone.Body, geschossen: false, distanz: 28 },
];

/**
 * Zone choice for one Pfeil. Every option is text on a button, so the selection is
 * never carried by colour alone.
 */
function ZonePicker({
  value,
  onChange,
  label,
}: {
  value: Wertungszone | null;
  onChange: (zone: Wertungszone) => void;
  label: string;
}) {
  return (
    <ToggleGroup
      type="single"
      value={value ?? ""}
      onValueChange={(next) => {
        if (next) {
          onChange(next as Wertungszone);
        }
      }}
      aria-label={label}
    >
      {ZONEN.map((zone) => (
        // Drawn at 36px so four zones fit a table cell on a phone, with the hit
        // area grown to 44px by the overlay. Neighbours sit side by side, so the
        // overlay only extends vertically.
        <ToggleGroupItem
          key={zone}
          value={zone}
          className="relative min-h-9 px-2 text-xs before:absolute before:-inset-y-1 before:content-[''] sm:px-3"
        >
          {zone}
        </ToggleGroupItem>
      ))}
    </ToggleGroup>
  );
}

function SortablePfeilRow({
  pfeil,
  position,
  punkte,
  offen,
  onToggleDistanz,
  onZoneChange,
  onGeschossenChange,
  onDistanzChange,
  onLoeschen,
}: {
  pfeil: Pfeil;
  position: number;
  punkte: number;
  offen: boolean;
  onToggleDistanz: () => void;
  onZoneChange: (zone: Wertungszone) => void;
  onGeschossenChange: (geschossen: boolean) => void;
  onDistanzChange: (distanz: number) => void;
  onLoeschen: () => void;
}) {
  const {
    attributes,
    listeners,
    setNodeRef,
    transform,
    transition,
    isDragging,
  } = useSortable({ id: pfeil.id });
  const detailId = useId();

  /*
   * An unshot arrow has no Pfeil position, so the accessible names must not fall
   * back to "Pfeil 0".
   */
  const bezeichnung =
    position > 0 ? `Pfeil ${position}` : "nicht geschossenen Pfeil";

  return (
    <Fragment>
      <TableRow
        ref={setNodeRef}
        style={{ transform: CSS.Translate.toString(transform), transition }}
        className={cn(
          "relative",
          isDragging && "z-10 bg-surface ring-1 ring-primary",
          !pfeil.geschossen && "text-muted-foreground",
        )}
      >
        <TableCell className="w-10 py-1 pr-0">
          {/*
            The handle is the only drag activator, so clicking a zone button never
            starts a drag. `Button` keeps the 44px target while drawing at 40px.
          */}
          <Button
            variant="ghost"
            size="icon"
            aria-label={`${bezeichnung} verschieben`}
            className="cursor-grab touch-none text-muted-foreground active:cursor-grabbing"
            {...attributes}
            {...listeners}
          >
            <GripVertical />
          </Button>
        </TableCell>

        <TableHeader scope="row" className="w-16 py-1">
          <span className="tabular-nums">
            {pfeil.geschossen ? position : "–"}
          </span>
        </TableHeader>

        <TableCell className="py-1">
          <ZonePicker
            value={pfeil.zone}
            onChange={onZoneChange}
            label={`Wertungszone für ${bezeichnung}`}
          />
        </TableCell>

        <TableCell className="py-1">
          <Checkbox
            label="geschossen"
            checked={pfeil.geschossen}
            onChange={(event) => onGeschossenChange(event.target.checked)}
            className="text-xs"
          />
        </TableCell>

        <TableCell className="w-16 py-1 text-right tabular-nums">
          {pfeil.geschossen ? punkte : "–"}
        </TableCell>

        <TableCell className="py-1">
          <div className="flex items-center justify-end gap-1">
            <Button
              variant={offen ? "secondary" : "ghost"}
              size="sm"
              aria-expanded={offen}
              aria-controls={offen ? detailId : undefined}
              onClick={onToggleDistanz}
              className="gap-1.5"
            >
              <Ruler />
              <span className="tabular-nums">{pfeil.distanz}</span>
              <span className="sr-only">Schritte, Distanz bearbeiten</span>
            </Button>
            <Button
              variant="ghost"
              size="icon"
              aria-label={`${bezeichnung} löschen`}
              onClick={onLoeschen}
              className="text-muted-foreground hover:text-destructive"
            >
              <Trash2 />
            </Button>
          </div>
        </TableCell>
      </TableRow>

      {/*
        The drawer is a second row, not an extra cell: a cell appended to this one
        would widen the table and push every column out of alignment. It is outside
        the sortable node, so only the data row follows the pointer.
      */}
      {offen ? (
        <TableRow className="hover:bg-transparent">
          <TableCell
            id={detailId}
            colSpan={6}
            className="bg-surface-muted/50 pt-0"
          >
            <div className="space-y-2 py-3">
              <p className="text-xs font-medium text-muted-foreground">
                Distanz zwischen den Pflöcken für {bezeichnung}
              </p>
              <CheckpointSlider
                checkpoints={PFLOCKE}
                value={pfeil.distanz}
                onValueChange={onDistanzChange}
                min={DISTANZ_MIN}
                max={DISTANZ_MAX}
                label={`Distanz für ${bezeichnung}`}
                einheit="Schritte"
              />
            </div>
          </TableCell>
        </TableRow>
      ) : null}
    </Fragment>
  );
}

/**
 * The row that creates the next Pfeil. Choosing a zone is the commit action, so the
 * row has no submit button and cannot be reordered.
 */
function EntwurfRow({
  naechstePosition,
  onAnlegen,
}: {
  naechstePosition: number;
  onAnlegen: (zone: Wertungszone) => void;
}) {
  return (
    <TableRow className="bg-surface-muted/30 hover:bg-surface-muted/30">
      <TableCell className="w-10 py-1 pr-0">
        <span
          aria-hidden="true"
          className="flex size-10 items-center justify-center text-muted-foreground"
        >
          <Plus />
        </span>
      </TableCell>
      <TableHeader scope="row" className="w-16 py-1 text-muted-foreground">
        <span className="tabular-nums">{naechstePosition}</span>
      </TableHeader>
      <TableCell className="py-1" colSpan={4}>
        <div className="flex flex-wrap items-center gap-x-3 gap-y-1">
          <ZonePicker
            value={null}
            onChange={onAnlegen}
            label={`Wertungszone für neuen Pfeil ${naechstePosition}`}
          />
          <span className="text-xs text-muted-foreground">
            Zone wählen, um den Pfeil anzulegen
          </span>
        </div>
      </TableCell>
    </TableRow>
  );
}

/**
 * The slider alone at a readable width, so its bands can be judged without the
 * table's column constraints.
 */
function CheckpointSliderDemo({
  checkpoints = PFLOCKE,
  disabled = false,
  caption,
}: {
  checkpoints?: Checkpoint[];
  disabled?: boolean;
  caption?: string;
}) {
  const [distanz, setDistanz] = useState(44);

  return (
    <div className="w-full max-w-md space-y-2">
      {caption ? (
        <p className="text-xs font-medium text-muted-foreground">{caption}</p>
      ) : null}
      <CheckpointSlider
        checkpoints={checkpoints}
        value={distanz}
        onValueChange={setDistanz}
        min={DISTANZ_MIN}
        max={DISTANZ_MAX}
        label="Distanz zwischen den Pflöcken"
        einheit="Schritte"
        disabled={disabled}
      />
      <p className="text-xs text-muted-foreground tabular-nums">
        {distanz} Schritte
      </p>
    </div>
  );
}

/**
 * The slider on its own, in the states the table does not show. A Ziel whose master
 * data has only one Pflock must still render, so the empty-checkpoint case is here.
 */
export function CheckpointSliderSection() {
  return (
    <ComponentSection
      id="checkpoint-slider"
      index="05"
      title="Distanz-Slider"
      description="Ein durchgehender Wert, dessen Bahn in farbige Abschnitte geteilt ist. Der Daumen rastet nicht ein; die benachbarten Marken stehen in der Legende und in der Ansage des Daumens."
    >
      <CheckpointSliderDemo caption="Vier Pflöcke" />
      <CheckpointSliderDemo
        caption="Ein Pflock (Legende bleibt, keine Bänder)"
        checkpoints={[PFLOCKE[1] as Checkpoint]}
      />
      <CheckpointSliderDemo caption="Deaktiviert (Wertung gesperrt)" disabled />
    </ComponentSection>
  );
}

export function PfeilTableSection() {
  const [pfeile, setPfeile] = useState<Pfeil[]>(START_PFEILE);
  const [offeneDistanz, setOffeneDistanz] = useState<string | null>(null);
  const [naechsteId, setNaechsteId] = useState(4);

  const zeigerSensor = useSensor(PointerSensor, {
    // A short press must not turn into a drag when the user meant to click.
    activationConstraint: { distance: 4 },
  });
  const tastaturSensor = useSensor(KeyboardSensor, {
    coordinateGetter: sortableKeyboardCoordinates,
  });
  const sensoren = useSensors(zeigerSensor, tastaturSensor);

  /*
   * Only shot arrows consume a Pfeil position, so the numbering follows the sequence
   * the archer actually shot. Moving a row renumbers everything below it.
   */
  const nummerierung = new Map<string, number>();
  let zaehler = 0;
  for (const pfeil of pfeile) {
    if (pfeil.geschossen) {
      zaehler += 1;
      nummerierung.set(pfeil.id, zaehler);
    }
  }

  const geschossen = pfeile.filter((pfeil) => pfeil.geschossen);
  const punkteVon = (pfeil: Pfeil) =>
    punkteFuer(nummerierung.get(pfeil.id) ?? 0, pfeil.zone);

  const ersterTreffer =
    geschossen
      .filter((pfeil) => pfeil.zone !== Wertungszone.Miss)
      .map(punkteVon)[0] ?? 0;
  const mehrschuss = geschossen.reduce(
    (summe, pfeil) => summe + punkteVon(pfeil),
    0,
  );

  function aendere(id: string, aenderung: Partial<Pfeil>) {
    setPfeile((bisher) =>
      bisher.map((pfeil) =>
        pfeil.id === id ? { ...pfeil, ...aenderung } : pfeil,
      ),
    );
  }

  function anlegen(zone: Wertungszone) {
    const id = `p${naechsteId}`;
    setNaechsteId((bisher) => bisher + 1);
    setPfeile((bisher) => [
      ...bisher,
      { id, zone, geschossen: true, distanz: 42 },
    ]);
  }

  return (
    <ComponentSection
      id="pfeile"
      index="03"
      title="Pfeile erfassen"
      description="Eine dichte Zeilentabelle mit sortierbaren Zeilen: Haarlinien statt Zebrastreifen, Zahlen rechtsbündig in Tabellenziffern, ein Griff pro Zeile als einziger Ziehpunkt. Die Zeilen lassen sich per Zeiger und per Tastatur umsortieren."
    >
      <div className="space-y-3">
        <DndContext
          sensors={sensoren}
          collisionDetection={closestCenter}
          onDragStart={() => setOffeneDistanz(null)}
          onDragEnd={({ active, over }) => {
            if (!over || active.id === over.id) {
              return;
            }
            setPfeile((bisher) => {
              const von = bisher.findIndex((pfeil) => pfeil.id === active.id);
              const nach = bisher.findIndex((pfeil) => pfeil.id === over.id);
              if (von < 0 || nach < 0) {
                return bisher;
              }
              return arrayMove(bisher, von, nach);
            });
          }}
        >
          <Table>
            <caption className="sr-only">
              Pfeile eines Ziels in Schussreihenfolge
            </caption>
            <TableHead>
              <TableRow>
                <TableHeader className="w-10 pr-0">
                  <span className="sr-only">Sortieren</span>
                </TableHeader>
                <TableHeader className="w-16">Pfeil</TableHeader>
                <TableHeader>Wertungszone</TableHeader>
                <TableHeader>Geschossen</TableHeader>
                <TableHeader className="w-16 text-right">Punkte</TableHeader>
                <TableHeader className="text-right">Distanz</TableHeader>
              </TableRow>
            </TableHead>
            <TableBody>
              <SortableContext
                items={pfeile.map((pfeil) => pfeil.id)}
                strategy={verticalListSortingStrategy}
              >
                {pfeile.map((pfeil) => (
                  <SortablePfeilRow
                    key={pfeil.id}
                    pfeil={pfeil}
                    position={nummerierung.get(pfeil.id) ?? 0}
                    punkte={punkteVon(pfeil)}
                    offen={offeneDistanz === pfeil.id}
                    onToggleDistanz={() =>
                      setOffeneDistanz((bisher) =>
                        bisher === pfeil.id ? null : pfeil.id,
                      )
                    }
                    onZoneChange={(zone) => aendere(pfeil.id, { zone })}
                    onGeschossenChange={(geschossen) =>
                      aendere(pfeil.id, { geschossen })
                    }
                    onDistanzChange={(distanz) =>
                      aendere(pfeil.id, { distanz })
                    }
                    onLoeschen={() =>
                      setPfeile((bisher) =>
                        bisher.filter((eintrag) => eintrag.id !== pfeil.id),
                      )
                    }
                  />
                ))}
              </SortableContext>
              <EntwurfRow naechstePosition={zaehler + 1} onAnlegen={anlegen} />
            </TableBody>
          </Table>
        </DndContext>

        <div className="flex flex-wrap items-center gap-x-6 gap-y-2 text-sm">
          <span className="flex items-center gap-2">
            <span className="text-muted-foreground">Erster Treffer</span>
            <Badge variant="outline" className="tabular-nums">
              {ersterTreffer}
            </Badge>
          </span>
          <span className="flex items-center gap-2">
            <span className="text-muted-foreground">Mehrschuss</span>
            <Badge variant="outline" className="tabular-nums">
              {mehrschuss}
            </Badge>
          </span>
        </div>

        <Example caption="Slider allein, Pflöcke aus den Masterdaten">
          <CheckpointSliderDemo />
        </Example>
      </div>
    </ComponentSection>
  );
}
