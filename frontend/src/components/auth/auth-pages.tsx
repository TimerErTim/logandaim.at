"use client";

import { CheckCircle2, CircleAlert } from "lucide-react";
import Link from "next/link";
import { useRouter, useSearchParams } from "next/navigation";
import { type FormEvent, type ReactNode, useMemo, useState } from "react";

import { ApiError, apiErrorMessage } from "@/api/errors";
import { useAuthMutations } from "@/api/hooks";
import { PageHeader } from "@/components/layout/page-header";
import { Alert } from "@/components/ui/alert";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { Field } from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import { HatchBand } from "@/components/ui/paper-rule";
import { PasswordInput } from "@/components/ui/password-input";

const PASSWORD_MIN = 8;
const PASSWORD_MAX = 128;

function useMailToken() {
  return useSearchParams().get("token") ?? "";
}

/** The password policy of the domain, counted in characters, not bytes. */
function passwordWithinPolicy(password: string) {
  const length = Array.from(password).length;
  return length >= PASSWORD_MIN && length <= PASSWORD_MAX;
}

/**
 * Where a failure belongs.
 *
 * A form's own submission error is printed on the sheet, immediately above the
 * action that caused it: wrong credentials, a throttled request, a refused write.
 * A failure of the *page* — the confirmation or recovery link in the URL is spent —
 * sits above the sheet, because no re-submission of the same form can fix it.
 */
type AuthFailure = {
  scope: "form" | "page";
  message: string;
};

type AuthFailureMessages = {
  unauthorized?: string;
  badRequest?: string;
  tooManyRequests?: string;
  /** A `400` the browser cannot cause: the mailed link is spent. */
  invalidLink?: string;
  fallback: string;
};

function authFailure(
  error: unknown,
  messages: AuthFailureMessages,
): AuthFailure | null {
  if (!error) return null;
  // The password policy is checked in the browser, so a refused submission with a
  // policy-conforming password can only be a spent link.
  if (
    messages.invalidLink &&
    error instanceof ApiError &&
    error.status === 400
  ) {
    return { scope: "page", message: messages.invalidLink };
  }
  const message = apiErrorMessage(error, messages);
  return message ? { scope: "form", message } : null;
}

function AuthAlert({
  variant,
  message,
}: {
  variant: "error" | "success";
  message: string;
}) {
  return (
    <Alert
      variant={variant}
      icon={
        variant === "error" ? (
          <CircleAlert aria-hidden="true" />
        ) : (
          <CheckCircle2 aria-hidden="true" />
        )
      }
    >
      <p>{message}</p>
    </Alert>
  );
}

type AuthPageProps = {
  title: string;
  description: ReactNode;
  /** A failure of the page as a whole, printed above the sheet. */
  pageError?: string | null;
  /** The page's links; `HatchBand` closes the page before them. */
  footer?: ReactNode;
  children: ReactNode;
};

/**
 * The frame of an auth page: a narrow column of the logbook.
 *
 * Signing in is one sheet with a handful of lines, so the page is a single narrow
 * column and the title block shares the sheet's measure. A full-width open page
 * would leave the header floating above a small card.
 *
 * The footer is the page's own register line, closed by the hachure wall — the
 * documented break before a footer. It is rendered once and *only* when it has
 * links, so no page ends in a rule followed by nothing.
 */
function AuthPage({
  title,
  description,
  pageError,
  footer,
  children,
}: AuthPageProps) {
  return (
    <div className="mx-auto w-full max-w-md space-y-6 p-6">
      <PageHeader
        register={["Konto"]}
        title={title}
        description={description}
      />

      {pageError ? <AuthAlert variant="error" message={pageError} /> : null}

      {children}

      {footer ? (
        <>
          <HatchBand dense height="md" />
          <footer className="flex flex-col gap-1 text-sm text-secondary">
            {footer}
          </footer>
        </>
      ) : null}
    </div>
  );
}

function AuthLink({ href, children }: { href: string; children: ReactNode }) {
  return (
    <Link
      href={href}
      className="inline-flex min-h-11 items-center rounded-sm text-ink underline underline-offset-4 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-primary"
    >
      {children}
    </Link>
  );
}

