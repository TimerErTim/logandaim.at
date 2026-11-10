"use client";

import { useRouter } from "next/navigation";
import { useEffect } from "react";
import { useAuth } from "@/api/auth-provider";
import { apiErrorMessage } from "@/api/errors";
import { useRundeQuery } from "@/api/hooks";
import { ErfassenAnsicht } from "@/components/runde/erfassen-ansicht";
import { FreieErfassenAnsicht } from "@/components/runde/freie-erfassen-ansicht";
import { Alert } from "@/components/ui/alert";
import { Skeleton } from "@/components/ui/skeleton";
import { useQueryParam } from "@/lib/use-query-param";

export default function RundeErfassenPage() {
  const phase = useAuth();
  const router = useRouter();
  const id = useQueryParam("id");
  const runde = useRundeQuery(id);
  useEffect(() => {
    if (phase.status === "anonymous") router.replace("/anmeldung");
  }, [phase.status, router]);

  if (id === null || runde.isLoading)
    return (
      <div aria-busy="true" className="space-y-4 p-6">
        <Skeleton className="h-10 w-64" />
        <Skeleton className="h-72 w-full" />
      </div>
    );
  if (!id)
    return (
      <div className="p-6">
        <Alert variant="warning">
          Es wurde keine Runde ausgewählt. Bitte öffnen Sie sie aus der
          Historie.
        </Alert>
      </div>
    );
  if (runde.isError || !runde.data)
    return (
      <div className="p-6">
        <Alert variant="error">
          {apiErrorMessage(runde.error, {
            fallback: "Die Runde konnte nicht geladen werden.",
          })}
        </Alert>
      </div>
    );
  return runde.data.herkunft === null ? (
    <FreieErfassenAnsicht runde={runde.data} />
  ) : (
    <ErfassenAnsicht runde={runde.data} />
  );
}
