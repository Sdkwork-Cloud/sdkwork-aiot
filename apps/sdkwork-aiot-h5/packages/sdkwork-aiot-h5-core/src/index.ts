import {
  createAiotAppClient,
  type SdkworkAiotAppClient,
  type SdkworkAiotAppClientConfig,
} from '@sdkwork/aiot-app-sdk';
import {
  readImportMetaEnvWithDefault,
  readOptionalBearerToken,
  readProcessEnv,
} from '@sdkwork/aiot-app-core';

let aiotAppSdkClient: SdkworkAiotAppClient | null = null;

export function createAiotH5AppSdkClientConfig(): SdkworkAiotAppClientConfig {
  return {
    accessToken: readOptionalBearerToken(readProcessEnv('SDKWORK_ACCESS_TOKEN')),
    baseUrl: readImportMetaEnvWithDefault(
      'VITE_SDKWORK_AIOT_APPLICATION_APP_HTTP_URL',
      'http://127.0.0.1:8082',
    ),
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
