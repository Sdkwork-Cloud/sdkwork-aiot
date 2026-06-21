import { formatDatetime, now, uuid } from '@sdkwork/utils';

export function createSessionId(prefix = 'aiot'): string {
  if (typeof crypto !== 'undefined' && typeof crypto.randomUUID === 'function') {
    return `${prefix}-${crypto.randomUUID()}`;
  }

  return `${prefix}-${uuid()}`;
}

export function createMessageId(prefix = 'msg'): string {
  return createSessionId(prefix);
}

export function nowIso(): string {
  return formatDatetime(now());
}

export function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => {
    setTimeout(resolve, ms);
  });
}

export {
  normalizeText,
  readBoolean,
  readNumber,
  readRecord,
  readString,
} from './value';
