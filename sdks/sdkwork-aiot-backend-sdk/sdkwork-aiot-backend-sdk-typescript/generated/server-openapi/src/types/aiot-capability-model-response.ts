import type { JsonValue } from './json-value';

export interface AiotCapabilityModelResponse {
  code: string;
  msg?: string;
  data: JsonValue;
}
