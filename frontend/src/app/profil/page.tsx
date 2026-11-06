"use client";

import { useRouter } from "next/navigation";
import { useEffect } from "react";

import { useAuth } from "@/api/auth-provider";
import { useBenutzerQuery } from "@/api/hooks";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";

export default function ProfilePage() {
  return <ProfileContent />;
}

function ProfileContent() {
  const phase = useAuth();
  const benutzer = useBenutzerQuery();
  const router = useRouter();

  useEffect(() => {
    if (phase.status === "anonymous") router.replace("/anmeldung");
  }, [phase.status, router]);

  return (
    <div className="mx-auto w-full max-w-3xl space-y-6 p-6">
      <header className="space-y-2">
        <p className="text-xs font-semibold tracking-[0.09em] text-secondary uppercase">
          Konto
        </p>
        <h1 className="font-serif text-2xl font-semibold tracking-tight text-ink">
          Profil
        </h1>
        <p className="text-sm text-secondary">Ihre Kontodaten im Überblick.</p>
      </header>
      <Card>
        <CardHeader>
          <CardTitle>Persönliche Angaben</CardTitle>
        </CardHeader>
        <CardContent>
          {phase.status === "authenticated" && benutzer.data ? (
            <dl className="grid gap-4 sm:grid-cols-2">
              <div className="space-y-1">
                <dt className="text-xs font-semibold tracking-[0.09em] text-secondary uppercase">
                  E-Mail-Adresse
                </dt>
                <dd className="text-sm text-ink">
                  {benutzer.data.eMailAdresse}
                </dd>
              </div>
              <div className="space-y-1">
                <dt className="text-xs font-semibold tracking-[0.09em] text-secondary uppercase">
                  Benutzer-ID
                </dt>
                <dd className="font-mono text-sm text-ink">
                  {benutzer.data.id}
                </dd>
              </div>
            </dl>
          ) : phase.status === "unavailable" ? (
            <p className="text-sm text-secondary">
              Das Profil ist momentan nicht verfügbar. Bitte versuchen Sie es
              später erneut.
            </p>
          ) : (
            <p className="text-sm text-secondary">Profil wird geladen …</p>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
