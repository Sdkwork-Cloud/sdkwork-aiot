import type { AiotCommand } from './aiot-command';

export interface AiotCommandListResponse {
  code: string;
  msg?: string;
  data: AiotCommand[];
}
