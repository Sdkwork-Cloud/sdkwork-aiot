import type { CapabilityDefinition } from './capability-definition';

export interface AiotCapabilityModelCreateRequest {
  capabilityModelId: string;
  displayName: string;
  version: string;
  capabilities?: CapabilityDefinition[];
}
