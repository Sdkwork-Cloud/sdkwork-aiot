import { useEffect, useState } from 'react';
import { getAiotH5AppSdkClient } from '@sdkwork/aiot-h5-core';

export function MobileDevicePage() {
  const [devices, setDevices] = useState<Array<{ deviceId: string; displayName: string; status: string }>>([]);

  useEffect(() => {
    void getAiotH5AppSdkClient().iot.devicesList().then((response) => {
      setDevices(Array.isArray(response.data) ? response.data.map((device) => ({
        deviceId: device.deviceId || device.id,
        displayName: device.displayName || device.deviceId || device.id,
        status: device.status,
      })) : []);
    }).catch(() => setDevices([]));
  }, []);

  return (
    <div className="space-y-3 p-4">
      <h1 className="text-xl font-semibold">设备</h1>
      {devices.map((device) => (
        <article className="rounded-2xl border border-zinc-200 bg-white p-4 shadow-sm" key={device.deviceId}>
          <h2 className="font-medium">{device.displayName}</h2>
          <p className="mt-1 text-sm text-zinc-500">{device.status}</p>
        </article>
      ))}
      {devices.length === 0 ? <p className="text-sm text-zinc-500">暂无设备</p> : null}
    </div>
  );
}
