"use client";

import { Menu } from "lucide-react";
import Link from "next/link";
import { usePathname } from "next/navigation";
import { usePageChrome } from "@/components/layout/page-chrome";
import { Sidebar } from "@/components/layout/sidebar";
import { Button } from "@/components/ui/button";
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

  return (
    <div className="flex h-screen overflow-hidden bg-background">
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
        <header className="sticky top-0 z-30 border-b border-border bg-surface">
          <div className="flex h-14 items-center gap-3 px-4">
            <label
              htmlFor="main-drawer"
              className="inline-flex size-11 cursor-pointer items-center justify-center rounded-md text-foreground hover:bg-surface-muted lg:hidden"
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
                    <span className="mx-1 text-muted-foreground">/</span>
                  ) : null}
                  {crumb.href ? (
                    <Link
                      href={crumb.href}
                      className="rounded-sm px-2 py-1 font-medium text-foreground hover:bg-surface-muted"
                    >
                      {crumb.label}
                    </Link>
                  ) : (
                    <span className="px-2 py-1 font-semibold text-foreground">
                      {crumb.label}
                    </span>
                  )}
                </span>
              ))}
            </nav>

            <div className="flex shrink-0 items-center gap-2">
              {actions.map((action) => (
                <Link key={action.href} href={action.href}>
                  <Button size="sm" variant="primary">
                    {action.label}
                  </Button>
                </Link>
              ))}
            </div>
          </div>
        </header>

        <main className="flex-1 overflow-y-auto">{children}</main>
      </div>
    </div>
  );
}
