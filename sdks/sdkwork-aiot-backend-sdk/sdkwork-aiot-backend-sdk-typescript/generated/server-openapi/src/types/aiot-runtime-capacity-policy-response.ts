import type { AiotRuntimeCapacityPolicy } from './aiot-runtime-capacity-policy';

export interface AiotRuntimeCapacityPolicyResponse {
  code: string;
  msg?: string;
  data: AiotRuntimeCapacityPolicy;
}
