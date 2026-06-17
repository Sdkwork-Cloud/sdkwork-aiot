import type { CapabilityProtocolMapping } from './capability-protocol-mapping';

export interface CapabilityDefinition {
  capabilityName: string;
  capabilityKind: 'property' | 'command' | 'event' | 'telemetry' | 'media' | 'ota';
  commands?: string[];
  events?: string[];
  protocolMappings?: CapabilityProtocolMapping[];
}
