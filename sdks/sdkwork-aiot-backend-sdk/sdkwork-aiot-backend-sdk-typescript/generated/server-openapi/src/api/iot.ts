import { backendApiPath } from './paths';
import type { HttpClient } from '../http/client';

import type { AiotCapabilityModelCreateRequest, AiotCapabilityModelListResponse, AiotCapabilityModelResponse, AiotCapabilityModelUpdateRequest, AiotCommandListResponse, AiotCredentialCreateRequest, AiotCredentialResponse, AiotDeviceCreateRequest, AiotDeviceListResponse, AiotDeviceUpdateRequest, AiotEventListResponse, AiotFirmwareArtifactCreateRequest, AiotFirmwareArtifactResponse, AiotFirmwareArtifactUpdateRequest, AiotFirmwareRolloutCreateRequest, AiotFirmwareRolloutResponse, AiotFirmwareRolloutUpdateRequest, AiotHardwareProfileCreateRequest, AiotHardwareProfileListResponse, AiotHardwareProfileResponse, AiotHardwareProfileUpdateRequest, AiotProductCreateRequest, AiotProductListResponse, AiotProductResponse, AiotProductUpdateRequest, AiotProtocolAdapterListResponse, AiotProtocolProfileCreateRequest, AiotProtocolProfileListResponse, AiotProtocolProfileResponse, AiotProtocolProfileUpdateRequest, AiotRuntimeCapacityPolicyResponse, AiotTwinUpdateRequest, StandardCollectionResponse, StandardResourceResponse } from '../types';


export class IotApi {
  private client: HttpClient;
  
  constructor(client: HttpClient) { 
    this.client = client; 
  }

/** List AIoT products */
  async productsList(xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, xSdkworkUserId?: string, xSdkworkDataScope?: string): Promise<AiotProductListResponse> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.get<AiotProductListResponse>(backendApiPath(`/iot/products`), undefined, requestHeaders);
  }

/** Create AIoT product */
  async productsCreate(body: AiotProductCreateRequest, xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, xSdkworkUserId?: string, xSdkworkDataScope?: string): Promise<AiotProductResponse> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.post<AiotProductResponse>(backendApiPath(`/iot/products`), body, undefined, requestHeaders, 'application/json');
  }

/** Retrieve AIoT product */
  async productsRetrieve(productId: string, xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, xSdkworkUserId?: string, xSdkworkDataScope?: string): Promise<AiotProductResponse> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.get<AiotProductResponse>(backendApiPath(`/iot/products/${serializePathParameter(productId, { name: 'productId', style: 'simple', explode: false })}`), undefined, requestHeaders);
  }

/** Update AIoT product */
  async productsUpdate(productId: string, xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, body?: AiotProductUpdateRequest, xSdkworkUserId?: string, xSdkworkDataScope?: string): Promise<AiotProductResponse> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.put<AiotProductResponse>(backendApiPath(`/iot/products/${serializePathParameter(productId, { name: 'productId', style: 'simple', explode: false })}`), body, undefined, requestHeaders, 'application/json');
  }

/** Delete AIoT product */
  async productsDelete(productId: string, xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, xSdkworkUserId?: string, xSdkworkDataScope?: string): Promise<void> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.delete<void>(backendApiPath(`/iot/products/${serializePathParameter(productId, { name: 'productId', style: 'simple', explode: false })}`), undefined, requestHeaders);
  }

/** List hardware profiles */
  async hardwareProfilesList(xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, xSdkworkUserId?: string, xSdkworkDataScope?: string): Promise<AiotHardwareProfileListResponse> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.get<AiotHardwareProfileListResponse>(backendApiPath(`/iot/hardware_profiles`), undefined, requestHeaders);
  }

/** Create hardware profile */
  async hardwareProfilesCreate(body: AiotHardwareProfileCreateRequest, xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, xSdkworkUserId?: string, xSdkworkDataScope?: string): Promise<AiotHardwareProfileResponse> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.post<AiotHardwareProfileResponse>(backendApiPath(`/iot/hardware_profiles`), body, undefined, requestHeaders, 'application/json');
  }

/** Retrieve hardware profile */
  async hardwareProfilesRetrieve(hardwareProfileId: string, xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, xSdkworkUserId?: string, xSdkworkDataScope?: string): Promise<AiotHardwareProfileResponse> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.get<AiotHardwareProfileResponse>(backendApiPath(`/iot/hardware_profiles/${serializePathParameter(hardwareProfileId, { name: 'hardwareProfileId', style: 'simple', explode: false })}`), undefined, requestHeaders);
  }

