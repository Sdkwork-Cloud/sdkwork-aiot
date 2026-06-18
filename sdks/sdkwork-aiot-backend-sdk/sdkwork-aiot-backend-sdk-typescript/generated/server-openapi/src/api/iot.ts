import { backendApiPath } from './paths';
import type { HttpClient } from '../http/client';

import type { AiotCapabilityModelCreateRequest, AiotCapabilityModelListResponse, AiotCapabilityModelResponse, AiotCapabilityModelUpdateRequest, AiotCommandListResponse, AiotCredentialCreateRequest, AiotCredentialResponse, AiotDeviceCreateRequest, AiotDeviceListResponse, AiotDeviceUpdateRequest, AiotEventListResponse, AiotFirmwareArtifactCreateRequest, AiotFirmwareArtifactResponse, AiotFirmwareArtifactUpdateRequest, AiotFirmwareRolloutCreateRequest, AiotFirmwareRolloutResponse, AiotFirmwareRolloutUpdateRequest, AiotHardwareProfileCreateRequest, AiotHardwareProfileListResponse, AiotHardwareProfileResponse, AiotHardwareProfileUpdateRequest, AiotProductCreateRequest, AiotProductListResponse, AiotProductResponse, AiotProductUpdateRequest, AiotProtocolAdapterListResponse, AiotProtocolProfileCreateRequest, AiotProtocolProfileListResponse, AiotProtocolProfileResponse, AiotProtocolProfileUpdateRequest, AiotRuntimeCapacityPolicyResponse, AiotTwinUpdateRequest, StandardCollectionResponse, StandardResourceResponse } from '../types';


export class IotApi {
  private client: HttpClient;
  
