"use client";

import {
  AlertTriangle,
  ArrowRight,
  Check,
  Info,
  Plus,
  Search,
  Target,
  Trash2,
} from "lucide-react";
import { useState } from "react";

import {
  ComponentSection,
  Example,
} from "@/components/dev-gallery/component-section";
import { Alert } from "@/components/ui/alert";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Checkbox } from "@/components/ui/checkbox";
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
import { Progress } from "@/components/ui/progress";
import { RadioGroup, RadioGroupOption } from "@/components/ui/radio-group";
import { SearchInput } from "@/components/ui/search-input";
import { Select } from "@/components/ui/select";
import { Separator } from "@/components/ui/separator";
import { Skeleton } from "@/components/ui/skeleton";
import { Slider } from "@/components/ui/slider";
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

const zonen = ["Spot", "Kill", "Body", "Miss"] as const;

export function ButtonSection() {
  return (
    <ComponentSection
      id="button"
      index="06"
      title="Button"
      description="Genau eine primäre Aktion pro Bildschirm. Sie ist die einzige gefüllte Fläche in Jägergrün; alles daneben bleibt Kontur. Destruktives steht nie vorne."
    >
      <Example caption="Varianten">
        <Button>Erfassen</Button>
        <Button variant="secondary">Abbrechen</Button>
        <Button variant="ghost">Mehr</Button>
        <Button variant="destructive">Löschen</Button>
      </Example>

      <Example caption="Größen">
        <Button size="sm">Small</Button>
        <Button size="md">Medium</Button>
        <Button size="lg">Large</Button>
      </Example>

      <Example caption="Zustände">
        <Button>Standard</Button>
        <Button disabled>Deaktiviert</Button>
        <Button aria-busy="true">Wird erfasst…</Button>
      </Example>

      <Example caption="Mit Icon">
        <Button>
          <Target /> Ziel wählen
        </Button>
        <Button variant="secondary">
          <Search /> Suchen
        </Button>
        <Button variant="ghost" size="icon" aria-label="Weiter">
          <ArrowRight />
        </Button>
        <Button variant="destructive" size="icon" aria-label="Löschen">
          <Trash2 />
        </Button>
      </Example>
    </ComponentSection>
  );
}

export function BadgeSection() {
  return (
    <ComponentSection
      id="badge"
      index="07"
      title="Badge"
      description="Ein Stempel, kein Klebepunkt: Haarlinie, getönte Fläche, gesperrte Versalien. Der Text trägt die Bedeutung, Farbe ist die zweite Ebene."
    >
      <Example caption="Semantisch">
        <Badge>Entwurf</Badge>
        <Badge variant="primary">Aktiv</Badge>
        <Badge variant="success">Erfasst</Badge>
        <Badge variant="warning">Offen</Badge>
        <Badge variant="error">Fehler</Badge>
        <Badge variant="outline">Manuell</Badge>
      </Example>
      <Example caption="Im Satz">
        <p className="text-sm">
          Runde 12 <Badge variant="success">Abgeschlossen</Badge>{" "}
          <Badge variant="outline">Training</Badge>
        </p>
      </Example>
    </ComponentSection>
  );
}

export function AlertSection() {
  return (
    <ComponentSection
      id="alert"
      index="08"
      title="Alert"
      description="Eine Randnotiz, kein Banner: farbige linke Kante, getönter Grund, Icon und Text. Fehler werden per role=alert angekündigt."
    >
      <Alert variant="info" icon={<Info className="size-4" />}>
        Info: Masterdaten werden vom Administrator gepflegt.
      </Alert>
      <Alert variant="success" icon={<Check className="size-4" />}>
        Wertung berechnet: Erster Treffer 1, Mehrschuss 5.
      </Alert>
      <Alert variant="error" icon={<AlertTriangle className="size-4" />}>
        Der Schuss konnte nicht gespeichert werden.
      </Alert>
    </ComponentSection>
  );
}

