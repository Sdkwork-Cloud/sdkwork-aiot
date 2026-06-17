import type { JsonValue } from './json-value';

export interface AiotProductResponse {
  code: string;
  msg?: string;
  data: JsonValue;
}
