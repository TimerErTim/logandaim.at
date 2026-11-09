"use client";

import { ChevronDown } from "lucide-react";
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
    <div className="relative">
      <Button
        size="icon"
        variant="ghost"
        aria-label="Zielübersicht öffnen"
        aria-expanded={expanded}
        onClick={() => setExpanded(!expanded)}
      >
        <ChevronDown
          className={
            expanded
              ? "rotate-180 transition-transform"
              : "transition-transform"
          }
          aria-hidden="true"
        />
        <span className="sr-only">
          Zielübersicht · {erfasste} / {ziele.length}
        </span>
      </Button>
      {expanded ? (
        <ol className="paper-overlay absolute top-full right-0 z-30 mt-2 grid w-72 max-w-[calc(100vw-2rem)] gap-2 rounded-md border border-subtle bg-elevated p-2">
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
    </div>
  );
}
