import { useCallback, useEffect, useMemo, useState } from 'react';
import { Bot, MessageSquarePlus, Send } from 'lucide-react';
import { Button, LoadingBlock, StatusNotice } from '@sdkwork/ui-pc-react';

import { createAgentWorkspaceManifest } from './agent';
import {
  createSdkworkAgentService,
  type SdkworkAgentCatalog,
  type SdkworkAgentServicePort,
} from './agent-service';

export interface SdkworkAgentPageProps {
  service?: SdkworkAgentServicePort;
}

export function SdkworkAgentPage({ service: serviceProp }: SdkworkAgentPageProps) {
  const service = useMemo(() => serviceProp ?? createSdkworkAgentService(), [serviceProp]);
  const [catalog, setCatalog] = useState<SdkworkAgentCatalog>(() => service.getCatalog());
  const [draft, setDraft] = useState('');
  const [error, setError] = useState<string | null>(null);
  const [isSending, setIsSending] = useState(false);

  const refresh = useCallback(() => {
    setCatalog(service.getCatalog());
  }, [service]);

  useEffect(() => {
    if (catalog.sessions.length === 0) {
      service.createSession(undefined, '默认智能体会话');
      refresh();
    }
  }, [catalog.sessions.length, refresh, service]);

  const handleSend = async () => {
    if (!draft.trim()) {
      return;
    }

    setIsSending(true);
    setError(null);
    try {
      await service.sendMessage(draft.trim());
      setDraft('');
      refresh();
    } catch (cause) {
      setError(cause instanceof Error ? cause.message : '发送消息失败');
    } finally {
      setIsSending(false);
    }
  };

  if (!catalog.activeSession) {
    return <LoadingBlock label="初始化智能体会话..." />;
  }

  return (
    <div className="h-full overflow-y-auto px-4 py-4 sm:px-5 sm:py-5">
      <div className="mx-auto max-w-6xl space-y-5">
        <section className="rounded-[2rem] bg-[linear-gradient(135deg,#111827,#312e81)] px-6 py-7 text-white shadow-[var(--sdk-shadow-lg)]">
          <div className="inline-flex items-center gap-2 rounded-full bg-white/10 px-3 py-1 text-[0.7rem] font-semibold uppercase tracking-[0.18em] text-white/72">
            <Bot className="h-3.5 w-3.5" />
            Agent Integration
          </div>
          <h1 className="mt-4 text-4xl font-semibold tracking-tight">智能体集成</h1>
          <p className="mt-3 max-w-2xl text-sm leading-7 text-white/72">
            与 AIoT 智能体进行多轮对话，通过设备命令 API 触发 assistant.chat 并展示会话历史。
          </p>
        </section>

        {error ? <StatusNotice tone="danger">{error}</StatusNotice> : null}

        <section className="grid gap-4 lg:grid-cols-[16rem_minmax(0,1fr)]">
          <aside className="rounded-3xl border border-zinc-200 bg-white p-4 shadow-sm">
            <div className="flex items-center justify-between">
              <h2 className="text-sm font-semibold text-zinc-900">会话</h2>
              <Button
                onClick={() => {
                  service.createSession(undefined, `会话 ${catalog.sessions.length + 1}`);
                  refresh();
                }}
                type="button"
                variant="outline"
              >
                <MessageSquarePlus className="h-4 w-4" />
              </Button>
            </div>
            <div className="mt-3 space-y-2">
              {catalog.sessions.map((session) => (
                <button
                  className={`w-full rounded-2xl px-3 py-2 text-left text-sm ${
                    catalog.activeSession?.id === session.id
                      ? 'bg-indigo-50 text-indigo-700'
                      : 'text-zinc-600 hover:bg-zinc-50'
                  }`}
                  key={session.id}
                  onClick={() => {
                    service.selectSession(session.id);
                    refresh();
                  }}
                  type="button"
                >
                  {session.title}
                </button>
              ))}
            </div>
          </aside>

          <div className="flex min-h-[32rem] flex-col rounded-3xl border border-zinc-200 bg-white shadow-sm">
            <div className="flex-1 space-y-4 overflow-y-auto p-5">
              {catalog.messages.map((message) => (
                <div
                  className={`max-w-[85%] rounded-2xl px-4 py-3 text-sm leading-6 ${
                    message.role === 'user'
                      ? 'ml-auto bg-indigo-600 text-white'
                      : 'bg-zinc-100 text-zinc-800'
                  }`}
                  key={message.id}
                >
                  <div className="mb-1 text-[0.65rem] font-semibold uppercase tracking-[0.16em] opacity-70">
                    {message.role === 'user' ? '用户' : '智能体'}
                  </div>
                  {message.content || (message.status === 'pending' ? '思考中...' : '')}
                </div>
              ))}
            </div>

            <div className="border-t border-zinc-200 p-4">
              <div className="flex gap-3">
                <input
                  className="flex-1 rounded-2xl border border-zinc-200 px-4 py-3 text-sm outline-none focus:border-indigo-500"
                  onChange={(event) => setDraft(event.target.value)}
                  onKeyDown={(event) => {
                    if (event.key === 'Enter' && !event.shiftKey) {
                      event.preventDefault();
                      void handleSend();
                    }
                  }}
                  placeholder="向 AIoT 智能体发送指令..."
                  value={draft}
                />
                <Button disabled={isSending} onClick={() => void handleSend()} type="button">
                  <Send className="mr-2 h-4 w-4" />
                  发送
                </Button>
              </div>
            </div>
          </div>
        </section>
      </div>
    </div>
  );
}

export const sdkworkAgentWorkspaceManifest = createAgentWorkspaceManifest();
