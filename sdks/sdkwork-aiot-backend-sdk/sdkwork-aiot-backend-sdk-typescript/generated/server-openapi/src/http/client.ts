import type { SdkworkBackendConfig } from '../types/common';
import type { RequestOptions, QueryParams } from '@sdkwork/sdk-common';
import type { AuthTokenManager } from '@sdkwork/sdk-common';
import { BaseHttpClient, withRetry } from '@sdkwork/sdk-common';

type HttpRequestOptions = RequestOptions & {
  method?: string;
  body?: unknown;
  headers?: Record<string, string>;
  contentType?: string;
};

export class HttpClient extends BaseHttpClient {
  private static readonly API_KEY_HEADER: string = 'Access-Token';
  private static readonly ACCESS_TOKEN_HEADER: string = 'Access-Token';
  private static readonly API_KEY_USE_BEARER = false;

  constructor(config: SdkworkBackendConfig) {
    super(config as any);
  }

  private getInternalAuthConfig(): any {
    const self = this as any;
    self.authConfig = self.authConfig || {};
    return self.authConfig;
  }

  private getInternalHeaders(): Record<string, string> {
    const self = this as any;
    self.config = self.config || {};
    self.config.headers = self.config.headers || {};
    return self.config.headers;
  }

  private buildRequestHeaders(
    headers?: Record<string, string>,
    contentType?: string,
  ): Record<string, string> | undefined {
    const mergedHeaders = {
      ...(headers ?? {}),
    };

    if (contentType && contentType.toLowerCase() !== 'multipart/form-data') {
      mergedHeaders['Content-Type'] = contentType;
    }

    return Object.keys(mergedHeaders).length > 0 ? mergedHeaders : undefined;
  }

  private buildRequestBody(body: unknown, contentType?: string): unknown {
    if (body == null) {
      return body;
    }

    const normalizedContentType = (contentType ?? '').toLowerCase();
    if (normalizedContentType === 'application/x-www-form-urlencoded') {
      return this.encodeFormBody(body);
    }

    return body;
  }

  private encodeFormBody(body: unknown): string {
    if (body instanceof URLSearchParams) {
      return body.toString();
    }
    if (typeof body === 'string') {
      return body;
    }

    const params = new URLSearchParams();
    if (body instanceof Map) {
      for (const [key, value] of body.entries()) {
        this.appendFormValue(params, String(key), value);
      }
      return params.toString();
    }
    if (typeof body === 'object') {
      for (const [key, value] of Object.entries(body as Record<string, unknown>)) {
        this.appendFormValue(params, key, value);
      }
      return params.toString();
    }

    params.append('value', String(body));
    return params.toString();
  }

  private appendFormValue(params: URLSearchParams, key: string, value: unknown): void {
    if (value == null) {
      return;
    }
    if (Array.isArray(value)) {
      value.forEach((item) => this.appendFormValue(params, key, item));
      return;
    }
    if (value instanceof Date) {
      params.append(key, value.toISOString());
      return;
    }
    if (typeof value === 'object') {
      params.append(key, JSON.stringify(value));
      return;
    }
    params.append(key, String(value));
  }

  setApiKey(apiKey: string): void {
    const authConfig = this.getInternalAuthConfig();
    const headers = this.getInternalHeaders();
    authConfig.apiKey = apiKey;
    authConfig.tokenManager?.clearTokens?.();

    if (HttpClient.API_KEY_HEADER === 'Authorization' && HttpClient.API_KEY_USE_BEARER) {
      authConfig.authMode = 'apikey';
      delete headers[HttpClient.ACCESS_TOKEN_HEADER];
      delete headers['Access-Token'];
      return;
    }

    authConfig.authMode = 'dual-token';
    headers[HttpClient.API_KEY_HEADER] = HttpClient.API_KEY_USE_BEARER
      ? `Bearer ${apiKey}`
      : apiKey;

    if (HttpClient.API_KEY_HEADER.toLowerCase() !== 'authorization') {
      delete headers['Authorization'];
    }
    if (HttpClient.API_KEY_HEADER.toLowerCase() !== HttpClient.ACCESS_TOKEN_HEADER.toLowerCase()) {
      delete headers[HttpClient.ACCESS_TOKEN_HEADER];
    }
    delete headers['Access-Token'];
  }

  setAuthToken(token: string): void {
    const headers = this.getInternalHeaders();
    if (HttpClient.API_KEY_HEADER.toLowerCase() !== 'authorization') {
      delete headers[HttpClient.API_KEY_HEADER];
    }
    super.setAuthToken(token);
  }

