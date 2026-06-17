import type { MediaResource } from './media-resource';

export interface AiotFirmwareArtifact {
  artifactId: string;
  artifactKey: string;
  version: string;
  resource: MediaResource;
  mediaResourceId: string;
  objectBlobId?: string;
  sha256: string;
  signature?: string;
  targetChipFamily?: string;
  targetRuntimeProfile?: string;
  status: string;
}
