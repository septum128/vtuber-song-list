import useSWR, { useSWRConfig } from "swr";
import { apiFetch } from "@/utils/api";
import { getToken } from "@/utils/storage";
import type { SongItemType } from "@/resources/types";

const KEY = "/api/admin/song_items";

function makeKey(videoId: number, page = 1) {
  const token = getToken();
  return token ? [KEY, token, videoId, page] : null;
}

export function useAdminSongItems(videoId: number, page = 1) {
  return useSWR<SongItemType[]>(
    makeKey(videoId, page),
    ([url]: [string]) => {
      const params = new URLSearchParams({
        video_id: String(videoId),
        page: String(page),
        count: "50",
      });
      return apiFetch<SongItemType[]>(`${url}?${params}`, { auth: true });
    }
  );
}

export function useAdminSongItemActions(videoId: number) {
  const { mutate } = useSWRConfig();

  async function create(params: {
    video_id: number;
    time?: string;
    title?: string;
    author?: string;
  }): Promise<SongItemType> {
    const item = await apiFetch<SongItemType>(KEY, {
      method: "POST",
      auth: true,
      body: JSON.stringify(params),
    });
    await mutate((key) => Array.isArray(key) && key[0] === KEY && key[2] === videoId);
    return item;
  }

  async function bulkCreate(params: {
    video_id: number;
    items: { time?: string; title?: string; author?: string }[];
  }): Promise<{ created: number }> {
    const result = await apiFetch<{ created: number }>(`${KEY}/bulk`, {
      method: "POST",
      auth: true,
      body: JSON.stringify(params),
    });
    await mutate((key) => Array.isArray(key) && key[0] === KEY && key[2] === videoId);
    return result;
  }

  async function remove(id: number): Promise<void> {
    await apiFetch(`${KEY}/${id}`, { method: "DELETE", auth: true });
    await mutate((key) => Array.isArray(key) && key[0] === KEY && key[2] === videoId);
  }

  return { create, bulkCreate, remove };
}
