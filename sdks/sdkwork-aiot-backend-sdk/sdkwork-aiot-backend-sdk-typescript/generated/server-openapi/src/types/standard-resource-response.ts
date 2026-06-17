import type { JsonValue } from './json-value';

export interface StandardResourceResponse {
  code: string;
  msg?: string;
  data: JsonValue;
}
