"use client";

/**
 * Design-Philosophien: a line-up of identity proposals, each rendered as the same
 * mini-kit so the variants can be compared like for like.
 *
 * Every variant sketches its own palette locally (this is a proposal, not a token
 * change) and shows: a primary/secondary action pair, a status trio, the four
 * Wertungszonen, a distance display, and a capture-table row — the elements whose
 * shape the product's character actually hangs on.
 *
 * Reachable only from `page.dev.tsx`; nothing here ships, and no token or
 * production component changes.
 */

/* ------------------------------------------------------------------ */
/* Mini-kit rendered by every variant                                   */
/* ------------------------------------------------------------------ */

type Zone = {
  label: string;
  kurz: string;
  /** Part of the target the zone covers, 0–1. */
  anteil: number;
  farbe: string;
  /** Text colour that keeps the label readable on `farbe`. */
  textfarbe: string;
};

type Variant = {
  id: string;
  name: string;
  these: string;
  /** One structural signature the variant would introduce app-wide. */
  struktur: string;
  risiko: string;
  /** Darker frames treat the canvas as a surface, not the page background. */
  dark?: boolean;
  render: (p: { zonen: Zone[] }) => React.ReactNode;
};

/* ------------------------------------------------------------------ */
/* Shared mini-kit pieces (each variant styles them its own way)        */
/* ------------------------------------------------------------------ */

function ZoneRow({ zonen }: { zonen: Zone[] }) {
  return (
    <div className="flex gap-1.5">
      {zonen.map((zone) => (
        <div
          key={zone.label}
          className="flex-1 rounded-[3px] px-1.5 py-2 text-center text-[11px] font-semibold leading-tight"
          style={{ background: zone.farbe, color: zone.textfarbe }}
        >
          {zone.label}
        </div>
      ))}
    </div>
  );
}

function MiniZeile({
  zellen,
  aktiveSpalte,
}: {
  zellen: { key: string; inhalt: React.ReactNode }[];
  aktiveSpalte: number;
}) {
  return (
    <div className="flex items-center text-xs">
      {zellen.map((zelle, i) => (
        <div
          key={zelle.key}
          className={
            i === aktiveSpalte
              ? "flex-1 font-semibold"
              : "flex-1 text-[color:var(--phil-muted,#888)]"
          }
        >
          {zelle.inhalt}
        </div>
      ))}
    </div>
  );
}

function Distance({ label, wert }: { label: string; wert: string }) {
  return (
    <div className="flex items-baseline gap-2 text-xs">
      <span className="text-[color:var(--phil-muted,#888)]">{label}</span>
      <span className="font-semibold">{wert}</span>
    </div>
  );
}

const ZELLAKTIV = 1;

function MiniKit({ zonen }: { zonen: Zone[] }) {
  return (
    <div className="space-y-3">
      <div className="flex flex-wrap gap-2">
        <button
          type="button"
          className="text-xs font-semibold"
          style={{
            background: "var(--phil-primary)",
            color: "var(--phil-on-primary)",
            padding: "8px 14px",
            borderRadius: "var(--phil-radius, 4px)",
            border: "1px solid var(--phil-border, transparent)",
          }}
        >
          Schuss erfassen
        </button>
        <button
          type="button"
          className="text-xs font-semibold"
          style={{
            background: "transparent",
            color: "var(--phil-ink)",
            padding: "8px 14px",
            borderRadius: "var(--phil-radius, 4px)",
            border: "1px solid var(--phil-border)",
          }}
        >
          Abbrechen
        </button>
      </div>
      <ZoneRow zonen={zonen} />
      <Distance label="Distanz" wert="42 m" />
      <div
        className="space-y-1 pt-2"
        style={{ borderTop: "1px solid var(--phil-border)" }}
      >
        <MiniZeile
          zellen={[
            { key: "ziel", inhalt: "Ziel 7" },
            { key: "zone", inhalt: "Kill" },
            { key: "punkte", inhalt: <span>18 P.</span> },
          ]}
          aktiveSpalte={ZELLAKTIV}
        />
        <MiniZeile
          zellen={[
            { key: "ziel", inhalt: "Ziel 8" },
            { key: "zone", inhalt: "Spot" },
            { key: "punkte", inhalt: <span>20 P.</span> },
          ]}
          aktiveSpalte={ZELLAKTIV}
        />
      </div>
    </div>
  );
}

/* ------------------------------------------------------------------ */
/* The variants                                                         */
/* ------------------------------------------------------------------ */

