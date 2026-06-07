import {
  fireEvent,
  render,
  screen,
} from "@testing-library/react";
import "@testing-library/jest-dom/vitest";
import { describe, expect, it, vi } from "vitest";
import { SdkworkThemeProvider } from "@sdkwork/ui-pc-react/theme";
import * as iotModule from "../src";

describe("sdkwork-aiot-pc-console-iot fleet grid", () => {
  it("renders empty state and node actions", () => {
    const IotFleetGrid = (iotModule as Record<string, any>).SdkworkIotFleetGrid;
    const createDefaultSdkworkIotNodes = (iotModule as Record<string, any>).createDefaultSdkworkIotNodes;
    const onNavigate = vi.fn();
    const onSelectNode = vi.fn();

    expect(IotFleetGrid).toBeTypeOf("function");

    const { rerender } = render(
      <SdkworkThemeProvider defaultTheme="light">
        <IotFleetGrid nodes={[]} />
      </SdkworkThemeProvider>,
    );

    expect(screen.getByText("No IoT nodes match the current fleet filters.")).toBeInTheDocument();

    rerender(
      <SdkworkThemeProvider defaultTheme="light">
        <IotFleetGrid
          nodes={createDefaultSdkworkIotNodes()}
          onNavigate={onNavigate}
          onSelectNode={onSelectNode}
          selectedNodeId="node-gateway-shanghai"
        />
      </SdkworkThemeProvider>,
    );

    fireEvent.click(
      screen.getByRole("button", {
        name: /open node route for shanghai hub gateway/i,
      }),
    );
    fireEvent.click(
      screen.getByRole("button", {
        name: /select shanghai hub gateway/i,
      }),
    );

    expect(onNavigate).toHaveBeenCalledWith("/iot/node-gateway-shanghai");
    expect(onSelectNode).toHaveBeenCalledWith("node-gateway-shanghai");
  });
});
