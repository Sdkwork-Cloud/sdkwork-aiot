export interface AiotProtocolProfileUpdateRequest {
  defaultProtocolId?: string;
  scope?: string;
  allowedTransports?: string[];
  allowedMessageClasses?: string[];
  capabilityBridges?: string[];
  status?: string;
}
