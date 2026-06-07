import { describe, expect, it } from "vitest";
import * as deviceModule from "../src";

describe("sdkwork-aiot-pc-console-device domain contract", () => {
  it("creates manifests, route intents, summaries, and deterministic catalog data", () => {
    const {
      createDefaultSdkworkManagedDevices,
      createDeviceRouteIntent,
      createDeviceWorkspaceManifest,
      createEmptySdkworkDeviceCatalog,
      devicePackageMeta,
      sortSdkworkManagedDevices,
      summarizeSdkworkDevices,
    } = deviceModule as Record<string, any>;

    expect(devicePackageMeta).toMatchObject({
      domain: "device",
      package: "@sdkwork/aiot-pc-console-device",
      product: "sdkwork-aiot",
      status: "ready",
    });

    expect(
      createDeviceWorkspaceManifest({
        title: "Device Center",
      }),
    ).toMatchObject({
      capability: "device",
      routePath: "/devices",
      title: "Device Center",
    });

    expect(
      createDeviceRouteIntent({
        deviceId: "device-field-laptop",
        section: "peripherals",
      }),
    ).toEqual({
      deviceId: "device-field-laptop",
      focusWindow: true,
      route: "/devices?section=peripherals&deviceId=device-field-laptop",
      section: "peripherals",
      source: "device-workspace",
      type: "device-route-intent",
    });

    const devices = createDefaultSdkworkManagedDevices();
    expect(sortSdkworkManagedDevices(devices)[0]).toMatchObject({
      id: "device-studio-workstation",
      isPrimary: true,
    });

    expect(summarizeSdkworkDevices(devices)).toMatchObject({
      connectedPeripherals: 3,
      criticalDevices: 1,
      healthyDevices: 1,
      postureAverage: 69,
      primaryDeviceId: "device-studio-workstation",
      totalDevices: 3,
      warningDevices: 1,
    });

    expect(
      createEmptySdkworkDeviceCatalog({
        devices,
        selectedDeviceId: "missing-device",
      }),
    ).toMatchObject({
      devices: expect.arrayContaining([
        expect.objectContaining({
          id: "device-render-node",
          postureScore: 39,
        }),
      ]),
      isAuthenticated: false,
      routeIntents: {
        overview: {
          route: "/devices",
        },
        peripherals: {
          route: "/devices?section=peripherals",
        },
        posture: {
          route: "/devices?section=posture",
        },
      },
      selectedDeviceId: "device-studio-workstation",
      summary: {
        totalDevices: 3,
      },
    });
  });
});
