import type { CapabilityDefinition } from './capability-definition';

export interface AiotCapabilityModelUpdateRequest {
  displayName?: string;
  version?: string;
  capabilities?: CapabilityDefinition[];
  status?: string;
}
