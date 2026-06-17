import type { JsonValue } from './json-value';

export interface AiotProtocolProfileListResponse {
  code: string;
  msg?: string;
  data: JsonValue[];
}
