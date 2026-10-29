"use client";

import { Play } from "lucide-react";
import Link from "next/link";
import { useRouter } from "next/navigation";
import { useState } from "react";

import { useAuth } from "@/api/auth-provider";
import { apiErrorMessage } from "@/api/errors";
import { useParcoursQuery, useStammdatenMutations } from "@/api/hooks";
import { PflockMarker } from "@/components/stammdaten/pflock-swatch";
import { Alert } from "@/components/ui/alert";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { RegisterLine } from "@/components/ui/paper-rule";
import { Separator } from "@/components/ui/separator";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { paths } from "@/lib/navigation";

/**
 * The public face of the logbook: every active Parcours of every active Anlage, no
 * sign-in required (AC-13). A signed-in visitor may start a Runde from here; a visitor
 * without an account still sees the list, which is the point of the route.
 */
export default function ParcoursPage() {
  const phase = useAuth();
  const parcours = useParcoursQuery();
  const { rundeStarten } = useStammdatenMutations();
  const router = useRouter();
  const [gestartet, setGestartet] = useState<string | null>(null);

  return (
    <div className="space-y-6 p-6">
      <header className="space-y-2">
        <RegisterLine items={["Öffentlich", "Parcours", "Anlagen"]} />
        <h1 className="font-serif text-2xl font-semibold tracking-tight text-ink">
          Parcours
        </h1>
        <p className="text-sm text-secondary">
          Alle aktiven Parcours der angeschlossenen Anlagen. Eine Runde lässt
          sich direkt starten.
        </p>
      </header>

      {parcours.isError ? (
        <Alert variant="error">
          {apiErrorMessage(parcours.error, {
            fallback: "Die Parcours konnten nicht geladen werden.",
          })}
        </Alert>
      ) : null}

      {rundeStarten.isError ? (
        <Alert variant="error">
          {apiErrorMessage(rundeStarten.error, {
            unauthorized: "Zum Starten einer Runde ist eine Anmeldung nötig.",
            fallback: "Die Runde konnte nicht gestartet werden.",
          })}
        </Alert>
      ) : null}

      {parcours.isLoading ? (
        <p className="text-sm text-secondary">Parcours werden geladen …</p>
      ) : (parcours.data?.length ?? 0) === 0 ? (
        <Card>
          <CardContent className="pt-5">
            <p className="text-sm text-secondary">
              Zurzeit ist kein Parcours freigegeben.
            </p>
          </CardContent>
        </Card>
      ) : (
        <div className="grid gap-6 lg:grid-cols-2">
          {parcours.data?.map((eintrag) => (
            <Card key={eintrag.parcours.id}>
              <CardHeader>
                <div className="flex flex-wrap items-center justify-between gap-2">
                  <CardTitle>{eintrag.parcours.name}</CardTitle>
                  <Badge variant="default">{eintrag.anlageName}</Badge>
                </div>
                <CardDescription>
                  {eintrag.parcours.pflöcke.length} Pflöcke ·{" "}
                  {eintrag.parcours.platzierungen.length} Ziele
                </CardDescription>
              </CardHeader>
              <CardContent className="space-y-4">
                <Table>
                  <TableHead>
                    <TableRow>
                      <TableHeader>Pflock</TableHeader>
                      <TableHeader>Farbe</TableHeader>
                    </TableRow>
                  </TableHead>
                  <TableBody>
                    {eintrag.parcours.pflöcke.map((pflock) => (
                      <TableRow key={pflock.id}>
                        <TableCell>{pflock.bezeichnung}</TableCell>
                        <TableCell>
                          <PflockMarker farbe={pflock.farbe} />
                        </TableCell>
                      </TableRow>
                    ))}
                  </TableBody>
                </Table>

                {phase.status === "authenticated" ? (
                  <div className="flex flex-wrap items-center gap-3">
                    <Button
                      disabled={rundeStarten.isPending}
                      onClick={() =>
                        rundeStarten.mutate(eintrag.parcours.id, {
                          onSuccess: (runde) => {
                            setGestartet(runde.id);
                            router.push(
                              `${paths.runden}/detail?id=${runde.id}`,
                            );
                          },
                        })
                      }
                    >
                      <Play />
                      {rundeStarten.isPending && gestartet === null
                        ? "Wird gestartet…"
                        : "Runde starten"}
                    </Button>
                  </div>
                ) : (
                  <>
                    <Separator />
                    <p className="text-sm text-secondary">
                      Zum Starten einer Runde{" "}
                      <Link
                        href="/anmeldung"
                        className="text-primary underline-offset-4 hover:underline"
                      >
                        melden Sie sich an
                      </Link>
                      .
                    </p>
                  </>
                )}
              </CardContent>
            </Card>
          ))}
        </div>
      )}
    </div>
  );
}
