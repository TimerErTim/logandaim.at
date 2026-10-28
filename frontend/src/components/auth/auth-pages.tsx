"use client";

import { CheckCircle2, CircleAlert } from "lucide-react";
import Link from "next/link";
import { useRouter, useSearchParams } from "next/navigation";
import { type FormEvent, useState } from "react";
import { apiErrorMessage } from "@/api/errors";
import { useAuthMutations } from "@/api/hooks";
import { Alert } from "@/components/ui/alert";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Field } from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import { Separator } from "@/components/ui/separator";

function useMailToken() {
  return useSearchParams().get("token") ?? "";
}

function AuthError({
  error,
  unauthorized,
  badRequest,
  tooManyRequests,
}: {
  error: unknown;
  unauthorized?: string;
  badRequest?: string;
  tooManyRequests?: string;
}) {
  const message = apiErrorMessage(error, {
    unauthorized,
    badRequest,
    tooManyRequests,
    fallback:
      "Die Anfrage konnte nicht verarbeitet werden. Bitte versuchen Sie es später erneut.",
  });
  if (!message) return null;
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
        <AuthError
          error={auth.login.error}
          unauthorized="E-Mail-Adresse oder Passwort ist falsch."
          tooManyRequests="Zu viele Anmeldeversuche. Bitte warten Sie und versuchen Sie es erneut."
        />
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
  const token = useMailToken();
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
            <AuthError
              error={auth.registerStart.error}
              badRequest="Bitte geben Sie eine gültige E-Mail-Adresse ein."
              tooManyRequests="Zu viele Registrierungsversuche. Bitte warten Sie und versuchen Sie es erneut."
            />
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
          <Field label="Passwort" hint="8 bis 128 Zeichen.">
            <Input
              autoComplete="new-password"
              type="password"
              minLength={8}
              maxLength={128}
              required
              value={password}
              onChange={(event) => setPassword(event.target.value)}
            />
          </Field>
          <AuthError
            error={auth.registerFinish.error}
            badRequest={
              Array.from(password).length < 8 ||
              Array.from(password).length > 128
                ? "Das Passwort muss 8 bis 128 Zeichen lang sein."
                : "Der Bestätigungslink ist ungültig oder abgelaufen. Bitte fordern Sie einen neuen Link an."
            }
            tooManyRequests="Zu viele Registrierungsversuche. Bitte warten Sie und versuchen Sie es erneut."
          />
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
  const router = useRouter();
  const auth = useAuthMutations();
  const token = useMailToken();
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [sent, setSent] = useState(false);
  function request(event: FormEvent) {
    event.preventDefault();
    auth.forgotPassword.mutate(email, { onSuccess: () => setSent(true) });
  }
  function reset(event: FormEvent) {
    event.preventDefault();
    auth.resetPassword.mutate(
      { token, neuesPasswort: password },
      { onSuccess: () => router.replace("/anmeldung") },
    );
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
            <AuthError
              error={auth.forgotPassword.error}
              badRequest="Bitte geben Sie eine gültige E-Mail-Adresse ein."
              tooManyRequests="Zu viele Anfragen. Bitte warten Sie und versuchen Sie es erneut."
            />
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
          <Field label="Neues Passwort" hint="8 bis 128 Zeichen.">
            <Input
              autoComplete="new-password"
              type="password"
              minLength={8}
              maxLength={128}
              required
              value={password}
              onChange={(event) => setPassword(event.target.value)}
            />
          </Field>
          <AuthError
            error={auth.resetPassword.error}
            badRequest={
              Array.from(password).length < 8 ||
              Array.from(password).length > 128
                ? "Das Passwort muss 8 bis 128 Zeichen lang sein."
                : "Der Wiederherstellungslink ist ungültig oder abgelaufen. Bitte fordern Sie einen neuen Link an."
            }
            tooManyRequests="Zu viele Anfragen. Bitte warten Sie und versuchen Sie es erneut."
          />
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