  setAccessToken(token: string): void {
    const headers = this.getInternalHeaders();
    if (HttpClient.API_KEY_HEADER.toLowerCase() !== HttpClient.ACCESS_TOKEN_HEADER.toLowerCase()) {
      delete headers[HttpClient.API_KEY_HEADER];
    }
    headers[HttpClient.ACCESS_TOKEN_HEADER] = token;
    delete headers['Access-Token'];
    super.setAccessToken(token);
  }

  setTokenManager(manager: AuthTokenManager): void {
    const baseProto = Object.getPrototypeOf(HttpClient.prototype) as { setTokenManager?: (this: HttpClient, m: AuthTokenManager) => void };
    if (typeof baseProto.setTokenManager === 'function') {
      baseProto.setTokenManager.call(this, manager);
      return;
    }
    this.getInternalAuthConfig().tokenManager = manager;
  }

  private applySdkworkAuthHeaders(headers?: Record<string, string>): Record<string, string> | undefined {
    const authConfig = this.getInternalAuthConfig();
    const tokenManager = authConfig.tokenManager;
    const accessToken = tokenManager?.getAccessToken?.();
    if (!accessToken || HttpClient.ACCESS_TOKEN_HEADER === 'Access-Token') {
      return headers;
    }

    const nextHeaders = {
      ...(headers ?? {}),
      [HttpClient.ACCESS_TOKEN_HEADER]: accessToken,
    };
    delete nextHeaders['Access-Token'];
    return nextHeaders;
  }

  async request<T>(path: string, options: HttpRequestOptions = {}): Promise<T> {
    const execute = (this as any).execute;
    if (typeof execute !== 'function') {
      throw new Error('BaseHttpClient execute method is not available');
    }
    const { body, headers, contentType, method = 'GET', ...rest } = options;
    const requestHeaders = this.applySdkworkAuthHeaders(headers);
    return withRetry(
      () => execute.call(this, { 
        url: path, 
        method,
        ...rest,
        body: this.buildRequestBody(body, contentType),
        headers: this.buildRequestHeaders(requestHeaders, body == null ? undefined : contentType),
      }),
      { maxRetries: 3 }
    );
  }

  async *streamJson<T>(path: string, options: HttpRequestOptions = {}): AsyncIterable<T> {
    const stream = (BaseHttpClient.prototype as any).stream;
    if (typeof stream !== 'function') {
      throw new Error('BaseHttpClient stream method is not available');
    }
    const { body, headers, contentType, method = 'GET', ...rest } = options;
    const authHeaders = this.applySdkworkAuthHeaders(headers);
    const requestHeaders = this.buildRequestHeaders(
      { Accept: 'text/event-stream', ...(authHeaders ?? {}) },
      body == null ? undefined : contentType,
    );

    for await (const data of stream.call(this, path, {
      method,
      ...rest,
      body: this.buildRequestBody(body, contentType),
      headers: requestHeaders,
    })) {
      if (data === '[DONE]') {
        return;
      }
      if (typeof data !== 'string' || data.trim().length === 0) {
        continue;
      }
      yield JSON.parse(data) as T;
    }
  }

  async get<T>(path: string, params?: QueryParams, headers?: Record<string, string>): Promise<T> {
    return this.request<T>(path, { method: 'GET', params, headers });
  }

  async post<T>(
    path: string,
    body?: unknown,
    params?: QueryParams,
    headers?: Record<string, string>,
    contentType?: string,
  ): Promise<T> {
    return this.request<T>(path, { method: 'POST', body, params, headers, contentType });
  }

  async put<T>(
    path: string,
    body?: unknown,
    params?: QueryParams,
    headers?: Record<string, string>,
    contentType?: string,
  ): Promise<T> {
    return this.request<T>(path, { method: 'PUT', body, params, headers, contentType });
  }

  async delete<T>(path: string, params?: QueryParams, headers?: Record<string, string>): Promise<T> {
    return this.request<T>(path, { method: 'DELETE', params, headers });
  }

  async patch<T>(
    path: string,
    body?: unknown,
    params?: QueryParams,
    headers?: Record<string, string>,
    contentType?: string,
  ): Promise<T> {
    return this.request<T>(path, { method: 'PATCH', body, params, headers, contentType });
  }
}

export function createHttpClient(config: SdkworkBackendConfig): HttpClient {
  return new HttpClient(config);
}
