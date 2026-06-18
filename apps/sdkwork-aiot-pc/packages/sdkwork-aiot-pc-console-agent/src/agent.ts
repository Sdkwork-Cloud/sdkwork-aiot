export interface SdkworkAgentWorkspaceManifest {
  capability: 'agent';
  description: string;
  host?: string;
  id: string;
  packageNames: string[];
  routePath: string;
  theme?: string;
  title: string;
}

export function createAgentWorkspaceManifest({
  description = '智能体集成：与 AIoT 设备智能体进行多轮对话，查看工具调用与设备联动结果。',
  host,
  id = 'sdkwork-aiot-agent',
  packageNames = ['@sdkwork/aiot-pc-console-agent'],
  routePath = '/agent',
  theme,
  title = '智能体集成',
}: {
  description?: string;
  host?: string;
  id?: string;
  packageNames?: string[];
  routePath?: string;
  theme?: string;
  title?: string;
} = {}): SdkworkAgentWorkspaceManifest {
  return {
    capability: 'agent',
    description,
    host,
    id,
    packageNames,
    routePath,
    theme,
    title,
  };
}