export const DESIGN_VARIANTEN: Variant[] = [
  {
    id: "feldbuch",
    name: "1 · Feldbuch & Wegmarkierung",
    these:
      "Der Papier-Scorecard im Köcher nachempfunden: warmes Papier, geriffelte Linien, die Raute als Wegmarke. Natur, nicht Bildschirm.",
    struktur: "Ledger statt Dashboard; Maßband-Ticks als Trenner.",
    risiko:
      "Kann altbacken wirken, wenn die Typo nicht konsequent warm bleibt.",
    render: ({ zonen }) => (
      <div
        style={{
          ["--phil-primary" as string]: "#4f6b3a",
          ["--phil-on-primary" as string]: "#f6f4ea",
          ["--phil-ink" as string]: "#2a271e",
          ["--phil-border" as string]: "#d3cbb8",
          ["--phil-radius" as string]: "3px",
        }}
        className="space-y-3 font-[var(--font-nunito-sans)]"
      >
        <MiniKit zonen={zonen} />
      </div>
    ),
  },
  {
    id: "jagdkarte",
    name: "2 · Jagdkarte & Kompass",
    these:
      "Topografische Karte als Mutter aller Flächen: Schraffuren, Konturlinien, Kartenschrift. Das UI liest sich wie ein Kartenausschnitt mit Einträgen.",
    struktur:
      "Jeder Abschnitt trägt eine Konturlinie; Zahlen stehen in Kartenschrift.",
    risiko: "Schräges Kaching: Karten-Metapher kann schnell dekorativ werden.",
    render: ({ zonen }) => (
      <div
        style={{
          ["--phil-primary" as string]: "#3d5c46",
          ["--phil-on-primary" as string]: "#eef3ea",
          ["--phil-ink" as string]: "#253326",
          ["--phil-border" as string]: "#a8b8a0",
          ["--phil-radius" as string]: "0px",
        }}
        className="space-y-3"
      >
        <div
          aria-hidden
          className="h-6 w-full"
          style={{
            background:
              "repeating-linear-gradient(115deg, transparent 0 6px, rgba(61,92,70,0.15) 6px 7px)",
          }}
        />
        <MiniKit zonen={zonen} />
      </div>
    ),
  },
  {
    id: "werkstatt",
    name: "3 · Werkstattnotiz & Bleistift",
    these:
      "Handwerkliche Werkstatt-Ästhetik: Zeichenpapier, Bleistiftlinien, nachlassende Sorgfalt. Das UI darf unperfekt aussehen, solange es funktioniert.",
    struktur: "Handgezeichnete Unterstreichungen; Ecken leicht verdreht.",
    risiko: "Zwischen charmant und schlampig ist der Abstand klein.",
    render: ({ zonen }) => (
      <div
        style={{
          ["--phil-primary" as string]: "#5a5442",
          ["--phil-on-primary" as string]: "#f4f2ec",
          ["--phil-ink" as string]: "#32302a",
          ["--phil-border" as string]: "#c9c4b4",
          ["--phil-radius" as string]: "1px",
        }}
        className="space-y-3"
      >
        <MiniKit zonen={zonen} />
        <p
          className="text-xs italic"
          style={{ color: "var(--phil-ink)", opacity: 0.75 }}
        >
          Bleistift-Hinweis: Distanz vor dem Schuss prüfen.
        </p>
      </div>
    ),
  },
  {
    id: "werkzeug",
    name: "4 · Werkzeug & Maschine",
    these:
      "Das Gegenteil der Werkstatt: kühle Werkzeug-Ästhetik, dunkler Stahl,kalte LED-Segmente. Gegenpol zu allem Naturalen: präzise, technisch, numerisch.",
    struktur: "Siebensegment-Look für Zahlen; scharfe Kanten überall.",
    risiko: "Kälte: Der Schütze soll sich wohlfühlen, nicht überwacht.",
    dark: true,
    render: ({ zonen }) => (
      <div
        style={{
          ["--phil-primary" as string]: "#7ec4b8",
          ["--phil-on-primary" as string]: "#0d1a18",
          ["--phil-ink" as string]: "#dfe8e6",
          ["--phil-border" as string]: "#2c3a38",
          ["--phil-radius" as string]: "2px",
        }}
        className="space-y-3 font-mono text-[13px]"
      >
        <div className="flex gap-2">
          <span
            className="px-2 py-0.5 text-[11px]"
            style={{
              background: "var(--phil-border)",
              color: "var(--phil-ink)",
            }}
          >
            RND 042
          </span>
        </div>
        <MiniKit zonen={zonen} />
      </div>
    ),
  },
  {
    id: "siegel",
    name: "5 · Siegel & Urkunde",
    these:
      "Formelles, fast zeremonielles Design: Urkunden-Papier, Siegel-Rahmen, geschwungene Linien. Der Runde wird Würde gegeben.",
    struktur: "Doppelte Rahmen; Zierlinien unter Überschriften.",
    risiko: "Zeremoniell kann zur nostalgischen Kostümierung werden.",
    render: ({ zonen }) => (
      <div
        style={{
          ["--phil-primary" as string]: "#6d3a2e",
          ["--phil-on-primary" as string]: "#f7f1e6",
          ["--phil-ink" as string]: "#3a2c22",
          ["--phil-border" as string]: "#c8b898",
          ["--phil-radius" as string]: "0px",
        }}
        className="space-y-3"
      >
        <div className="p-2" style={{ border: "2px solid var(--phil-border)" }}>
          <MiniKit zonen={zonen} />
        </div>
      </div>
    ),
  },
  {
    id: "signal",
    name: "6 · Signal & Fahne",
    these:
      "Der Parcours ist das Vorbild: große Fahnen, klare Farben, Signalfunktion. Nichts dezent; alles ruft seine Bedeutung.",
    struktur: "Großflächige Farbblöcke als Status; Poster-Typo.",
    risiko: "Lärm: vier Zonen plus Status ergeben viel Farbe pro Screen.",
    render: ({ zonen }) => (
      <div
        style={{
          ["--phil-primary" as string]: "#c8461f",
          ["--phil-on-primary" as string]: "#fdf4ec",
          ["--phil-ink" as string]: "#201610",
          ["--phil-border" as string]: "#e0c8b0",
          ["--phil-radius" as string]: "0px",
        }}
        className="space-y-3"
      >
        <MiniKit zonen={zonen} />
      </div>
    ),
  },
  {
    id: "bergfuehrer",
    name: "7 · Bergführer & Route",
    these:
      "Alpinismus-Literatur: Route, Einstieg, Sicherung. UI als Wegbeschreibung, Zahlen als Höhenmeter.",
    struktur: "Schritt-for-Schritt-Aufbau; Routenpunkte mit Nummern.",
    risiko: "Metapher kippt bei reinen Zahlen-Ansichten (Historie).",
    render: ({ zonen }) => (
      <div
        style={{
          ["--phil-primary" as string]: "#2f5d8a",
          ["--phil-on-primary" as string]: "#eaf2f8",
          ["--phil-ink" as string]: "#1d2a38",
          ["--phil-border" as string]: "#b5c5d4",
          ["--phil-radius" as string]: "4px",
        }}
        className="space-y-3"
      >
        <div className="flex items-center gap-2 text-xs font-semibold">
          <span
            className="flex size-5 items-center justify-center rounded-full text-[10px]"
            style={{
              background: "var(--phil-primary)",
              color: "var(--phil-on-primary)",
            }}
          >
            3
          </span>
          Ziel 3 von 32
        </div>
        <MiniKit zonen={zonen} />
      </div>
    ),
  },
  {
    id: "schrebergarten",
    name: "8 · Schrebergarten & Vereinsheim",
    these:
      "Der Verein als Zuhause: Gartenzäune, handgemalte Schilder, warmthellige Kombination. UI als Vereins-Infopinwand.",
    struktur:
      "Karten wie an einer Pinwand; jede Karte ein handgeschriebenes Schild.",
    risiko: "Kitsch-Gefahr; muss sich vom Larp unterscheiden.",
    render: ({ zonen }) => (
      <div
        style={{
          ["--phil-primary" as string]: "#7a8a3a",
          ["--phil-on-primary" as string]: "#f6f4e4",
          ["--phil-ink" as string]: "#33331f",
          ["--phil-border" as string]: "#cfc9a4",
          ["--phil-radius" as string]: "6px",
        }}
        className="space-y-3"
      >
        <MiniKit zonen={zonen} />
      </div>
    ),
  },
  {
    id: "druckstock",
    name: "9 · Druckstock & Letterpress",
    these:
      "Buchdruck: Prägung statt Farbe. Kontrast entsteht durch Tiefe und Textur, nicht durch Buntheit; eine Tinte, ein Papier.",
    struktur: "Eintinten-Design; Status über Prägung und Füllmuster.",
    risiko: "Fehlbarmachung von Status: Rot ist als 'Miss' schwer ersetzbar.",
    render: ({ zonen }) => (
      <div
        style={{
          ["--phil-primary" as string]: "#2b2b28",
          ["--phil-on-primary" as string]: "#f2efe6",
          ["--phil-ink" as string]: "#2b2b28",
          ["--phil-border" as string]: "#b9b4a6",
          ["--phil-radius" as string]: "0px",
        }}
        className="space-y-3"
      >
        <MiniKit zonen={zonen} />
      </div>
    ),
  },
  {
    id: "nachtjagd",
    name: "10 · Nachtjagd & Leuchtfeuer",
    these:
      "Die Dämmerung am Parcours: dunkle Flächen, Wegmarken leuchten als kleinster Farbpunkt. Die Farbe, die es im Alltag gibt, kommt vom Inhalt.",
    struktur:
      "Inhaltliche Farbe (Pflöcke, Zonen) leuchtet; die UI selbst bleibt farblos.",
    risiko: "Leuchtfeuer-Look kippt schnell ins Gaming-Ambiente.",
    dark: true,
    render: ({ zonen }) => (
      <div
        style={{
          ["--phil-primary" as string]: "#c9a227",
          ["--phil-on-primary" as string]: "#151a14",
          ["--phil-ink" as string]: "#e8e4d4",
          ["--phil-border" as string]: "#33362c",
          ["--phil-radius" as string]: "4px",
        }}
        className="space-y-3"
      >
        <MiniKit zonen={zonen} />
      </div>
    ),
  },
];

