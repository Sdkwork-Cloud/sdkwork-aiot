import { describe, expect, it } from "vitest";
import * as iotModule from "../src";

describe("sdkwork-aiot-pc-console-iot domain contract", () => {
  it("creates manifests, route intents, summaries, and deterministic catalog data", () => {
    const {
      createDefaultSdkworkIotNodes,
      createIotRouteIntent,
      createIotWorkspaceManifest,
      createEmptySdkworkIotCatalog,
      iotPackageMeta,
      sortSdkworkIotNodes,
      summarizeSdkworkIotFleet,
    } = iotModule as Record<string, any>;

    expect(iotPackageMeta).toMatchObject({
      domain: "iot",
      package: "@sdkwork/aiot-pc-console-iot",
      product: "sdkwork-aiot",
      status: "ready",
    });

    expect(
      createIotWorkspaceManifest({
        title: "IoT Operations Center",
      }),
    ).toMatchObject({
      capability: "iot",
      routePath: "/iot",
      title: "IoT Operations Center",
    });

    expect(
      createIotRouteIntent({
        nodeId: "node-gateway-shanghai",
        section: "alerts",
      }),
    ).toEqual({
      focusWindow: true,
      nodeId: "node-gateway-shanghai",
      route: "/iot?section=alerts&nodeId=node-gateway-shanghai",
      section: "alerts",
      source: "iot-workspace",
      type: "iot-route-intent",
    });

    const nodes = createDefaultSdkworkIotNodes();
    expect(sortSdkworkIotNodes(nodes)[0]).toMatchObject({
      id: "node-gateway-shanghai",
      kind: "gateway",
    });

    expect(summarizeSdkworkIotFleet(nodes)).toMatchObject({
      criticalNodes: 1,
      gatewayCount: 2,
      healthyNodes: 2,
      offlineNodes: 1,
      onlineNodes: 3,
      sensorCount: 2,
      totalNodes: 4,
      warningNodes: 1,
    });

    expect(
      createEmptySdkworkIotCatalog({
        nodes,
        selectedNodeId: "missing-node",
      }),
    ).toMatchObject({
      nodes: expect.arrayContaining([
        expect.objectContaining({
          id: "node-sensor-plant-west",
          healthLevel: "critical",
        }),
      ]),
      routeIntents: {
        alerts: {
          route: "/iot?section=alerts",
        },
        fleet: {
          route: "/iot?section=fleet",
        },
        overview: {
          route: "/iot",
        },
        posture: {
          route: "/iot?section=posture",
        },
        remoteControl: {
          route: "/iot?section=remote-control",
        },
      },
      selectedNodeId: "node-gateway-shanghai",
      summary: {
        totalNodes: 4,
      },
    });
  });
});
