"use client";

import {
  ComponentSection,
  Example,
} from "@/components/dev-gallery/component-section";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  DogEaredSheet,
  HatchBand,
  MarginNote,
  Perforation,
  RegisterLine,
  RulerEdge,
  SheetNumber,
  StampMark,
} from "@/components/ui/paper-rule";
import { Separator } from "@/components/ui/separator";

/**
 * The furniture of the logbook page: the quiet marks that separate and orient
 * blocks without becoming borders. Judged here as specimens, because they are the
 * one part of the identity a reader should never consciously notice.
 */
export function PaperFurnitureSection() {
  return (
    <ComponentSection
      id="furniture"
      index="14"
      title="Papier-Möbel"
      description="Die leisen Marken einer Logbuchseite: Schraffenwand, Perforation, Randnotiz, Blattnummer, Registerzeile, Maßkante, Eselsohr und Stempel. Sie trennen und orientieren, ohne Rahmen um Inhalte zu legen."
    >
      <Example
        caption="Schraffenwand — Trennung zwischen Hauptabschnitten"
        className="block"
      >
        <div className="w-full space-y-2">
          <p className="text-sm">Erster Abschnitt</p>
          <HatchBand />
          <p className="text-sm">Zweiter Abschnitt</p>
          <HatchBand height="sm" />
          <p className="text-sm">Nach einer kurzen Pause</p>
        </div>
      </Example>

      <Example
        caption="Dichte Schraffenwand — vor einem Fußblock"
        className="block"
      >
        <div className="w-full space-y-2">
          <p className="text-sm">Letzter Abschnitt</p>
          <HatchBand dense height="lg" />
        </div>
      </Example>

      <Example caption="Perforation — trennbare Beilage" className="block">
        <div className="w-full space-y-2">
          <p className="text-sm">Eintrag mit Anhang</p>
          <Perforation />
          <p className="text-xs text-secondary">Anhang: Messwerte</p>
        </div>
      </Example>

      <Example caption="Blattnummer und Registerzeile">
        <div className="w-full space-y-2">
          <RegisterLine items={["Vorgang", "Oktober", "Entwurf"]} />
          <p className="flex items-baseline gap-2 text-sm">
            <SheetNumber>014</SheetNumber>
            <span>Eintrag mit Blattnummer in der Randspalte</span>
          </p>
        </div>
      </Example>

      <Example caption="Randnotiz — eine Bemerkung am Rand, kein Alert">
        <MarginNote label="Hinweis">
          Diese Notiz begleitet den Eintrag, sie verlangt keine Aktion.
        </MarginNote>
      </Example>

      <Example caption="Maßkante — das Blatt nennt seinen Maßstab">
        <div className="flex w-full items-stretch gap-3">
          <RulerEdge />
          <p className="self-center text-sm">Block mit geeichter Kante</p>
        </div>
      </Example>

      <Example caption="Eselsohr — die Seite wurde gelesen">
        <DogEaredSheet className="border border-subtle bg-surface p-3">
          <p className="text-sm">Bereits gesichteter Eintrag</p>
        </DogEaredSheet>
      </Example>

      <Example caption="Stempelmarke — Urteil auf Dokumentebene">
        <StampMark>Geprüft</StampMark>
        <StampMark>Archiv</StampMark>
        <StampMark className="text-impact">Überfällig</StampMark>
      </Example>

      <Example caption="Zusammengesetzt — eine Seite aus Möbeln">
        <div className="w-full space-y-3 border border-subtle bg-surface p-4">
          <RegisterLine items={["Buch", "Blatt 014"]} />
          <Separator />
          <p className="text-sm">
            Ein Abschnitt, der von einer Schraffenwand getragen wird, statt von
            einer Karte umschlossen zu sein.
          </p>
          <HatchBand height="sm" />
          <div className="flex items-center justify-between gap-3">
            <MarginNote label="Rand">Kurze Bemerkung</MarginNote>
            <div className="flex items-center gap-2">
              <Badge variant="outline">Entwurf</Badge>
              <Button size="sm" variant="secondary">
                Öffnen
              </Button>
            </div>
          </div>
        </div>
      </Example>
    </ComponentSection>
  );
}
