"use client";

/**
 * Vertiefung: Jagdkarte & Kompass.
 *
 * Variante 2 aus der Design-Philosophien line-up, nach der PO-Wahl eine Stufe
 * weiter gedacht. Der Vorschlag: das ganze UI liest sich wie ein Kartenausschnitt.
 * Konturlinien trennen Abschnitte, Distanz steht als echter Maßstab, die
 * Wertungszonen bekommen kartografische Schraffuren (Muster trägt die Bedeutung
 * mit, nicht nur Farbe), und die Runde ist ein Weg mit nummerierten Wegmarken
 * und einer "Du bist hier"-Markierung.
 *
 * Alles hier ist Vorschlag: lokale Inline-Styles, keine Token- oder
 * Produktionskomponenten-Änderung. Nur von `page.dev.tsx` erreichbar.
 */

type Zone = {
  label: string;
  kurz: string;
  /** Kartografische Füllung: Muster statt reiner Farbe. */
  muster: "punkte" | "voll" | "schraffur" | "leer";
  farbe: string;
  textfarbe: string;
};

type Palette = {
  paper: string;
  ink: string;
  muted: string;
  border: string;
  primary: string;
  onPrimary: string;
};

const HELL: Palette = {
  paper: "#f2efe4",
  ink: "#26332a",
  muted: "#4a5548",
  border: "#a8b8a0",
  primary: "#3d5c46",
  onPrimary: "#eef3ea",
};

const NACHT: Palette = {
  paper: "#141a16",
  ink: "#d8dfd6",
  muted: "#9aa596",
  border: "#37443a",
  primary: "#a7d09a",
  onPrimary: "#182b12",
};

const ZONEN: Zone[] = [
  {
    label: "Spot",
    kurz: "S",
    muster: "punkte",
    farbe: "#e8d44d",
    textfarbe: "#221c04",
  },
  {
    label: "Kill",
    kurz: "K",
    muster: "voll",
    farbe: "#b04a2f",
    textfarbe: "#fdf4ef",
  },
  {
    label: "Body",
    kurz: "B",
    muster: "schraffur",
    farbe: "#7d6f48",
    textfarbe: "#f9f6ec",
  },
  {
    label: "Miss",
    kurz: "M",
    muster: "leer",
    farbe: "#d8d2c0",
    textfarbe: "#23200f",
  },
];

/* ------------------------------------------------------------------ */
/* Kartenelemente                                                       */
/* ------------------------------------------------------------------ */

/** Kopfzeile wie auf einem Kartenblatt: Titel, Blattnummer, Nordpfeil. */
function KartenKopf({ p }: { p: Palette }) {
  return (
    <div
      className="flex items-center justify-between px-3 py-2"
      style={{
        borderBottom: `1px solid ${p.border}`,
        backgroundImage: `repeating-linear-gradient(115deg, transparent 0 6px, ${p.primary}1f 6px 7px)`,
      }}
    >
      <div>
        <p
          className="text-[11px] font-bold tracking-[0.18em]"
          style={{ color: p.ink }}
        >
          GROSSRAMING
        </p>
        <p className="text-[10px]" style={{ color: p.muted }}>
          Blatt 07/32 · Runde vom 24.10.2026
        </p>
      </div>
      <svg
        width="26"
        height="26"
        viewBox="0 0 26 26"
        role="img"
        aria-label="Nordpfeil"
      >
        <path d="M13 2 L17 17 L13 13.5 L9 17 Z" fill={p.primary} />
        <text
          x="13"
          y="25"
          textAnchor="middle"
          fontSize="8"
          fontWeight="700"
          fill={p.ink}
        >
          N
        </text>
      </svg>
    </div>
  );
}

function zonenStil(zone: Zone, p: Palette): React.CSSProperties {
  switch (zone.muster) {
    case "punkte":
      return {
        background: zone.farbe,
        backgroundImage: `radial-gradient(${p.ink} 1px, transparent 1.2px)`,
        backgroundSize: "5px 5px",
      };
    case "voll":
      return { background: zone.farbe };
    case "schraffur":
      return {
        background: zone.farbe,
        backgroundImage: `repeating-linear-gradient(45deg, transparent 0 3px, ${p.ink}33 3px 5px)`,
      };
    case "leer":
      return { background: "transparent", border: `1px dashed ${p.border}` };
  }
}