  constructor(client: HttpClient) { 
    this.client = client; 
  }

/** List AIoT products */
  async productsList(): Promise<AiotProductListResponse> {
    return this.client.get<AiotProductListResponse>(backendApiPath(`/iot/products`));
  }

/** Create AIoT product */
  async productsCreate(body: AiotProductCreateRequest): Promise<AiotProductResponse> {
    return this.client.post<AiotProductResponse>(backendApiPath(`/iot/products`), body, undefined, undefined, 'application/json');
  }

/** Retrieve AIoT product */
  async productsRetrieve(productId: string): Promise<AiotProductResponse> {
    return this.client.get<AiotProductResponse>(backendApiPath(`/iot/products/${serializePathParameter(productId, { name: 'productId', style: 'simple', explode: false })}`));
  }

/** Update AIoT product */
  async productsUpdate(productId: string, body?: AiotProductUpdateRequest): Promise<AiotProductResponse> {
    return this.client.put<AiotProductResponse>(backendApiPath(`/iot/products/${serializePathParameter(productId, { name: 'productId', style: 'simple', explode: false })}`), body, undefined, undefined, 'application/json');
  }

/** Delete AIoT product */
  async productsDelete(productId: string): Promise<void> {
    return this.client.delete<void>(backendApiPath(`/iot/products/${serializePathParameter(productId, { name: 'productId', style: 'simple', explode: false })}`));
  }

/** List hardware profiles */
  async hardwareProfilesList(): Promise<AiotHardwareProfileListResponse> {
    return this.client.get<AiotHardwareProfileListResponse>(backendApiPath(`/iot/hardware_profiles`));
  }

/** Create hardware profile */
  async hardwareProfilesCreate(body: AiotHardwareProfileCreateRequest): Promise<AiotHardwareProfileResponse> {
    return this.client.post<AiotHardwareProfileResponse>(backendApiPath(`/iot/hardware_profiles`), body, undefined, undefined, 'application/json');
  }

/** Retrieve hardware profile */
  async hardwareProfilesRetrieve(hardwareProfileId: string): Promise<AiotHardwareProfileResponse> {
    return this.client.get<AiotHardwareProfileResponse>(backendApiPath(`/iot/hardware_profiles/${serializePathParameter(hardwareProfileId, { name: 'hardwareProfileId', style: 'simple', explode: false })}`));
  }

/** Update hardware profile */
  async hardwareProfilesUpdate(hardwareProfileId: string, body?: AiotHardwareProfileUpdateRequest): Promise<AiotHardwareProfileResponse> {
    return this.client.put<AiotHardwareProfileResponse>(backendApiPath(`/iot/hardware_profiles/${serializePathParameter(hardwareProfileId, { name: 'hardwareProfileId', style: 'simple', explode: false })}`), body, undefined, undefined, 'application/json');
  }

/** Delete hardware profile */
  async hardwareProfilesDelete(hardwareProfileId: string): Promise<void> {
    return this.client.delete<void>(backendApiPath(`/iot/hardware_profiles/${serializePathParameter(hardwareProfileId, { name: 'hardwareProfileId', style: 'simple', explode: false })}`));
  }

/** List protocol profiles */
  async protocolProfilesList(): Promise<AiotProtocolProfileListResponse> {
    return this.client.get<AiotProtocolProfileListResponse>(backendApiPath(`/iot/protocol_profiles`));
  }

/** Create protocol profile */
  async protocolProfilesCreate(body: AiotProtocolProfileCreateRequest): Promise<AiotProtocolProfileResponse> {
    return this.client.post<AiotProtocolProfileResponse>(backendApiPath(`/iot/protocol_profiles`), body, undefined, undefined, 'application/json');
  }

/** Retrieve protocol profile */
  async protocolProfilesRetrieve(protocolProfileId: string): Promise<AiotProtocolProfileResponse> {
    return this.client.get<AiotProtocolProfileResponse>(backendApiPath(`/iot/protocol_profiles/${serializePathParameter(protocolProfileId, { name: 'protocolProfileId', style: 'simple', explode: false })}`));
  }

/** Update protocol profile */
  async protocolProfilesUpdate(protocolProfileId: string, body?: AiotProtocolProfileUpdateRequest): Promise<AiotProtocolProfileResponse> {
    return this.client.put<AiotProtocolProfileResponse>(backendApiPath(`/iot/protocol_profiles/${serializePathParameter(protocolProfileId, { name: 'protocolProfileId', style: 'simple', explode: false })}`), body, undefined, undefined, 'application/json');
  }

/** Delete protocol profile */
  async protocolProfilesDelete(protocolProfileId: string): Promise<void> {
    return this.client.delete<void>(backendApiPath(`/iot/protocol_profiles/${serializePathParameter(protocolProfileId, { name: 'protocolProfileId', style: 'simple', explode: false })}`));
  }

/** List capability models */
  async capabilityModelsList(): Promise<AiotCapabilityModelListResponse> {
    return this.client.get<AiotCapabilityModelListResponse>(backendApiPath(`/iot/capability_models`));
  }

/** Create capability model */
  async capabilityModelsCreate(body: AiotCapabilityModelCreateRequest): Promise<AiotCapabilityModelResponse> {
    return this.client.post<AiotCapabilityModelResponse>(backendApiPath(`/iot/capability_models`), body, undefined, undefined, 'application/json');
  }

/** Retrieve capability model */
  async capabilityModelsRetrieve(capabilityModelId: string): Promise<AiotCapabilityModelResponse> {
    return this.client.get<AiotCapabilityModelResponse>(backendApiPath(`/iot/capability_models/${serializePathParameter(capabilityModelId, { name: 'capabilityModelId', style: 'simple', explode: false })}`));
  }

/** Update capability model */
  async capabilityModelsUpdate(capabilityModelId: string, body?: AiotCapabilityModelUpdateRequest): Promise<AiotCapabilityModelResponse> {
    return this.client.put<AiotCapabilityModelResponse>(backendApiPath(`/iot/capability_models/${serializePathParameter(capabilityModelId, { name: 'capabilityModelId', style: 'simple', explode: false })}`), body, undefined, undefined, 'application/json');
  }

/** Delete capability model */
  async capabilityModelsDelete(capabilityModelId: string): Promise<void> {
    return this.client.delete<void>(backendApiPath(`/iot/capability_models/${serializePathParameter(capabilityModelId, { name: 'capabilityModelId', style: 'simple', explode: false })}`));
  }

/** List AIoT devices */
  async devicesList(): Promise<AiotDeviceListResponse> {
    return this.client.get<AiotDeviceListResponse>(backendApiPath(`/iot/devices`));
  }

/** Create AIoT device */
  async devicesCreate(body: AiotDeviceCreateRequest, idempotencyKey?: string): Promise<StandardResourceResponse> {
    const requestHeaders = buildRequestHeaders(
      {
        'Idempotency-Key': { value: idempotencyKey, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.post<StandardResourceResponse>(backendApiPath(`/iot/devices`), body, undefined, requestHeaders, 'application/json');
  }

/** Retrieve AIoT device */
  async devicesRetrieve(deviceId: string): Promise<StandardResourceResponse> {
    return this.client.get<StandardResourceResponse>(backendApiPath(`/iot/devices/${serializePathParameter(deviceId, { name: 'deviceId', style: 'simple', explode: false })}`));
  }

/** Update AIoT device */
  async devicesUpdate(deviceId: string, body: AiotDeviceUpdateRequest, idempotencyKey?: string): Promise<StandardResourceResponse> {
    const requestHeaders = buildRequestHeaders(
      {
        'Idempotency-Key': { value: idempotencyKey, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.put<StandardResourceResponse>(backendApiPath(`/iot/devices/${serializePathParameter(deviceId, { name: 'deviceId', style: 'simple', explode: false })}`), body, undefined, requestHeaders, 'application/json');
  }

/** Delete AIoT device */
  async devicesDelete(deviceId: string): Promise<void> {
    return this.client.delete<void>(backendApiPath(`/iot/devices/${serializePathParameter(deviceId, { name: 'deviceId', style: 'simple', explode: false })}`));
  }

/** List device credentials */
  async devicesCredentialsList(deviceId: string): Promise<StandardCollectionResponse> {
    return this.client.get<StandardCollectionResponse>(backendApiPath(`/iot/devices/${serializePathParameter(deviceId, { name: 'deviceId', style: 'simple', explode: false })}/credentials`));
  }

/** Create device credential */
  async devicesCredentialsCreate(deviceId: string, body: AiotCredentialCreateRequest, idempotencyKey?: string): Promise<AiotCredentialResponse> {
    const requestHeaders = buildRequestHeaders(
      {
        'Idempotency-Key': { value: idempotencyKey, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.post<AiotCredentialResponse>(backendApiPath(`/iot/devices/${serializePathParameter(deviceId, { name: 'deviceId', style: 'simple', explode: false })}/credentials`), body, undefined, requestHeaders, 'application/json');
  }

/** Retrieve device credential */
  async devicesCredentialsRetrieve(deviceId: string, credentialId: string): Promise<StandardResourceResponse> {
    return this.client.get<StandardResourceResponse>(backendApiPath(`/iot/devices/${serializePathParameter(deviceId, { name: 'deviceId', style: 'simple', explode: false })}/credentials/${serializePathParameter(credentialId, { name: 'credentialId', style: 'simple', explode: false })}`));
  }

/** Revoke device credential */
  async devicesCredentialsDelete(deviceId: string, credentialId: string): Promise<void> {
    return this.client.delete<void>(backendApiPath(`/iot/devices/${serializePathParameter(deviceId, { name: 'deviceId', style: 'simple', explode: false })}/credentials/${serializePathParameter(credentialId, { name: 'credentialId', style: 'simple', explode: false })}`));
  }

/** List device sessions */
  async devicesSessionsList(deviceId: string): Promise<StandardCollectionResponse> {
    return this.client.get<StandardCollectionResponse>(backendApiPath(`/iot/devices/${serializePathParameter(deviceId, { name: 'deviceId', style: 'simple', explode: false })}/sessions`));
  }

/** Disconnect device session */
  async devicesSessionsDisconnect(deviceId: string, sessionId: string): Promise<void> {
    return this.client.delete<void>(backendApiPath(`/iot/devices/${serializePathParameter(deviceId, { name: 'deviceId', style: 'simple', explode: false })}/sessions/${serializePathParameter(sessionId, { name: 'sessionId', style: 'simple', explode: false })}`));
  }

/** List device capabilities */
  async devicesCapabilitiesList(deviceId: string): Promise<StandardCollectionResponse> {
    return this.client.get<StandardCollectionResponse>(backendApiPath(`/iot/devices/${serializePathParameter(deviceId, { name: 'deviceId', style: 'simple', explode: false })}/capabilities`));
  }

/** List device commands */
  async devicesCommandsList(deviceId: string): Promise<AiotCommandListResponse> {
    return this.client.get<AiotCommandListResponse>(backendApiPath(`/iot/devices/${serializePathParameter(deviceId, { name: 'deviceId', style: 'simple', explode: false })}/commands`));
  }

/** Cancel device command */
  async devicesCommandsCancel(deviceId: string, commandId: string): Promise<StandardResourceResponse> {
    return this.client.post<StandardResourceResponse>(backendApiPath(`/iot/devices/${serializePathParameter(deviceId, { name: 'deviceId', style: 'simple', explode: false })}/commands/${serializePathParameter(commandId, { name: 'commandId', style: 'simple', explode: false })}/cancel`));
  }

/** Update backend device twin */
  async devicesTwinUpdate(deviceId: string, body: AiotTwinUpdateRequest): Promise<StandardResourceResponse> {
    return this.client.patch<StandardResourceResponse>(backendApiPath(`/iot/devices/${serializePathParameter(deviceId, { name: 'deviceId', style: 'simple', explode: false })}/twin`), body, undefined, undefined, 'application/json');
  }

/** Retrieve backend device twin */
  async devicesTwinRetrieve(deviceId: string): Promise<StandardResourceResponse> {
    return this.client.get<StandardResourceResponse>(backendApiPath(`/iot/devices/${serializePathParameter(deviceId, { name: 'deviceId', style: 'simple', explode: false })}/twin`));
  }

/** List firmware artifacts */
  async firmwareArtifactsList(): Promise<StandardCollectionResponse> {
    return this.client.get<StandardCollectionResponse>(backendApiPath(`/iot/firmware_artifacts`));
  }

/** Create firmware artifact metadata */
  async firmwareArtifactsCreate(body: AiotFirmwareArtifactCreateRequest, idempotencyKey?: string): Promise<AiotFirmwareArtifactResponse> {
    const requestHeaders = buildRequestHeaders(
      {
        'Idempotency-Key': { value: idempotencyKey, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.post<AiotFirmwareArtifactResponse>(backendApiPath(`/iot/firmware_artifacts`), body, undefined, requestHeaders, 'application/json');
  }

/** Retrieve firmware artifact */
  async firmwareArtifactsRetrieve(artifactId: string): Promise<AiotFirmwareArtifactResponse> {
    return this.client.get<AiotFirmwareArtifactResponse>(backendApiPath(`/iot/firmware_artifacts/${serializePathParameter(artifactId, { name: 'artifactId', style: 'simple', explode: false })}`));
  }

/** Update firmware artifact metadata */
  async firmwareArtifactsUpdate(artifactId: string, body?: AiotFirmwareArtifactUpdateRequest): Promise<AiotFirmwareArtifactResponse> {
    return this.client.put<AiotFirmwareArtifactResponse>(backendApiPath(`/iot/firmware_artifacts/${serializePathParameter(artifactId, { name: 'artifactId', style: 'simple', explode: false })}`), body, undefined, undefined, 'application/json');
  }

/** Delete firmware artifact metadata */
  async firmwareArtifactsDelete(artifactId: string): Promise<void> {
    return this.client.delete<void>(backendApiPath(`/iot/firmware_artifacts/${serializePathParameter(artifactId, { name: 'artifactId', style: 'simple', explode: false })}`));
  }

/** List firmware rollouts */
  async firmwareRolloutsList(): Promise<StandardCollectionResponse> {
    return this.client.get<StandardCollectionResponse>(backendApiPath(`/iot/firmware_rollouts`));
  }

/** Create firmware rollout */
  async firmwareRolloutsCreate(body: AiotFirmwareRolloutCreateRequest, idempotencyKey?: string): Promise<AiotFirmwareRolloutResponse> {
    const requestHeaders = buildRequestHeaders(
      {
        'Idempotency-Key': { value: idempotencyKey, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.post<AiotFirmwareRolloutResponse>(backendApiPath(`/iot/firmware_rollouts`), body, undefined, requestHeaders, 'application/json');
  }

/** Retrieve firmware rollout */
  async firmwareRolloutsRetrieve(rolloutId: string): Promise<AiotFirmwareRolloutResponse> {
    return this.client.get<AiotFirmwareRolloutResponse>(backendApiPath(`/iot/firmware_rollouts/${serializePathParameter(rolloutId, { name: 'rolloutId', style: 'simple', explode: false })}`));
  }

/** Update firmware rollout */
  async firmwareRolloutsUpdate(rolloutId: string, body?: AiotFirmwareRolloutUpdateRequest): Promise<AiotFirmwareRolloutResponse> {
    return this.client.put<AiotFirmwareRolloutResponse>(backendApiPath(`/iot/firmware_rollouts/${serializePathParameter(rolloutId, { name: 'rolloutId', style: 'simple', explode: false })}`), body, undefined, undefined, 'application/json');
  }

/** Delete firmware rollout */
  async firmwareRolloutsDelete(rolloutId: string): Promise<void> {
    return this.client.delete<void>(backendApiPath(`/iot/firmware_rollouts/${serializePathParameter(rolloutId, { name: 'rolloutId', style: 'simple', explode: false })}`));
  }

/** List platform IoT events */
  async eventsList(): Promise<AiotEventListResponse> {
    return this.client.get<AiotEventListResponse>(backendApiPath(`/iot/events`));
  }

/** List protocol adapters */
  async protocolAdaptersList(): Promise<AiotProtocolAdapterListResponse> {
    return this.client.get<AiotProtocolAdapterListResponse>(backendApiPath(`/iot/protocol_adapters`));
  }

/** Retrieve AIoT runtime capacity and backpressure policy */
  async runtimeCapacityRetrieve(): Promise<AiotRuntimeCapacityPolicyResponse> {
    return this.client.get<AiotRuntimeCapacityPolicyResponse>(backendApiPath(`/iot/runtime/capacity`));
  }
}

export function createIotApi(client: HttpClient): IotApi {
  return new IotApi(client);
}

function appendQueryString(path: string, rawQueryString: string): string {
  const query = rawQueryString.replace(/^\?+/, '');
  if (!query) {
    return path;
  }
  return path.includes('?') ? `${path}&${query}` : `${path}?${query}`;
}

interface PathParameterSpec {
  name: string;
  style: string;
  explode: boolean;
}

function serializePathParameter(value: unknown, spec: PathParameterSpec): string {
  if (value === undefined || value === null) {
    return '';
  }

  const style = spec.style || 'simple';
  if (Array.isArray(value)) {
    return serializePathArray(spec.name, value, style, spec.explode);
  }
  if (typeof value === 'object') {
    return serializePathObject(spec.name, value as Record<string, unknown>, style, spec.explode);
  }
  return pathPrefix(spec.name, style, false) + encodePathValue(serializePathPrimitive(value));
}

function serializePathArray(name: string, values: unknown[], style: string, explode: boolean): string {
  const serialized = values
    .filter((item) => item !== undefined && item !== null)
    .map((item) => encodePathValue(serializePathPrimitive(item)));
  if (serialized.length === 0) {
    return pathPrefix(name, style, false);
  }
  if (style === 'matrix') {
    return explode
      ? serialized.map((item) => `;${name}=${item}`).join('')
      : `;${name}=${serialized.join(',')}`;
  }
  return pathPrefix(name, style, false) + serialized.join(explode ? '.' : ',');
}

function serializePathObject(name: string, value: Record<string, unknown>, style: string, explode: boolean): string {
  const entries = Object.entries(value).filter(([, entryValue]) => entryValue !== undefined && entryValue !== null);
  if (entries.length === 0) {
    return pathPrefix(name, style, true);
  }
  if (style === 'matrix') {
    return explode
      ? entries.map(([key, entryValue]) => `;${encodePathValue(key)}=${encodePathValue(serializePathPrimitive(entryValue))}`).join('')
      : `;${name}=${entries.flatMap(([key, entryValue]) => [encodePathValue(key), encodePathValue(serializePathPrimitive(entryValue))]).join(',')}`;
  }
  const serialized = explode
    ? entries.map(([key, entryValue]) => `${encodePathValue(key)}=${encodePathValue(serializePathPrimitive(entryValue))}`).join(style === 'label' ? '.' : ',')
    : entries.flatMap(([key, entryValue]) => [encodePathValue(key), encodePathValue(serializePathPrimitive(entryValue))]).join(',');
  return pathPrefix(name, style, true) + serialized;
}

function pathPrefix(name: string, style: string, _objectValue: boolean): string {
  if (style === 'label') return '.';
  if (style === 'matrix') return `;${name}`;
  return '';
}

function encodePathValue(value: string): string {
  return encodeURIComponent(value);
}

function serializePathPrimitive(value: unknown): string {
  if (value instanceof Date) {
    return value.toISOString();
  }
  if (typeof value === 'object') {
    return JSON.stringify(value);
  }
  return String(value);
}

function buildRequestHeaders(
  headers: Record<string, HeaderParameterSpec | undefined>,
  cookies: Record<string, HeaderParameterSpec | undefined> = {},
): Record<string, string> | undefined {
  const requestHeaders: Record<string, string> = {};

  for (const [name, parameter] of Object.entries(headers)) {
    const serialized = serializeParameterValue(parameter);
    if (serialized !== undefined) {
      requestHeaders[name] = serialized;
    }
  }

  const cookieHeader = buildCookieHeader(cookies);
  if (cookieHeader) {
    requestHeaders.Cookie = requestHeaders.Cookie
      ? `${requestHeaders.Cookie}; ${cookieHeader}`
      : cookieHeader;
  }

  return Object.keys(requestHeaders).length > 0 ? requestHeaders : undefined;
}

interface HeaderParameterSpec {
  value: unknown;
  style: string;
  explode: boolean;
  contentType?: string;
}

function buildCookieHeader(cookies: Record<string, HeaderParameterSpec | undefined>): string | undefined {
  const pairs: string[] = [];
  for (const [name, parameter] of Object.entries(cookies)) {
    const serialized = serializeParameterValue(parameter);
    if (serialized !== undefined) {
      pairs.push(`${encodeURIComponent(name)}=${encodeURIComponent(serialized)}`);
    }
  }
  return pairs.length > 0 ? pairs.join('; ') : undefined;
}

function serializeParameterValue(parameter: HeaderParameterSpec | undefined): string | undefined {
  const value = parameter?.value;
  if (value === undefined || value === null) {
    return undefined;
  }
  if (parameter?.contentType) {
    return JSON.stringify(value);
  }
  if (value instanceof Date) {
    return value.toISOString();
  }
  if (Array.isArray(value)) {
    return value.map((item) => serializeHeaderPrimitive(item)).join(',');
  }
  if (typeof value === 'object' && value !== null) {
    return serializeHeaderObject(value as Record<string, unknown>, parameter?.explode === true);
  }
  return serializeHeaderPrimitive(value);
}

function serializeHeaderObject(value: Record<string, unknown>, explode: boolean): string {
  const entries = Object.entries(value).filter(([, entryValue]) => entryValue !== undefined && entryValue !== null);
  if (explode) {
    return entries.map(([key, entryValue]) => `${key}=${serializeHeaderPrimitive(entryValue)}`).join(',');
  }
  return entries.flatMap(([key, entryValue]) => [key, serializeHeaderPrimitive(entryValue)]).join(',');
}

function serializeHeaderPrimitive(value: unknown): string {
  if (value instanceof Date) {
    return value.toISOString();
  }
  return String(value);
}