/* ------------------------------------------------------------------ */
/* Section shell                                                        */
/* ------------------------------------------------------------------ */

function VariantFrame({ variant, zonen }: { variant: Variant; zonen: Zone[] }) {
  return (
    <div className="space-y-2">
      <h3 className="text-sm font-semibold">{variant.name}</h3>
      <p className="max-w-prose text-xs text-[color:var(--phil-muted,#888)]">
        {variant.these}
      </p>
      <p className="text-xs text-[color:var(--phil-muted,#888)]">
        <span className="font-semibold">Struktur:</span> {variant.struktur}{" "}
        <span className="font-semibold">Risiko:</span> {variant.risiko}
      </p>
      <div
        className={`rounded-md border p-4 ${
          variant.dark ? "border-[#33362c]" : "border-[#d3cbb8]"
        }`}
        style={{
          background: variant.dark ? "#141613" : "#f8f5ec",
          ["--phil-muted" as string]: variant.dark ? "#9a988a" : "#5f5a49",
        }}
      >
        {variant.render({ zonen })}
      </div>
    </div>
  );
}

/** Renders all ten proposals as one comparable line-up. */
export function PhilosophiesSection() {
  const zonen: Zone[] = [
    {
      label: "Spot",
      kurz: "S",
      anteil: 0.08,
      farbe: "#e8d44d",
      textfarbe: "#221c04",
    },
    {
      label: "Kill",
      kurz: "K",
      anteil: 0.18,
      farbe: "#b04a2f",
      textfarbe: "#fdf4ef",
    },
    {
      label: "Body",
      kurz: "B",
      anteil: 0.5,
      farbe: "#8a7a52",
      textfarbe: "#f9f6ec",
    },
    {
      label: "Miss",
      kurz: "M",
      anteil: 1.0,
      farbe: "#d8d2c0",
      textfarbe: "#23200f",
    },
  ];

  return (
    <section
      id="philosophien"
      aria-labelledby="philosophien-h"
      className="space-y-4"
    >
      <header className="space-y-1">
        <h2
          id="philosophien-h"
          className="text-base font-semibold tracking-tight"
        >
          Design-Philosophien
        </h2>
        <p className="max-w-prose text-sm text-muted-foreground">
          Zehn Identitätsvarianten als Vorschlagslinie-up. Jede Variante zeigt
          denselben Mini-Ausschnitt (Aktionen, Zonen, Distanz, Tabellenzeile)
          mit eigener Skizzen-Palette, damit sie direkt vergleichbar sind.
          Nichts davon ist entschieden; die Token stehen unverändert in
          globals.css.
        </p>
      </header>
      <div className="grid gap-8 xl:grid-cols-2">
        {DESIGN_VARIANTEN.map((variant) => (
          <VariantFrame key={variant.id} variant={variant} zonen={zonen} />
        ))}
      </div>
    </section>
  );
}
