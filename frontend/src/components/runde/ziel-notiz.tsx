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

  const error = zielNotizSpeichern.isError
    ? "Die Notiz konnte nicht gespeichert werden."
    : undefined;
  const characterCount = Array.from(value).length;

  return (
    <Field
      label={`Notiz für ${ziel.tiername}`}
      labelHidden
      hint={`${characterCount} / 1000`}
      hintAlignment="end"
      error={error}
      className="space-y-1.5"
    >
      <Textarea
        placeholder="Notiz"
        value={value}
        autoGrow
        className={value ? undefined : "bg-surface-muted focus:bg-elevated"}
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
