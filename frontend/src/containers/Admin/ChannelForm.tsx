import { useForm } from "react-hook-form";
import { useAlerts } from "@/context/AlertsProvider";
import { useAdminChannelActions } from "@/hooks/useAdminChannels";
import { ChannelKind } from "@/resources/enums";
import type { ChannelType } from "@/resources/types";

type FormValues = {
  channel_id: string;
  name: string;
  custom_name: string;
  twitter_id: string;
  kind: number;
};

type Props =
  | { mode: "create"; channel?: undefined; onSuccess: () => void }
  | { mode: "edit"; channel: ChannelType; onSuccess: () => void };

export function ChannelForm({ mode, channel, onSuccess }: Props) {
  const { create, update } = useAdminChannelActions();
  const { addAlert } = useAlerts();

  const {
    register,
    handleSubmit,
    formState: { errors, isSubmitting },
  } = useForm<FormValues>({
    defaultValues:
      mode === "edit"
        ? {
            channel_id: channel.channel_id,
            name: channel.name ?? "",
            custom_name: channel.custom_name,
            twitter_id: channel.twitter_id ?? "",
            kind: channel.kind,
          }
        : { kind: ChannelKind.HIDDEN },
  });

  async function onSubmit(values: FormValues) {
    try {
      if (mode === "create") {
        await create({
          channel_id: values.channel_id,
          name: values.name || undefined,
          custom_name: values.custom_name,
          twitter_id: values.twitter_id || undefined,
          kind: Number(values.kind),
        });
        addAlert("success", "チャンネルを作成しました");
      } else {
        await update(channel.id, {
          name: values.name || null,
          custom_name: values.custom_name,
          twitter_id: values.twitter_id || null,
          kind: Number(values.kind),
        });
        addAlert("success", "チャンネルを更新しました");
      }
      onSuccess();
    } catch (e) {
      addAlert("danger", e instanceof Error ? e.message : "保存に失敗しました");
    }
  }

  return (
    <form onSubmit={handleSubmit(onSubmit)} noValidate>
      <div className="mb-3">
        <label className="form-label small fw-semibold">
          YouTube Channel ID <span className="text-danger">*</span>
        </label>
        <input
          type="text"
          className={`form-control form-control-sm font-monospace ${errors.channel_id ? "is-invalid" : ""}`}
          readOnly={mode === "edit"}
          {...register("channel_id", { required: "必須です" })}
        />
        {errors.channel_id && (
          <div className="invalid-feedback">{errors.channel_id.message}</div>
        )}
      </div>

      <div className="row g-2 mb-3">
        <div className="col-12 col-sm-6">
          <label className="form-label small fw-semibold">チャンネル名</label>
          <input
            type="text"
            className="form-control form-control-sm"
            {...register("name")}
          />
        </div>
        <div className="col-12 col-sm-6">
          <label className="form-label small fw-semibold">
            カスタム名 <span className="text-danger">*</span>
          </label>
          <input
            type="text"
            className={`form-control form-control-sm ${errors.custom_name ? "is-invalid" : ""}`}
            {...register("custom_name", { required: "必須です" })}
          />
          {errors.custom_name && (
            <div className="invalid-feedback">{errors.custom_name.message}</div>
          )}
        </div>
      </div>

      <div className="row g-2 mb-3">
        <div className="col-12 col-sm-6">
          <label className="form-label small fw-semibold">Twitter ID</label>
          <input
            type="text"
            className="form-control form-control-sm"
            placeholder="例: AZKi_VDiVA"
            {...register("twitter_id")}
          />
        </div>
        <div className="col-12 col-sm-6">
          <label className="form-label small fw-semibold">公開設定</label>
          <select className="form-select form-select-sm" {...register("kind")}>
            <option value={ChannelKind.HIDDEN}>非公開</option>
            <option value={ChannelKind.PUBLISHED}>公開中</option>
          </select>
        </div>
      </div>

      <button
        type="submit"
        className="btn btn-sm btn-primary"
        disabled={isSubmitting}
      >
        {isSubmitting ? "保存中..." : mode === "create" ? "作成する" : "更新する"}
      </button>
    </form>
  );
}