/** Update hardware profile */
  async hardwareProfilesUpdate(hardwareProfileId: string, xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, body?: AiotHardwareProfileUpdateRequest, xSdkworkUserId?: string, xSdkworkDataScope?: string): Promise<AiotHardwareProfileResponse> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.put<AiotHardwareProfileResponse>(backendApiPath(`/iot/hardware_profiles/${serializePathParameter(hardwareProfileId, { name: 'hardwareProfileId', style: 'simple', explode: false })}`), body, undefined, requestHeaders, 'application/json');
  }

/** Delete hardware profile */
  async hardwareProfilesDelete(hardwareProfileId: string, xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, xSdkworkUserId?: string, xSdkworkDataScope?: string): Promise<void> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.delete<void>(backendApiPath(`/iot/hardware_profiles/${serializePathParameter(hardwareProfileId, { name: 'hardwareProfileId', style: 'simple', explode: false })}`), undefined, requestHeaders);
  }

/** List protocol profiles */
  async protocolProfilesList(xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, xSdkworkUserId?: string, xSdkworkDataScope?: string): Promise<AiotProtocolProfileListResponse> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.get<AiotProtocolProfileListResponse>(backendApiPath(`/iot/protocol_profiles`), undefined, requestHeaders);
  }

/** Create protocol profile */
  async protocolProfilesCreate(body: AiotProtocolProfileCreateRequest, xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, xSdkworkUserId?: string, xSdkworkDataScope?: string): Promise<AiotProtocolProfileResponse> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.post<AiotProtocolProfileResponse>(backendApiPath(`/iot/protocol_profiles`), body, undefined, requestHeaders, 'application/json');
  }

/** Retrieve protocol profile */
  async protocolProfilesRetrieve(protocolProfileId: string, xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, xSdkworkUserId?: string, xSdkworkDataScope?: string): Promise<AiotProtocolProfileResponse> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.get<AiotProtocolProfileResponse>(backendApiPath(`/iot/protocol_profiles/${serializePathParameter(protocolProfileId, { name: 'protocolProfileId', style: 'simple', explode: false })}`), undefined, requestHeaders);
  }

/** Update protocol profile */
  async protocolProfilesUpdate(protocolProfileId: string, xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, body?: AiotProtocolProfileUpdateRequest, xSdkworkUserId?: string, xSdkworkDataScope?: string): Promise<AiotProtocolProfileResponse> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.put<AiotProtocolProfileResponse>(backendApiPath(`/iot/protocol_profiles/${serializePathParameter(protocolProfileId, { name: 'protocolProfileId', style: 'simple', explode: false })}`), body, undefined, requestHeaders, 'application/json');
  }

/** Delete protocol profile */
  async protocolProfilesDelete(protocolProfileId: string, xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, xSdkworkUserId?: string, xSdkworkDataScope?: string): Promise<void> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.delete<void>(backendApiPath(`/iot/protocol_profiles/${serializePathParameter(protocolProfileId, { name: 'protocolProfileId', style: 'simple', explode: false })}`), undefined, requestHeaders);
  }

/** List capability models */
  async capabilityModelsList(xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, xSdkworkUserId?: string, xSdkworkDataScope?: string): Promise<AiotCapabilityModelListResponse> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.get<AiotCapabilityModelListResponse>(backendApiPath(`/iot/capability_models`), undefined, requestHeaders);
  }

/** Create capability model */
  async capabilityModelsCreate(body: AiotCapabilityModelCreateRequest, xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, xSdkworkUserId?: string, xSdkworkDataScope?: string): Promise<AiotCapabilityModelResponse> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.post<AiotCapabilityModelResponse>(backendApiPath(`/iot/capability_models`), body, undefined, requestHeaders, 'application/json');
  }

/** Retrieve capability model */
  async capabilityModelsRetrieve(capabilityModelId: string, xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, xSdkworkUserId?: string, xSdkworkDataScope?: string): Promise<AiotCapabilityModelResponse> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.get<AiotCapabilityModelResponse>(backendApiPath(`/iot/capability_models/${serializePathParameter(capabilityModelId, { name: 'capabilityModelId', style: 'simple', explode: false })}`), undefined, requestHeaders);
  }

/** Update capability model */
  async capabilityModelsUpdate(capabilityModelId: string, xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, body?: AiotCapabilityModelUpdateRequest, xSdkworkUserId?: string, xSdkworkDataScope?: string): Promise<AiotCapabilityModelResponse> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.put<AiotCapabilityModelResponse>(backendApiPath(`/iot/capability_models/${serializePathParameter(capabilityModelId, { name: 'capabilityModelId', style: 'simple', explode: false })}`), body, undefined, requestHeaders, 'application/json');
  }

