import { useForm } from "react-hook-form";
import { useAlerts } from "@/context/AlertsProvider";
import { useAdminSongItemActions } from "@/hooks/useAdminSongItems";

type FormValues = {
  time: string;
  title: string;
  author: string;
};

// HH:MM:SS または MM:SS 形式
const TIME_PATTERN = /^(\d{2}:)?\d{1,2}:\d{2}$/;

type Props = {
  videoId: number;
  onSuccess: () => void;
};

export function SongItemForm({ videoId, onSuccess }: Props) {
  const { create } = useAdminSongItemActions(videoId);
  const { addAlert } = useAlerts();

  const {
    register,
    handleSubmit,
    reset,
    formState: { errors, isSubmitting },
  } = useForm<FormValues>({ defaultValues: { time: "", title: "", author: "" } });

  async function onSubmit(values: FormValues) {
    try {
      await create({
        video_id: videoId,
        time: values.time || undefined,
        title: values.title || undefined,
        author: values.author || undefined,
      });
      addAlert("success", "セトリを追加しました");
      reset();
      onSuccess();
    } catch (e) {
      addAlert("danger", e instanceof Error ? e.message : "追加に失敗しました");
    }
  }

  return (
    <form onSubmit={handleSubmit(onSubmit)} noValidate>
      <div className="row g-2">
        <div className="col-12 col-sm-4">
          <label className="form-label small fw-semibold">
            時間 <span className="text-body-secondary fw-normal">（HH:MM:SS）</span>
          </label>
          <input
            type="text"
            className={`form-control form-control-sm font-monospace ${errors.time ? "is-invalid" : ""}`}
            placeholder="例: 01:23:45"
            {...register("time", {
              validate: (v) =>
                !v || TIME_PATTERN.test(v) || "HH:MM:SS または MM:SS 形式で入力してください",
            })}
          />
          {errors.time && <div className="invalid-feedback">{errors.time.message}</div>}
        </div>
        <div className="col-12 col-sm-4">
          <label className="form-label small fw-semibold">曲名</label>
          <input
            type="text"
            className="form-control form-control-sm"
            {...register("title")}
          />
        </div>
        <div className="col-12 col-sm-4">
          <label className="form-label small fw-semibold">アーティスト</label>
          <input
            type="text"
            className="form-control form-control-sm"
            {...register("author")}
          />
        </div>
      </div>
      <div className="mt-2">
        <button
          type="submit"
          className="btn btn-sm btn-primary"
          disabled={isSubmitting}
        >
          {isSubmitting ? "追加中..." : "追加する"}
        </button>
      </div>
    </form>
  );
}
