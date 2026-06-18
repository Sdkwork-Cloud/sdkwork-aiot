import { afterEach, describe, expect, it } from 'vitest';

import {
  resolveAiotAppApiBaseUrl,
  resolveAiotEdgeIngressWebSocketBaseUrl,
  resolveAiotPlatformApiGatewayBaseUrl,
} from '../src/sdk/sdkBaseUrls';
import {
  VITE_SDKWORK_AIOT_APPLICATION_APP_HTTP_URL,
  VITE_SDKWORK_AIOT_PLATFORM_API_GATEWAY_HTTP_URL,
} from '../src/sdk/topologyEnvKeys';

const originalImportMetaEnv = { ...import.meta.env };

afterEach(() => {
  Object.assign(import.meta.env, originalImportMetaEnv);
});

describe('sdkwork-aiot-pc-core sdkBaseUrls', () => {
  it('resolves application app-http base url from topology env keys', () => {
    import.meta.env[VITE_SDKWORK_AIOT_APPLICATION_APP_HTTP_URL] =
      'http://127.0.0.1:18082/app/v3/api/iot';

    expect(resolveAiotAppApiBaseUrl()).toBe('http://127.0.0.1:18082');
  });

  it('resolves platform api-gateway base url for IAM and appbase SDKs', () => {
    import.meta.env[VITE_SDKWORK_AIOT_PLATFORM_API_GATEWAY_HTTP_URL] =
      'http://127.0.0.1:3900/app/v3/api';

    expect(resolveAiotPlatformApiGatewayBaseUrl()).toBe('http://127.0.0.1:3900');
  });

  it('derives edge websocket url from edge ingress http url when websocket env is unset', () => {
    import.meta.env[VITE_SDKWORK_AIOT_APPLICATION_APP_HTTP_URL] = 'http://127.0.0.1:18082';
    delete import.meta.env.VITE_SDKWORK_AIOT_EDGE_DEVICE_INGRESS_HTTP_URL;
    delete import.meta.env.VITE_SDKWORK_AIOT_EDGE_DEVICE_INGRESS_WEBSOCKET_URL;

    expect(resolveAiotEdgeIngressWebSocketBaseUrl()).toBe('ws://127.0.0.1:18080');
  });
});
