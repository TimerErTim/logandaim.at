"use client";

import {
  AlertTriangle,
  Check,
  Info,
  Search,
  Target,
  Trash2,
} from "lucide-react";
import { useState } from "react";

import {
  ComponentSection,
  Example,
  touchTargetOutline,
} from "@/components/dev-gallery/component-section";
import { FeldbuchSection } from "@/components/dev-gallery/feldbuch-section";
import { JagdkarteSection } from "@/components/dev-gallery/jagdkarte-section";
import {
  CheckpointSliderSection,
  PfeilTableSection,
} from "@/components/dev-gallery/pfeil-table-section";
import { PhilosophiesSection } from "@/components/dev-gallery/philosophies-section";
import { TokenSwatches } from "@/components/dev-gallery/token-swatches";
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
      title="Button"
      description="Genau eine primäre Aktion pro Bildschirm. Sekundär steht daneben, Ghost sitzt im Inhalt. Destruktive Aktionen sind nie primär."
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
      title="Badge"
      description="Statusbegriffe tragen ihre Bedeutung immer im Text; Farbe ist nur die zweite Ebene."
    >
      <Example>
        <Badge>Entwurf</Badge>
        <Badge variant="success">Erfasst</Badge>
        <Badge variant="warning">Offen</Badge>
        <Badge variant="error">Fehler</Badge>
        <Badge variant="outline">Manuell</Badge>
      </Example>
    </ComponentSection>
  );
}

export function AlertSection() {
  return (
    <ComponentSection
      id="alert"
      title="Alert"
      description="Fehler erscheinen mit role=alert, damit Screenreader sie vorlesen. Jede Variante hat außerdem ein Icon als zweiten Kanal."
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
      title="Card"
      description="Fläche für einen zusammenhängenden Inhalt: Kopf mit Titel, Inhalt, optionaler Fuß."
    >
      <Card>
        <CardHeader>
          <CardTitle>Parcours Großraming</CardTitle>
          <CardDescription>32 Ziele, 4 Pflöcke</CardDescription>
        </CardHeader>
        <CardContent className="text-sm">
          Letzte Runde am 12. Oktober, 284 Punkte.
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
      title="Formular"
      description="Jedes Feld hat ein sichtbares Label; Fehler stehen unter dem Feld und werden angekündigt."
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
      <Field label="Notiz">
        <Textarea placeholder="Wind von rechts…" rows={2} />
      </Field>
      <SearchInput placeholder="Parcours suchen" />
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
      <Button disabled>Deaktiviert</Button>
      <fieldset className="space-y-2">
        <legend className="text-xs font-medium text-muted-foreground">
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
        <p className="text-xs text-muted-foreground">Gewählt: {zone}</p>
      </fieldset>
    </ComponentSection>
  );
}

export function DataSection() {
  return (
    <ComponentSection
      id="data"
      title="Tabelle und Fortschritt"
      description="Zahlen stehen rechtsbündig und tabellarisch, damit Spalten ruhig bleiben."
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
              <TableCell>{row.pfeil}</TableCell>
              <TableCell>{row.zone}</TableCell>
              <TableCell className="text-right tabular-nums">
                {row.punkte}
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
      <div className="space-y-2">
        <Progress value={64} aria-label="Rundenfortschritt" />
        <div className="flex gap-2">
          <Skeleton className="h-4 w-40" />
          <Skeleton className="h-4 w-24" />
        </div>
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
      title="Dialog, Menü, Tooltip, Slider"
      description="Überlagerungen schließen mit Escape und halten den Fokus. Der Slider misst die Distanz zwischen zwei Pflöcken."
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

      <Example caption="Slider">
        <div className="w-full space-y-2">
          <Slider
            value={[distanz]}
            onValueChange={([next]) => setDistanz(next ?? distanz)}
            min={10}
            max={80}
            step={1}
            aria-label="Distanz"
          />
          <p className="text-xs text-muted-foreground">
            {distanz} Schritte zwischen den Pflöcken
          </p>
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

/** Renders the token swatches and every component section in rule order. */
export function GallerySections({
  showTouchTargets,
}: {
  showTouchTargets: boolean;
}) {
  return (
    <div className={showTouchTargets ? touchTargetOutline : undefined}>
      <div className="space-y-10">
        <TokenSwatches />
        <PhilosophiesSection />
        <JagdkarteSection />
        <FeldbuchSection />
        <PfeilTableSection />
        <CheckpointSliderSection />
        <ButtonSection />
        <BadgeSection />
        <AlertSection />
        <CardSection />
        <FormSection />
        <DataSection />
        <OverlaySection />
      </div>
    </div>
  );
}
