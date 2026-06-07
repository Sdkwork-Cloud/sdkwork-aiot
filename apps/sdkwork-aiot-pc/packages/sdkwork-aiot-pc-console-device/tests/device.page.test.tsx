import {
  fireEvent,
  render,
  screen,
} from "@testing-library/react";
import "@testing-library/jest-dom/vitest";
import { describe, expect, it, vi } from "vitest";
import { SdkworkThemeProvider } from "@sdkwork/ui-pc-react/theme";
import * as deviceModule from "../src";

function createCatalog() {
  const createDefaultSdkworkManagedDevices = (deviceModule as Record<string, any>).createDefaultSdkworkManagedDevices;
  const createEmptySdkworkDeviceCatalog = (deviceModule as Record<string, any>).createEmptySdkworkDeviceCatalog;

  return createEmptySdkworkDeviceCatalog({
    devices: createDefaultSdkworkManagedDevices(),
    isAuthenticated: true,
    selectedDeviceId: "device-studio-workstation",
  });
}

describe("sdkwork-aiot-pc-console-device page", () => {
  it("renders device center, filters managed devices, and navigates to a device route", async () => {
    const Page = (deviceModule as Record<string, any>).SdkworkDevicePage;
    const onNavigate = vi.fn();

    expect(Page).toBeTypeOf("function");

    render(
      <SdkworkThemeProvider defaultTheme="light">
        <Page
          onNavigate={onNavigate}
          service={{
            getCatalog: vi.fn().mockResolvedValue(createCatalog()),
            getEmptyCatalog: vi.fn().mockReturnValue(createCatalog()),
          }}
        />
      </SdkworkThemeProvider>,
    );

    expect(
      await screen.findByRole("heading", {
        name: /device center/i,
      }),
    ).toBeInTheDocument();

    fireEvent.click(
      screen.getByRole("button", {
        name: /^critical$/i,
      }),
    );
    expect(screen.getAllByText("Render Node").length).toBeGreaterThan(0);

    fireEvent.click(
      screen.getByRole("button", {
        name: /select render node/i,
      }),
    );
    fireEvent.click(
      screen.getByRole("button", {
        name: /open device route for render node/i,
      }),
    );

    expect(onNavigate).toHaveBeenCalledWith("/devices/device-render-node");
  });
});
