import useSWR, { useSWRConfig } from "swr";
import { apiFetch } from "@/utils/api";
import { getToken } from "@/utils/storage";
import type { VideoType } from "@/resources/types";

const KEY = "/api/admin/videos";

function makeKey(channelId?: number, onlySongLives = false, page = 1) {
  const token = getToken();
  return token ? [KEY, token, channelId ?? null, onlySongLives, page] : null;
}

export function useAdminVideos(channelId?: number, onlySongLives = false, page = 1) {
  return useSWR<VideoType[]>(
    makeKey(channelId, onlySongLives, page),
    ([url]: [string]) => {
      const params = new URLSearchParams({ page: String(page), count: "30" });
      if (channelId !== undefined) params.set("channel_id", String(channelId));
      if (onlySongLives) params.set("only_song_lives", "true");
      return apiFetch<VideoType[]>(`${url}?${params}`, { auth: true });
    }
  );
}

export function useAdminVideoActions() {
  const { mutate } = useSWRConfig();

  async function create(params: {
    video_id: string;
    channel_id: number;
  }): Promise<VideoType> {
    const video = await apiFetch<VideoType>(KEY, {
      method: "POST",
      auth: true,
      body: JSON.stringify(params),
    });
    await mutate((key) => Array.isArray(key) && key[0] === KEY);
    return video;
  }

  async function update(
    id: number,
    params: {
      title?: string;
      published?: boolean;
      kind?: number;
      status?: number;
    }
  ): Promise<VideoType> {
    const video = await apiFetch<VideoType>(`${KEY}/${id}`, {
      method: "PATCH",
      auth: true,
      body: JSON.stringify(params),
    });
    await mutate((key) => Array.isArray(key) && key[0] === KEY);
    return video;
  }

  return { create, update };
}
