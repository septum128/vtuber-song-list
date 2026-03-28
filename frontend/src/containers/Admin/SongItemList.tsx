import { useState } from "react";
import { useAdminSongItems, useAdminSongItemActions } from "@/hooks/useAdminSongItems";
import { useAlerts } from "@/context/AlertsProvider";
import { Loading } from "@/components/Common/Loading";
import { Modal } from "@/components/Common/Modal";
import { Pagination } from "@/components/SongLists/Pagination";
import { SongItemForm } from "./SongItemForm";
import { ImportSongItemForm } from "./ImportSongItemForm";

const PER_PAGE = 50;

type Props = {
  videoId: number;
  videoTitle: string;
};

export function SongItemList({ videoId, videoTitle }: Props) {
  const [page, setPage] = useState(1);
  const [showForm, setShowForm] = useState(false);
  const [showImport, setShowImport] = useState(false);
  const [deletingId, setDeletingId] = useState<number | null>(null);

  const { data: items, isLoading } = useAdminSongItems(videoId, page);
  const { remove } = useAdminSongItemActions(videoId);
  const { addAlert } = useAlerts();

  async function handleDelete(id: number) {
    if (!confirm("このセトリを削除しますか？")) return;
    setDeletingId(id);
    try {
      await remove(id);
      addAlert("success", "削除しました");
    } catch (e) {
      addAlert("danger", e instanceof Error ? e.message : "削除に失敗しました");
    } finally {
      setDeletingId(null);
    }
  }

  return (
    <div>
      <div className="d-flex align-items-center gap-2 mb-1 flex-wrap">
        <h2 className="h5 mb-0">セトリ管理</h2>
        <div className="ms-auto d-flex gap-2">
          <button
            type="button"
            className="btn btn-sm btn-outline-secondary"
            onClick={() => setShowImport(true)}
          >
            インポート
          </button>
          <button
            type="button"
            className="btn btn-sm btn-primary"
            onClick={() => setShowForm((v) => !v)}
          >
            {showForm ? "キャンセル" : "+ 追加"}
          </button>
        </div>
      </div>
      <p className="text-body-secondary small mb-3">{videoTitle}</p>

      {showForm && (
        <div className="card card-body mb-3">
          <SongItemForm videoId={videoId} onSuccess={() => setShowForm(false)} />
        </div>
      )}

      {isLoading ? (
        <Loading />
      ) : !items || items.length === 0 ? (
        <p className="text-body-secondary small">セトリがありません。</p>
      ) : (
        <>
          <div className="table-responsive">
            <table className="table table-sm small">
              <thead className="table-light">
                <tr>
                  <th>ID</th>
                  <th>時間</th>
                  <th>曲名</th>
                  <th>アーティスト</th>
                  <th>操作</th>
                </tr>
              </thead>
              <tbody>
                {items.map((item) => (
                  <tr key={item.id}>
                    <td>{item.id}</td>
                    <td className="font-monospace">{item.diff.time ?? "—"}</td>
                    <td>{item.diff.title ?? <span className="text-body-secondary">未設定</span>}</td>
                    <td>{item.diff.author ?? "—"}</td>
                    <td>
                      <button
                        type="button"
                        className="btn btn-xs btn-sm btn-outline-danger"
                        onClick={() => void handleDelete(item.id)}
                        disabled={deletingId === item.id}
                      >
                        削除
                      </button>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
          <Pagination
            page={page}
            perPage={PER_PAGE}
            itemCount={items.length}
            onPageChange={setPage}
          />
        </>
      )}
      <Modal
        show={showImport}
        onClose={() => setShowImport(false)}
        title="セトリをインポート"
      >
        <ImportSongItemForm
          videoId={videoId}
          onSuccess={() => setShowImport(false)}
        />
      </Modal>
    </div>
  );
}
