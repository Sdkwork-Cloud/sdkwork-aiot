import type { JsonValue } from './json-value';

export interface AiotFirmwareRolloutUpdateRequest {
  targetPolicy?: JsonValue;
  status?: string;
}
