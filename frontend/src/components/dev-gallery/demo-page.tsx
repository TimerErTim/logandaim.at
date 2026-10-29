"use client";

import {
  AlertTriangle,
  ArrowRight,
  Check,
  Download,
  Filter,
  Info,
  Plus,
  Printer,
  Ruler,
  Search,
  Trash2,
} from "lucide-react";
import { useState } from "react";

import {
  GridPaper,
  Legend,
  SeriesPlot,
} from "@/components/dev-gallery/data-figures";
import { Alert } from "@/components/ui/alert";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Checkbox } from "@/components/ui/checkbox";
import { CheckpointSlider } from "@/components/ui/checkpoint-slider";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Field } from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import {
  HatchBand,
  MarginNote,
  Perforation,
  RegisterLine,
  RulerEdge,
  SheetNumber,
  StampMark,
} from "@/components/ui/paper-rule";
import { Progress } from "@/components/ui/progress";
import { RadioGroup, RadioGroupOption } from "@/components/ui/radio-group";
import { SearchInput } from "@/components/ui/search-input";
import { Select } from "@/components/ui/select";
import { Separator } from "@/components/ui/separator";
import { Skeleton } from "@/components/ui/skeleton";
import { Switch } from "@/components/ui/switch";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableKeyValue,
  TableRow,
} from "@/components/ui/table";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Textarea } from "@/components/ui/textarea";
import { ToggleGroup, ToggleGroupItem } from "@/components/ui/toggle-group";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { WorkflowSteps } from "@/components/workflow/workflow-steps";

/**
 * A whole screen, assembled from the system rather than shown as parts.
 *
 * Nothing here is a specimen: it is a plausible page of the logbook — running head,
 * sheet number, title block, actions, tabs, metrics, a drawing, a ledger, a form and
 * a footer — so the design system is judged under composition, density and
 * neighbouring contrast. Still domain-free: the fixtures are neutral German labels.
 *
 * Layout uses container queries, not viewport ones: the page is rendered inside a
 * plate narrower than the window, and it must still behave like a real page.
 */

const ZONEN = ["Alpha", "Beta", "Gamma", "Delta"] as const;
type Zone = (typeof ZONEN)[number];

const CHECKPOINTS = [
  { id: "a", label: "Niedrig", wert: 12, farbe: "var(--border-strong)" },
  { id: "b", label: "Mittel", wert: 24, farbe: "var(--semantic-info)" },
  { id: "c", label: "Hoch", wert: 55, farbe: "var(--semantic-warning)" },
  { id: "d", label: "Spitze", wert: 62, farbe: "var(--semantic-impact)" },
];

const MESSREIHE = [
  { id: "p1", x: 4, y: 24, label: "1", value: 20 },
  { id: "p2", x: 19, y: 40, label: "2", value: 18 },
  { id: "p3", x: 34, y: 16, label: "3", value: 20 },
  { id: "p4", x: 49, y: 58, label: "4", value: 10 },
  { id: "p5", x: 64, y: 34, label: "5", value: 18 },
  { id: "p6", x: 79, y: 70, label: "6", value: 5 },
  { id: "p7", x: 93, y: 28, label: "7", value: 20 },
];

type Row = {
  id: string;
  nr: number;
  titel: string;
  status: "Aktiv" | "Entwurf" | "Prüfung";
  zone: Zone;
  fortschritt: number;
  menge: number;
};

const START_ROWS: Row[] = [
  {
    id: "r1",
    nr: 1,
    titel: "Eintrag Alpha",
    status: "Aktiv",
    zone: "Beta",
    fortschritt: 82,
    menge: 1,
  },
  {
    id: "r2",
    nr: 2,
    titel: "Eintrag Beta",
    status: "Prüfung",
    zone: "Delta",
    fortschritt: 54,
    menge: 5,
  },
  {
    id: "r3",
    nr: 3,
    titel: "Eintrag Gamma",
    status: "Entwurf",
    zone: "Alpha",
    fortschritt: 12,
    menge: 0,
  },
];

