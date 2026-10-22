import standaloneConfig from "@/standalone-config.json";

export type FrontendConfig = typeof standaloneConfig & {
  dev: boolean;
};

async function loadConfig(): Promise<FrontendConfig> {
  return {
    remoteApiUrl:
      process.env.NEXT_PUBLIC_YAMS_API_URL ?? standaloneConfig.remoteApiUrl,
    dev: process.env.NEXT_PUBLIC_YAMS_DEV
      ? process.env.NEXT_PUBLIC_YAMS_DEV === "true"
      : process.env.NODE_ENV === "development",
  };
}

let cachedConfig: FrontendConfig | null = null;
export async function getConfig(): Promise<FrontendConfig> {
  if (cachedConfig) {
    return cachedConfig;
  }
  cachedConfig = await loadConfig();
  return cachedConfig;
}
