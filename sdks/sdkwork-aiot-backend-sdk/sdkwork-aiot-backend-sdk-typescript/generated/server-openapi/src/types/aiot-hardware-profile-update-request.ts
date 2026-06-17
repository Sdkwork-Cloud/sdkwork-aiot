export interface AiotHardwareProfileUpdateRequest {
  chipFamily?: string;
  hardwareClasses?: string[];
  runtimeProfiles?: string[];
  connectivityProfiles?: string[];
  securityProfiles?: string[];
  otaProfiles?: string[];
  status?: string;
}
