"use client";

import { CheckCircle2, CircleAlert } from "lucide-react";
import Link from "next/link";
import { useRouter } from "next/navigation";
import { type FormEvent, useEffect, useState } from "react";
import { ApiError } from "@/api/errors";
import { useAuthMutations } from "@/api/hooks";
import { Alert } from "@/components/ui/alert";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Field } from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import { Separator } from "@/components/ui/separator";

function useMailToken() {
  const [token, setToken] = useState("");
  useEffect(
    () =>
      setToken(new URLSearchParams(window.location.search).get("token") ?? ""),
    [],
  );
  return [token, setToken] as const;
}

function AuthError({ error }: { error: unknown }) {
  if (!error) return null;
  const status = error instanceof ApiError ? error.status : undefined;
  const message =
    status === 401
      ? "E-Mail-Adresse oder Passwort ist falsch."
      : status === 400
        ? "Der Link ist ungültig oder abgelaufen."
        : error instanceof ApiError && error.structured?.message
          ? error.structured.message
          : "Die Anfrage konnte nicht verarbeitet werden. Bitte versuchen Sie es später erneut.";
  return (
    <Alert variant="error" icon={<CircleAlert aria-hidden="true" />}>
      <p>{message}</p>
    </Alert>
  );
}

function AuthCard({
  title,
  children,
}: {
  title: string;
  children: React.ReactNode;
}) {
  return (
    <div className="mx-auto w-full max-w-md p-6">
      <Card>
        <CardHeader>
          <CardTitle>{title}</CardTitle>
        </CardHeader>
        <CardContent className="space-y-5">{children}</CardContent>
      </Card>
    </div>
  );
}

export function LoginPage() {
  const router = useRouter();
  const auth = useAuthMutations();
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  function submit(event: FormEvent) {
    event.preventDefault();
    auth.login.mutate(
      { eMailAdresse: email, passwort: password },
      { onSuccess: () => router.replace("/") },
    );
  }
  return (
    <AuthCard title="Anmelden">
      <form className="space-y-4" onSubmit={submit}>
        <Field label="E-Mail-Adresse">
          <Input
            autoComplete="email"
            type="email"
            required
            value={email}
            onChange={(event) => setEmail(event.target.value)}
          />
        </Field>
        <Field label="Passwort">
          <Input
            autoComplete="current-password"
            type="password"
            required
            value={password}
            onChange={(event) => setPassword(event.target.value)}
          />
        </Field>
        <AuthError error={auth.login.error} />
        <Button
          type="submit"
          size="lg"
          className="w-full"
          disabled={auth.login.isPending}
        >
          {auth.login.isPending ? "Wird angemeldet …" : "Anmelden"}
        </Button>
      </form>
      <Separator />
      <p className="text-sm text-secondary">
        <Link
          className="text-ink underline underline-offset-4"
          href="/passwort"
        >
          Passwort vergessen?
        </Link>
      </p>
      <Separator />
      <p className="text-sm text-secondary">
        Noch kein Konto?{" "}
        <Link
          className="text-ink underline underline-offset-4"
          href="/registrierung"
        >
          Registrieren
        </Link>
      </p>
    </AuthCard>
  );
}

