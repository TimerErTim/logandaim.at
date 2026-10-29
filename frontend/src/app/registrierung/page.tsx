import { Suspense } from "react";
import { RegistrationPage } from "@/components/auth/auth-pages";

export default function RegistrierungRoute() {
  return (
    <Suspense fallback={null}>
      <RegistrationPage />
    </Suspense>
  );
}
