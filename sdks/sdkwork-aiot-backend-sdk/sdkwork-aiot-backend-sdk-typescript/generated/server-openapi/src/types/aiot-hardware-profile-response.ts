import type { JsonValue } from './json-value';

export interface AiotHardwareProfileResponse {
  code: string;
  msg?: string;
  data: JsonValue;
}
