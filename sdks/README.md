# SDKs

Generated SDK families and route manifests for SDKWork AIoT.

| Family | Surface | TypeScript package |
| --- | --- | --- |
| `sdkwork-aiot-app-sdk` | app-api | `@sdkwork/aiot-app-sdk` |
| `sdkwork-aiot-backend-sdk` | backend-api | `@sdkwork/aiot-backend-sdk` |

Authoritative OpenAPI inputs live under `apis/`. Route manifests live under `sdks/_route-manifests/`.

Verification:

```bash
pnpm sdk:check
pnpm sdk:generate
```
