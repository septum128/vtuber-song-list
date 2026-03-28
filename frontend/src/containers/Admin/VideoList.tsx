import { useState } from "react";
import Link from "next/link";
import { format } from "date-fns";
import { ja } from "date-fns/locale";
import { useAdminVideos } from "@/hooks/useAdminVideos";
import { useAdminChannels } from "@/hooks/useAdminChannels";
import { Loading } from "@/components/Common/Loading";
import { Modal } from "@/components/Common/Modal";
import { Pagination } from "@/components/SongLists/Pagination";
import { VideoForm } from "./VideoForm";
import { CreateVideoForm } from "./CreateVideoForm";
import type { VideoType } from "@/resources/types";

const PER_PAGE = 30;

type Props = {
  initialChannelId?: number;
};

export function VideoList({ initialChannelId }: Props) {
  const [channelId, setChannelId] = useState<number | undefined>(initialChannelId);
  const [onlySongLives, setOnlySongLives] = useState(false);
  const [page, setPage] = useState(1);
  const [editTarget, setEditTarget] = useState<VideoType | null>(null);
  const [showCreate, setShowCreate] = useState(false);

  const { data: channels } = useAdminChannels();
  const { data: videos, isLoading } = useAdminVideos(channelId, onlySongLives, page);

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
                  <th>ID</th>
                  <th>タイトル</th>
                  <th>配信日</th>
                  <th>公開</th>
                  <th>操作</th>
                </tr>
              </thead>
              <tbody>
                {videos.map((video) => {
                  const publishedAt = format(
                    new Date(video.published_at),
                    "yyyy/MM/dd",
                    { locale: ja }
                  );
                  return (
                    <tr key={video.id}>
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
                            style={{ maxWidth: "20rem" }}
                          >
                            {video.title}
                          </span>
                        </a>
                      </td>
                      <td className="text-body-secondary">{publishedAt}</td>
                      <td>
                        {video.published ? (
                          <span className="badge bg-success">公開</span>
                        ) : (
                          <span className="badge bg-secondary">非公開</span>
                        )}
                      </td>
                      <td>
                        <div className="d-flex gap-1">
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

      <Modal
        show={editTarget !== null}
        onClose={() => setEditTarget(null)}
        title="動画編集"
      >
        {editTarget && (
          <VideoForm video={editTarget} onSuccess={() => setEditTarget(null)} />
        )}
      </Modal>

      <Modal
        show={showCreate}
        onClose={() => setShowCreate(false)}
        title="動画を追加"
      >
        <CreateVideoForm onSuccess={() => setShowCreate(false)} />
      </Modal>
    </div>
  );
}
