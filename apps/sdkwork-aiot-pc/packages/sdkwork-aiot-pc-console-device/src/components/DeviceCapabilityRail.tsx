import type { SdkworkManagedDevice } from "../device";

export interface SdkworkDeviceCapabilityRailProps {
  device: SdkworkManagedDevice | null;
}

export function SdkworkDeviceCapabilityRail({
  device,
}: SdkworkDeviceCapabilityRailProps) {
  if (!device) {
    return (
      <aside className="rounded-[1.5rem] border border-dashed border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] p-6 text-sm text-[var(--sdk-color-text-secondary)]">
        Select a managed device to inspect posture, capability coverage, and peripheral readiness.
      </aside>
    );
  }

  return (
    <aside className="rounded-[1.5rem] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] p-6 shadow-[var(--sdk-shadow-sm)]">
      <div className="text-xs font-semibold uppercase tracking-[0.18em] text-[var(--sdk-color-text-muted)]">
        Device posture
      </div>
      <h2 className="mt-3 text-2xl font-semibold text-[var(--sdk-color-text-primary)]">{device.name}</h2>
      <div className="mt-4 space-y-3 text-sm">
        <div className="flex items-center justify-between gap-4">
          <span className="text-[var(--sdk-color-text-muted)]">Hostname</span>
          <span className="font-medium text-[var(--sdk-color-text-primary)]">{device.hostname}</span>
        </div>
        <div className="flex items-center justify-between gap-4">
          <span className="text-[var(--sdk-color-text-muted)]">Last seen</span>
          <span className="font-medium text-[var(--sdk-color-text-primary)]">{device.lastSeenAt.slice(0, 16).replace("T", " ")}</span>
        </div>
        <div className="flex items-center justify-between gap-4">
          <span className="text-[var(--sdk-color-text-muted)]">Battery</span>
          <span className="font-medium text-[var(--sdk-color-text-primary)]">
            {typeof device.batteryPercent === "number" ? `${device.batteryPercent}%` : "AC powered"}
          </span>
        </div>
      </div>

      <div className="mt-5 space-y-3">
        {device.capabilities.map((capability) => (
          <div
            className="rounded-[1rem] bg-[var(--sdk-color-surface-panel-muted)] p-4"
            key={capability.id}
          >
            <div className="flex items-center justify-between gap-4">
              <div>
                <div className="text-sm font-medium text-[var(--sdk-color-text-primary)]">{capability.label}</div>
                <div className="mt-1 text-xs uppercase tracking-[0.14em] text-[var(--sdk-color-text-muted)]">{capability.area}</div>
              </div>
              <div className="text-sm font-semibold text-[var(--sdk-color-text-primary)]">{capability.score}%</div>
            </div>
          </div>
        ))}
      </div>
    </aside>
  );
}
