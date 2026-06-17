import type { JsonValue } from './json-value';

export interface AiotProductListResponse {
  code: string;
  msg?: string;
  data: JsonValue[];
}
