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

import {
  ComponentSection,
  Example,
} from "@/components/dev-gallery/component-section";
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
 * Dense-table plate: a sortable table, a per-row segmented control, a per-row
 * drawer holding a continuous slider, a draft row, and drag-and-drop reordering.
 *
 * The data is deliberately generic. This plate exists to stress the design rules —
 * row density, inline controls, a nested drawer, a drag handle — not to describe
 * any product entity, so the fixtures are neutral labels and numbers.
 */

const STATES = ["Alpha", "Beta", "Gamma", "Delta"] as const;
type State = (typeof STATES)[number];

/** Band colours are design tokens, never ad-hoc hex values. */
const CHECKPOINTS: Checkpoint[] = [
  { id: "a", label: "Low", wert: 12, farbe: "var(--border-strong)" },
  { id: "b", label: "Mid", wert: 24, farbe: "var(--semantic-info)" },
  { id: "c", label: "High", wert: 55, farbe: "var(--semantic-warning)" },
  { id: "d", label: "Peak", wert: 62, farbe: "var(--semantic-impact)" },
];

const MEASURE_MIN = 5;
const MEASURE_MAX = 80;

type Row = {
  id: string;
  state: State;
  /** A row that is allowed to be empty; rendered in secondary ink. */
  active: boolean;
  /** The continuous value shown in the drawer. */
  measure: number;
};

const INITIAL_ROWS: Row[] = [
  { id: "r1", state: "Beta", active: true, measure: 42 },
  { id: "r2", state: "Delta", active: true, measure: 42 },
  { id: "r3", state: "Gamma", active: false, measure: 28 },
];

/** Segmented state picker. Every option is text on a button, never colour alone. */
function StatePicker({
  value,
  onChange,
  label,
}: {
  value: State | null;
  onChange: (next: State) => void;
  label: string;
}) {
  return (
    <ToggleGroup
      type="single"
      value={value ?? ""}
      onValueChange={(next) => {
        if (next) {
          onChange(next as State);
        }
      }}
      aria-label={label}
    >
      {STATES.map((state) => (
        // Drawn at 36px so four options fit a table cell on a phone; the hit area
        // grows to 44px via the overlay. Neighbours sit side by side, so the
        // overlay only extends vertically.
        <ToggleGroupItem
          key={state}
          value={state}
          className="relative min-h-9 px-2 text-xs before:absolute before:-inset-y-1 before:content-[''] sm:px-3"
        >
          {state}
        </ToggleGroupItem>
      ))}
    </ToggleGroup>
  );
}

