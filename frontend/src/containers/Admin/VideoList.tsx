import { useState } from "react";
import Link from "next/link";
import { format } from "date-fns";
import { ja } from "date-fns/locale";
import { useAdminVideos, useAdminVideoActions } from "@/hooks/useAdminVideos";
import { useAdminChannels } from "@/hooks/useAdminChannels";
import { useAlerts } from "@/context/AlertsProvider";
import { Loading } from "@/components/Common/Loading";
import { Modal } from "@/components/Common/Modal";
import { Pagination } from "@/components/SongLists/Pagination";
import { VideoForm } from "./VideoForm";
import { CreateVideoForm } from "./CreateVideoForm";
import { BulkCreateVideoForm } from "./BulkCreateVideoForm";
import {
  buildVideoListText,
  copyText,
  downloadTextFile,
  exportFilename,
} from "@/utils/videoExport";
import type { VideoType } from "@/resources/types";

const PER_PAGE = 30;

const STATUS_SONG_ITEMS_CREATED = 20;

const STATUS_LABELS: Record<number, { label: string; cls: string }> = {
  0: { label: "未処理", cls: "bg-secondary" },
  10: { label: "取得済", cls: "bg-primary" },
  11: { label: "コメント不可", cls: "bg-warning text-dark" },
  20: { label: "セトリ作成済", cls: "bg-info text-dark" },
  25: { label: "履歴補完済", cls: "bg-info text-dark" },
  30: { label: "Spotify検索済", cls: "bg-info text-dark" },
  35: { label: "Spotify完了", cls: "bg-info text-dark" },
  40: { label: "完了", cls: "bg-success" },
};

function StatusBadge({ status }: { status: number }) {
  const s = STATUS_LABELS[status] ?? { label: `status:${status}`, cls: "bg-secondary" };
  return <span className={`badge ${s.cls}`}>{s.label}</span>;
}

type Props = {
  initialChannelId?: number;
};

