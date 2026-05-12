import { useForm } from "react-hook-form";
import { useAlerts } from "@/context/AlertsProvider";
import { useAdminVideoActions } from "@/hooks/useAdminVideos";
import { VideoKind, VideoStatus } from "@/resources/enums";
import type { VideoType } from "@/resources/types";

const JST_OFFSET_MS = 9 * 60 * 60 * 1000;

function toJstDatetimeLocal(utcIso: string): string {
  const jstMs = new Date(utcIso).getTime() + JST_OFFSET_MS;
  return new Date(jstMs).toISOString().slice(0, 16);
}

function fromJstDatetimeLocalToUtc(jstLocal: string): string {
  const utcMs = new Date(`${jstLocal}:00Z`).getTime() - JST_OFFSET_MS;
  return new Date(utcMs).toISOString();
}

type FormValues = {
  title: string;
  published: boolean;
  kind: number;
  status: number;
  published_at: string;
};

type Props = {
  video: VideoType;
  onSuccess: () => void;
};

export function VideoForm({ video, onSuccess }: Props) {
  const { update } = useAdminVideoActions();
  const { addAlert } = useAlerts();

  const {
    register,
    handleSubmit,
    formState: { isSubmitting },
  } = useForm<FormValues>({
    defaultValues: {
      title: video.title,
      published: video.published,
      kind: video.kind,
      status: video.status,
      published_at: toJstDatetimeLocal(video.published_at),
    },
  });

  async function onSubmit(values: FormValues) {
    try {
      await update(video.id, {
        title: values.title,
        published: values.published,
        kind: Number(values.kind),
        status: Number(values.status),
        published_at: fromJstDatetimeLocalToUtc(values.published_at),
      });
      addAlert("success", "動画を更新しました");
      onSuccess();
    } catch (e) {
      addAlert("danger", e instanceof Error ? e.message : "更新に失敗しました");
    }
  }

  return (
    <form onSubmit={handleSubmit(onSubmit)} noValidate>
      <div className="mb-3">
        <label className="form-label small fw-semibold">タイトル</label>
        <input
          type="text"
          className="form-control form-control-sm"
          {...register("title")}
        />
      </div>

      <div className="row g-2 mb-3">
        <div className="col-12 col-sm-4">
          <label className="form-label small fw-semibold">種別</label>
          <select className="form-select form-select-sm" {...register("kind")}>
            <option value={VideoKind.VIDEO}>通常動画</option>
            <option value={VideoKind.LIVE}>ライブ配信</option>
            <option value={VideoKind.SHORT}>ショート</option>
          </select>
        </div>
        <div className="col-12 col-sm-4">
          <label className="form-label small fw-semibold">ステータス</label>
          <select className="form-select form-select-sm" {...register("status")}>
            <option value={VideoStatus.READY}>ready</option>
            <option value={VideoStatus.FETCHED}>fetched</option>
            <option value={VideoStatus.SONG_ITEMS_CREATED}>song_items_created</option>
            <option value={VideoStatus.FETCHED_HISTORY}>fetched_history</option>
            <option value={VideoStatus.SPOTIFY_FETCHED}>spotify_fetched</option>
            <option value={VideoStatus.SPOTIFY_COMPLETED}>spotify_completed</option>
            <option value={VideoStatus.COMPLETED}>completed</option>
          </select>
        </div>
        <div className="col-12 col-sm-4 d-flex align-items-end">
          <div className="form-check mb-2">
            <input
              type="checkbox"
              className="form-check-input"
              id="published"
              {...register("published")}
            />
            <label className="form-check-label small" htmlFor="published">
              公開済み
            </label>
          </div>
        </div>
      </div>

      <div className="mb-3">
        <label className="form-label small fw-semibold">配信日</label>
        <input
          type="datetime-local"
          className="form-control form-control-sm"
          {...register("published_at")}
        />
      </div>

      <button type="submit" className="btn btn-sm btn-primary" disabled={isSubmitting}>
        {isSubmitting ? "保存中..." : "更新する"}
      </button>
    </form>
  );
}