/** Wertungszonen als Kartenblöcke: Muster + Text, nie Farbe allein. */
function ZonenZeile({ p, zonen }: { p: Palette; zonen: Zone[] }) {
  return (
    <div>
      <div className="flex gap-1.5">
        {zonen.map((zone) => (
          <div
            key={zone.label}
            className="flex-1 rounded-[3px] px-1.5 py-2 text-center text-[11px] font-semibold leading-tight"
            style={{ ...zonenStil(zone, p), color: zone.textfarbe }}
          >
            {zone.label}
          </div>
        ))}
      </div>
      <p className="mt-1 text-[10px]" style={{ color: p.muted }}>
        Legende: Punkte = Spot, voll = Kill, Schraffur = Body, leer = Miss.
      </p>
    </div>
  );
}

/** Die Runde als Weg: Pflöcke nummeriert, gestrichelter Weg, Positionsmarke. */
function Wegmarken({ p }: { p: Palette }) {
  const pflöcke = [
    { x: 24, y: 64, nr: "1" },
    { x: 104, y: 34, nr: "2" },
    { x: 196, y: 58, nr: "3" },
  ];
  const etappen = [
    { x: 58, y: 42, label: "38 m" },
    { x: 144, y: 38, label: "42 m" },
    { x: 236, y: 34, label: "26 m" },
  ];
  return (
    <svg
      viewBox="0 0 320 92"
      role="img"
      aria-label="Weg der Runde: Pflöcke 1 bis 3, danach Ziel 7. Du bist am Ziel."
      className="w-full"
    >
      <polyline
        points="24,64 104,34 196,58 288,26"
        fill="none"
        stroke={p.primary}
        strokeWidth="2"
        strokeDasharray="6 4"
      />
      {etappen.map((e) => (
        <text key={e.label} x={e.x} y={e.y} fontSize="8" fill={p.muted}>
          {e.label}
        </text>
      ))}
      {pflöcke.map((m) => (
        <g key={m.nr}>
          <circle
            cx={m.x}
            cy={m.y}
            r="9"
            fill={p.paper}
            stroke={p.primary}
            strokeWidth="1.5"
          />
          <text
            x={m.x}
            y={m.y + 3}
            textAnchor="middle"
            fontSize="9"
            fontWeight="700"
            fill={p.ink}
          >
            {m.nr}
          </text>
        </g>
      ))}
      {/* Positionsmarke: gefülltes Dreieck, Spitze zeigt auf das aktuelle Ziel. */}
      <path d="M288 12 L294 24 L282 24 Z" fill={p.primary} />
      <text
        x="288"
        y="42"
        textAnchor="middle"
        fontSize="9"
        fontWeight="700"
        fill={p.ink}
      >
        Ziel 7
      </text>
    </svg>
  );
}

/** Distanz als Karten-Maßstab: ein Strich = 10 m, der Wert steht daneben. */
function Massstab({ p, meter }: { p: Palette; meter: number }) {
  return (
    <div className="space-y-1">
      <div className="flex items-baseline gap-2">
        <span className="text-xs" style={{ color: p.muted }}>
          Distanz
        </span>
        <span className="text-sm font-bold" style={{ color: p.ink }}>
          {meter} m
        </span>
      </div>
      <div>
        <div className="flex h-2" style={{ border: `1px solid ${p.ink}` }}>
          {["0-10", "10-20", "20-30", "30-40", "40-50"].map((strecke, i) => (
            <div
              key={strecke}
              className="flex-1"
              style={{
                borderRight: i < 4 ? `1px solid ${p.ink}` : undefined,
                background: i % 2 === 0 ? p.ink : "transparent",
              }}
            />
          ))}
        </div>
        <div
          className="mt-0.5 flex justify-between font-mono text-[9px]"
          style={{ color: p.muted }}
        >
          <span>0</span>
          <span>50 m</span>
        </div>
        <p className="text-[10px]" style={{ color: p.muted }}>
          Maßstab: ein Strich = 10 m
        </p>
      </div>
    </div>
  );
}

