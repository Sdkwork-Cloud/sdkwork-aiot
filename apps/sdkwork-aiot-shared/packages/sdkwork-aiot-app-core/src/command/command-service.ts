import type {
  AiotCommand,
  AiotCommandCreateRequest,
  SdkworkAiotAppClient,
} from '@sdkwork/aiot-app-sdk';

import { createMessageId, nowIso, readRecord, readString, sleep } from '../utils/session';

export interface CreateAiotCommandServiceOptions {
  aiotClient: SdkworkAiotAppClient;
}

export interface ExecuteDeviceCommandInput {
  capabilityName: string;
  commandName: string;
  deviceId: string;
  idempotencyKey?: string;
  payload?: Record<string, unknown>;
  sessionId?: string;
}

export interface AiotCommandService {
  executeCommand(input: ExecuteDeviceCommandInput): Promise<AiotCommand>;
  speak(deviceId: string, text: string, sessionId?: string, lang?: string): Promise<AiotCommand>;
}

function mapCommandResponse(data: unknown): AiotCommand {
  const record = readRecord(data);
  return {
    ackAt: readString(record.ackAt) || undefined,
    capabilityName: readString(record.capabilityName),
    commandId: readString(record.commandId),
    commandName: readString(record.commandName),
    createdAt: readString(record.createdAt),
    deviceId: readString(record.deviceId),
    requestPayload: record.requestPayload ?? {},
    result: record.result as AiotCommand['result'],
    resultAt: readString(record.resultAt) || undefined,
    sessionId: readString(record.sessionId) || undefined,
    status: readString(record.status, 'accepted'),
    timeoutAt: readString(record.timeoutAt) || undefined,
    traceId: readString(record.traceId) || undefined,
  };
}

export function createAiotCommandService(
  options: CreateAiotCommandServiceOptions,
): AiotCommandService {
  const { aiotClient } = options;

  return {
    async executeCommand(input) {
      const body: AiotCommandCreateRequest = {
        capabilityName: input.capabilityName,
        commandName: input.commandName,
        payload: input.payload ?? {},
        ...(input.sessionId ? { sessionId: input.sessionId } : {}),
      };

      const response = await aiotClient.iot.devicesCommandsCreate(
        input.deviceId,
        body,
        input.idempotencyKey,
      );

      return mapCommandResponse(response.data);
    },

    async speak(deviceId, text, sessionId, lang = 'zh-CN') {
      return this.executeCommand({
        capabilityName: 'audio.playback',
        commandName: 'speak',
        deviceId,
        payload: { lang, text },
        sessionId,
      });
    },
  };
}

export async function pollCommandResult(
  aiotClient: SdkworkAiotAppClient,
  deviceId: string,
  commandId: string,
  options: { intervalMs?: number; maxAttempts?: number } = {},
): Promise<AiotCommand | null> {
  const intervalMs = options.intervalMs ?? 400;
  const maxAttempts = options.maxAttempts ?? 12;

  for (let attempt = 0; attempt < maxAttempts; attempt += 1) {
    const events = await aiotClient.iot.devicesEventsList(deviceId);
    const items = Array.isArray(events.data) ? events.data : [];
    const match = items.find((event) => {
      const payload = readRecord(event.payload);
      return readString(payload.commandId) === commandId;
    });

    if (match) {
      const payload = readRecord(match.payload);
      return {
        ackAt: readString(payload.ackAt) || undefined,
        capabilityName: readString(payload.capabilityName),
        commandId,
        commandName: readString(payload.commandName),
        createdAt: readString(match.occurredAt, nowIso()),
        deviceId,
        requestPayload: payload.requestPayload ?? {},
        result: payload.result as AiotCommand['result'],
        resultAt: readString(payload.resultAt) || undefined,
        sessionId: readString(payload.sessionId) || undefined,
        status: readString(payload.status, 'completed'),
        traceId: readString(payload.traceId) || undefined,
      };
    }

    await sleep(intervalMs);
  }

  return null;
}

export function createLocalAssistantReply(userText: string): string {
  const trimmed = userText.trim();
  if (!trimmed) {
    return '请告诉我您需要什么帮助。';
  }

  return `已收到您的指令：「${trimmed}」。AIoT 智能体正在处理设备联动与场景编排。`;
}

export { createMessageId };
