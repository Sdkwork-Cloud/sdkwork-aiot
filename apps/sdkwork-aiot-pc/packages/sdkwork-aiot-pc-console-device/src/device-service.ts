import { readPcReactRuntimeSession } from "@sdkwork/core-pc-react";
import type {
  AiotDevice,
  SdkworkAiotAppClient,
} from "@sdkwork/aiot-app-sdk";
import {
  createEmptySdkworkDeviceCatalog,
  type SdkworkDeviceCatalogData,
  type SdkworkDeviceCapability,
  type SdkworkDevicePeripheral,
  type SdkworkManagedDevice,
} from "./device";

export interface GetSdkworkDeviceCatalogInput {
  deviceId?: string | null;
}

export interface CreateSdkworkDeviceServiceOptions {
  aiotAppContext?: {
    dataScope?: string;
    organizationId: string;
    permissionScope?: string;
    tenantId: string;
    userId?: string;
  };
  aiotClient?: SdkworkAiotAppClient;
  devices?: readonly SdkworkManagedDevice[];
  getSessionTokens?: () => {
    authToken?: string;
  };
}

interface AiotDevicesListParams {
  xSdkworkTenantId: string;
  xSdkworkOrganizationId: string;
  xSdkworkUserId?: string;
  xSdkworkDataScope?: string;
  xSdkworkPermissionScope: string;
}

export interface SdkworkDeviceService {
  getCatalog(input?: GetSdkworkDeviceCatalogInput): Promise<SdkworkDeviceCatalogData>;
  getEmptyCatalog(input?: GetSdkworkDeviceCatalogInput): SdkworkDeviceCatalogData;
}

function normalizeText(value: string | undefined): string {
  return (value ?? "").trim().toLowerCase();
}

function readRecord(value: unknown): Record<string, unknown> {
  return value && typeof value === "object" && !Array.isArray(value)
    ? value as Record<string, unknown>
    : {};
}

function readString(value: unknown, fallback = ""): string {
  if (typeof value === "string") {
    return value;
  }
  if (typeof value === "number" || typeof value === "boolean") {
    return String(value);
  }
  return fallback;
}

function readNumber(value: unknown, fallback = 0): number {
  if (typeof value === "number" && Number.isFinite(value)) {
    return value;
  }
  if (typeof value === "string" && value.trim() !== "") {
    const parsed = Number(value);
    return Number.isFinite(parsed) ? parsed : fallback;
  }
  return fallback;
}

function readBoolean(value: unknown, fallback = false): boolean {
  if (typeof value === "boolean") {
    return value;
  }
  if (typeof value === "string") {
    const normalized = value.trim().toLowerCase();
    if (normalized === "true") {
      return true;
    }
    if (normalized === "false") {
      return false;
    }
  }
  return fallback;
}

function toListParams(options: CreateSdkworkDeviceServiceOptions): AiotDevicesListParams {
  const context = options.aiotAppContext;
  if (!context?.tenantId || !context.organizationId) {
    throw new Error("Device catalog requires tenantId and organizationId AIoT app context.");
  }

  return {
    xSdkworkTenantId: context.tenantId,
    xSdkworkOrganizationId: context.organizationId,
    xSdkworkUserId: context.userId,
    xSdkworkDataScope: context.dataScope,
    xSdkworkPermissionScope: context.permissionScope ?? "iot.devices.read",
  };
}

function normalizeHealth(status: string): SdkworkManagedDevice["healthLevel"] {
  const normalized = status.toLowerCase();
  if (normalized === "online" || normalized === "healthy") {
    return "healthy";
  }
  if (normalized === "warning" || normalized === "degraded") {
    return "warning";
  }
  return "critical";
}

function readLabels(value: unknown): string[] {
  return Array.isArray(value)
    ? value.map((item) => readString(item)).filter(Boolean)
    : [];
}