export function LoginPage() {
  const router = useRouter();
  const auth = useAuthMutations();
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");

  const failure = authFailure(auth.login.error, {
    unauthorized: "E-Mail-Adresse oder Passwort ist falsch.",
    tooManyRequests:
      "Zu viele Anmeldeversuche. Bitte warten Sie und versuchen Sie es erneut.",
    fallback:
      "Die Anmeldung konnte nicht verarbeitet werden. Bitte versuchen Sie es später erneut.",
  });

  function submit(event: FormEvent) {
    event.preventDefault();
    auth.login.mutate(
      { eMailAdresse: email, passwort: password },
      { onSuccess: () => router.replace("/") },
    );
  }

  return (
    <AuthPage
      title="Anmelden"
      description="Öffnen Sie Ihr Logbuch mit E-Mail-Adresse und Passwort."
      footer={
        <>
          <p>
            Noch kein Konto?{" "}
            <AuthLink href="/registrierung">Registrieren</AuthLink>
          </p>
          <p>
            <AuthLink href="/passwort">Passwort vergessen?</AuthLink>
          </p>
        </>
      }
    >
      <Card>
        <CardContent>
          <form className="space-y-5" onSubmit={submit}>
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
              <PasswordInput
                autoComplete="current-password"
                required
                value={password}
                onChange={(event) => setPassword(event.target.value)}
              />
            </Field>

            {failure?.scope === "form" ? (
              <AuthAlert variant="error" message={failure.message} />
            ) : null}

            <Button
              type="submit"
              size="field"
              className="w-full"
              disabled={auth.login.isPending}
            >
              {auth.login.isPending ? "Wird angemeldet …" : "Anmelden"}
            </Button>
          </form>
        </CardContent>
      </Card>
    </AuthPage>
  );
}

export function RegistrationPage() {
  const router = useRouter();
  const auth = useAuthMutations();
  const token = useMailToken();
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [confirmation, setConfirmation] = useState("");
  const [sent, setSent] = useState(false);
  const finishing = Boolean(token);

  // Compared once per keystroke of either field, never per render: the two values
  // are the only inputs, so `useMemo` is the whole of the check. The message waits
  // until both fields have something in them, so it does not fire on the first key.
  const mismatch = useMemo(
    () =>
      password.length > 0 &&
      confirmation.length > 0 &&
      password !== confirmation,
    [password, confirmation],
  );

  const startFailure = authFailure(auth.registerStart.error, {
    badRequest: "Bitte geben Sie eine gültige E-Mail-Adresse ein.",
    tooManyRequests:
      "Zu viele Registrierungsversuche. Bitte warten Sie und versuchen Sie es erneut.",
    fallback:
      "Die Registrierung konnte nicht gestartet werden. Bitte versuchen Sie es später erneut.",
  });

  const finishFailure = authFailure(auth.registerFinish.error, {
    // A `400` for a policy-conforming password is a spent confirmation link.
    invalidLink: passwordWithinPolicy(password)
      ? "Der Bestätigungslink ist ungültig oder abgelaufen. Bitte fordern Sie einen neuen Link an."
      : undefined,
    badRequest: "Das Passwort muss 8 bis 128 Zeichen lang sein.",
    tooManyRequests:
      "Zu viele Registrierungsversuche. Bitte warten Sie und versuchen Sie es erneut.",
    fallback:
      "Das Konto konnte nicht erstellt werden. Bitte versuchen Sie es später erneut.",
  });

  function start(event: FormEvent) {
    event.preventDefault();
    auth.registerStart.mutate(email, { onSuccess: () => setSent(true) });
  }

  function finish(event: FormEvent) {
    event.preventDefault();
    // Belt and braces: the button is disabled while the two differ, and nothing but
    // the chosen password ever reaches the request.
    if (mismatch) return;
    auth.registerFinish.mutate(
      { token, passwort: password },
      { onSuccess: () => router.replace("/") },
    );
  }

  return (
    <AuthPage
      title={finishing ? "Registrierung abschließen" : "Registrieren"}
      description={
        finishing
          ? "Ihre E-Mail-Adresse ist bestätigt. Legen Sie jetzt das Passwort für Ihr Konto fest."
          : "In zwei Schritten: Adresse bestätigen, dann Passwort festlegen."
      }
      pageError={finishFailure?.scope === "page" ? finishFailure.message : null}
      footer={
        <>
          <p>
            Bereits registriert? <AuthLink href="/anmeldung">Anmelden</AuthLink>
          </p>
          {finishing ? (
            <p>
              Kein gültiger Link?{" "}
              <AuthLink href="/registrierung">
                Neuen Bestätigungslink anfordern
              </AuthLink>
            </p>
          ) : null}
        </>
      }
    >
      <Card>
        <CardContent>
          {!finishing ? (
            <form className="space-y-5" onSubmit={start}>
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

              {sent ? (
                <AuthAlert
                  variant="success"
                  message="Sie erhalten eine E-Mail mit dem Bestätigungslink."
                />
              ) : null}

              {startFailure ? (
                <AuthAlert variant="error" message={startFailure.message} />
              ) : null}

              <Button
                type="submit"
                size="field"
                className="w-full"
                disabled={auth.registerStart.isPending}
              >
                {auth.registerStart.isPending
                  ? "Wird gesendet …"
                  : "Bestätigungslink anfordern"}
              </Button>
            </form>
          ) : (
            <form className="space-y-5" onSubmit={finish}>
              <Field label="Passwort" hint="8 bis 128 Zeichen.">
                <PasswordInput
                  autoComplete="new-password"
                  minLength={PASSWORD_MIN}
                  maxLength={PASSWORD_MAX}
                  required
                  value={password}
                  onChange={(event) => setPassword(event.target.value)}
                />
              </Field>

              <Field
                label="Passwort bestätigen"
                error={
                  mismatch ? "Die Passwörter stimmen nicht überein." : undefined
                }
              >
                <PasswordInput
                  autoComplete="new-password"
                  minLength={PASSWORD_MIN}
                  maxLength={PASSWORD_MAX}
                  required
                  aria-invalid={mismatch || undefined}
                  value={confirmation}
                  onChange={(event) => setConfirmation(event.target.value)}
                />
              </Field>

              {finishFailure?.scope === "form" ? (
                <AuthAlert variant="error" message={finishFailure.message} />
              ) : null}

              <Button
                type="submit"
                size="field"
                className="w-full"
                disabled={auth.registerFinish.isPending || mismatch}
              >
                {auth.registerFinish.isPending
                  ? "Wird erstellt …"
                  : "Konto erstellen"}
              </Button>
            </form>
          )}
        </CardContent>
      </Card>
    </AuthPage>
  );
}