/** Delete capability model */
  async capabilityModelsDelete(capabilityModelId: string, xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, xSdkworkUserId?: string, xSdkworkDataScope?: string): Promise<void> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.delete<void>(backendApiPath(`/iot/capability_models/${serializePathParameter(capabilityModelId, { name: 'capabilityModelId', style: 'simple', explode: false })}`), undefined, requestHeaders);
  }

/** List AIoT devices */
  async devicesList(xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, xSdkworkUserId?: string, xSdkworkDataScope?: string): Promise<AiotDeviceListResponse> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.get<AiotDeviceListResponse>(backendApiPath(`/iot/devices`), undefined, requestHeaders);
  }

/** Create AIoT device */
  async devicesCreate(body: AiotDeviceCreateRequest, xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, xSdkworkUserId?: string, xSdkworkDataScope?: string, idempotencyKey?: string): Promise<StandardResourceResponse> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
        'Idempotency-Key': { value: idempotencyKey, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.post<StandardResourceResponse>(backendApiPath(`/iot/devices`), body, undefined, requestHeaders, 'application/json');
  }

/** Retrieve AIoT device */
  async devicesRetrieve(deviceId: string, xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, xSdkworkUserId?: string, xSdkworkDataScope?: string): Promise<StandardResourceResponse> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.get<StandardResourceResponse>(backendApiPath(`/iot/devices/${serializePathParameter(deviceId, { name: 'deviceId', style: 'simple', explode: false })}`), undefined, requestHeaders);
  }

/** Update AIoT device */
  async devicesUpdate(deviceId: string, body: AiotDeviceUpdateRequest, xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, xSdkworkUserId?: string, xSdkworkDataScope?: string, idempotencyKey?: string): Promise<StandardResourceResponse> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
        'Idempotency-Key': { value: idempotencyKey, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.put<StandardResourceResponse>(backendApiPath(`/iot/devices/${serializePathParameter(deviceId, { name: 'deviceId', style: 'simple', explode: false })}`), body, undefined, requestHeaders, 'application/json');
  }

/** Delete AIoT device */
  async devicesDelete(deviceId: string, xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, xSdkworkUserId?: string, xSdkworkDataScope?: string): Promise<void> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.delete<void>(backendApiPath(`/iot/devices/${serializePathParameter(deviceId, { name: 'deviceId', style: 'simple', explode: false })}`), undefined, requestHeaders);
  }

/** List device credentials */
  async devicesCredentialsList(deviceId: string, xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, xSdkworkUserId?: string, xSdkworkDataScope?: string): Promise<StandardCollectionResponse> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.get<StandardCollectionResponse>(backendApiPath(`/iot/devices/${serializePathParameter(deviceId, { name: 'deviceId', style: 'simple', explode: false })}/credentials`), undefined, requestHeaders);
  }

/** Create device credential */
  async devicesCredentialsCreate(deviceId: string, body: AiotCredentialCreateRequest, xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, xSdkworkUserId?: string, xSdkworkDataScope?: string, idempotencyKey?: string): Promise<AiotCredentialResponse> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
        'Idempotency-Key': { value: idempotencyKey, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.post<AiotCredentialResponse>(backendApiPath(`/iot/devices/${serializePathParameter(deviceId, { name: 'deviceId', style: 'simple', explode: false })}/credentials`), body, undefined, requestHeaders, 'application/json');
  }

/** Retrieve device credential */
  async devicesCredentialsRetrieve(deviceId: string, credentialId: string, xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, xSdkworkUserId?: string, xSdkworkDataScope?: string): Promise<StandardResourceResponse> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.get<StandardResourceResponse>(backendApiPath(`/iot/devices/${serializePathParameter(deviceId, { name: 'deviceId', style: 'simple', explode: false })}/credentials/${serializePathParameter(credentialId, { name: 'credentialId', style: 'simple', explode: false })}`), undefined, requestHeaders);
  }

/** Revoke device credential */
  async devicesCredentialsDelete(deviceId: string, credentialId: string, xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, xSdkworkUserId?: string, xSdkworkDataScope?: string): Promise<void> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.delete<void>(backendApiPath(`/iot/devices/${serializePathParameter(deviceId, { name: 'deviceId', style: 'simple', explode: false })}/credentials/${serializePathParameter(credentialId, { name: 'credentialId', style: 'simple', explode: false })}`), undefined, requestHeaders);
  }