function SortableRow({
  row,
  index,
  open,
  onToggleMeasure,
  onStateChange,
  onActiveChange,
  onMeasureChange,
  onDelete,
}: {
  row: Row;
  index: number;
  open: boolean;
  onToggleMeasure: () => void;
  onStateChange: (next: State) => void;
  onActiveChange: (next: boolean) => void;
  onMeasureChange: (next: number) => void;
  onDelete: () => void;
}) {
  const {
    attributes,
    listeners,
    setNodeRef,
    transform,
    transition,
    isDragging,
  } = useSortable({ id: row.id });
  const detailId = useId();

  const name = `Zeile ${index + 1}`;

  return (
    <Fragment>
      <TableRow
        ref={setNodeRef}
        style={{ transform: CSS.Translate.toString(transform), transition }}
        className={cn(
          "relative",
          isDragging && "z-10 bg-surface ring-1 ring-primary",
          !row.active && "text-secondary",
        )}
      >
        <TableCell className="w-10 py-1 pr-0">
          {/*
            The handle is the only drag activator, so clicking a state button never
            starts a drag. `Button` keeps the 44px target while drawing at 40px.
          */}
          <Button
            variant="ghost"
            size="icon"
            aria-label={`Verschieben: ${name}`}
            className="cursor-grab touch-none text-secondary active:cursor-grabbing"
            {...attributes}
            {...listeners}
          >
            <GripVertical />
          </Button>
        </TableCell>

        <TableHeader scope="row" className="w-16 py-1">
          <span className="font-mono text-xs tabular-nums">{index + 1}</span>
        </TableHeader>

        <TableCell className="py-1">
          <StatePicker
            value={row.state}
            onChange={onStateChange}
            label={`Status für ${name}`}
          />
        </TableCell>

        <TableCell className="py-1">
          <Checkbox
            label={`${name} aktiv`}
            checked={row.active}
            onChange={(event) => onActiveChange(event.target.checked)}
            className="min-h-9"
          />
        </TableCell>

        <TableCell className="py-1 text-right">
          <Button
            variant="ghost"
            size="sm"
            aria-expanded={open}
            aria-controls={open ? detailId : undefined}
            onClick={onToggleMeasure}
            className="gap-1.5"
          >
            <Ruler />
            <span className="tabular-nums">{row.measure}</span>
            <span className="sr-only">, Messwert bearbeiten</span>
          </Button>
        </TableCell>

        <TableCell className="py-1 pr-1 text-right">
          <div className="flex items-center justify-end gap-0.5">
            <Button
              variant="ghost"
              size="icon"
              aria-label={`Löschen: ${name}`}
              onClick={onDelete}
              className="text-secondary hover:text-impact"
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
      {open ? (
        <TableRow className="hover:bg-transparent">
          <TableCell
            id={detailId}
            colSpan={6}
            className="bg-surface-muted/50 pt-0"
          >
            <div className="space-y-2 py-3">
              <p className="text-xs font-medium text-secondary">
                Messwert für {name}
              </p>
              <CheckpointSlider
                checkpoints={CHECKPOINTS}
                value={row.measure}
                onValueChange={onMeasureChange}
                min={MEASURE_MIN}
                max={MEASURE_MAX}
                label={`Measure for ${name}`}
                einheit="Einheiten"
              />
            </div>
          </TableCell>
        </TableRow>
      ) : null}
    </Fragment>
  );
}

/**
 * The row that creates the next entry. Choosing a state is the commit action, so the
 * row has no submit button and cannot be reordered.
 */
function DraftRow({
  nextIndex,
  onCreate,
}: {
  nextIndex: number;
  onCreate: (state: State) => void;
}) {
  return (
    <TableRow className="bg-surface-muted/30 hover:bg-surface-muted/30">
      <TableCell className="w-10 py-1 pr-0">
        <span
          aria-hidden="true"
          className="flex size-10 items-center justify-center text-secondary"
        >
          <Plus />
        </span>
      </TableCell>
      <TableHeader scope="row" className="w-16 py-1 text-secondary">
        <span className="font-mono text-xs tabular-nums">{nextIndex}</span>
      </TableHeader>
      <TableCell className="py-1" colSpan={4}>
        <div className="flex flex-wrap items-center gap-x-3 gap-y-1">
          <StatePicker
            value={null}
            onChange={onCreate}
            label={`Status für neue Zeile ${nextIndex}`}
          />
          <span className="text-xs text-secondary">
            Status wählen, um die Zeile anzulegen
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
  checkpoints = CHECKPOINTS,
  disabled = false,
  caption,
}: {
  checkpoints?: Checkpoint[];
  disabled?: boolean;
  caption?: string;
}) {
  const [measure, setMeasure] = useState(44);

  return (
    <div className="w-full max-w-md space-y-2">
      {caption ? (
        <p className="text-xs font-medium text-secondary">{caption}</p>
      ) : null}
      <CheckpointSlider
        checkpoints={checkpoints}
        value={measure}
        onValueChange={setMeasure}
        min={MEASURE_MIN}
        max={MEASURE_MAX}
        label="Messwert"
        einheit="Einheiten"
        disabled={disabled}
      />
      <p className="text-xs text-secondary tabular-nums">{measure} Einheiten</p>
    </div>
  );
}

/**
 * The slider on its own, in the states the table does not show. A scale with only
 * one checkpoint must still render, so the empty-checkpoint case is here.
 */
export function CheckpointSliderSection() {
  return (
    <ComponentSection
      id="checkpoint-slider"
      index="05"
      title="Bereichs-Slider"
      description="Ein kontinuierlicher Wert, dessen Bahn in farbige Bänder geteilt ist. Der Griff rastet nie ein; die benachbarten Marken stehen in der Legende und werden vom Griff angesagt."
    >
      <CheckpointSliderDemo caption="Vier Marken" />
      <CheckpointSliderDemo
        caption="Eine Marke (Legende bleibt, keine Bänder)"
        checkpoints={[CHECKPOINTS[1] as Checkpoint]}
      />
      <CheckpointSliderDemo caption="Deaktiviert" disabled />
    </ComponentSection>
  );
}

