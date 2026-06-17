import type { AiotCodecKind } from './aiot-codec-kind';
import type { AiotDeviceAuthMode } from './aiot-device-auth-mode';
import type { AiotSessionPolicy } from './aiot-session-policy';
import type { AiotTransportBinding } from './aiot-transport-binding';

export interface AiotProtocolAdapter {
  path: string;
  protocolId: string;
  pluginId: string;
  scope: 'StandardAdapter' | 'CompatibilityPlugin' | 'BridgeAdapter';
  transport: AiotTransportBinding;
  transports: AiotTransportBinding[];
  codecs: AiotCodecKind[];
  sessionPolicies: AiotSessionPolicy[];
  securityModes: AiotDeviceAuthMode[];
  hardwareFamilies: string[];
  runtimeProfiles: string[];
  firmwareProfiles: string[];
  kind: 'deviceSession' | 'otaMetadata' | 'provisioning' | 'bridgeIngress' | 'callback';
}
