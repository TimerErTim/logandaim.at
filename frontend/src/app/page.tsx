"use client";

import { useHealthQuery } from "@/api/hooks";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";

export default function HomePage() {
  const health = useHealthQuery();

  return (
    <div className="p-6">
      <Card>
        <CardHeader>
          <CardTitle>logandaim.at</CardTitle>
        </CardHeader>
        <CardContent className="space-y-2 text-sm text-zinc-600 dark:text-zinc-300">
          <p>Platzhalter. Die Anwendung wird hier aufgebaut.</p>
          <p>
            API-Status:{" "}
            {health.isLoading
              ? "wird geprüft"
              : health.isError
                ? "nicht erreichbar"
                : health.data}
          </p>
        </CardContent>
      </Card>
    </div>
  );
}
