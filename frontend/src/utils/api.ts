import { getToken } from "./storage";
import type { ApiErrorResponse } from "@/resources/types";

type FetchOptions = RequestInit & { auth?: boolean };

export async function apiFetch<T>(
  path: string,
  options: FetchOptions = {}
): Promise<T> {
  const { auth = false, headers: extraHeaders, ...rest } = options;
  const token = auth ? getToken() : null;

  const headers: HeadersInit = {
    "Content-Type": "application/json",
    ...(token ? { Authorization: `Bearer ${token}` } : {}),
    ...(extraHeaders as Record<string, string> | undefined),
  };

  const res = await fetch(path, { headers, ...rest });

  if (!res.ok) {
    const err: ApiErrorResponse = await res
      .json()
      .catch(() => ({ message: `HTTP ${res.status}` }));
    throw new Error(err.message);
  }

  if (res.status === 204) return undefined as T;
  return res.json() as Promise<T>;
}

export function buildQuery(
  params: Record<string, string | number | boolean | undefined | null>
): string {
  const query = new URLSearchParams();
  for (const [key, value] of Object.entries(params)) {
    if (value != null && value !== "") {
      query.set(key, String(value));
    }
  }
  const qs = query.toString();
  return qs ? `?${qs}` : "";
}