export function VideoList({ initialChannelId }: Props) {
  const [channelId, setChannelId] = useState<number | undefined>(initialChannelId);
  const [onlySongLives, setOnlySongLives] = useState(false);
  const [page, setPage] = useState(1);
  const [editTarget, setEditTarget] = useState<VideoType | null>(null);
  const [showCreate, setShowCreate] = useState(false);
  const [showBulkCreate, setShowBulkCreate] = useState(false);
  // ページを跨いだ選択も保持するため、id だけでなく動画データごと持つ。
  const [selected, setSelected] = useState<Map<number, VideoType>>(new Map());
  const [processingIds, setProcessingIds] = useState<Set<number>>(new Set());

  const { addAlert } = useAlerts();
  const { data: channels } = useAdminChannels();
  const { data: videos, isLoading } = useAdminVideos(channelId, onlySongLives, page);
  const { fetchSetlist, bulkFetchSetlist } = useAdminVideoActions();

  const allSelected =
    !!videos && videos.length > 0 && videos.every((v) => selected.has(v.id));
  const someSelected = selected.size > 0;

  function toggleSelect(video: VideoType) {
    setSelected((prev) => {
      const next = new Map(prev);
      if (next.has(video.id)) next.delete(video.id);
      else next.set(video.id, video);
      return next;
    });
  }

  function toggleSelectAll() {
    setSelected((prev) => {
      const next = new Map(prev);
      if (allSelected) {
        videos?.forEach((v) => next.delete(v.id));
      } else {
        videos?.forEach((v) => next.set(v.id, v));
      }
      return next;
    });
  }

  function clearSelection() {
    setSelected(new Map());
  }

  function selectedVideoList(): VideoType[] {
    return Array.from(selected.values()).sort((a, b) => a.id - b.id);
  }

  function handleExportText() {
    const text = buildVideoListText(selectedVideoList());
    downloadTextFile(exportFilename(), text);
    addAlert("success", `${selected.size}件の動画を書き出しました`);
  }

  async function handleCopyText() {
    try {
      await copyText(buildVideoListText(selectedVideoList()));
      addAlert("success", `${selected.size}件の動画をクリップボードにコピーしました`);
    } catch (e) {
      addAlert("danger", e instanceof Error ? e.message : "コピーに失敗しました");
    }
  }

  async function handleFetchSetlist(videoId: number, force: boolean) {
    setProcessingIds((prev) => new Set([...prev, videoId]));
    try {
      await fetchSetlist(videoId, force);
      addAlert("success", force ? "強制再取得ジョブをキューしました" : "セトリ取得ジョブをキューしました");
    } catch (e) {
      addAlert("danger", e instanceof Error ? e.message : "エラーが発生しました");
    } finally {
      setProcessingIds((prev) => {
        const next = new Set(prev);
        next.delete(videoId);
        return next;
      });
    }
  }

  async function handleBulkFetchSetlist(force: boolean) {
    if (selected.size === 0) return;
    try {
      await bulkFetchSetlist(Array.from(selected.keys()), force);
      addAlert(
        "success",
        force
          ? `${selected.size}件の強制再取得ジョブをキューしました`
          : `${selected.size}件のセトリ取得ジョブをキューしました`
      );
      clearSelection();
    } catch (e) {
      addAlert("danger", e instanceof Error ? e.message : "エラーが発生しました");
    }
  }

  return (
    <div>
      <div className="d-flex align-items-center gap-3 mb-3 flex-wrap">
        <h2 className="h5 mb-0">動画管理</h2>
        <button
          type="button"
          className="btn btn-sm btn-outline-primary"
          onClick={() => setShowCreate(true)}
        >
          動画を追加
        </button>
        <button
          type="button"
          className="btn btn-sm btn-outline-secondary"
          onClick={() => setShowBulkCreate(true)}
        >
          TSVで一括登録
        </button>
        <select
          className="form-select form-select-sm"
          style={{ maxWidth: "16rem" }}
          value={channelId ?? ""}
          onChange={(e) => {
            setChannelId(e.target.value ? Number(e.target.value) : undefined);
            setPage(1);
          }}
        >
          <option value="">すべてのチャンネル</option>
          {channels?.map((ch) => (
            <option key={ch.id} value={ch.id}>
              {ch.custom_name}
            </option>
          ))}
        </select>
        <div className="form-check mb-0">
          <input
            type="checkbox"
            className="form-check-input"
            id="only-song-lives"
            checked={onlySongLives}
            onChange={(e) => {
              setOnlySongLives(e.target.checked);
              setPage(1);
            }}
          />
          <label className="form-check-label small" htmlFor="only-song-lives">
            歌枠のみ
          </label>
        </div>
      </div>

      {someSelected && (
        <div className="d-flex align-items-center gap-2 mb-3 p-2 bg-body-secondary rounded">
          <span className="small text-body-secondary">{selected.size}件選択中</span>
          <button
            type="button"
            className="btn btn-sm btn-outline-primary"
            onClick={() => handleBulkFetchSetlist(false)}
          >
            セトリ取得
          </button>
          <button
            type="button"
            className="btn btn-sm btn-outline-danger"
            onClick={() => handleBulkFetchSetlist(true)}
          >
            強制再取得
          </button>
          <button
            type="button"
            className="btn btn-sm btn-outline-secondary"
            onClick={handleExportText}
          >
            テキストで書き出し
          </button>
          <button
            type="button"
            className="btn btn-sm btn-outline-secondary"
            onClick={handleCopyText}
          >
            クリップボードにコピー
          </button>
          <button
            type="button"
            className="btn btn-sm btn-link text-body-secondary"
            onClick={clearSelection}
          >
            選択解除
          </button>
        </div>
      )}

      {isLoading ? (
        <Loading />
      ) : !videos || videos.length === 0 ? (
        <p className="text-body-secondary small">動画がありません。</p>
      ) : (
        <>
          <div className="table-responsive">
            <table className="table table-sm small">
              <thead className="table-light">
                <tr>
                  <th style={{ width: "2rem" }}>
                    <input
                      type="checkbox"
                      className="form-check-input"
                      checked={allSelected}
                      onChange={toggleSelectAll}
                    />
                  </th>
                  <th>ID</th>
                  <th>タイトル</th>
                  <th>配信日</th>
                  <th>ステータス</th>
                  <th>公開</th>
                  <th>操作</th>
                </tr>
              </thead>
              <tbody>
                {videos.map((video) => {
                  const publishedAt = format(new Date(video.published_at), "yyyy/MM/dd", {
                    locale: ja,
                  });
                  const isProcessing = processingIds.has(video.id);
                  const isProcessed = video.status >= STATUS_SONG_ITEMS_CREATED;
                  return (
                    <tr key={video.id}>
                      <td>
                        <input
                          type="checkbox"
                          className="form-check-input"
                          checked={selected.has(video.id)}
                          onChange={() => toggleSelect(video)}
                        />
                      </td>
                      <td>{video.id}</td>
                      <td>
                        <a
                          href={`https://www.youtube.com/watch?v=${video.video_id}`}
                          target="_blank"
                          rel="noopener noreferrer"
                          className="text-decoration-none"
                        >
                          <span
                            className="d-inline-block text-truncate"
                            style={{ maxWidth: "18rem" }}
                          >
                            {video.title}
                          </span>
                        </a>
                      </td>
                      <td className="text-body-secondary">{publishedAt}</td>
                      <td>
                        <StatusBadge status={video.status} />
                      </td>
                      <td>
                        {video.published ? (
                          <span className="badge bg-success">公開</span>
                        ) : (
                          <span className="badge bg-secondary">非公開</span>
                        )}
                      </td>
                      <td>
                        <div className="d-flex gap-1 flex-wrap">
                          <button
                            type="button"
                            className="btn btn-xs btn-sm btn-outline-secondary"
                            onClick={() => setEditTarget(video)}
                          >
                            編集
                          </button>
                          <Link
                            href={{
                              pathname: `/admin/videos/${video.id}/song_items`,
                              query: { title: video.title },
                            }}
                            className="btn btn-xs btn-sm btn-outline-secondary"
                          >
                            セトリ
                          </Link>
                          {!isProcessed ? (
                            <button
                              type="button"
                              className="btn btn-xs btn-sm btn-outline-primary"
                              disabled={isProcessing}
                              onClick={() => handleFetchSetlist(video.id, false)}
                            >
                              {isProcessing ? "…" : "セトリ取得"}
                            </button>
                          ) : (
                            <button
                              type="button"
                              className="btn btn-xs btn-sm btn-outline-danger"
                              disabled={isProcessing}
                              onClick={() => handleFetchSetlist(video.id, true)}
                            >
                              {isProcessing ? "…" : "再取得"}
                            </button>
                          )}
                        </div>
                      </td>
                    </tr>
                  );
                })}
              </tbody>
            </table>
          </div>
          <Pagination
            page={page}
            perPage={PER_PAGE}
            itemCount={videos.length}
            onPageChange={setPage}
          />
        </>
      )}

      <Modal show={editTarget !== null} onClose={() => setEditTarget(null)} title="動画編集">
        {editTarget && (
          <VideoForm video={editTarget} onSuccess={() => setEditTarget(null)} />
        )}
      </Modal>

      <Modal show={showCreate} onClose={() => setShowCreate(false)} title="動画を追加">
        <CreateVideoForm onSuccess={() => setShowCreate(false)} />
      </Modal>

      <Modal
        show={showBulkCreate}
        onClose={() => setShowBulkCreate(false)}
        title="TSVで一括登録"
      >
        <BulkCreateVideoForm onSuccess={() => setShowBulkCreate(false)} />
      </Modal>
    </div>
  );
}
