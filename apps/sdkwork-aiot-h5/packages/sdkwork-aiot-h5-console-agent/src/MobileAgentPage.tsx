import { useEffect, useRef, useState } from 'react';
import { createAiotAgentService, getAiotH5AppSdkClient } from '@sdkwork/aiot-h5-core';
import type { AiotAgentService } from '@sdkwork/aiot-app-core';

export function MobileAgentPage() {
  const agentServiceRef = useRef<AiotAgentService | null>(null);
  const [messages, setMessages] = useState<Array<{ id: string; role: string; content: string }>>([]);
  const [draft, setDraft] = useState('');
  const [sessionId, setSessionId] = useState<string | null>(null);
  const [isSending, setIsSending] = useState(false);
  const [lastError, setLastError] = useState<string | null>(null);

  useEffect(() => {
    if (agentServiceRef.current) {
      return;
    }

    const agentService = createAiotAgentService({ aiotClient: getAiotH5AppSdkClient() });
    const session = agentService.createSession('mobile-default-device', '移动端智能体');
    agentServiceRef.current = agentService;
    setSessionId(session.id);
    setMessages(agentService.getMessages(session.id));
  }, []);

  const handleSend = async () => {
    const agentService = agentServiceRef.current;
    if (!draft.trim() || !sessionId || !agentService || isSending) {
      return;
    }

    setIsSending(true);
    setLastError(null);
    try {
      await agentService.sendMessage({
        deviceId: 'mobile-default-device',
        sessionId,
        text: draft.trim(),
      });
      setMessages(agentService.getMessages(sessionId));
      setDraft('');
    } catch (error) {
      setLastError(error instanceof Error ? error.message : '智能体消息发送失败');
    } finally {
      setIsSending(false);
    }
  };

  return (
    <div className="flex h-full flex-col p-4">
      <h1 className="text-xl font-semibold">智能体</h1>
      {lastError ? <p className="mt-2 text-sm text-red-600">{lastError}</p> : null}
      <div className="mt-4 flex-1 space-y-3 overflow-y-auto">
        {messages.map((message) => (
          <div
            className={`max-w-[85%] rounded-2xl px-3 py-2 text-sm ${
              message.role === 'user' ? 'ml-auto bg-indigo-600 text-white' : 'bg-white text-zinc-800'
            }`}
            key={message.id}
          >
            {message.content}
          </div>
        ))}
      </div>
      <div className="mt-4 flex gap-2">
        <input
          className="flex-1 rounded-2xl border border-zinc-200 px-3 py-2 text-sm"
          disabled={isSending}
          onChange={(event) => setDraft(event.target.value)}
          placeholder="向智能体提问..."
          value={draft}
        />
        <button
          className="rounded-2xl bg-indigo-600 px-4 py-2 text-sm text-white disabled:opacity-60"
          disabled={isSending}
          onClick={() => void handleSend()}
          type="button"
        >
          {isSending ? '发送中...' : '发送'}
        </button>
      </div>
    </div>
  );
}
