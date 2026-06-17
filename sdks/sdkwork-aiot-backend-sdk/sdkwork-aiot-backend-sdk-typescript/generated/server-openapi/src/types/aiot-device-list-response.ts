import type { JsonValue } from './json-value';

export interface AiotDeviceListResponse {
  code: string;
  msg?: string;
  data: JsonValue[];
}
