import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useRef,
  useState,
  type ReactNode,
} from "react";
import { useRouter } from "next/router";

export type AlertKind = "danger" | "notice" | "success";

export type Alert = {
  id: string;
  kind: AlertKind;
  message: string;
};

type AlertsContextValue = {
  alerts: Alert[];
  addAlert: (kind: AlertKind, message: string) => void;
  removeAlert: (id: string) => void;
  clearAlerts: () => void;
};

const AlertsContext = createContext<AlertsContextValue | null>(null);

const AUTO_DISMISS_MS = 3000;

export function AlertsProvider({ children }: { children: ReactNode }) {
  const [alerts, setAlerts] = useState<Alert[]>([]);
  const timers = useRef<Map<string, ReturnType<typeof setTimeout>>>(new Map());
  const router = useRouter();

  const removeAlert = useCallback((id: string) => {
    setAlerts((prev) => prev.filter((a) => a.id !== id));
    const timer = timers.current.get(id);
    if (timer) {
      clearTimeout(timer);
      timers.current.delete(id);
    }
  }, []);

  const addAlert = useCallback(
    (kind: AlertKind, message: string) => {
      const id = `${Date.now()}-${Math.random()}`;
      setAlerts((prev) => [...prev, { id, kind, message }]);
      const timer = setTimeout(() => removeAlert(id), AUTO_DISMISS_MS);
      timers.current.set(id, timer);
    },
    [removeAlert]
  );

  const clearAlerts = useCallback(() => {
    for (const timer of timers.current.values()) clearTimeout(timer);
    timers.current.clear();
    setAlerts([]);
  }, []);

  useEffect(() => {
    router.events.on("routeChangeStart", clearAlerts);
    return () => {
      router.events.off("routeChangeStart", clearAlerts);
    };
  }, [router.events, clearAlerts]);

  // Clean up all timers on unmount
  useEffect(() => {
    const timersRef = timers.current;
    return () => {
      for (const timer of timersRef.values()) clearTimeout(timer);
    };
  }, []);

  return (
    <AlertsContext.Provider value={{ alerts, addAlert, removeAlert, clearAlerts }}>
      {children}
    </AlertsContext.Provider>
  );
}

export function useAlerts(): AlertsContextValue {
  const ctx = useContext(AlertsContext);
  if (!ctx) throw new Error("useAlerts must be used within AlertsProvider");
  return ctx;
}
