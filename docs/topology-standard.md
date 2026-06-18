# SDKWork AIoT Topology

Archetype: `application-rest-edge-device` (`specs/topology.spec.json`, `schemaVersion: 2`).

Platform standard: `../sdkwork-specs/APP_RUNTIME_TOPOLOGY_ADOPTION.md`

## Default dev profile

`self-hosted.split-services.development` — start the split-service stack with:

```bash
pnpm aiot:dev
```

Cloud development profile:

```bash
pnpm aiot:dev:cloud
```

## Surfaces

| Surface id | Plane | Service |
| --- | --- | --- |
| `application.app-http` | application | `sdkwork-aiot-app-api` |
| `application.admin-http` | application | `sdkwork-aiot-admin-api` |
| `edge.device-ingress` | edge | `sdkwork-aiot-gateway` |
| `platform.api-gateway` | platform | `sdkwork-api-gateway` (sibling repo) |

Edge ingress is never proxied by the platform gateway.

## Loader

`scripts/lib/aiot-topology.mjs` → `@sdkwork/app-topology`.

PC renderer client keys: `apps/sdkwork-aiot-pc/packages/sdkwork-aiot-pc-core/src/sdk/topologyEnvKeys.ts`.

Validate:

```bash
pnpm test:topology-validate
```
