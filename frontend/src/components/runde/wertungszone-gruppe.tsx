"use client";

import { Wertungszone } from "@/api/logandaim-api";
import { Button } from "@/components/ui/button";

export function WertungszoneGruppe({
  disabled,
  busy,
  onSelect,
}: {
  disabled: boolean;
  busy: boolean;
  onSelect: (zone: Wertungszone) => void;
}) {
  return (
    <div className="grid grid-cols-2 gap-2 sm:grid-cols-4">
      {[
        Wertungszone.Spot,
        Wertungszone.Kill,
        Wertungszone.Body,
        Wertungszone.Miss,
      ].map((zone) => (
        <Button
          key={zone}
          size="field"
          disabled={disabled || busy}
          onClick={() => onSelect(zone)}
        >
          {zone}
        </Button>
      ))}
    </div>
  );
}
