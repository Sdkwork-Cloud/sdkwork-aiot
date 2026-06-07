import type { SdkworkDeviceSummary } from "../device";

export interface SdkworkDeviceHealthCardsProps {
  summary: SdkworkDeviceSummary;
}

export function SdkworkDeviceHealthCards({
  summary,
}: SdkworkDeviceHealthCardsProps) {
  const cards = [
    { id: "healthy", label: "Healthy", value: summary.healthyDevices },
    { id: "warning", label: "Warning", value: summary.warningDevices },
    { id: "critical", label: "Critical", value: summary.criticalDevices },
    { id: "posture", label: "Posture", value: `${summary.postureAverage}%` },
  ];

  return (
    <div className="grid gap-4 sm:grid-cols-2 xl:grid-cols-4">
      {cards.map((card) => (
        <div
          className="rounded-[1.35rem] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] p-4 shadow-[var(--sdk-shadow-sm)]"
          key={card.id}
        >
          <div className="text-sm text-[var(--sdk-color-text-secondary)]">{card.label}</div>
          <div className="mt-2 text-2xl font-semibold text-[var(--sdk-color-text-primary)]">
            {card.value}
          </div>
        </div>
      ))}
    </div>
  );
}
