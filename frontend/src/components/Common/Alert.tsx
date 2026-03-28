import { useAlerts, type Alert } from "@/context/AlertsProvider";

const VARIANT_MAP: Record<string, string> = {
  danger: "danger",
  notice: "warning",
  success: "success",
};

function AlertItem({ alert }: { alert: Alert }) {
  const { removeAlert } = useAlerts();
  const variant = VARIANT_MAP[alert.kind] ?? "info";

  return (
    <div
      className={`alert alert-${variant} alert-dismissible fade show`}
      role="alert"
    >
      {alert.message}
      <button
        type="button"
        className="btn-close"
        aria-label="Close"
        onClick={() => removeAlert(alert.id)}
      />
    </div>
  );
}

export function AlertList() {
  const { alerts } = useAlerts();
  if (alerts.length === 0) return null;

  return (
    <div className="container mt-3">
      {alerts.map((alert) => (
        <AlertItem key={alert.id} alert={alert} />
      ))}
    </div>
  );
}
