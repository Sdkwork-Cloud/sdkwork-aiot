import type { JsonValue } from './json-value';

export interface AiotHardwareProfileListResponse {
  code: string;
  msg?: string;
  data: JsonValue[];
}
