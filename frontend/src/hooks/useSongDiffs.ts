import useSWR, { useSWRConfig } from "swr";
import { apiFetch } from "@/utils/api";
import { getToken } from "@/utils/storage";
import type { SongDiffType } from "@/resources/types";

function makeKey(songItemId: number) {
  const token = getToken();
  return token ? [`/api/member/song_items/${songItemId}/song_diffs`, token] : null;
}

export function useSongDiffs(songItemId: number | null) {
  return useSWR<SongDiffType[]>(
    songItemId != null ? makeKey(songItemId) : null,
    ([url]: [string]) => apiFetch<SongDiffType[]>(url, { auth: true })
  );
}

export function useCreateSongDiff(songItemId: number) {
  const { mutate } = useSWRConfig();

  async function create(params: {
    time?: string;
    title?: string;
    author?: string;
  }): Promise<SongDiffType> {
    const diff = await apiFetch<SongDiffType>(
      `/api/member/song_items/${songItemId}/song_diffs`,
      {
        method: "POST",
        auth: true,
        body: JSON.stringify(params),
      }
    );
    await mutate(makeKey(songItemId));
    return diff;
  }

  return { create };
}
