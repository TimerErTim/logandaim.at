"use client";

import { useState } from "react";
import type { RundeZiel } from "@/api/logandaim-api";
import { Button } from "@/components/ui/button";

export function Zieluebersicht({
  ziele,
  currentIndex,
  onSelect,
}: {
  ziele: RundeZiel[];
  currentIndex: number;
  onSelect: (index: number) => void;
}) {
  const [expanded, setExpanded] = useState(false);
  const erfasste = ziele.filter((ziel) => ziel.schüsse.length > 0).length;
  return (
    <section className="space-y-3">
      <Button
        variant="secondary"
        aria-expanded={expanded}
        onClick={() => setExpanded(!expanded)}
      >
        Zielübersicht · {erfasste} / {ziele.length}
      </Button>
      {expanded ? (
        <ol className="grid gap-2 sm:grid-cols-2 lg:grid-cols-3">
          {ziele.map((ziel, index) => (
            <li key={ziel.zielId}>
              <Button
                variant={currentIndex === index ? "primary" : "secondary"}
                className="w-full justify-start"
                onClick={() => {
                  onSelect(index);
                  setExpanded(false);
                }}
              >
                Ziel {index + 1}: {ziel.tiername} · {ziel.schüsse.length}{" "}
                Schüsse
              </Button>
            </li>
          ))}
        </ol>
      ) : null}
    </section>
  );
}
