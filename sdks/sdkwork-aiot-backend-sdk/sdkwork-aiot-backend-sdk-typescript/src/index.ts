import {
  createClient as createGeneratedAiotBackendClient,
  SdkworkBackendClient,
} from "../generated/server-openapi/src/index.js";
import type { SdkworkBackendConfig } from "../generated/server-openapi/src/types/common.js";

export { SdkworkBackendClient, createGeneratedAiotBackendClient };
export * from "../generated/server-openapi/src/types/index.js";
export * from "../generated/server-openapi/src/api/index.js";
export * from "../generated/server-openapi/src/http/index.js";
export * from "../generated/server-openapi/src/auth/index.js";
export type { SdkworkBackendConfig } from "../generated/server-openapi/src/types/common.js";

export type SdkworkAiotBackendClient = SdkworkBackendClient;
export type SdkworkAiotBackendClientConfig = SdkworkBackendConfig;

export function createAiotBackendClient(
  config: SdkworkBackendConfig,
): SdkworkAiotBackendClient {
  return createGeneratedAiotBackendClient(config);
}

export function createClient(config: SdkworkBackendConfig): SdkworkAiotBackendClient {
  return createAiotBackendClient(config);
}
