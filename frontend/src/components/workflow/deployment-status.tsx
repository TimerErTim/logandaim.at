"use client";

import { useHealthQuery } from "@/api/hooks";
import { useLogandaimApi } from "@/api/provider";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";

export function DeploymentStatus() {
  const { apiBaseUrl } = useLogandaimApi();
  const health = useHealthQuery();

  return (
    <Card>
      <CardHeader>
        <CardTitle>Deployment</CardTitle>
        <CardDescription>
          Transport adapter and backend health for this session.
        </CardDescription>
      </CardHeader>
      <CardContent>
        <dl className="grid gap-3 text-sm sm:grid-cols-2">
          <div>
            <dt className="font-medium text-secondary">API base</dt>
            <dd className="mt-0.5 font-mono text-xs">{apiBaseUrl}</dd>
          </div>
          <div>
            <dt className="font-medium text-secondary">Health</dt>
            <dd className="mt-0.5">
              {health.isPending ? "…" : (health.data ?? "—")}
            </dd>
          </div>
        </dl>
      </CardContent>
    </Card>
  );
}
