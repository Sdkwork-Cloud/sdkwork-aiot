# SDKWork AIoT PC

Console packages for device and IoT fleet management in the SDKWork AIoT PC application workspace.

## Topology

Client runtime reads v2 topology surface keys from `specs/topology.spec.json` via `@sdkwork/aiot-pc-core`:

- `VITE_SDKWORK_AIOT_APPLICATION_APP_HTTP_URL` — app API (`application.app-http`)
- `VITE_SDKWORK_AIOT_APPLICATION_ADMIN_HTTP_URL` — admin API (`application.admin-http`)
- `VITE_SDKWORK_AIOT_EDGE_DEVICE_INGRESS_HTTP_URL` — device edge ingress (`edge.device-ingress`)
- `VITE_SDKWORK_AIOT_PLATFORM_API_GATEWAY_HTTP_URL` — platform gateway for IAM/appbase SDKs

Start the backend stack from the repository root:

```bash
pnpm aiot:dev
```

Optional renderer overrides: copy `.env.example` into this directory when running a Vite shell locally.
