import { useFavoriteList, useFavoriteIds } from "@/hooks/useFavorites";
import { SongItemList } from "@/components/SongLists/SongItemList";
import { Loading } from "@/components/Common/Loading";

export function FavoriteList() {
  const { items, isLoading, error } = useFavoriteList();
  const { favoriteIds } = useFavoriteIds();

  if (isLoading) return <Loading />;
  if (error) {
    return (
      <div className="alert alert-danger">
        お気に入りの取得に失敗しました: {error.message}
      </div>
    );
  }
  if (items.length === 0) {
    return <p className="text-body-secondary">お気に入りした曲はまだありません。</p>;
  }

  return (
    <SongItemList
      items={items}
      page={1}
      perPage={items.length}
      onPageChange={() => undefined}
      favoriteIds={favoriteIds}
      showFavorite
    />
  );
}
