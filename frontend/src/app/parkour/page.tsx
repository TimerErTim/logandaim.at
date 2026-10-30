"use client";

import { useRouter } from "next/navigation";
import { useEffect } from "react";

import { paths } from "@/lib/navigation";

/**
 * The old public Parkour list, now at `/parkour`. The archer journey begins at the
 * Anlagen directory, one level up, so this route only redirects.
 */
export default function ParkourRedirectPage() {
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
