import {
  createAiotAppClient,
  type SdkworkAiotAppClient,
  type SdkworkAiotAppClientConfig,
} from '@sdkwork/aiot-app-sdk';

function normalizeBearerToken(value: string | undefined): string | undefined {
  const trimmed = value?.trim();
  return trimmed && trimmed.length > 0 ? trimmed : undefined;
}

function readEnv(key: string, fallback: string): string {
  const value = import.meta.env[key];
  return typeof value === 'string' && value.trim().length > 0 ? value.trim() : fallback;
}

let aiotAppSdkClient: SdkworkAiotAppClient | null = null;

export function createAiotH5AppSdkClientConfig(): SdkworkAiotAppClientConfig {
  return {
    accessToken: normalizeBearerToken(readEnv('VITE_SDKWORK_ACCESS_TOKEN', '')),
    authToken: normalizeBearerToken(readEnv('VITE_SDKWORK_AUTH_TOKEN', '')),
    baseUrl: readEnv('VITE_SDKWORK_AIOT_APPLICATION_APP_HTTP_URL', 'http://127.0.0.1:8082'),
    platform: 'h5',
  };
}

export function initAiotH5AppSdkClient(
  config: SdkworkAiotAppClientConfig = createAiotH5AppSdkClientConfig(),
): SdkworkAiotAppClient {
  aiotAppSdkClient = createAiotAppClient(config);
  return aiotAppSdkClient;
}

export function getAiotH5AppSdkClient(): SdkworkAiotAppClient {
  return aiotAppSdkClient ?? initAiotH5AppSdkClient();
}

export {
  createAiotAgentService,
  createAiotCommandService,
  createAiotVoiceService,
} from '@sdkwork/aiot-app-core';