export function PasswordPage() {
  const router = useRouter();
  const auth = useAuthMutations();
  const token = useMailToken();
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [sent, setSent] = useState(false);
  const resetting = Boolean(token);

  const requestFailure = authFailure(auth.forgotPassword.error, {
    badRequest: "Bitte geben Sie eine gültige E-Mail-Adresse ein.",
    tooManyRequests:
      "Zu viele Anfragen. Bitte warten Sie und versuchen Sie es erneut.",
    fallback:
      "Die Anfrage konnte nicht verarbeitet werden. Bitte versuchen Sie es später erneut.",
  });

  const resetFailure = authFailure(auth.resetPassword.error, {
    invalidLink: passwordWithinPolicy(password)
      ? "Der Wiederherstellungslink ist ungültig oder abgelaufen. Bitte fordern Sie einen neuen Link an."
      : undefined,
    badRequest: "Das Passwort muss 8 bis 128 Zeichen lang sein.",
    tooManyRequests:
      "Zu viele Anfragen. Bitte warten Sie und versuchen Sie es erneut.",
    fallback:
      "Das Passwort konnte nicht geändert werden. Bitte versuchen Sie es später erneut.",
  });

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
    <AuthPage
      title={resetting ? "Neues Passwort" : "Passwort wiederherstellen"}
      description={
        resetting
          ? "Legen Sie ein neues Passwort für Ihr Konto fest."
          : "Wir senden Ihnen einen Link, mit dem Sie ein neues Passwort festlegen."
      }
      pageError={resetFailure?.scope === "page" ? resetFailure.message : null}
      footer={
        <>
          <p>
            <AuthLink href="/anmeldung">Zur Anmeldung</AuthLink>
          </p>
          {resetting ? (
            <p>
              Kein gültiger Link?{" "}
              <AuthLink href="/passwort">
                Neuen Wiederherstellungslink anfordern
              </AuthLink>
            </p>
          ) : null}
        </>
      }
    >
      <Card>
        <CardContent>
          {!resetting ? (
            <form className="space-y-5" onSubmit={request}>
              <Field
                label="E-Mail-Adresse"
                hint="Falls ein Konto zu dieser Adresse gehört, erhalten Sie eine E-Mail."
              >
                <Input
                  autoComplete="email"
                  type="email"
                  required
                  value={email}
                  onChange={(event) => setEmail(event.target.value)}
                />
              </Field>

              {sent ? (
                <AuthAlert
                  variant="success"
                  message="Falls ein Konto zu dieser Adresse gehört, erhalten Sie eine E-Mail mit weiteren Schritten."
                />
              ) : null}

              {requestFailure ? (
                <AuthAlert variant="error" message={requestFailure.message} />
              ) : null}

              <Button
                type="submit"
                size="field"
                className="w-full"
                disabled={auth.forgotPassword.isPending}
              >
                {auth.forgotPassword.isPending
                  ? "Wird gesendet …"
                  : "Wiederherstellungslink anfordern"}
              </Button>
            </form>
          ) : (
            <form className="space-y-5" onSubmit={reset}>
              <Field label="Neues Passwort" hint="8 bis 128 Zeichen.">
                <PasswordInput
                  autoComplete="new-password"
                  minLength={PASSWORD_MIN}
                  maxLength={PASSWORD_MAX}
                  required
                  value={password}
                  onChange={(event) => setPassword(event.target.value)}
                />
              </Field>

              {resetFailure?.scope === "form" ? (
                <AuthAlert variant="error" message={resetFailure.message} />
              ) : null}

              <Button
                type="submit"
                size="field"
                className="w-full"
                disabled={auth.resetPassword.isPending}
              >
                {auth.resetPassword.isPending
                  ? "Wird geändert …"
                  : "Passwort ändern"}
              </Button>
            </form>
          )}
        </CardContent>
      </Card>
    </AuthPage>
  );
}
