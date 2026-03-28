import useSWR, { useSWRConfig } from "swr";
import { apiFetch } from "@/utils/api";
import { getToken } from "@/utils/storage";
import type { ChannelType } from "@/resources/types";

const KEY = "/api/admin/channels";

function makeKey() {
  const token = getToken();
  return token ? [KEY, token] : null;
}

export function useAdminChannels() {
  return useSWR<ChannelType[]>(
    makeKey(),
    ([url]: [string]) => apiFetch<ChannelType[]>(url, { auth: true })
  );
}

export function useAdminChannelActions() {
  const { mutate } = useSWRConfig();

  async function create(params: {
    channel_id: string;
    name?: string;
    custom_name: string;
    twitter_id?: string;
    kind: number;
  }): Promise<ChannelType> {
    const channel = await apiFetch<ChannelType>(KEY, {
      method: "POST",
      auth: true,
      body: JSON.stringify(params),
    });
    await mutate(makeKey());
    return channel;
  }

  async function update(
    id: number,
    params: {
      name?: string | null;
      custom_name?: string;
      twitter_id?: string | null;
      kind?: number;
    }
  ): Promise<ChannelType> {
    const channel = await apiFetch<ChannelType>(`${KEY}/${id}`, {
      method: "PATCH",
      auth: true,
      body: JSON.stringify(params),
    });
    await mutate(makeKey());
    return channel;
  }

  async function remove(id: number): Promise<void> {
    await apiFetch(`${KEY}/${id}`, { method: "DELETE", auth: true });
    await mutate(makeKey());
  }

  return { create, update, remove };
}