export function RegistrationPage() {
  const router = useRouter();
  const auth = useAuthMutations();
  const [token] = useMailToken();
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [sent, setSent] = useState(false);
  function start(event: FormEvent) {
    event.preventDefault();
    auth.registerStart.mutate(email, { onSuccess: () => setSent(true) });
  }
  function finish(event: FormEvent) {
    event.preventDefault();
    auth.registerFinish.mutate(
      { token, passwort: password },
      { onSuccess: () => router.replace("/") },
    );
  }
  return (
    <AuthCard title="Registrieren">
      {!token ? (
        <>
          {sent ? (
            <Alert variant="success" icon={<CheckCircle2 aria-hidden="true" />}>
              <p>Sie erhalten eine E-Mail mit dem Bestätigungslink.</p>
            </Alert>
          ) : null}
          <form className="space-y-4" onSubmit={start}>
            <Field
              label="E-Mail-Adresse"
              hint="Sie erhalten einen Link zur Bestätigung."
            >
              <Input
                autoComplete="email"
                type="email"
                required
                value={email}
                onChange={(event) => setEmail(event.target.value)}
              />
            </Field>
            <AuthError error={auth.registerStart.error} />
            <Button
              type="submit"
              size="lg"
              className="w-full"
              disabled={auth.registerStart.isPending}
            >
              {auth.registerStart.isPending
                ? "Wird gesendet …"
                : "Bestätigungslink anfordern"}
            </Button>
          </form>
        </>
      ) : (
        <form className="space-y-4" onSubmit={finish}>
          <p className="text-sm text-secondary">
            Legen Sie ein Passwort fest, um die Registrierung abzuschließen.
          </p>
          <Field label="Passwort" hint="Mindestens 8 Zeichen.">
            <Input
              autoComplete="new-password"
              type="password"
              minLength={8}
              required
              value={password}
              onChange={(event) => setPassword(event.target.value)}
            />
          </Field>
          <AuthError error={auth.registerFinish.error} />
          <Button
            type="submit"
            size="lg"
            className="w-full"
            disabled={auth.registerFinish.isPending}
          >
            {auth.registerFinish.isPending
              ? "Wird erstellt …"
              : "Konto erstellen"}
          </Button>
        </form>
      )}
      <Separator />
      <p className="text-sm text-secondary">
        Bereits registriert?{" "}
        <Link
          className="text-ink underline underline-offset-4"
          href="/anmeldung"
        >
          Anmelden
        </Link>
      </p>
    </AuthCard>
  );
}

export function PasswordPage() {
  const auth = useAuthMutations();
  const [token] = useMailToken();
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [sent, setSent] = useState(false);
  function request(event: FormEvent) {
    event.preventDefault();
    auth.forgotPassword.mutate(email, { onSuccess: () => setSent(true) });
  }
  function reset(event: FormEvent) {
    event.preventDefault();
    auth.resetPassword.mutate({ token, neuesPasswort: password });
  }
  return (
    <AuthCard title="Passwort wiederherstellen">
      {!token ? (
        <>
          {sent ? (
            <Alert variant="success" icon={<CheckCircle2 aria-hidden="true" />}>
              <p>
                Falls ein Konto zu dieser Adresse gehört, erhalten Sie eine
                E-Mail mit weiteren Schritten.
              </p>
            </Alert>
          ) : null}
          <form className="space-y-4" onSubmit={request}>
            <Field label="E-Mail-Adresse">
              <Input
                autoComplete="email"
                type="email"
                required
                value={email}
                onChange={(event) => setEmail(event.target.value)}
              />
            </Field>
            <AuthError error={auth.forgotPassword.error} />
            <Button
              type="submit"
              size="lg"
              className="w-full"
              disabled={auth.forgotPassword.isPending}
            >
              {auth.forgotPassword.isPending
                ? "Wird gesendet …"
                : "Wiederherstellungslink anfordern"}
            </Button>
          </form>
          <Separator />
        </>
      ) : (
        <form className="space-y-4" onSubmit={reset}>
          <Field label="Neues Passwort" hint="Mindestens 8 Zeichen.">
            <Input
              autoComplete="new-password"
              type="password"
              minLength={8}
              required
              value={password}
              onChange={(event) => setPassword(event.target.value)}
            />
          </Field>
          <AuthError error={auth.resetPassword.error} />
          {auth.resetPassword.isSuccess ? (
            <Alert variant="success" icon={<CheckCircle2 aria-hidden="true" />}>
              <p>
                Das Passwort wurde geändert. Sie können sich jetzt anmelden.
              </p>
            </Alert>
          ) : null}
          <Button
            type="submit"
            size="lg"
            className="w-full"
            disabled={
              auth.resetPassword.isPending || auth.resetPassword.isSuccess
            }
          >
            {auth.resetPassword.isPending
              ? "Wird geändert …"
              : "Passwort ändern"}
          </Button>
        </form>
      )}
      <Separator />
      <p className="text-sm text-secondary">
        <Link
          className="text-ink underline underline-offset-4"
          href="/anmeldung"
        >
          Zur Anmeldung
        </Link>
      </p>
    </AuthCard>
  );
}
