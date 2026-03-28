import useSWR from "swr";
import { apiFetch, buildQuery } from "@/utils/api";
import type { VideoType, VideosQuery } from "@/resources/types";

export function useVideos(query: VideosQuery = {}) {
  const qs = buildQuery(query as Record<string, string | number | boolean | undefined | null>);
  const key = `/api/videos${qs}`;
  return useSWR<VideoType[]>(key, (url: string) => apiFetch<VideoType[]>(url));
}

export function useVideo(id: number | null) {
  return useSWR<VideoType>(
    id != null ? `/api/videos/${id}` : null,
    (url: string) => apiFetch<VideoType>(url)
  );
}
