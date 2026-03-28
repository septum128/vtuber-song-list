import useSWR from "swr";
import { apiFetch, buildQuery } from "@/utils/api";
import type { SongItemType, SongItemsQuery } from "@/resources/types";

export function useSongItems(query: SongItemsQuery = {}) {
  const qs = buildQuery(query as Record<string, string | number | boolean | undefined | null>);
  const key = `/api/song_items${qs}`;
  return useSWR<SongItemType[]>(key, (url: string) => apiFetch<SongItemType[]>(url));
}

export function useSongItem(id: number | null) {
  return useSWR<SongItemType>(
    id != null ? `/api/song_items/${id}` : null,
    (url: string) => apiFetch<SongItemType>(url)
  );
}
