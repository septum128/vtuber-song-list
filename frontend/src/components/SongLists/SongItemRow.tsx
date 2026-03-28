import { useState } from "react";
import { format } from "date-fns";
import { ja } from "date-fns/locale";
import type { SongItemType } from "@/resources/types";
import { FavoriteButton } from "./FavoriteButton";
import { Modal } from "@/components/Common/Modal";
import { SongDiffList } from "./SongDiffList";
import { SongDiffForm } from "./SongDiffForm";
import { useSongDiffs } from "@/hooks/useSongDiffs";

type Props = {
  item: SongItemType;
  favorited?: boolean;
  showFavorite?: boolean;
  showEdit?: boolean;
};

function timeToSeconds(time: string): number {
  const parts = time.split(":").map(Number);
  if (parts.length === 3) {
    return parts[0] * 3600 + parts[1] * 60 + parts[2];
  }
  return parts[0] * 60 + parts[1];
}

function youtubeUrl(videoId: string, time?: string | null): string {
  const base = `https://www.youtube.com/watch?v=${videoId}`;
  if (!time) return base;
  return `${base}&t=${timeToSeconds(time)}`;
}

function EditModal({ item, onClose }: { item: SongItemType; onClose: () => void }) {
  const { data: diffs, isLoading } = useSongDiffs(item.id);

  return (
    <div>
      <h6 className="fw-semibold mb-2">修正履歴</h6>
      {isLoading ? (
        <div className="text-center py-2">
          <span className="spinner-border spinner-border-sm" role="status" />
        </div>
      ) : (
        <SongDiffList diffs={diffs ?? []} />
      )}

      <hr />

      <h6 className="fw-semibold mb-2">修正を提案する</h6>
      <SongDiffForm
        songItemId={item.id}
        defaultValues={{
          time: item.diff.time ?? "",
          title: item.diff.title ?? "",
          author: item.diff.author ?? "",
        }}
        onSuccess={onClose}
      />
    </div>
  );
}

export function SongItemRow({
  item,
  favorited = false,
  showFavorite = false,
  showEdit = false,
}: Props) {
  const { diff, video } = item;
  const [modalOpen, setModalOpen] = useState(false);
  const isActive = item.latest_diff_id != null;
  const publishedAt = format(new Date(video.published_at), "yyyy/MM/dd", {
    locale: ja,
  });

  return (
    <>
      <tr className={isActive ? "" : "text-body-secondary"}>
        {showFavorite && (
          <td className="align-middle text-center" style={{ width: "2rem" }}>
            <FavoriteButton songItemId={item.id} favorited={favorited} />
          </td>
        )}
        <td className="align-middle">
          {diff.time ? (
            <a
              href={youtubeUrl(video.video_id, diff.time)}
              target="_blank"
              rel="noopener noreferrer"
              className="text-decoration-none font-monospace small"
            >
              {diff.time}
            </a>
          ) : (
            <span className="font-monospace small text-body-secondary">—</span>
          )}
        </td>
        <td className="align-middle">
          {diff.title ?? <span className="text-body-secondary">未設定</span>}
        </td>
        <td className="align-middle small">
          {diff.author ?? <span className="text-body-secondary">—</span>}
        </td>
        <td className="align-middle small">
          <a
            href={youtubeUrl(video.video_id)}
            target="_blank"
            rel="noopener noreferrer"
            className="text-decoration-none text-body-secondary"
            title={video.title}
          >
            <span className="d-inline-block text-truncate" style={{ maxWidth: "18rem" }}>
              {video.title}
            </span>
          </a>
          <div className="text-body-tertiary" style={{ fontSize: "0.75rem" }}>
            {publishedAt}
          </div>
        </td>
        {showEdit && (
          <td className="align-middle text-center" style={{ width: "5rem" }}>
            <button
              type="button"
              className="btn btn-sm btn-outline-secondary"
              onClick={() => setModalOpen(true)}
            >
              修正を提案
            </button>
          </td>
        )}
      </tr>

      {showEdit && (
        <Modal
          show={modalOpen}
          onClose={() => setModalOpen(false)}
          title={`修正を提案 — ${diff.title ?? "未設定"}`}
        >
          <EditModal item={item} onClose={() => setModalOpen(false)} />
        </Modal>
      )}
    </>
  );
}
