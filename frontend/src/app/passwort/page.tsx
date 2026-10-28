import { Suspense } from "react";
import { PasswordPage } from "@/components/auth/auth-pages";

export default function PasswortRoute() {
  return (
    <Suspense fallback={null}>
      <PasswordPage />
    </Suspense>
  );
}
