import Image from "next/image";
import { format } from "date-fns";
import { ja } from "date-fns/locale";
import type { VideoType } from "@/resources/types";
import { VideoKind } from "@/resources/enums";

type Props = {
  video: VideoType;
};

const KIND_LABEL: Record<number, { label: string; variant: string }> = {
  [VideoKind.LIVE]: { label: "ライブ", variant: "danger" },
  [VideoKind.SHORT]: { label: "ショート", variant: "secondary" },
};

export function VideoCard({ video }: Props) {
  const kindInfo = KIND_LABEL[video.kind];
  const publishedAt = format(new Date(video.published_at), "yyyy/MM/dd", {
    locale: ja,
  });

  return (
    <div className="card h-100">
      <div className="position-relative" style={{ aspectRatio: "16/9" }}>
        <Image
          src={`https://img.youtube.com/vi/${video.video_id}/mqdefault.jpg`}
          alt={video.title}
          fill
          className="card-img-top"
          style={{ objectFit: "cover" }}
          sizes="(max-width: 768px) 100vw, (max-width: 1200px) 50vw, 33vw"
        />
      </div>
      <div className="card-body d-flex flex-column">
        <div className="mb-1 d-flex align-items-center gap-2">
          <span className="text-body-secondary small">{publishedAt}</span>
          {kindInfo && (
            <span className={`badge bg-${kindInfo.variant}`}>
              {kindInfo.label}
            </span>
          )}
        </div>
        <p className="card-text small fw-semibold">{video.title}</p>
        <div className="mt-auto">
          <a
            href={`https://www.youtube.com/watch?v=${video.video_id}`}
            target="_blank"
            rel="noopener noreferrer"
            className="btn btn-sm btn-outline-danger"
          >
            YouTube で見る
          </a>
        </div>
      </div>
    </div>
  );
}
