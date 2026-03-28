import useSWR from "swr";
import { apiFetch } from "@/utils/api";
import type { ChannelType } from "@/resources/types";

export function useChannels() {
  return useSWR<ChannelType[]>(
    "/api/channels",
    (url: string) => apiFetch<ChannelType[]>(url),
    { revalidateOnFocus: false }
  );
}

export function useChannel(id: number | null) {
  return useSWR<ChannelType>(
    id != null ? `/api/channels/${id}` : null,
    (url: string) => apiFetch<ChannelType>(url)
  );
}
