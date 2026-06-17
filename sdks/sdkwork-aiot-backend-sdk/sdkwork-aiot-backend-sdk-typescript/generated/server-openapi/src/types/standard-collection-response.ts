import type { JsonValue } from './json-value';

export interface StandardCollectionResponse {
  code: string;
  msg?: string;
  data: JsonValue[];
}
