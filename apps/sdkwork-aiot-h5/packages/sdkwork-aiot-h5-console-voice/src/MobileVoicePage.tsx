import { useEffect, useState } from 'react';
import { createAiotVoiceService, getAiotH5AppSdkClient } from '@sdkwork/aiot-h5-core';

export function MobileVoicePage() {
  const [devices, setDevices] = useState<Array<{ deviceId: string; displayName: string }>>([]);
  const [draft, setDraft] = useState('');
  const [selectedDeviceId, setSelectedDeviceId] = useState<string | null>(null);

  useEffect(() => {
    const voiceService = createAiotVoiceService({ aiotClient: getAiotH5AppSdkClient() });
    void voiceService.listVoiceDevices().then((items) => {
      setDevices(items);
      setSelectedDeviceId(items[0]?.deviceId ?? null);
    });
  }, []);

  const handleSpeak = async () => {
    if (!draft.trim()) return;
    const voiceService = createAiotVoiceService({ aiotClient: getAiotH5AppSdkClient() });
    if (selectedDeviceId) {
      await voiceService.speakOnDevice(selectedDeviceId, draft.trim());
    }
    await voiceService.speakLocally(draft.trim());
    setDraft('');
  };

  return (
    <div className="space-y-4 p-4">
      <h1 className="text-xl font-semibold">语音对话</h1>
      <select
        className="w-full rounded-2xl border border-zinc-200 px-3 py-2 text-sm"
        onChange={(event) => setSelectedDeviceId(event.target.value)}
        value={selectedDeviceId ?? ''}
      >
        {devices.map((device) => (
          <option key={device.deviceId} value={device.deviceId}>{device.displayName}</option>
        ))}
      </select>
      <textarea
        className="min-h-28 w-full rounded-2xl border border-zinc-200 px-3 py-2 text-sm"
        onChange={(event) => setDraft(event.target.value)}
        placeholder="输入或说出指令..."
        value={draft}
      />
      <button className="w-full rounded-2xl bg-cyan-600 px-4 py-3 text-sm font-medium text-white" onClick={() => void handleSpeak()} type="button">
        发送并播放
      </button>
    </div>
  );
}
