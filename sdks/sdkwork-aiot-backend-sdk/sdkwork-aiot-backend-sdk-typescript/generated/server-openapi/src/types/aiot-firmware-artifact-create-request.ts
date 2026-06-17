import type { MediaResource } from './media-resource';

export interface AiotFirmwareArtifactCreateRequest {
  artifactKey: string;
  version: string;
  resource: MediaResource;
  sha256: string;
  signature?: string;
  targetChipFamily?: string;
  targetRuntimeProfile?: string;
}