export function SortableTableSection() {
  const [rows, setRows] = useState<Row[]>(INITIAL_ROWS);
  const [openMeasure, setOpenMeasure] = useState<string | null>(null);
  const [nextId, setNextId] = useState(4);

  const pointerSensor = useSensor(PointerSensor, {
    // A short press must not turn into a drag when the user meant to click.
    activationConstraint: { distance: 4 },
  });
  const keyboardSensor = useSensor(KeyboardSensor, {
    coordinateGetter: sortableKeyboardCoordinates,
  });
  const sensors = useSensors(pointerSensor, keyboardSensor);

  function update(id: string, patch: Partial<Row>) {
    setRows((previous) =>
      previous.map((row) => (row.id === id ? { ...row, ...patch } : row)),
    );
  }

  function create(state: State) {
    const id = `r${nextId}`;
    setNextId((previous) => previous + 1);
    setRows((previous) => [
      ...previous,
      { id, state, active: true, measure: 42 },
    ]);
  }

  return (
    <ComponentSection
      id="sortable-table"
      index="03"
      title="Sortierbare Tabelle"
      description="Eine dichte Tabelle mit umsortierbaren Zeilen: Haarlinien statt Zebrastreifen, Zahlen rechtsbündig in Tabellenziffern, ein Griff pro Zeile als einziger Auslöser fürs Ziehen. Zeilen lassen sich per Zeiger und per Tastatur umsortieren."
    >
      <DndContext
        sensors={sensors}
        collisionDetection={closestCenter}
        onDragStart={() => setOpenMeasure(null)}
        onDragEnd={({ active, over }) => {
          if (!over || active.id === over.id) {
            return;
          }
          setRows((previous) => {
            const from = previous.findIndex((row) => row.id === active.id);
            const to = previous.findIndex((row) => row.id === over.id);
            if (from < 0 || to < 0) {
              return previous;
            }
            return arrayMove(previous, from, to);
          });
        }}
      >
        <Table>
          <caption className="sr-only">Zeilen in aktueller Reihenfolge</caption>
          <TableHead>
            <TableRow>
              <TableHeader className="w-10 pr-0">
                <span className="sr-only">Reorder</span>
              </TableHeader>
              <TableHeader className="w-16">#</TableHeader>
              <TableHeader>Status</TableHeader>
              <TableHeader>Aktiv</TableHeader>
              <TableHeader className="text-right">Messwert</TableHeader>
              <TableHeader className="w-14 text-right">
                <span className="sr-only">Actions</span>
              </TableHeader>
            </TableRow>
          </TableHead>
          <TableBody>
            <SortableContext
              items={rows.map((row) => row.id)}
              strategy={verticalListSortingStrategy}
            >
              {rows.map((row, index) => (
                <SortableRow
                  key={row.id}
                  row={row}
                  index={index}
                  open={openMeasure === row.id}
                  onToggleMeasure={() =>
                    setOpenMeasure((current) =>
                      current === row.id ? null : row.id,
                    )
                  }
                  onStateChange={(state) => update(row.id, { state })}
                  onActiveChange={(active) => update(row.id, { active })}
                  onMeasureChange={(measure) => update(row.id, { measure })}
                  onDelete={() => {
                    setOpenMeasure(null);
                    setRows((previous) =>
                      previous.filter((entry) => entry.id !== row.id),
                    );
                  }}
                />
              ))}
            </SortableContext>
            <DraftRow nextIndex={rows.length + 1} onCreate={create} />
          </TableBody>
        </Table>
      </DndContext>
    </ComponentSection>
  );
}

/** Kept for the plate's examples; a caption row above the table. */
export function TableCaptionExample() {
  return (
    <Example caption="Tabellenziffern">
      <span className="font-mono text-sm tabular-nums">1 · 2 · 3 · 42.5</span>
    </Example>
  );
}
