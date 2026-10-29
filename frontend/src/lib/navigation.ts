import type { LucideIcon } from "lucide-react";
import { Compass, History, Home, Route } from "lucide-react";

export type NavItem = {
  id: string;
  label: string;
  href: string;
  icon: LucideIcon;
  recursive?: boolean;
};

export type NavCategory = {
  label: string;
  items: NavItem[];
  /**
   * A category only staff may see. A plain `Benutzer` never administers an Anlage,
   * so the whole group is hidden instead of shown as a dead end.
   */
  verwaltungOnly?: boolean;
};

/** Strip trailing slash so `/foo/` matches `/foo` (Next `trailingSlash: true`). */
export function normalizePath(path: string): string {
  if (path.length > 1 && path.endsWith("/")) {
    return path.slice(0, -1);
  }
  return path;
}

export function isExactPath(pathname: string, href: string): boolean {
  return normalizePath(pathname) === normalizePath(href);
}

export const paths = {
  home: "/",
  anlagen: "/anlagen",
  anlagenDetail: "/anlagen/detail",
  /** Redirect source only; the archer journey starts at `anlagen`. */
  parcours: "/parcours",
  runden: "/runden",
  verwaltung: "/verwaltung",
  verwaltungAnlage: "/verwaltung/anlage",
  verwaltungAnlageNeu: "/verwaltung/anlage/neu",
  verwaltungParcours: "/verwaltung/parcours",
  profil: "/profil",
  optionen: "/optionen",
} as const;

/**
 * Add one item per product area. The archer's logbook is the directory of Anlagen and
 * the history of their own Runden; the staff's Verwaltung is a separate group, marked
 * so it can be filtered out for a caller who administers nothing.
 */
export const sidebarCategories: NavCategory[] = [
  {
    label: "Logbuch",
    items: [
      {
        id: "anlagen",
        label: "Anlagen",
        href: paths.anlagen,
        icon: Compass,
        recursive: true,
      },
      {
        id: "historie",
        label: "Historie",
        href: paths.runden,
        icon: History,
        recursive: true,
      },
    ],
  },
  {
    label: "Verwaltung",
    verwaltungOnly: true,
    items: [
      {
        id: "verwaltung",
        label: "Verwaltung",
        href: paths.verwaltung,
        icon: Route,
        recursive: true,
      },
    ],
  },
];

export const homeNavItem: NavItem = {
  id: "home",
  label: "Home",
  href: paths.home,
  icon: Home,
  recursive: false,
};

/**
 * The categories a caller may see. A `verwaltungOnly` group is hidden for anyone who
 * neither is a Systemadministrator nor holds a Verwaltung on at least one Anlage, so a
 * plain Benutzer never reads a navigation entry that leads to a 403.
 */
export function visibleSidebarCategories(
  categories: NavCategory[],
  {
    istSystemadministrator,
    anzahlAnlagen,
  }: { istSystemadministrator: boolean; anzahlAnlagen: number },
): NavCategory[] {
  return categories.filter(
    (category) =>
      !category.verwaltungOnly || istSystemadministrator || anzahlAnlagen > 0,
  );
}

type BreadcrumbSegment = {
  label: string;
  href?: string;
};

const routeLabels: Record<string, string> = {
  anlagen: "Anlagen",
  parcours: "Parcours",
  historie: "Historie",
  runden: "Historie",
  verwaltung: "Verwaltung",
  anlage: "Anlage",
  detail: "Runde",
  anmeldung: "Anmelden",
  registrierung: "Registrieren",
  passwort: "Passwort wiederherstellen",
  profil: "Profil",
  optionen: "Optionen",
};

export function breadcrumbsForPathname(
  pathname: string,
  dynamicLabel?: string,
): BreadcrumbSegment[] {
  const segments = pathname.split("/").filter(Boolean);
  if (segments.length === 0) {
    return [{ label: "Home", href: paths.home }];
  }

  const crumbs: BreadcrumbSegment[] = [];
  let href = "";

  for (let index = 0; index < segments.length; index += 1) {
    const segment = segments[index] ?? "";
    href += `/${segment}`;
    const isLast = index === segments.length - 1;
    const isUuid =
      /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(
        segment,
      );

    let label = routeLabels[segment] ?? segment;
    if (isUuid && dynamicLabel) {
      label = dynamicLabel;
    } else if (isUuid) {
      label = "Detail";
    }

    crumbs.push({
      label,
      href: isLast ? undefined : href,
    });
  }

  return crumbs;
}

export function isNavItemActive(
  pathname: string,
  href: string,
  recursive = true,
): boolean {
  const normalizedPath = normalizePath(pathname);
  const normalizedHref = normalizePath(href);

  if (recursive) {
    return (
      normalizedPath === normalizedHref ||
      normalizedPath.startsWith(`${normalizedHref}/`)
    );
  }
  return normalizedPath === normalizedHref;
}
