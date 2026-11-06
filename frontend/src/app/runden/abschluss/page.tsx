"use client";

import { useRouter } from "next/navigation";
import { useEffect } from "react";
import { useAuth } from "@/api/auth-provider";
import { apiErrorMessage } from "@/api/errors";
import { useRundeQuery } from "@/api/hooks";
import { Rundenabschluss } from "@/components/runde/rundenabschluss";
import { Alert } from "@/components/ui/alert";
import { Skeleton } from "@/components/ui/skeleton";
import { useQueryParam } from "@/lib/use-query-param";

export default function RundeAbschlussPage() {
  const phase = useAuth();
  const router = useRouter();
  const id = useQueryParam("id");
  const runde = useRundeQuery(id);
  useEffect(() => {
    if (phase.status === "anonymous") router.replace("/anmeldung");
  }, [phase.status, router]);
  if (id === null || runde.isLoading)
    return (
      <div className="p-6">
        <Skeleton className="h-72 w-full" />
      </div>
    );
  if (!id)
    return (
      <div className="p-6">
        <Alert variant="warning">Es wurde keine Runde ausgewählt.</Alert>
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
  return (
    <div className="p-6">
      <Rundenabschluss runde={runde.data} />
    </div>
  );
}