export function CardSection() {
  return (
    <ComponentSection
      id="card"
      index="09"
      title="Card"
      description="Ein Blatt Papier: Haarlinie, ein Hauch Schatten, ein Titelblock mit Trennlinie. Der Titel steht in Lora, alles andere in Inter."
    >
      <Card>
        <CardHeader>
          <CardTitle>Parcours Großraming</CardTitle>
          <CardDescription>32 Ziele, 4 Pflöcke</CardDescription>
        </CardHeader>
        <CardContent className="space-y-3 pt-0">
          <TableKeyValue
            labels={[
              { id: "letzte", label: "Letzte Runde", value: "12. Oktober" },
              { id: "punkte", label: "Punkte", value: "284 / 320" },
            ]}
          />
        </CardContent>
        <CardFooter>
          <Button size="sm">Runde beginnen</Button>
          <Button size="sm" variant="ghost">
            Details
          </Button>
        </CardFooter>
      </Card>
    </ComponentSection>
  );
}

export function FormSection() {
  const [zone, setZone] = useState<string>("Kill");

  return (
    <ComponentSection
      id="form"
      index="10"
      title="Formular"
      description="Flache Felder mit feiner Kontur, die bei Fokus nach Jägergrün wechselt. Jedes Feld hat ein sichtbares Label; Fehler stehen darunter und ersetzen den Hinweis."
    >
      <Field label="Ziel" hint="Kennung des Ziels">
        <Input defaultValue="Ziel 12" />
      </Field>
      <Field label="Pfeil">
        <Input type="number" min={1} defaultValue={1} />
      </Field>
      <Field label="E-Mail" error="Bitte eine gültige Adresse eingeben.">
        <Input aria-invalid defaultValue="keine-adresse" />
      </Field>
      <Field label="Pflock">
        <Select defaultValue="blau">
          <option value="weiss">Weiß</option>
          <option value="blau">Blau</option>
          <option value="rot">Rot</option>
          <option value="gelb">Gelb</option>
        </Select>
      </Field>
      <Field label="Notiz" optional>
        <Textarea placeholder="Wind von rechts…" rows={2} />
      </Field>
      <SearchInput placeholder="Parcours suchen" aria-label="Parcours suchen" />
      <Checkbox label="Runde als Training markieren" />
      <div className="flex min-h-11 items-center gap-3">
        <Switch aria-labelledby="gallery-distance-label" defaultChecked />
        <span id="gallery-distance-label" className="text-sm text-foreground">
          Distanz anzeigen
        </span>
      </div>
      <RadioGroup defaultValue="recurve" aria-label="Bogenklasse">
        <RadioGroupOption value="blankbogen" label="Blankbogen" />
        <RadioGroupOption value="recurve" label="Recurve" />
        <RadioGroupOption value="compound" label="Compound" />
      </RadioGroup>
      <fieldset className="space-y-2">
        <legend className="text-[10px] font-semibold tracking-[0.09em] text-muted-foreground uppercase">
          Wertungszone
        </legend>
        <ToggleGroup
          type="single"
          value={zone}
          onValueChange={(next) => next && setZone(next)}
          aria-label="Wertungszone"
        >
          {zonen.map((wert) => (
            <ToggleGroupItem key={wert} value={wert}>
              {wert}
            </ToggleGroupItem>
          ))}
        </ToggleGroup>
      </fieldset>
    </ComponentSection>
  );
}

