/**
 * Generated SDK placeholder.
 *
 * This package boundary is reserved for SDKWork OpenAPI generation from
 * ../openapi/sdkwork-aiot-backend-sdk.openapi.json. Do not add handwritten
 * transport logic here; update OpenAPI and regenerate the SDK instead.
 */
export interface SdkworkAiotBackendClient {
  iot: {
    products: {
      list: unknown;
    };
    hardwareProfiles: {
      list: unknown;
    };
    protocolProfiles: {
      list: unknown;
    };
    capabilityModels: {
      retrieve: unknown;
    };
    devices: {
      list: unknown;
      credentials: {
        create: unknown;
      };
    };
    firmwareArtifacts: {
      create: unknown;
    };
    firmwareRollouts: {
      create: unknown;
    };
    protocolAdapters: {
      list: unknown;
    };
    runtime: {
      capacity: {
        retrieve: unknown;
      };
    };
  };
}
