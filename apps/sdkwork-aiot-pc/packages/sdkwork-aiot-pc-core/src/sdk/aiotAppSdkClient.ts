import { readPcReactRuntimeSession } from '@sdkwork/core-pc-react';
import {
  createAiotAppClient,
  type SdkworkAiotAppClient,
  type SdkworkAiotAppClientConfig,
} from '@sdkwork/aiot-app-sdk';

import { resolveAiotAppApiBaseUrl } from './sdkBaseUrls';

export type SdkworkAiotPcAppClientConfig = SdkworkAiotAppClientConfig;

let aiotAppSdkClient: SdkworkAiotAppClient | null = null;

function normalizeBearerToken(value: string | undefined): string | undefined {
  const trimmed = value?.trim();
  return trimmed && trimmed.length > 0 ? trimmed : undefined;
}

export function createAiotAppSdkClientConfig(
  session = readPcReactRuntimeSession(),
): SdkworkAiotPcAppClientConfig {
  return {
    baseUrl: resolveAiotAppApiBaseUrl(),
    authToken: normalizeBearerToken(session.authToken),
    accessToken: normalizeBearerToken(session.accessToken),
    platform: 'pc',
  };
}

export function initAiotAppSdkClient(
  config: SdkworkAiotPcAppClientConfig = createAiotAppSdkClientConfig(),
): SdkworkAiotAppClient {
  aiotAppSdkClient = createAiotAppClient(config);
  return aiotAppSdkClient;
}

export function getAiotAppSdkClient(): SdkworkAiotAppClient {
  return aiotAppSdkClient ?? initAiotAppSdkClient();
}

export function resetAiotAppSdkClient(): void {
  aiotAppSdkClient = null;
}