/** List device sessions */
  async devicesSessionsList(deviceId: string, xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, xSdkworkUserId?: string, xSdkworkDataScope?: string): Promise<StandardCollectionResponse> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.get<StandardCollectionResponse>(backendApiPath(`/iot/devices/${serializePathParameter(deviceId, { name: 'deviceId', style: 'simple', explode: false })}/sessions`), undefined, requestHeaders);
  }

/** Disconnect device session */
  async devicesSessionsDisconnect(deviceId: string, sessionId: string, xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, xSdkworkUserId?: string, xSdkworkDataScope?: string): Promise<void> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.delete<void>(backendApiPath(`/iot/devices/${serializePathParameter(deviceId, { name: 'deviceId', style: 'simple', explode: false })}/sessions/${serializePathParameter(sessionId, { name: 'sessionId', style: 'simple', explode: false })}`), undefined, requestHeaders);
  }

/** List device capabilities */
  async devicesCapabilitiesList(deviceId: string, xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, xSdkworkUserId?: string, xSdkworkDataScope?: string): Promise<StandardCollectionResponse> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.get<StandardCollectionResponse>(backendApiPath(`/iot/devices/${serializePathParameter(deviceId, { name: 'deviceId', style: 'simple', explode: false })}/capabilities`), undefined, requestHeaders);
  }

/** List device commands */
  async devicesCommandsList(deviceId: string, xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, xSdkworkUserId?: string, xSdkworkDataScope?: string): Promise<AiotCommandListResponse> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.get<AiotCommandListResponse>(backendApiPath(`/iot/devices/${serializePathParameter(deviceId, { name: 'deviceId', style: 'simple', explode: false })}/commands`), undefined, requestHeaders);
  }

/** Cancel device command */
  async devicesCommandsCancel(deviceId: string, commandId: string, xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, xSdkworkUserId?: string, xSdkworkDataScope?: string): Promise<StandardResourceResponse> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.post<StandardResourceResponse>(backendApiPath(`/iot/devices/${serializePathParameter(deviceId, { name: 'deviceId', style: 'simple', explode: false })}/commands/${serializePathParameter(commandId, { name: 'commandId', style: 'simple', explode: false })}/cancel`), undefined, undefined, requestHeaders);
  }

/** Update backend device twin */
  async devicesTwinUpdate(deviceId: string, body: AiotTwinUpdateRequest, xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, xSdkworkUserId?: string, xSdkworkDataScope?: string): Promise<StandardResourceResponse> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.patch<StandardResourceResponse>(backendApiPath(`/iot/devices/${serializePathParameter(deviceId, { name: 'deviceId', style: 'simple', explode: false })}/twin`), body, undefined, requestHeaders, 'application/json');
  }

/** Retrieve backend device twin */
  async devicesTwinRetrieve(deviceId: string, xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, xSdkworkUserId?: string, xSdkworkDataScope?: string): Promise<StandardResourceResponse> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.get<StandardResourceResponse>(backendApiPath(`/iot/devices/${serializePathParameter(deviceId, { name: 'deviceId', style: 'simple', explode: false })}/twin`), undefined, requestHeaders);
  }

/** List firmware artifacts */
  async firmwareArtifactsList(xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, xSdkworkUserId?: string, xSdkworkDataScope?: string): Promise<StandardCollectionResponse> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.get<StandardCollectionResponse>(backendApiPath(`/iot/firmware_artifacts`), undefined, requestHeaders);
  }

/** Create firmware artifact metadata */
  async firmwareArtifactsCreate(body: AiotFirmwareArtifactCreateRequest, xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, xSdkworkUserId?: string, xSdkworkDataScope?: string, idempotencyKey?: string): Promise<AiotFirmwareArtifactResponse> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
        'Idempotency-Key': { value: idempotencyKey, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.post<AiotFirmwareArtifactResponse>(backendApiPath(`/iot/firmware_artifacts`), body, undefined, requestHeaders, 'application/json');
  }

/** Retrieve firmware artifact */
  async firmwareArtifactsRetrieve(artifactId: string, xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, xSdkworkUserId?: string, xSdkworkDataScope?: string): Promise<AiotFirmwareArtifactResponse> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.get<AiotFirmwareArtifactResponse>(backendApiPath(`/iot/firmware_artifacts/${serializePathParameter(artifactId, { name: 'artifactId', style: 'simple', explode: false })}`), undefined, requestHeaders);
  }

