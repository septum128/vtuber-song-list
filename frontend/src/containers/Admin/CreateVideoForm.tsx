import { useForm } from "react-hook-form";
import { useAlerts } from "@/context/AlertsProvider";
import { useAdminVideoActions } from "@/hooks/useAdminVideos";
import { useAdminChannels } from "@/hooks/useAdminChannels";

type FormValues = {
  video_id: string;
  channel_id: number;
};

type Props = {
  onSuccess: () => void;
};

export function CreateVideoForm({ onSuccess }: Props) {
  const { create } = useAdminVideoActions();
  const { addAlert } = useAlerts();
  const { data: channels } = useAdminChannels();

  const {
    register,
    handleSubmit,
    formState: { isSubmitting, errors },
  } = useForm<FormValues>();

  async function onSubmit(values: FormValues) {
    try {
      await create({
        video_id: values.video_id.trim(),
        channel_id: Number(values.channel_id),
      });
      addAlert("success", "動画を追加しました");
      onSuccess();
    } catch (e) {
      addAlert("danger", e instanceof Error ? e.message : "追加に失敗しました");
    }
  }

  return (
    <form onSubmit={handleSubmit(onSubmit)} noValidate>
      <div className="mb-3">
        <label className="form-label small fw-semibold">
          YouTube動画IDまたはURL
        </label>
        <input
          type="text"
          className={`form-control form-control-sm${errors.video_id ? " is-invalid" : ""}`}
          placeholder="例: dQw4w9WgXcQ または https://www.youtube.com/watch?v=..."
          {...register("video_id", { required: "動画IDは必須です" })}
        />
        {errors.video_id && (
          <div className="invalid-feedback">{errors.video_id.message}</div>
        )}
      </div>

      <div className="mb-3">
        <label className="form-label small fw-semibold">チャンネル</label>
        <select
          className={`form-select form-select-sm${errors.channel_id ? " is-invalid" : ""}`}
          {...register("channel_id", { required: "チャンネルは必須です" })}
        >
          <option value="">選択してください</option>
          {channels?.map((ch) => (
            <option key={ch.id} value={ch.id}>
              {ch.custom_name}
            </option>
          ))}
        </select>
        {errors.channel_id && (
          <div className="invalid-feedback">{errors.channel_id.message}</div>
        )}
      </div>

      <button
        type="submit"
        className="btn btn-sm btn-primary"
        disabled={isSubmitting}
      >
        {isSubmitting ? "追加中..." : "追加する"}
      </button>
    </form>
  );
}
