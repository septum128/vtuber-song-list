import useSWR, { useSWRConfig } from "swr";
import { apiFetch } from "@/utils/api";
import { getToken } from "@/utils/storage";
import type { VideoType } from "@/resources/types";

export type BulkCreateItem = {
  url: string;
  detail: string;
};

export type BulkCreateResult = {
  succeeded: BulkCreateItem[];
  skipped: BulkCreateItem[];
  failed: BulkCreateItem[];
};

export type AdminVideoSort = "published_at" | "id";

const KEY = "/api/admin/videos";

function makeKey(
  channelId?: number,
  onlySongLives = false,
  page = 1,
  perPage = 30,
  sort: AdminVideoSort = "published_at"
) {
  const token = getToken();
  return token ? [KEY, token, channelId ?? null, onlySongLives, page, perPage, sort] : null;
}

export function useAdminVideos(
  channelId?: number,
  onlySongLives = false,
  page = 1,
  perPage = 30,
  sort: AdminVideoSort = "published_at"
) {
  return useSWR<VideoType[]>(
    makeKey(channelId, onlySongLives, page, perPage, sort),
    ([url]: [string]) => {
      const params = new URLSearchParams({ page: String(page), count: String(perPage) });
      if (channelId !== undefined) params.set("channel_id", String(channelId));
      if (onlySongLives) params.set("only_song_lives", "true");
      if (sort !== "published_at") params.set("sort", sort);
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
      published_at?: string;
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

  async function bulkCreate(tsv: string): Promise<BulkCreateResult> {
    const result = await apiFetch<BulkCreateResult>(`${KEY}/bulk`, {
      method: "POST",
      auth: true,
      body: JSON.stringify({ tsv }),
    });
    await mutate((key) => Array.isArray(key) && key[0] === KEY);
    return result;
  }

  async function fetchSetlist(videoId: number, force = false): Promise<void> {
    await apiFetch<{ message: string }>(`${KEY}/${videoId}/fetch_setlist`, {
      method: "POST",
      auth: true,
      body: JSON.stringify({ force }),
    });
  }

  async function bulkFetchSetlist(videoIds: number[], force = false): Promise<void> {
    await apiFetch<{ message: string }>(`${KEY}/bulk_fetch_setlist`, {
      method: "POST",
      auth: true,
      body: JSON.stringify({ video_ids: videoIds, force }),
    });
  }

  async function bulkPublish(videoIds: number[], published: boolean): Promise<void> {
    await apiFetch<{ updated: number }>(`${KEY}/bulk_publish`, {
      method: "POST",
      auth: true,
      body: JSON.stringify({ video_ids: videoIds, published }),
    });
    await mutate((key) => Array.isArray(key) && key[0] === KEY);
  }

  return { create, update, bulkCreate, fetchSetlist, bulkFetchSetlist, bulkPublish };
}
