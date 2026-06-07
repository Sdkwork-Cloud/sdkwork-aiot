import { readPcReactRuntimeSession } from "@sdkwork/core-pc-react";
import type {
  AiotDevice,
  SdkworkAiotAppClient,
} from "@sdkwork/aiot-app-sdk";
import {
  createEmptySdkworkIotCatalog,
  type SdkworkIotAlert,
  type SdkworkIotCatalogData,
  type SdkworkIotNode,
} from "./iot";

export interface GetSdkworkIotCatalogInput {
  nodeId?: string | null;
}

export interface CreateSdkworkIotServiceOptions {
  aiotAppContext?: {
    dataScope?: string;
    organizationId: string;
    permissionScope?: string;
    tenantId: string;
    userId?: string;
  };
  aiotClient?: SdkworkAiotAppClient;
  alerts?: readonly SdkworkIotAlert[];
  getSessionTokens?: () => {
    authToken?: string;
  };
  nodes?: readonly SdkworkIotNode[];
}

interface AiotDevicesListParams {
  xSdkworkTenantId: string;
  xSdkworkOrganizationId: string;
  xSdkworkUserId?: string;
  xSdkworkDataScope?: string;
  xSdkworkPermissionScope: string;
}

export interface SdkworkIotService {
  getCatalog(input?: GetSdkworkIotCatalogInput): Promise<SdkworkIotCatalogData>;
  getEmptyCatalog(input?: GetSdkworkIotCatalogInput): SdkworkIotCatalogData;
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

function toListParams(options: CreateSdkworkIotServiceOptions): AiotDevicesListParams {
  const context = options.aiotAppContext;
  if (!context?.tenantId || !context.organizationId) {
    throw new Error("AIoT catalog requires tenantId and organizationId app context.");
  }

  return {
    xSdkworkTenantId: context.tenantId,
    xSdkworkOrganizationId: context.organizationId,
    xSdkworkUserId: context.userId,
    xSdkworkDataScope: context.dataScope,
    xSdkworkPermissionScope: context.permissionScope ?? "iot.devices.read",
  };
}

function normalizeHealth(status: string): SdkworkIotNode["healthLevel"] {
  const normalized = status.toLowerCase();
  if (normalized === "online" || normalized === "healthy") {
    return "healthy";
  }
  if (normalized === "warning" || normalized === "degraded") {
    return "warning";
  }
  return "critical";
}

function mapAiotDeviceToNode(device: AiotDevice): SdkworkIotNode {
  const metadata = readRecord(device.metadata);
  const deviceId = device.deviceId || device.id;
  const status = readString(device.status);
  const kind = readString(metadata.kind, readString(metadata.type, device.chipFamily ? "gateway" : "sensor"));
  const labels = Array.isArray(metadata.labels)
    ? metadata.labels.map((item) => readString(item)).filter(Boolean)
    : [];

  return {
    firmwareVersion: readString(metadata.firmwareVersion),
    gatewayId: readString(metadata.gatewayId) || undefined,
    healthLevel: normalizeHealth(status),
    id: deviceId,
    kind: kind === "gateway" ? "gateway" : "sensor",
    labels,
    lastSeenAt: device.lastSeenAt ?? "",
    name: device.displayName || deviceId,
    online: status.toLowerCase() === "online",
    postureScore: readNumber(metadata.postureScore, status.toLowerCase() === "online" ? 90 : 30),
    route: `/iot/${deviceId}`,
    sensors: [],
    site: readString(metadata.site, readString(metadata.location, "Unassigned")),
  };
}

async function loadSdkNodes(options: CreateSdkworkIotServiceOptions): Promise<SdkworkIotNode[] | undefined> {
  if (!options.aiotClient) {
    return undefined;
  }

  const response = await options.aiotClient.iot.devices.list(toListParams(options));
  return Array.isArray(response.data) ? response.data.map(mapAiotDeviceToNode) : [];
}

export function createSdkworkIotService(
  options: CreateSdkworkIotServiceOptions = {},
): SdkworkIotService {
  const getSessionTokens = options.getSessionTokens ?? (() => readPcReactRuntimeSession());

  return {
    async getCatalog(input = {}) {
      const sdkNodes = await loadSdkNodes(options);
      return createEmptySdkworkIotCatalog({
        alerts: options.alerts,
        isAuthenticated: Boolean(normalizeText(getSessionTokens().authToken)),
        nodes: sdkNodes ?? options.nodes,
        selectedNodeId: input.nodeId ?? null,
      });
    },

    getEmptyCatalog(input = {}) {
      return createEmptySdkworkIotCatalog({
        alerts: options.alerts,
        isAuthenticated: Boolean(normalizeText(getSessionTokens().authToken)),
        nodes: options.nodes,
        selectedNodeId: input.nodeId ?? null,
      });
    },
  };
}

export const sdkworkIotService = createSdkworkIotService();
