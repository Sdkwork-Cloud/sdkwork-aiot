export interface AiotCredentialCreateRequest {
  credentialType: 'bearer_token' | 'hmac' | 'mtls_x509' | 'hardware_attestation';
  expiresAt?: string;
}
