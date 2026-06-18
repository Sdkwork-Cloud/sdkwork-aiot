import { useEffect, useState } from 'react';
import { createAiotAgentService, getAiotH5AppSdkClient } from '@sdkwork/aiot-h5-core';

export function MobileAgentPage() {
  const [messages, setMessages] = useState<Array<{ id: string; role: string; content: string }>>([]);
  const [draft, setDraft] = useState('');
  const [sessionId, setSessionId] = useState<string | null>(null);

  useEffect(() => {
    const agentService = createAiotAgentService({ aiotClient: getAiotH5AppSdkClient() });
    const session = agentService.createSession('mobile-default-device', '移动端智能体');
    setSessionId(session.id);
    setMessages(agentService.getMessages(session.id));
  }, []);

  const handleSend = async () => {
    if (!draft.trim() || !sessionId) return;
    const agentService = createAiotAgentService({ aiotClient: getAiotH5AppSdkClient() });
    await agentService.sendMessage({
      deviceId: 'mobile-default-device',
      sessionId,
      text: draft.trim(),
    });
    setMessages(agentService.getMessages(sessionId));
    setDraft('');
  };

  return (
    <div className="flex h-full flex-col p-4">
      <h1 className="text-xl font-semibold">智能体</h1>
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
          onChange={(event) => setDraft(event.target.value)}
          placeholder="向智能体提问..."
          value={draft}
        />
        <button className="rounded-2xl bg-indigo-600 px-4 py-2 text-sm text-white" onClick={() => void handleSend()} type="button">
          发送
        </button>
      </div>
    </div>
  );
}
