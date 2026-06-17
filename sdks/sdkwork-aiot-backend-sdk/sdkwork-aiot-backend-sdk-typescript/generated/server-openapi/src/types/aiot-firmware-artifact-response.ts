import type { AiotFirmwareArtifact } from './aiot-firmware-artifact';

export interface AiotFirmwareArtifactResponse {
  code: string;
  msg?: string;
  data: AiotFirmwareArtifact;
}
