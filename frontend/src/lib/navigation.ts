import type { LucideIcon } from "lucide-react";
import { Home } from "lucide-react";

export type NavAction = {
  label: string;
  href: string;
};

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
} as const;

/** Empty until the first product slice adds routes. */
export const sidebarCategories: NavCategory[] = [];

export const homeNavItem: NavItem = {
  id: "home",
  label: "Home",
  href: paths.home,
  icon: Home,
  recursive: false,
};

type BreadcrumbSegment = {
  label: string;
  href?: string;
};

const routeLabels: Record<string, string> = {};

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

export function navActionsForPathname(_pathname: string): NavAction[] {
  return [];
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
