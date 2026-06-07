import { describe, expect, it } from "vitest";
import * as deviceModule from "../src";

describe("sdkwork-aiot-pc-console-device service", () => {
  it("marks catalog authentication state from runtime session tokens and keeps selected device", async () => {
    const createDefaultSdkworkManagedDevices = (deviceModule as Record<string, any>).createDefaultSdkworkManagedDevices;
    const createSdkworkDeviceService = (deviceModule as Record<string, any>).createSdkworkDeviceService;

    const devices = createDefaultSdkworkManagedDevices();
    const authenticatedService = createSdkworkDeviceService({
      devices,
      getSessionTokens: () => ({
        authToken: " session-token ",
      }),
    });

    const authenticatedCatalog = await authenticatedService.getCatalog({
      deviceId: "device-render-node",
    });
    expect(authenticatedCatalog.isAuthenticated).toBe(true);
    expect(authenticatedCatalog.selectedDeviceId).toBe("device-render-node");

    const anonymousService = createSdkworkDeviceService({
      devices,
      getSessionTokens: () => ({
        authToken: "   ",
      }),
    });

    expect(
      anonymousService.getEmptyCatalog({
        deviceId: "missing-device",
      }),
    ).toMatchObject({
      isAuthenticated: false,
      selectedDeviceId: "device-studio-workstation",
      summary: {
        totalDevices: 3,
      },
    });
  });
});
