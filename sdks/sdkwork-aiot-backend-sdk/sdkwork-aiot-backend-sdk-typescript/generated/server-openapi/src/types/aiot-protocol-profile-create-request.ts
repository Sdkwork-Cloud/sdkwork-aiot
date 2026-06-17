export interface AiotProtocolProfileCreateRequest {
  protocolProfileId: string;
  defaultProtocolId: string;
  scope?: string;
  allowedTransports?: string[];
  allowedMessageClasses?: string[];
  capabilityBridges?: string[];
}
