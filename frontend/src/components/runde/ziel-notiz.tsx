"use client";

import { useEffect, useRef, useState } from "react";
import { useStammdatenMutations } from "@/api/hooks";
import type { Runde } from "@/api/logandaim-api";
import { Field } from "@/components/ui/field";
import { Textarea } from "@/components/ui/textarea";

type ZielNotizProps = {
  rundeId: string;
  ziel: Runde["ziele"][number];
  disabled: boolean;
};

export function ZielNotiz({ rundeId, ziel, disabled }: ZielNotizProps) {
  const { zielNotizSpeichern } = useStammdatenMutations();
  const [value, setValue] = useState(ziel.notiz ?? "");
  const cancelSave = useRef(false);

  useEffect(() => {
    setValue(ziel.notiz ?? "");
  }, [ziel.notiz]);

  const save = () => {
    if (value !== (ziel.notiz ?? "") && !zielNotizSpeichern.isPending) {
      zielNotizSpeichern.mutate({ rundeId, zielId: ziel.zielId, notiz: value });
    }
  };

  return (
    <Field
      label="Notiz"
      hint={`${Array.from(value).length} / 1000`}
      error={
        zielNotizSpeichern.isError
          ? "Die Notiz konnte nicht gespeichert werden."
          : undefined
      }
    >
      <Textarea
        aria-label={`Notiz für ${ziel.tiername}`}
        value={value}
        rows={2}
        disabled={disabled || zielNotizSpeichern.isPending}
        onChange={(event) => {
          const next = event.target.value;
          if (Array.from(next).length <= 1000) setValue(next);
        }}
        onKeyDown={(event) => {
          if (event.key === "Escape") {
            cancelSave.current = true;
            setValue(ziel.notiz ?? "");
            event.currentTarget.blur();
          }
        }}
        onBlur={() => {
          if (cancelSave.current) {
            cancelSave.current = false;
            return;
          }
          save();
        }}
      />
    </Field>
  );
}
