import type { JsonValue } from './json-value';

export interface AiotProtocolProfileResponse {
  code: string;
  msg?: string;
  data: JsonValue;
}