function readCapabilities(value: unknown): SdkworkDeviceCapability[] {
  if (!Array.isArray(value)) {
    return [];
  }

  return value.map((item, index) => {
    const record = readRecord(item);
    const area = readString(record.area);
    const status = readString(record.status);
    return {
      area: area === "graphics" || area === "network" || area === "security" || area === "storage" ? area : "compute",
      id: readString(record.id, `capability-${index + 1}`),
      label: readString(record.label, readString(record.title, `Capability ${index + 1}`)),
      score: readNumber(record.score),
      status: status === "limited" || status === "missing" ? status : "available",
    };
  });
}

function readPeripherals(value: unknown): SdkworkDevicePeripheral[] {
  if (!Array.isArray(value)) {
    return [];
  }

  return value.map((item, index) => {
    const record = readRecord(item);
    const type = readString(record.type);
    const driverState = readString(record.driverState);
    return {
      connected: readBoolean(record.connected),
      driverState: driverState === "blocked" || driverState === "update-required" ? driverState : "ready",
      healthLevel: normalizeHealth(readString(record.healthLevel, readString(record.status, "healthy"))),
      id: readString(record.id, `peripheral-${index + 1}`),
      title: readString(record.title, readString(record.name, `Peripheral ${index + 1}`)),
      type: type === "camera" || type === "display" || type === "input" || type === "storage" ? type : "audio",
    };
  });
}

function readBatteryPercent(value: unknown): number | null {
  if (value === null || value === undefined || value === "") {
    return null;
  }
  const battery = readNumber(value, Number.NaN);
  return Number.isFinite(battery) ? Math.max(0, Math.min(100, Math.round(battery))) : null;
}

function mapAiotDeviceToManagedDevice(device: AiotDevice): SdkworkManagedDevice {
  const metadata = readRecord(device.metadata);
  const deviceId = device.deviceId || device.id;
  const status = readString(device.status);
  const online = status.toLowerCase() === "online" || status.toLowerCase() === "healthy";

  return {
    batteryPercent: readBatteryPercent(metadata.batteryPercent),
    capabilities: readCapabilities(metadata.capabilities),
    healthLevel: normalizeHealth(status),
    hostname: readString(metadata.hostname, readString(metadata.hostName, device.clientId ?? deviceId)),
    id: deviceId,
    isPrimary: readBoolean(metadata.isPrimary, false),
    labels: readLabels(metadata.labels),
    lastSeenAt: device.lastSeenAt ?? "",
    name: device.displayName || deviceId,
    online,
    osName: readString(metadata.osName, readString(metadata.os, readString(metadata.platform, device.chipFamily ?? ""))),
    peripherals: readPeripherals(metadata.peripherals),
    postureScore: readNumber(metadata.postureScore, online ? 90 : 30),
    route: `/devices/${deviceId}`,
  };
}

async function loadSdkDevices(options: CreateSdkworkDeviceServiceOptions): Promise<SdkworkManagedDevice[] | undefined> {
  if (!options.aiotClient) {
    return undefined;
  }

  const response = await options.aiotClient.iot.devices.list(toListParams(options));
  return Array.isArray(response.data) ? response.data.map(mapAiotDeviceToManagedDevice) : [];
}

export function createSdkworkDeviceService(
  options: CreateSdkworkDeviceServiceOptions = {},
): SdkworkDeviceService {
  const getSessionTokens = options.getSessionTokens ?? (() => readPcReactRuntimeSession());

  return {
    async getCatalog(input = {}) {
      const sdkDevices = await loadSdkDevices(options);
      return createEmptySdkworkDeviceCatalog({
        devices: sdkDevices ?? options.devices,
        isAuthenticated: Boolean(normalizeText(getSessionTokens().authToken)),
        selectedDeviceId: input.deviceId ?? null,
      });
    },

    getEmptyCatalog(input = {}) {
      return createEmptySdkworkDeviceCatalog({
        devices: options.devices,
        isAuthenticated: Boolean(normalizeText(getSessionTokens().authToken)),
        selectedDeviceId: input.deviceId ?? null,
      });
    },
  };
}

export const sdkworkDeviceService = createSdkworkDeviceService();