/** Update firmware artifact metadata */
  async firmwareArtifactsUpdate(artifactId: string, xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, body?: AiotFirmwareArtifactUpdateRequest, xSdkworkUserId?: string, xSdkworkDataScope?: string): Promise<AiotFirmwareArtifactResponse> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.put<AiotFirmwareArtifactResponse>(backendApiPath(`/iot/firmware_artifacts/${serializePathParameter(artifactId, { name: 'artifactId', style: 'simple', explode: false })}`), body, undefined, requestHeaders, 'application/json');
  }

/** Delete firmware artifact metadata */
  async firmwareArtifactsDelete(artifactId: string, xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, xSdkworkUserId?: string, xSdkworkDataScope?: string): Promise<void> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.delete<void>(backendApiPath(`/iot/firmware_artifacts/${serializePathParameter(artifactId, { name: 'artifactId', style: 'simple', explode: false })}`), undefined, requestHeaders);
  }

/** List firmware rollouts */
  async firmwareRolloutsList(xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, xSdkworkUserId?: string, xSdkworkDataScope?: string): Promise<StandardCollectionResponse> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.get<StandardCollectionResponse>(backendApiPath(`/iot/firmware_rollouts`), undefined, requestHeaders);
  }

/** Create firmware rollout */
  async firmwareRolloutsCreate(body: AiotFirmwareRolloutCreateRequest, xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, xSdkworkUserId?: string, xSdkworkDataScope?: string, idempotencyKey?: string): Promise<AiotFirmwareRolloutResponse> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
        'Idempotency-Key': { value: idempotencyKey, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.post<AiotFirmwareRolloutResponse>(backendApiPath(`/iot/firmware_rollouts`), body, undefined, requestHeaders, 'application/json');
  }

/** Retrieve firmware rollout */
  async firmwareRolloutsRetrieve(rolloutId: string, xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, xSdkworkUserId?: string, xSdkworkDataScope?: string): Promise<AiotFirmwareRolloutResponse> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.get<AiotFirmwareRolloutResponse>(backendApiPath(`/iot/firmware_rollouts/${serializePathParameter(rolloutId, { name: 'rolloutId', style: 'simple', explode: false })}`), undefined, requestHeaders);
  }

/** Update firmware rollout */
  async firmwareRolloutsUpdate(rolloutId: string, xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, body?: AiotFirmwareRolloutUpdateRequest, xSdkworkUserId?: string, xSdkworkDataScope?: string): Promise<AiotFirmwareRolloutResponse> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.put<AiotFirmwareRolloutResponse>(backendApiPath(`/iot/firmware_rollouts/${serializePathParameter(rolloutId, { name: 'rolloutId', style: 'simple', explode: false })}`), body, undefined, requestHeaders, 'application/json');
  }

/** Delete firmware rollout */
  async firmwareRolloutsDelete(rolloutId: string, xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, xSdkworkUserId?: string, xSdkworkDataScope?: string): Promise<void> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.delete<void>(backendApiPath(`/iot/firmware_rollouts/${serializePathParameter(rolloutId, { name: 'rolloutId', style: 'simple', explode: false })}`), undefined, requestHeaders);
  }

/** List platform IoT events */
  async eventsList(xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, xSdkworkUserId?: string, xSdkworkDataScope?: string): Promise<AiotEventListResponse> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.get<AiotEventListResponse>(backendApiPath(`/iot/events`), undefined, requestHeaders);
  }

/** List protocol adapters */
  async protocolAdaptersList(xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, xSdkworkUserId?: string, xSdkworkDataScope?: string): Promise<AiotProtocolAdapterListResponse> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.get<AiotProtocolAdapterListResponse>(backendApiPath(`/iot/protocol_adapters`), undefined, requestHeaders);
  }

/** Retrieve AIoT runtime capacity and backpressure policy */
  async runtimeCapacityRetrieve(xSdkworkTenantId: string, xSdkworkOrganizationId: string, xSdkworkPermissionScope: string, xSdkworkUserId?: string, xSdkworkDataScope?: string): Promise<AiotRuntimeCapacityPolicyResponse> {
    const requestHeaders = buildRequestHeaders(
      {
        'X-Sdkwork-Tenant-Id': { value: xSdkworkTenantId, style: 'simple', explode: false },
        'X-Sdkwork-Organization-Id': { value: xSdkworkOrganizationId, style: 'simple', explode: false },
        'X-Sdkwork-User-Id': { value: xSdkworkUserId, style: 'simple', explode: false },
        'X-Sdkwork-Data-Scope': { value: xSdkworkDataScope, style: 'simple', explode: false },
        'X-Sdkwork-Permission-Scope': { value: xSdkworkPermissionScope, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.get<AiotRuntimeCapacityPolicyResponse>(backendApiPath(`/iot/runtime/capacity`), undefined, requestHeaders);
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
