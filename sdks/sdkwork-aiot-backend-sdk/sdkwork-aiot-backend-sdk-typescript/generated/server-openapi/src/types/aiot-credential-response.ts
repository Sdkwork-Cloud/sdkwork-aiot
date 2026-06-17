import type { JsonValue } from './json-value';

export interface AiotCredentialResponse {
  code: string;
  msg?: string;
  data: JsonValue;
}
