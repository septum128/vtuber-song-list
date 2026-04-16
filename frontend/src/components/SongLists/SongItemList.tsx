import type { SongItemType } from "@/resources/types";
import { SongItemRow } from "./SongItemRow";
import { Pagination } from "./Pagination";

type Props = {
  items: SongItemType[];
  page: number;
  perPage: number;
  onPageChange: (page: number) => void;
  favoriteIds?: Set<number>;
  showFavorite?: boolean;
  showEdit?: boolean;
  showChannel?: boolean;
};

export function SongItemList({
  items,
  page,
  perPage,
  onPageChange,
  favoriteIds,
  showFavorite = false,
  showEdit = false,
  showChannel = false,
}: Props) {
  if (items.length === 0) {
    return <p className="text-body-secondary">曲が見つかりませんでした。</p>;
  }

  return (
    <>
      <div className="table-responsive">
        <table className="table table-hover table-sm align-middle">
          <thead className="table-light">
            <tr>
              {showFavorite && <th scope="col" style={{ width: "2rem" }} />}
              <th scope="col">時間</th>
              <th scope="col">曲名</th>
              <th scope="col">アーティスト</th>
              {showChannel && <th scope="col">Vtuber</th>}
              <th scope="col">動画</th>
              {showEdit && <th scope="col" style={{ width: "5rem" }} />}
            </tr>
          </thead>
          <tbody>
            {items.map((item) => (
              <SongItemRow
                key={item.id}
                item={item}
                showFavorite={showFavorite}
                showEdit={showEdit}
                showChannel={showChannel}
                favorited={favoriteIds?.has(item.id) ?? false}
              />
            ))}
          </tbody>
        </table>
      </div>
      <Pagination
        page={page}
        perPage={perPage}
        itemCount={items.length}
        onPageChange={onPageChange}
      />
    </>
  );
}