export function DataSection() {
  return (
    <ComponentSection
      id="data"
      index="11"
      title="Tabelle, Maß und Ablauf"
      description="Ein Liniensystem, kein Zebrastreifen: Zeilen durch Haarlinien getrennt, Zahlen rechtsbündig in Tabellenziffern. Der Fortschritt ist ein Maßband mit Teilstrichen."
    >
      <Table>
        <TableHead>
          <TableRow>
            <TableHeader>Pfeil</TableHeader>
            <TableHeader>Zone</TableHeader>
            <TableHeader className="text-right">Punkte</TableHeader>
          </TableRow>
        </TableHead>
        <TableBody>
          {[
            { pfeil: 1, zone: "Kill", punkte: 1 },
            { pfeil: 2, zone: "Body", punkte: 5 },
            { pfeil: 3, zone: "Miss", punkte: 0 },
          ].map((row) => (
            <TableRow key={row.pfeil}>
              <TableCell className="font-mono text-xs tabular-nums">
                {row.pfeil}
              </TableCell>
              <TableCell>{row.zone}</TableCell>
              <TableCell className="text-right font-medium tabular-nums">
                {row.punkte}
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>

      <div className="space-y-2">
        <div className="flex items-baseline justify-between text-xs">
          <span className="text-muted-foreground">Rundenfortschritt</span>
          <span className="font-mono tabular-nums">64 %</span>
        </div>
        <Progress value={64} aria-label="Rundenfortschritt" />
      </div>

      <div className="flex gap-2">
        <Skeleton className="h-4 w-40" />
        <Skeleton className="h-4 w-24" />
      </div>

      <Separator />

      <WorkflowSteps
        currentStepId="pfeile"
        steps={[
          { id: "parcours", label: "Parcours", done: true },
          { id: "pfeile", label: "Pfeile", done: false },
          { id: "wertung", label: "Wertung", done: false, optional: true },
        ]}
      />
    </ComponentSection>
  );
}

export function OverlaySection() {
  const [distanz, setDistanz] = useState(42);

  return (
    <ComponentSection
      id="overlay"
      index="12"
      title="Dialog, Menü, Tooltip, Slider"
      description="Überlagerungen sind abgelegte Blätter: sie schweben wirklich und schließen mit Escape. Der Slider misst wie ein Lineal, ohne zu einrasten."
    >
      <Example caption="Auslöser">
        <Dialog>
          <DialogTrigger asChild>
            <Button variant="secondary">Runde abschließen</Button>
          </DialogTrigger>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Runde abschließen?</DialogTitle>
              <DialogDescription>
                Die Runde wird gespeichert und erscheint in deiner Historie.
              </DialogDescription>
            </DialogHeader>
            <DialogFooter>
              <Button variant="secondary">Abbrechen</Button>
              <Button>Speichern</Button>
            </DialogFooter>
          </DialogContent>
        </Dialog>

        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button variant="secondary">Aktionen</Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent align="start">
            <DropdownMenuLabel>Runde</DropdownMenuLabel>
            <DropdownMenuItem>Bearbeiten</DropdownMenuItem>
            <DropdownMenuItem>Duplizieren</DropdownMenuItem>
            <DropdownMenuSeparator />
            <DropdownMenuItem>Löschen</DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>

        <TooltipProvider>
          <Tooltip>
            <TooltipTrigger asChild>
              <Button variant="ghost" size="icon" aria-label="Hilfe">
                <Info />
              </Button>
            </TooltipTrigger>
            <TooltipContent>Distanz zwischen den Pflöcken</TooltipContent>
          </Tooltip>
        </TooltipProvider>
      </Example>

      <Example caption="Slider" className="block">
        <div className="w-full space-y-2">
          <Slider
            value={[distanz]}
            onValueChange={([next]) => setDistanz(next ?? distanz)}
            min={10}
            max={80}
            step={1}
            aria-label="Distanz"
          />
          <div className="flex items-baseline justify-between font-mono text-xs text-muted-foreground tabular-nums">
            <span>10</span>
            <span className="text-sm font-semibold text-foreground">
              {distanz} m
            </span>
            <span>80</span>
          </div>
        </div>
      </Example>

      <Tabs defaultValue="heute">
        <TabsList>
          <TabsTrigger value="heute">Heute</TabsTrigger>
          <TabsTrigger value="woche">Woche</TabsTrigger>
        </TabsList>
        <TabsContent value="heute" className="text-sm">
          Zwei Runden erfasst.
        </TabsContent>
        <TabsContent value="woche" className="text-sm">
          Neun Runden erfasst.
        </TabsContent>
      </Tabs>
    </ComponentSection>
  );
}

export function MiscSection() {
  return (
    <ComponentSection
      id="misc"
      index="13"
      title="Text, Hinweis, Primärstempel"
      description="Die kleinen Bausteine, die auf fast jeder Seite vorkommen."
    >
      <div className="space-y-3">
        <p className="text-sm">
          Fließtext in Inter. Die Zeile bleibt bei etwa 68 Zeichen, damit das
          Auge den nächsten Satz findet.
        </p>
        <p className="text-sm text-muted-foreground">
          Nebentext für Hinweise, Einheiten und Zeitstempel.
        </p>
        <p className="font-mono text-xs text-muted-foreground">
          0712 · 284 / 320 · 42 m
        </p>
      </div>
      <Separator />
      <Example caption="Badge im Fließtext">
        <Badge variant="primary">Neu</Badge>
        <Badge variant="outline">Archiv</Badge>
        <Button variant="ghost" size="sm">
          <Plus /> Eintrag
        </Button>
      </Example>
    </ComponentSection>
  );
}
