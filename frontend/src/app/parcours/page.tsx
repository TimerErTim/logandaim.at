"use client";

import { useRouter } from "next/navigation";
import { useEffect } from "react";

import { paths } from "@/lib/navigation";

/**
 * The old public Parcours list. The archer journey now begins at the Anlagen
 * directory, one level up, so this route only redirects — it stays on disk because
 * links to `/parcours` were handed out and must keep resolving.
 */
export default function ParcoursRedirectPage() {
  const router = useRouter();

  useEffect(() => {
    router.replace(paths.anlagen);
  }, [router]);

  return (
    <div className="p-6">
      <p className="text-sm text-secondary">Weiterleitung zu den Anlagen …</p>
    </div>
  );
}
