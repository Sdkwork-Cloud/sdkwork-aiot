export type SdkworkVoiceRouteSection = 'devices' | 'dialogue' | 'overview';

export interface SdkworkVoiceWorkspaceManifest {
  capability: 'voice';
  description: string;
  host?: string;
  id: string;
  packageNames: string[];
  routePath: string;
  theme?: string;
  title: string;
}

export interface CreateVoiceWorkspaceManifestOptions {
  description?: string;
  host?: string;
  id?: string;
  packageNames?: string[];
  routePath?: string;
  theme?: string;
  title?: string;
}

export function createVoiceWorkspaceManifest({
  description = '智能语音对话：选择 AIoT 设备，使用语音识别与 TTS 播放进行自然语言交互。',
  host,
  id = 'sdkwork-aiot-voice',
  packageNames = ['@sdkwork/aiot-pc-console-voice'],
  routePath = '/voice',
  theme,
  title = '智能语音对话',
}: CreateVoiceWorkspaceManifestOptions = {}): SdkworkVoiceWorkspaceManifest {
  return {
    capability: 'voice',
    description,
    host,
    id,
    packageNames,
    routePath,
    theme,
    title,
  };
}
