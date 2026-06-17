import type { JsonValue } from './json-value';

export interface AiotCapabilityModelListResponse {
  code: string;
  msg?: string;
  data: JsonValue[];
}
