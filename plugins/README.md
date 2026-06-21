# Plugins

Application and runtime plugin source packages for SDKWork AIoT.

Protocol adapter plugins live in `crates/sdkwork-aiot-adapter-*` and are registered through `sdkwork-aiot-protocol` manifests. This root directory is reserved for additional SDKWork plugin packages when they are promoted out of crates.

See `specs/component.spec.json` for the protocol plugin standard and `crates/sdkwork-aiot-adapter-xiaozhi` for the reference Xiaozhi compatibility plugin.
