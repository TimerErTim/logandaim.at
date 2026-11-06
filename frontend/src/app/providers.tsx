"use client";

import { QueryClientProvider } from "@tanstack/react-query";
import { useState } from "react";
import { AuthProvider } from "@/api/auth-provider";
import { LogandaimApiProvider } from "@/api/provider";
import { createQueryClient } from "@/lib/query-client";

export function Providers({ children }: { children: React.ReactNode }) {
  const [queryClient] = useState(() => createQueryClient());

  return (
    <QueryClientProvider client={queryClient}>
      <LogandaimApiProvider>
        <AuthProvider>{children}</AuthProvider>
      </LogandaimApiProvider>
    </QueryClientProvider>
  );
}
