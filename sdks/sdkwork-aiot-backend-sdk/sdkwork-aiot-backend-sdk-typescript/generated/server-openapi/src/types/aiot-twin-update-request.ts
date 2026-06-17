import type { JsonValue } from './json-value';

export interface AiotTwinUpdateRequest {
  desired?: Record<string, JsonValue>;
  reported?: Record<string, JsonValue>;
}