/** Erfassungszeilen mit Karten-Koordinate als Maschinenwert (Mono). */
function Fangzeilen({ p }: { p: Palette }) {
  const reihen = [
    { ziel: "Z7", ref: "0421/0388", zone: "Kill", punkte: 18 },
    { ziel: "Z8", ref: "0442/0391", zone: "Spot", punkte: 20 },
  ];
  return (
    <div
      className="space-y-1 pt-2"
      style={{ borderTop: `1px solid ${p.border}` }}
    >
      {reihen.map((r) => (
        <div key={r.ziel} className="flex items-center gap-2 text-xs">
          <span className="font-semibold" style={{ color: p.ink }}>
            {r.ziel}
          </span>
          <span className="font-mono text-[10px]" style={{ color: p.muted }}>
            {r.ref}
          </span>
          <span className="ml-auto" style={{ color: p.muted }}>
            {r.zone}
          </span>
          <span className="font-mono font-semibold" style={{ color: p.ink }}>
            {r.punkte} P.
          </span>
        </div>
      ))}
    </div>
  );
}

/** Der Kartenausschnitt: Kopf, Aktionen, Zonen, Weg, Maßstab, Zeilen. */
function Karte({ p, zonen }: { p: Palette; zonen: Zone[] }) {
  return (
    <div
      className="space-y-3 rounded-md p-3"
      style={{ border: `1px solid ${p.border}`, background: p.paper }}
    >
      <KartenKopf p={p} />
      <div className="flex flex-wrap gap-2">
        <button
          type="button"
          className="text-xs font-semibold"
          style={{
            background: p.primary,
            color: p.onPrimary,
            padding: "8px 14px",
            borderRadius: "0px",
            border: "1px solid transparent",
          }}
        >
          Schuss erfassen
        </button>
        <button
          type="button"
          className="text-xs font-semibold"
          style={{
            background: "transparent",
            color: p.ink,
            padding: "8px 14px",
            borderRadius: "0px",
            border: `1px solid ${p.border}`,
          }}
        >
          Abbrechen
        </button>
      </div>
      <ZonenZeile p={p} zonen={zonen} />
      <Wegmarken p={p} />
      <Massstab p={p} meter={42} />
      <Fangzeilen p={p} />
    </div>
  );
}

function KarteFrame({
  titel,
  untertitel,
  p,
  zonen,
}: {
  titel: string;
  untertitel: string;
  p: Palette;
  zonen: Zone[];
}) {
  return (
    <div className="space-y-2">
      <h3 className="text-sm font-semibold">{titel}</h3>
      <p className="max-w-prose text-xs text-muted-foreground">{untertitel}</p>
      <Karte p={p} zonen={zonen} />
    </div>
  );
}

/** Vertiefte Jagdkarte: Tagkarte und Nachtkarte nebeneinander. */
export function JagdkarteSection() {
  return (
    <section id="jagdkarte" aria-labelledby="jagdkarte-h" className="space-y-4">
      <header className="space-y-1">
        <h2 id="jagdkarte-h" className="text-base font-semibold tracking-tight">
          Vertiefung: Jagdkarte &amp; Kompass
        </h2>
        <p className="max-w-prose text-sm text-muted-foreground">
          Variante 2 aus den Design-Philosophien, eine Stufe weiter gedacht. Die
          These: das UI ist ein Kartenausschnitt. Konturlinien trennen
          Abschnitte, die Distanz steht als echter Maßstab, Wertungszonen tragen
          kartografische Muster, und die Runde ist ein Weg mit nummerierten
          Wegmarken und einer Positionsmarke — nichts davon ist entschieden.
        </p>
        <p className="max-w-prose text-xs text-muted-foreground">
          <span className="font-semibold">Struktur:</span> Kartenkopf pro
          Ansicht; Weg statt Liste; Maßstab statt freier Zahl.{" "}
          <span className="font-semibold">Risiko:</span> Die Karten-Metapher
          kann dekorativ werden; Muster dürfen nie die Lesbarkeit der
          Zonen-Texte kosten.
        </p>
      </header>
      <div className="grid gap-6 lg:grid-cols-2">
        <KarteFrame
          titel="Tagkarte"
          untertitel="Papier, Konturen, Kartenschrift. Wärmer als das heutige Paper, Kartengrün statt Wiesengrün."
          p={HELL}
          zonen={ZONEN}
        />
        <KarteFrame
          titel="Nachtkarte"
          untertitel="Dunkle Karte wie ein Nachtsicht-Ausschnitt: der Weg leuchtet, alles andere bleibt ruhig."
          p={NACHT}
          zonen={ZONEN}
        />
      </div>
    </section>
  );
}
