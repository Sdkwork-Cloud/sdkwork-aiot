# @sdkwork/aiot-pc-console-iot

## Purpose

PC user console for the SDKWork AIoT product IoT node fleet surface. Renders
gateway and sensor fleet posture, alert timeline, site posture overview, and
remote-control intents from the canonical `@sdkwork/aiot-app-sdk` resources.

## Placement

- Product: `sdkwork-aiot`
- Architecture: `pc-console`
- Domain: `iot`
- Capability: `iot-fleet`
- Status: `ready`
- Supersedes: `@sdkwork/iot-pc-react`

## Depends on

- `@sdkwork/aiot-app-sdk` for the canonical device / command / twin resources
- `@sdkwork/core-pc-react` for SDK runtime, env, and session integration
- `@sdkwork/ui-pc-react` for shared UI primitives and patterns

## Public API

Public exports are declared in `src/index.ts` and tracked in
`specs/component.spec.json` under `contracts.publicExports`.

## Required SDK Surface

- `client.iot.devices.list` (from `@sdkwork/aiot-app-sdk`)

## Configuration

Configuration keys and runtime entrypoints are declared in
`specs/component.spec.json`.

## Security

Do not add secrets, live tokens, manual auth headers, or app-local
credential handling to this module. All authentication flows through
`@sdkwork/core-pc-react` and `@sdkwork/aiot-app-sdk`.

## Extension Points

Extension points are limited to declared public exports, runtime
entrypoints, SDK clients, events, and config keys.

## Verification

- `pnpm --filter @sdkwork/aiot-pc-console-iot typecheck`
- `pnpm --filter @sdkwork/aiot-pc-console-iot test`

## Owner And Status

Owner and lifecycle status are tracked in `specs/component.spec.json`.
