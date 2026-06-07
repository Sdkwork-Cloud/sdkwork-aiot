import {
  fireEvent,
  render,
  screen,
} from "@testing-library/react";
import "@testing-library/jest-dom/vitest";
import { describe, expect, it, vi } from "vitest";
import { SdkworkThemeProvider } from "@sdkwork/ui-pc-react/theme";
import * as deviceModule from "../src";

describe("sdkwork-aiot-pc-console-device inventory grid", () => {
  it("renders empty state and device actions", () => {
    const DeviceInventoryGrid = (deviceModule as Record<string, any>).SdkworkDeviceInventoryGrid;
    const createDefaultSdkworkManagedDevices = (deviceModule as Record<string, any>).createDefaultSdkworkManagedDevices;
    const onNavigate = vi.fn();
    const onSelectDevice = vi.fn();

    expect(DeviceInventoryGrid).toBeTypeOf("function");

    const { rerender } = render(
      <SdkworkThemeProvider defaultTheme="light">
        <DeviceInventoryGrid devices={[]} />
      </SdkworkThemeProvider>,
    );

    expect(screen.getByText("No managed devices match the current device filters.")).toBeInTheDocument();

    rerender(
      <SdkworkThemeProvider defaultTheme="light">
        <DeviceInventoryGrid
          devices={createDefaultSdkworkManagedDevices()}
          onNavigate={onNavigate}
          onSelectDevice={onSelectDevice}
          selectedDeviceId="device-studio-workstation"
        />
      </SdkworkThemeProvider>,
    );

    fireEvent.click(
      screen.getByRole("button", {
        name: /open device route for studio workstation/i,
      }),
    );
    fireEvent.click(
      screen.getByRole("button", {
        name: /select studio workstation/i,
      }),
    );

    expect(onNavigate).toHaveBeenCalledWith("/devices/device-studio-workstation");
    expect(onSelectDevice).toHaveBeenCalledWith("device-studio-workstation");
  });
});
