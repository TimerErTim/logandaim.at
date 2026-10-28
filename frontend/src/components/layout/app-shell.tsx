"use client";

import {
  CircleAlert,
  LogIn,
  LogOut,
  Menu,
  Settings2,
  UserRound,
} from "lucide-react";
import Link from "next/link";
import { usePathname } from "next/navigation";
import { useAuth } from "@/api/auth-provider";
import { useAuthMutations, useBenutzerQuery } from "@/api/hooks";
import { usePageChrome } from "@/components/layout/page-chrome";
import { Sidebar } from "@/components/layout/sidebar";
import { Alert } from "@/components/ui/alert";
import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import {
  breadcrumbsForPathname,
  homeNavItem,
  navActionsForPathname,
  sidebarCategories,
} from "@/lib/navigation";

export function AppShell({ children }: { children: React.ReactNode }) {
  const pathname = usePathname();
  const { breadcrumbLabel } = usePageChrome();

  const breadcrumbs = breadcrumbsForPathname(pathname, breadcrumbLabel);
  const actions = navActionsForPathname(pathname);
  const phase = useAuth();
  const benutzer = useBenutzerQuery();
  const auth = useAuthMutations();
  const onAuthRoute = ["/anmeldung", "/registrierung", "/passwort"].includes(
    pathname.replace(/\/$/, ""),
  );

  return (
    <div className="flex h-screen overflow-hidden bg-canvas">
      <input id="main-drawer" type="checkbox" className="peer hidden" />

      <Sidebar
        categories={sidebarCategories}
        homeItem={homeNavItem}
        pathname={pathname}
      />

      <label
        htmlFor="main-drawer"
        aria-label="Navigation schließen"
        className="pointer-events-none fixed inset-0 z-40 bg-black/50 opacity-0 transition-opacity peer-checked:pointer-events-auto peer-checked:opacity-100 lg:hidden"
      />

      <div className="relative flex min-w-0 flex-1 flex-col overflow-hidden">
        <header className="sticky top-0 z-30 border-b border-subtle bg-surface">
          <div className="flex h-14 items-center gap-3 px-4">
            <label
              htmlFor="main-drawer"
              className="inline-flex size-11 cursor-pointer items-center justify-center rounded-md text-ink hover:bg-surface-muted lg:hidden"
            >
              <Menu className="size-5" />
              <span className="sr-only">Navigation öffnen</span>
            </label>

            <nav
              aria-label="Breadcrumb"
              className="flex min-w-0 flex-1 items-center gap-1 overflow-x-auto text-sm"
            >
              {breadcrumbs.map((crumb, index) => (
                <span
                  key={`${crumb.label}-${index}`}
                  className="flex items-center"
                >
                  {index > 0 ? (
                    <span className="mx-1 text-secondary">/</span>
                  ) : null}
                  {crumb.href ? (
                    <Link
                      href={crumb.href}
                      className="rounded-sm px-2 py-1 font-medium text-ink hover:bg-surface-muted"
                    >
                      {crumb.label}
                    </Link>
                  ) : (
                    <span className="px-2 py-1 font-semibold text-ink">
                      {crumb.label}
                    </span>
                  )}
                </span>
              ))}
            </nav>

            <div className="flex shrink-0 items-center gap-2">
              {actions.map((action) => (
                <Link key={action.href} href={action.href}>
                  <Button size="sm" variant="secondary">
                    {action.label}
                  </Button>
                </Link>
              ))}
              {phase.status === "authenticated" && benutzer.data ? (
                <DropdownMenu>
                  <DropdownMenuTrigger asChild>
                    <Button
                      size="icon"
                      variant="secondary"
                      aria-label="Profilmenü öffnen"
                      disabled={
                        auth.logout.isPending || auth.deleteAccount.isPending
                      }
                    >
                      <UserRound />
                    </Button>
                  </DropdownMenuTrigger>
                  <DropdownMenuContent align="end" className="w-64">
                    <DropdownMenuLabel className="normal-case">
                      <span className="block truncate text-secondary">
                        Angemeldet als
                      </span>
                      <span className="mt-1 block truncate text-sm font-medium text-ink">
                        {benutzer.data.eMailAdresse}
                      </span>
                    </DropdownMenuLabel>
                    <DropdownMenuSeparator />
                    <DropdownMenuItem asChild>
                      <Link href="/profil">
                        <UserRound aria-hidden="true" /> Profil
                      </Link>
                    </DropdownMenuItem>
                    <DropdownMenuItem asChild>
                      <Link href="/optionen">
                        <Settings2 aria-hidden="true" /> Optionen
                      </Link>
                    </DropdownMenuItem>
                    <DropdownMenuSeparator />
                    <DropdownMenuItem
                      onSelect={() =>
                        auth.logout.mutate(undefined, {
                          onSuccess: () => window.location.assign("/"),
                        })
                      }
                    >
                      <LogOut aria-hidden="true" /> Abmelden
                    </DropdownMenuItem>
                  </DropdownMenuContent>
                </DropdownMenu>
              ) : !onAuthRoute ? (
                <Link href="/anmeldung">
                  <Button
                    size="sm"
                    variant={onAuthRoute ? "secondary" : "primary"}
                  >
                    <LogIn />
                    Anmelden
                  </Button>
                </Link>
              ) : null}
            </div>
          </div>
        </header>

        <main className="flex-1 overflow-y-auto">
          {phase.status === "unavailable" ? (
            <div className="mx-auto w-full max-w-5xl px-6 pt-6">
              <Alert
                variant="warning"
                icon={<CircleAlert aria-hidden="true" />}
              >
                <p>
                  Ihre Sitzung konnte nicht überprüft werden. Bitte laden Sie
                  die Seite erneut oder melden Sie sich an.
                </p>
              </Alert>
            </div>
          ) : null}
          {children}
        </main>
      </div>
    </div>
  );
}
