import {
  fireEvent,
  render,
  screen,
} from "@testing-library/react";
import "@testing-library/jest-dom/vitest";
import { describe, expect, it, vi } from "vitest";
import { SdkworkThemeProvider } from "@sdkwork/ui-pc-react/theme";
import * as iotModule from "../src";

function createCatalog() {
  const createDefaultSdkworkIotNodes = (iotModule as Record<string, any>).createDefaultSdkworkIotNodes;
  const createDefaultSdkworkIotAlerts = (iotModule as Record<string, any>).createDefaultSdkworkIotAlerts;
  const createEmptySdkworkIotCatalog = (iotModule as Record<string, any>).createEmptySdkworkIotCatalog;

  return createEmptySdkworkIotCatalog({
    alerts: createDefaultSdkworkIotAlerts(),
    isAuthenticated: true,
    nodes: createDefaultSdkworkIotNodes(),
    selectedNodeId: "node-gateway-shanghai",
  });
}

describe("sdkwork-aiot-pc-console-iot page", () => {
  it("renders iot center, filters fleet nodes, and navigates to a node route", async () => {
    const Page = (iotModule as Record<string, any>).SdkworkIotPage;
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
        name: /iot operations center/i,
      }),
    ).toBeInTheDocument();

    fireEvent.click(
      screen.getByRole("button", {
        name: /^critical$/i,
      }),
    );
    expect(screen.getAllByText("Plant West Valve Sensor").length).toBeGreaterThan(0);

    fireEvent.click(
      screen.getByRole("button", {
        name: /select plant west valve sensor/i,
      }),
    );
    fireEvent.click(
      screen.getByRole("button", {
        name: /open node route for plant west valve sensor/i,
      }),
    );

    expect(onNavigate).toHaveBeenCalledWith("/iot/node-sensor-plant-west");
  });
});
