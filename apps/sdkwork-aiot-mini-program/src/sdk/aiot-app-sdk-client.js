/**
 * WeChat mini program transport adapter for the @sdkwork/aiot-app-sdk contract.
 * Business modules must consume this client instead of assembling raw wx.request calls.
 */

const STORAGE_KEYS = {
  appApiBaseUrl: 'SDKWORK_AIOT_APP_API_BASE_URL',
  authToken: 'SDKWORK_AUTH_TOKEN',
  accessToken: 'SDKWORK_ACCESS_TOKEN',
};

const DEFAULT_APP_API_BASE_URL = 'http://127.0.0.1:8082';

let cachedClient = null;

function readStorage(key) {
  const value = wx.getStorageSync(key);
  return typeof value === 'string' && value.trim() ? value.trim() : undefined;
}

function createAiotAppSdkClientConfig() {
  return {
    baseUrl: readStorage(STORAGE_KEYS.appApiBaseUrl) || DEFAULT_APP_API_BASE_URL,
    authToken: readStorage(STORAGE_KEYS.authToken),
    accessToken: readStorage(STORAGE_KEYS.accessToken),
    platform: 'mini-program',
  };
}

function request(method, path, options = {}) {
  const config = createAiotAppSdkClientConfig();
  const headers = {
    'Content-Type': 'application/json',
  };

  if (config.authToken) {
    headers.Authorization = `Bearer ${config.authToken}`;
  }
  if (config.accessToken) {
    headers['Access-Token'] = config.accessToken;
  }
  if (options.idempotencyKey) {
    headers['Idempotency-Key'] = options.idempotencyKey;
  }

  return new Promise((resolve, reject) => {
    wx.request({
      url: `${config.baseUrl}/app/v3/api${path}`,
      method,
      data: options.data,
      header: headers,
      success(response) {
        if (response.statusCode >= 200 && response.statusCode < 300) {
          resolve({
            code: String(response.data?.code ?? '0'),
            data: response.data?.data ?? response.data,
          });
          return;
        }

        reject(new Error(`api.http.${response.statusCode}`));
      },
      fail(error) {
        reject(error instanceof Error ? error : new Error(String(error?.errMsg || 'request failed')));
      },
    });
  });
}

function createAiotAppSdkClient() {
  return {
    iot: {
      devicesList() {
        return request('GET', '/iot/devices');
      },
      devicesCommandsCreate(deviceId, body, idempotencyKey) {
        return request('POST', `/iot/devices/${deviceId}/commands`, {
          data: body,
          idempotencyKey,
        });
      },
    },
  };
}

function getAiotAppSdkClient() {
  if (!cachedClient) {
    cachedClient = createAiotAppSdkClient();
  }
  return cachedClient;
}

function resetAiotAppSdkClient() {
  cachedClient = null;
}

module.exports = {
  createAiotAppSdkClient,
  createAiotAppSdkClientConfig,
  getAiotAppSdkClient,
  resetAiotAppSdkClient,
};
