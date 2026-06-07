import type { SdkworkIotAlert } from "../iot";

export interface SdkworkIotAlertTimelineProps {
  alerts: readonly SdkworkIotAlert[];
  onNavigate?: (route: string) => void;
}

function labelSeverity(value: SdkworkIotAlert["severity"]): string {
  if (value === "critical") {
    return "Critical";
  }

  if (value === "warning") {
    return "Warning";
  }

  return "Info";
}

export function SdkworkIotAlertTimeline({
  alerts,
  onNavigate,
}: SdkworkIotAlertTimelineProps) {
  return (
    <section className="space-y-3 rounded-[1.2rem] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] p-4">
      <div className="flex items-center justify-between gap-3">
        <h3 className="text-sm font-semibold uppercase tracking-[0.16em] text-[var(--sdk-color-text-muted)]">
          Alert timeline
        </h3>
        <div className="text-xs text-[var(--sdk-color-text-secondary)]">{alerts.length} alerts</div>
      </div>

      {alerts.length === 0 ? (
        <p className="text-sm text-[var(--sdk-color-text-secondary)]">No alerts under current filters.</p>
      ) : (
        <div className="space-y-2">
          {alerts.map((alert) => {
            const severityClasses = alert.severity === "critical"
              ? "bg-rose-500/12 text-rose-500"
              : alert.severity === "warning"
                ? "bg-amber-500/12 text-amber-500"
                : "bg-sky-500/12 text-sky-500";

            return (
              <article
                className="rounded-[0.95rem] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel-muted)] p-3"
                key={alert.id}
              >
                <div className="flex items-start justify-between gap-3">
                  <div className="min-w-0">
                    <p className="truncate text-sm font-semibold text-[var(--sdk-color-text-primary)]">{alert.title}</p>
                    <p className="mt-1 text-xs text-[var(--sdk-color-text-secondary)]">{alert.createdAt}</p>
                  </div>
                  <span className={`rounded-full px-2 py-0.5 text-[0.68rem] font-semibold uppercase tracking-[0.08em] ${severityClasses}`}>
                    {labelSeverity(alert.severity)}
                  </span>
                </div>
                <div className="mt-2 flex items-center justify-between gap-3">
                  <span className="text-xs text-[var(--sdk-color-text-secondary)]">
                    {alert.acknowledged ? "Acknowledged" : "Pending acknowledgment"}
                  </span>
                  <button
                    className="rounded-[0.7rem] border border-[var(--sdk-color-border-default)] px-2.5 py-1 text-xs font-semibold text-[var(--sdk-color-text-secondary)] transition-colors hover:border-cyan-500/50 hover:text-cyan-500"
                    onClick={() => onNavigate?.(alert.route)}
                    type="button"
                  >
                    Open alert
                  </button>
                </div>
              </article>
            );
          })}
        </div>
      )}
    </section>
  );
}
