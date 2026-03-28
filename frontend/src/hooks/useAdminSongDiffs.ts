import useSWR, { useSWRConfig } from "swr";
import { apiFetch } from "@/utils/api";
import { getToken } from "@/utils/storage";
import type { SongDiffType } from "@/resources/types";

function makeKey(status?: number, page?: number) {
  const token = getToken();
  if (!token) return null;
  return ["/api/admin/song_diffs", token, status ?? null, page ?? 1];
}

export function useAdminSongDiffs(status?: number, page = 1) {
  return useSWR<SongDiffType[]>(
    makeKey(status, page),
    ([url]: [string]) => {
      const params = new URLSearchParams({ page: String(page), count: "30" });
      if (status !== undefined) params.set("status", String(status));
      return apiFetch<SongDiffType[]>(`${url}?${params}`, { auth: true });
    }
  );
}

export function useAdminSongDiffActions() {
  const { mutate } = useSWRConfig();

  async function approve(id: number): Promise<SongDiffType> {
    const diff = await apiFetch<SongDiffType>(`/api/admin/song_diffs/${id}/approve`, {
      method: "PATCH",
      auth: true,
    });
    await mutate((key) => Array.isArray(key) && key[0] === "/api/admin/song_diffs");
    return diff;
  }

  async function reject(id: number): Promise<SongDiffType> {
    const diff = await apiFetch<SongDiffType>(`/api/admin/song_diffs/${id}/reject`, {
      method: "PATCH",
      auth: true,
    });
    await mutate((key) => Array.isArray(key) && key[0] === "/api/admin/song_diffs");
    return diff;
  }

  return { approve, reject };
}
