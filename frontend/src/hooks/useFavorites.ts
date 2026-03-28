import useSWR, { useSWRConfig } from "swr";
import { apiFetch } from "@/utils/api";
import { getToken } from "@/utils/storage";
import type { SongItemType } from "@/resources/types";

const IDS_KEY = "/api/member/favorites/ids";
const LIST_KEY = "/api/member/favorites";

type IdsResponse = { song_item_ids: number[] };

function makeKey(endpoint: string) {
  const token = getToken();
  return token ? [endpoint, token] : null;
}

/** お気に入り曲 ID セットを返す（ハートボタンの状態判定に使用） */
export function useFavoriteIds(): {
  favoriteIds: Set<number>;
  isLoading: boolean;
} {
  const { data, isLoading } = useSWR<IdsResponse>(
    makeKey(IDS_KEY),
    ([url]: [string]) => apiFetch<IdsResponse>(url, { auth: true }),
    { revalidateOnFocus: false }
  );
  return {
    favoriteIds: new Set(data?.song_item_ids ?? []),
    isLoading,
  };
}

/** お気に入り曲の詳細一覧を返す（お気に入りページ用） */
export function useFavoriteList(): {
  items: SongItemType[];
  isLoading: boolean;
  error: Error | undefined;
} {
  const { data, isLoading, error } = useSWR<SongItemType[]>(
    makeKey(LIST_KEY),
    ([url]: [string]) => apiFetch<SongItemType[]>(url, { auth: true })
  );
  return { items: data ?? [], isLoading, error };
}

/** お気に入りの追加・削除トグル */
export function useToggleFavorite() {
  const { mutate } = useSWRConfig();

  async function toggle(songItemId: number, currentlyFavorited: boolean): Promise<void> {
    const token = getToken();
    if (!token) return;

    if (currentlyFavorited) {
      await apiFetch(`/api/member/favorites/${songItemId}`, {
        method: "DELETE",
        auth: true,
      });
    } else {
      await apiFetch("/api/member/favorites", {
        method: "POST",
        auth: true,
        body: JSON.stringify({ song_item_id: songItemId }),
      });
    }

    // 両方のキャッシュを再検証
    await mutate(makeKey(IDS_KEY));
    await mutate(makeKey(LIST_KEY));
  }

  return { toggle };
}
