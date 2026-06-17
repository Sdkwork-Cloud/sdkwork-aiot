import type { JsonValue } from './json-value';

export interface AiotFirmwareRolloutResponse {
  code: string;
  msg?: string;
  data: JsonValue;
}
