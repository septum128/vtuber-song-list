import { useState } from "react";
import { useForm } from "react-hook-form";
import { useCreateSongDiff } from "@/hooks/useSongDiffs";
import { useAlerts } from "@/context/AlertsProvider";

type FormValues = {
  time: string;
  title: string;
  author: string;
};

type Props = {
  songItemId: number;
  defaultValues: FormValues;
  onSuccess: () => void;
};

// HH:MM:SS または MM:SS 形式
const TIME_PATTERN = /^(\d{2}:)?\d{1,2}:\d{2}$/;

export function SongDiffForm({ songItemId, defaultValues, onSuccess }: Props) {
  const { create } = useCreateSongDiff(songItemId);
  const { addAlert } = useAlerts();
  const [submitting, setSubmitting] = useState(false);

  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<FormValues>({ defaultValues });

  async function onSubmit(values: FormValues) {
    setSubmitting(true);
    try {
      await create({
        time: values.time || undefined,
        title: values.title || undefined,
        author: values.author || undefined,
      });
      addAlert("success", "修正を投稿しました。承認をお待ちください。");
      onSuccess();
    } catch (e) {
      addAlert("danger", e instanceof Error ? e.message : "投稿に失敗しました");
    } finally {
      setSubmitting(false);
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
          {errors.time && (
            <div className="invalid-feedback">{errors.time.message}</div>
          )}
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

      <div className="mt-3">
        <button
          type="submit"
          className="btn btn-sm btn-primary"
          disabled={submitting}
        >
          {submitting ? (
            <>
              <span
                className="spinner-border spinner-border-sm me-1"
                role="status"
                aria-hidden="true"
              />
              投稿中...
            </>
          ) : (
            "修正を投稿する"
          )}
        </button>
      </div>
    </form>
  );
}
