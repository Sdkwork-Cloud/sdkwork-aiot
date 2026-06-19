# Backend TypeScript SDK Generation

Generated output lives under `generated/server-openapi/`. The hand-written `src/index.ts` re-exports the generated client and types.

## Regenerate

From this package directory:

```bash
npm install
npm run generate
```

This uses `@sdkwork/sdk-generator` (`sdkgen` CLI). The `sdkwork-sdk-generate` binary referenced in older manifests is not published; use `sdkgen generate` instead.

## sdkwork-v3 profile note

Generation currently omits `--standard-profile sdkwork-v3` because the backend OpenAPI authority still has unresolved SDKWork v3 validation items (security scheme naming, dual-token requirements on operations, and related problem-response coverage). Once those OpenAPI fixes land, regenerate with:

```bash
sdkgen generate -i ../../../../apis/backend-api/iot/sdkwork-aiot-backend-api.openapi.json -o generated/server-openapi -n sdkwork-aiot-backend-sdk -t backend -l typescript --package-name "@sdkwork/aiot-backend-sdk" --api-prefix "/backend/v3/api" --standard-profile sdkwork-v3 --sdk-version 0.1.0
```

## Verification

```bash
npm run typecheck
cargo test -p sdkwork-aiot-architecture -p sdkwork-iot-platform-service
```
