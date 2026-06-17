import type { JsonValue } from './json-value';

export interface AiotFirmwareRolloutCreateRequest {
  artifactId: string;
  targetPolicy: JsonValue;
}