const statusVariant = {
  Aktiv: "success",
  Prüfung: "warning",
  Entwurf: "outline",
} as const;

export function DemoPage() {
  const [tab, setTab] = useState("übersicht");
  const [rows, setRows] = useState<Row[]>(START_ROWS);
  const [messwert, setMesswert] = useState(44);
  const [detail, setDetail] = useState(true);
  const [modus, setModus] = useState("geprüft");

  function setZone(id: string, zone: Zone) {
    setRows((previous) =>
      previous.map((row) => (row.id === id ? { ...row, zone } : row)),
    );
  }

  return (
    <div className="@container">
      <div className="space-y-6 bg-canvas p-4 text-ink sm:p-6">
        {/* Running head and plate number: the page introduces itself before the title. */}
        <div className="flex items-start justify-between gap-3">
          <RegisterLine items={["Buch", "Oktober", "Entwurf"]} />
          <SheetNumber>014</SheetNumber>
        </div>

        {/* Title block: serif, one line of ink, one of secondary. */}
        <header className="space-y-3">
          <div className="flex flex-wrap items-start justify-between gap-4">
            <div className="min-w-0 space-y-1.5">
              <div className="flex flex-wrap items-center gap-2">
                <Badge variant="primary">Entwurf</Badge>
                <Badge variant="success">Erfasst</Badge>
                <Badge variant="outline">Manuell</Badge>
              </div>
              <h1 className="font-serif text-2xl leading-tight font-semibold tracking-tight text-balance">
                Eintrag 014 — Monatsblatt
              </h1>
              <p className="max-w-prose text-sm text-secondary">
                Übersicht über die Einträge des laufenden Blattes, mit
                Kennzahlen, Messreihe und den offenen Zeilen.
              </p>
            </div>

            <div className="flex flex-wrap items-center gap-2">
              <Button>
                <Plus /> Eintrag erfassen
              </Button>
              <Button variant="secondary">
                <Download /> Exportieren
              </Button>
              <Button variant="ghost">Verwerfen</Button>

              <DropdownMenu>
                <DropdownMenuTrigger asChild>
                  <Button variant="secondary" aria-label="Weitere Aktionen">
                    Aktionen
                  </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent align="end">
                  <DropdownMenuLabel>Blatt</DropdownMenuLabel>
                  <DropdownMenuItem>
                    <Printer /> Drucken
                  </DropdownMenuItem>
                  <DropdownMenuItem>
                    <Ruler /> Maß prüfen
                  </DropdownMenuItem>
                  <DropdownMenuSeparator />
                  <DropdownMenuItem>
                    <Trash2 /> Blatt verwerfen
                  </DropdownMenuItem>
                </DropdownMenuContent>
              </DropdownMenu>

              <TooltipProvider>
                <Tooltip>
                  <TooltipTrigger asChild>
                    <Button
                      variant="ghost"
                      size="icon"
                      aria-label="Hinweis zum Blatt"
                    >
                      <Info />
                    </Button>
                  </TooltipTrigger>
                  <TooltipContent>
                    Ein Blatt fasst alle Einträge eines Monats.
                  </TooltipContent>
                </Tooltip>
              </TooltipProvider>
            </div>
          </div>

          <Separator />
        </header>

        {/* Filter row: the only row with controls that are about the view. */}
        <div className="flex flex-wrap items-end justify-between gap-3">
          <div className="flex flex-wrap items-end gap-3">
            <div className="w-56">
              <Field label="Suche">
                <SearchInput placeholder="Eintrag suchen" />
              </Field>
            </div>
            <div className="w-40">
              <Field label="Kategorie">
                <Select defaultValue="beta">
                  <option value="alpha">Alpha</option>
                  <option value="beta">Beta</option>
                  <option value="gamma">Gamma</option>
                  <option value="delta">Delta</option>
                </Select>
              </Field>
            </div>
            <Button variant="secondary" size="md">
              <Filter /> Filter
            </Button>
          </div>
          <p className="font-mono text-xs text-secondary tabular-nums">
            3 von 12 Zeilen
          </p>
        </div>

        <Tabs value={tab} onValueChange={setTab}>
          <TabsList>
            <TabsTrigger value="übersicht">Übersicht</TabsTrigger>
            <TabsTrigger value="verlauf">Verlauf</TabsTrigger>
            <TabsTrigger value="einstellungen">Einstellungen</TabsTrigger>
          </TabsList>

          <TabsContent value="übersicht" className="space-y-6">
            <Alert
              variant="warning"
              icon={<AlertTriangle className="size-4" />}
            >
              Zwei Zeilen sind noch nicht geprüft. Prüfen Sie sie vor dem
              Abschließen des Blattes.
            </Alert>

            {/* Metrics: ruled, right-aligned figures with dotted leaders. */}
            <div className="grid gap-4 @md:grid-cols-3">
              <Card>
                <CardHeader>
                  <CardTitle>Fortschritt</CardTitle>
                  <CardDescription>Geprüfte Zeilen des Blattes</CardDescription>
                </CardHeader>
                <CardContent className="space-y-3 pt-0">
                  <p className="font-mono text-2xl font-semibold tabular-nums">
                    64 %
                  </p>
                  <Progress value={64} aria-label="Fortschritt" />
                  <TableKeyValue
                    labels={[
                      { id: "fertig", label: "Fertig", value: "8" },
                      { id: "offen", label: "Offen", value: "4" },
                    ]}
                  />
                </CardContent>
              </Card>

              <Card>
                <CardHeader>
                  <CardTitle>Kennzahlen</CardTitle>
                  <CardDescription>Laufendes Blatt</CardDescription>
                </CardHeader>
                <CardContent className="pt-0">
                  <TableKeyValue
                    labels={[
                      { id: "summe", label: "Summe", value: "1 284" },
                      { id: "mittel", label: "Mittel", value: "107,0" },
                      { id: "max", label: "Höchstwert", value: "320" },
                      { id: "einheit", label: "Einheit", value: "Punkte" },
                    ]}
                  />
                </CardContent>
              </Card>

              <Card>
                <CardHeader>
                  <CardTitle>Status</CardTitle>
                  <CardDescription>Urteil auf Dokumentebene</CardDescription>
                </CardHeader>
                <CardContent className="space-y-3 pt-0">
                  <div className="flex flex-wrap items-center gap-2">
                    <StampMark>Geprüft</StampMark>
                    <StampMark>Oktober</StampMark>
                  </div>
                  <WorkflowSteps
                    currentStepId="prüfen"
                    steps={[
                      { id: "erfassen", label: "Erfassen", done: true },
                      { id: "prüfen", label: "Prüfen", done: false },
                      {
                        id: "abschließen",
                        label: "Abschließen",
                        done: false,
                      },
                    ]}
                  />
                </CardContent>
              </Card>
            </div>

            {/* Drawing and reading side by side: the picture is commentary, the list is the value. */}
            <div className="grid gap-4 @2xl:grid-cols-3">
              <Card className="@2xl:col-span-2">
                <CardHeader>
                  <CardTitle>Messreihe</CardTitle>
                  <CardDescription>
                    Werte der letzten sieben Zeilen
                  </CardDescription>
                </CardHeader>
                <CardContent className="space-y-3 pt-0">
                  <div className="relative overflow-hidden rounded-sm border border-subtle bg-surface p-3">
                    <GridPaper />
                    <div className="relative">
                      <SeriesPlot
                        points={MESSREIHE}
                        label="Messreihe, Punkte 1 bis 7"
                      />
                    </div>
                  </div>
                  <p className="text-xs text-secondary">
                    Bleistiftlinie auf Millimeterpapier. Jeder Punkt trägt
                    seinen Wert in Mono; die Zeichnung trägt keine Bedeutung
                    allein.
                  </p>
                </CardContent>
              </Card>

              <Card>
                <CardHeader>
                  <CardTitle>Zeichenerklärung</CardTitle>
                  <CardDescription>Lesart der Messreihe</CardDescription>
                </CardHeader>
                <CardContent className="pt-0">
                  <Legend
                    title="Legende"
                    entries={[
                      {
                        id: "linie",
                        sample: (
                          <svg width="18" height="12" aria-hidden="true">
                            <circle
                              cx="9"
                              cy="6"
                              r="5"
                              fill="none"
                              stroke="var(--border-strong)"
                              strokeWidth="1"
                            />
                          </svg>
                        ),
                        text: "Rasterlinie",
                      },
                      {
                        id: "punkt",
                        sample: (
                          <svg width="18" height="12" aria-hidden="true">
                            <circle
                              cx="9"
                              cy="6"
                              r="3"
                              fill="var(--color-primary)"
                            />
                          </svg>
                        ),
                        text: "Bezugspunkt",
                      },
                      {
                        id: "marke",
                        sample: (
                          <svg width="18" height="12" aria-hidden="true">
                            <circle
                              cx="9"
                              cy="6"
                              r="3.2"
                              fill="none"
                              stroke="var(--semantic-impact)"
                              strokeWidth="1.2"
                            />
                          </svg>
                        ),
                        text: "Hervorgehobene Marke",
                      },
                    ]}
                  />
                </CardContent>
              </Card>
            </div>

            {/* The ledger: dense rows, inline controls, one destructive action per row. */}
            <Card>
              <div className="flex flex-wrap items-start justify-between gap-3 px-6 pt-5 pb-4">
                <div className="min-w-0 space-y-1">
                  <CardTitle>Zeilen</CardTitle>
                  <CardDescription>
                    Status wählen, um eine Zeile einzuordnen
                  </CardDescription>
                </div>
                <Button variant="ghost" size="sm">
                  <ArrowRight /> Alle öffnen
                </Button>
              </div>
              <div aria-hidden="true" className="rule-ticks-quiet mx-6 h-px" />
              <CardContent className="px-0 pt-0 pb-0">
                <Table>
                  <caption className="sr-only">
                    Zeilen des Blattes mit Status, Zone und Fortschritt
                  </caption>
                  <TableHead>
                    <TableRow>
                      <TableHeader className="w-8 pl-6">#</TableHeader>
                      <TableHeader>Titel</TableHeader>
                      <TableHeader>Status</TableHeader>
                      <TableHeader className="hidden @lg:table-cell">
                        Zone
                      </TableHeader>
                      <TableHeader className="hidden @xl:table-cell">
                        Fortschritt
                      </TableHeader>
                      <TableHeader className="text-right">Menge</TableHeader>
                      <TableHeader className="w-10 pr-6 text-right">
                        <span className="sr-only">Aktionen</span>
                      </TableHeader>
                    </TableRow>
                  </TableHead>
                  <TableBody>
                    {rows.map((row) => (
                      <TableRow key={row.id}>
                        <TableCell className="pl-6 font-mono text-xs text-secondary tabular-nums">
                          {row.nr}
                        </TableCell>
                        <TableCell className="font-medium">
                          {row.titel}
                        </TableCell>
                        <TableCell>
                          <Badge variant={statusVariant[row.status]}>
                            {row.status}
                          </Badge>
                        </TableCell>
                        <TableCell className="hidden @lg:table-cell">
                          <ToggleGroup
                            type="single"
                            value={row.zone}
                            onValueChange={(next) => {
                              if (next) {
                                setZone(row.id, next as Zone);
                              }
                            }}
                            aria-label={`Zone für ${row.titel}`}
                          >
                            {ZONEN.map((zone) => (
                              <ToggleGroupItem
                                key={zone}
                                value={zone}
                                className="px-2 text-xs"
                              >
                                {zone}
                              </ToggleGroupItem>
                            ))}
                          </ToggleGroup>
                        </TableCell>
                        <TableCell className="hidden w-28 @xl:table-cell">
                          <Progress
                            value={row.fortschritt}
                            aria-label={`Fortschritt ${row.titel}`}
                          />
                        </TableCell>
                        <TableCell className="text-right font-medium tabular-nums">
                          {row.menge}
                        </TableCell>
                        <TableCell className="pr-6 text-right">
                          <Button
                            variant="ghost"
                            size="icon"
                            aria-label={`Öffnen: ${row.titel}`}
                            className="text-secondary"
                          >
                            <ArrowRight />
                          </Button>
                        </TableCell>
                      </TableRow>
                    ))}
                  </TableBody>
                </Table>
              </CardContent>
              <div className="flex flex-wrap items-center justify-between gap-3 border-t border-subtle px-6 py-3">
                <p className="text-xs text-secondary">
                  Zeilen werden in der Reihenfolge des Blattes geführt.
                </p>
                <div className="flex items-center gap-2">
                  <Button variant="ghost" size="sm">
                    Zurücksetzen
                  </Button>
                  <Button size="sm">Speichern</Button>
                </div>
              </div>
            </Card>

            <Perforation />

            {/* Form block: the row a user actually fills in. */}
            <div className="grid gap-4 @2xl:grid-cols-3">
              <Card className="@2xl:col-span-2">
                <CardHeader>
                  <CardTitle>Neue Zeile</CardTitle>
                  <CardDescription>
                    Pflichtfelder sind mit einem Feldnamen versehen; Hinweise
                    stehen unter dem Feld.
                  </CardDescription>
                </CardHeader>
                <CardContent className="space-y-4 pt-0">
                  <div className="grid gap-4 @md:grid-cols-2">
                    <Field label="Titel" hint="Kurze Kennung des Eintrags">
                      <Input defaultValue="Eintrag Delta" />
                    </Field>
                    <Field label="Menge" hint="Ganze Zahl, 1 bis 99">
                      <Input type="number" min={1} max={99} defaultValue={1} />
                    </Field>
                    <Field label="Kategorie">
                      <Select defaultValue="beta">
                        <option value="alpha">Alpha</option>
                        <option value="beta">Beta</option>
                        <option value="gamma">Gamma</option>
                      </Select>
                    </Field>
                    <Field
                      label="Referenz"
                      error="Die Referenz ist bereits vergeben."
                    >
                      <Input aria-invalid defaultValue="Ref. 12" />
                    </Field>
                  </div>

                  <Field label="Notiz" optional>
                    <Textarea placeholder="Kurze Bemerkung…" rows={2} />
                  </Field>

                  <Field label="Messwert">
                    <CheckpointSlider
                      checkpoints={CHECKPOINTS}
                      value={messwert}
                      onValueChange={setMesswert}
                      min={5}
                      max={80}
                      label="Messwert"
                      einheit="Punkte"
                    />
                  </Field>

                  <div className="grid gap-3 @md:grid-cols-2">
                    <Checkbox label="Als Entwurf markieren" defaultChecked />
                    <div className="flex min-h-11 items-center gap-3">
                      <Switch
                        aria-labelledby="demo-detail-label"
                        checked={detail}
                        onCheckedChange={setDetail}
                      />
                      <span id="demo-detail-label" className="text-sm text-ink">
                        Details anzeigen
                      </span>
                    </div>
                  </div>

                  <fieldset className="space-y-1.5">
                    <legend className="text-[11px] font-semibold tracking-[0.07em] text-secondary uppercase">
                      Abschluss
                    </legend>
                    <RadioGroup value={modus} onValueChange={setModus}>
                      <RadioGroupOption
                        value="geprüft"
                        label="Geprüft übernehmen"
                      />
                      <RadioGroupOption value="später" label="Später prüfen" />
                    </RadioGroup>
                  </fieldset>
                </CardContent>
                <div className="flex flex-wrap items-center justify-end gap-2 border-t border-subtle px-6 py-3">
                  <Button variant="secondary">Abbrechen</Button>
                  <Button>
                    <Check /> Zeile anlegen
                  </Button>
                </div>
              </Card>

              <div className="flex gap-3">
                <RulerEdge />
                <div className="space-y-4 py-1">
                  <MarginNote label="Hinweis">
                    Diese Bemerkung begleitet den Eintrag. Sie verlangt keine
                    Aktion.
                  </MarginNote>
                  <Perforation />
                  <div className="space-y-2">
                    <p className="text-[10px] font-semibold tracking-[0.09em] text-secondary uppercase">
                      Laden
                    </p>
                    <div className="space-y-2">
                      <Skeleton className="h-4 w-full" />
                      <Skeleton className="h-4 w-3/4" />
                      <Skeleton className="h-4 w-1/2" />
                    </div>
                  </div>
                  <Dialog>
                    <DialogTrigger asChild>
                      <Button variant="secondary" size="sm">
                        Blatt abschließen
                      </Button>
                    </DialogTrigger>
                    <DialogContent>
                      <DialogHeader>
                        <DialogTitle>Blatt abschließen?</DialogTitle>
                        <DialogDescription>
                          Das Blatt wird gespeichert und erscheint in der
                          Übersicht. Offene Zeilen bleiben offen.
                        </DialogDescription>
                      </DialogHeader>
                      <DialogFooter>
                        <Button variant="secondary">Abbrechen</Button>
                        <Button>Speichern</Button>
                      </DialogFooter>
                    </DialogContent>
                  </Dialog>
                </div>
              </div>
            </div>
          </TabsContent>

          <TabsContent value="verlauf" className="space-y-4">
            <Card>
              <CardHeader>
                <CardTitle>Verlauf</CardTitle>
                <CardDescription>Acht Einträge im Oktober</CardDescription>
              </CardHeader>
              <CardContent className="space-y-3 pt-0">
                {[
                  { zeit: "12. Okt · 09:14", text: "Zeile 1 geprüft" },
                  { zeit: "11. Okt · 17:02", text: "Zeile 3 angelegt" },
                  { zeit: "10. Okt · 08:40", text: "Blatt geöffnet" },
                ].map((eintrag) => (
                  <div
                    key={eintrag.zeit}
                    className="flex flex-wrap items-baseline gap-x-3 gap-y-1 border-b border-subtle pb-2 last:border-0"
                  >
                    <span className="font-mono text-[11px] text-secondary tabular-nums">
                      {eintrag.zeit}
                    </span>
                    <span className="text-sm">{eintrag.text}</span>
                  </div>
                ))}
              </CardContent>
            </Card>
          </TabsContent>

          <TabsContent value="einstellungen" className="space-y-4">
            <Card>
              <CardHeader>
                <CardTitle>Einstellungen des Blattes</CardTitle>
                <CardDescription>
                  Diese Angaben gelten für das gesamte Blatt.
                </CardDescription>
              </CardHeader>
              <CardContent className="space-y-4 pt-0">
                <Field label="Titel">
                  <Input defaultValue="Monatsblatt Oktober" />
                </Field>
                <div className="flex min-h-11 items-center gap-3">
                  <Switch aria-labelledby="demo-freeze-label" defaultChecked />
                  <span id="demo-freeze-label" className="text-sm text-ink">
                    Geprüfte Zeilen sperren
                  </span>
                </div>
                <Checkbox label="Beim Öffnen nach offenen Zeilen filtern" />
                <Alert variant="info" icon={<Search className="size-4" />}>
                  Die Suche durchsucht nur die Zeilen dieses Blattes.
                </Alert>
              </CardContent>
            </Card>
          </TabsContent>
        </Tabs>

        {/* The page signs itself off, as a printed sheet does. */}
        <div className="space-y-3">
          <HatchBand dense height="md" />
          <footer className="flex flex-wrap items-center justify-between gap-3 pb-1">
            <p className="flex flex-wrap items-center gap-x-3 gap-y-1 font-mono text-[11px] text-secondary">
              <span>Blatt 014</span>
              <span aria-hidden="true">·</span>
              <span>Oktober</span>
              <span aria-hidden="true">·</span>
              <span>3 / 12 Zeilen</span>
            </p>
            <div className="flex items-center gap-2">
              <Button variant="ghost" size="sm">
                Drucken
              </Button>
              <Button variant="secondary" size="sm">
                Zur Übersicht
              </Button>
            </div>
          </footer>
        </div>
      </div>
    </div>
  );
}
