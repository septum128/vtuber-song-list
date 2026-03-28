import useSWR, { useSWRConfig } from "swr";
import { apiFetch } from "@/utils/api";
import { getToken, setToken, removeToken } from "@/utils/storage";
import type { AuthResponse, UserType } from "@/resources/types";

const USER_ENDPOINT = "/api/user";

function userFetcher(url: string): Promise<UserType> {
  return apiFetch<UserType>(url, { auth: true });
}

export function useAuth() {
  const token = getToken();
  const { mutate } = useSWRConfig();

  const { data: user, isLoading } = useSWR<UserType>(
    token ? USER_ENDPOINT : null,
    userFetcher,
    { revalidateOnFocus: false }
  );

  async function login(name: string, password: string): Promise<void> {
    const res = await apiFetch<AuthResponse>("/api/session", {
      method: "POST",
      body: JSON.stringify({ name, password }),
    });
    setToken(res.token);
    await mutate(USER_ENDPOINT, res.user, false);
  }

  async function register(
    name: string,
    password: string,
    passwordConfirmation: string
  ): Promise<void> {
    if (password !== passwordConfirmation) {
      throw new Error("パスワードと確認用パスワードが一致しません");
    }
    const res = await apiFetch<AuthResponse>("/api/user", {
      method: "POST",
      body: JSON.stringify({ name, password, password_confirmation: passwordConfirmation }),
    });
    setToken(res.token);
    await mutate(USER_ENDPOINT, res.user, false);
  }

  async function logout(): Promise<void> {
    try {
      await apiFetch("/api/session", { method: "DELETE", auth: true });
    } finally {
      removeToken();
      await mutate(USER_ENDPOINT, null, false);
    }
  }

  return { user: user ?? null, isLoading, login, register, logout };
}
