export interface AiotHardwareProfileCreateRequest {
  hardwareProfileId: string;
  chipFamily: string;
  hardwareClasses?: string[];
  runtimeProfiles?: string[];
  connectivityProfiles?: string[];
  securityProfiles?: string[];
  otaProfiles?: string[];
}
