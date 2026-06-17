import type { AiotProtocolAdapter } from './aiot-protocol-adapter';

export interface AiotProtocolAdapterListResponse {
  code: string;
  msg?: string;
  data: AiotProtocolAdapter[];
}
