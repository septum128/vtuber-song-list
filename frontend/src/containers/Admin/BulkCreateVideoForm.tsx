import { useRef, useState } from "react";
import { useAlerts } from "@/context/AlertsProvider";
import { useAdminVideoActions, type BulkCreateResult } from "@/hooks/useAdminVideos";

type Props = {
  onSuccess: () => void;
};

export function BulkCreateVideoForm({ onSuccess }: Props) {
  const { bulkCreate } = useAdminVideoActions();
  const { addAlert } = useAlerts();
  const fileInputRef = useRef<HTMLInputElement>(null);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [result, setResult] = useState<BulkCreateResult | null>(null);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    const file = fileInputRef.current?.files?.[0];
    if (!file) {
      addAlert("danger", "ファイルを選択してください");
      return;
    }

    const tsv = await file.text();
    setIsSubmitting(true);
    try {
      const res = await bulkCreate(tsv);
      setResult(res);
      if (res.succeeded.length > 0) {
        addAlert(
          "success",
          `${res.succeeded.length}件登録しました（スキップ: ${res.skipped.length}件、失敗: ${res.failed.length}件）`
        );
        onSuccess();
      } else {
        addAlert("notice", `登録成功: 0件（スキップ: ${res.skipped.length}件、失敗: ${res.failed.length}件）`);
      }
    } catch (e) {
      addAlert("danger", e instanceof Error ? e.message : "一括登録に失敗しました");
    } finally {
      setIsSubmitting(false);
    }
  }

  return (
    <div>
      <p className="small text-body-secondary mb-3">
        1行目はヘッダー行（<code>チャンネル名{"\t"}URL</code>）として扱われます。
        チャンネル名はカスタム名と一致する必要があります。
      </p>
      <form onSubmit={handleSubmit} noValidate>
        <div className="mb-3">
          <label className="form-label small fw-semibold">TSVファイル</label>
          <input
            type="file"
            accept=".tsv,text/tab-separated-values,text/plain"
            className="form-control form-control-sm"
            ref={fileInputRef}
          />
        </div>
        <button
          type="submit"
          className="btn btn-sm btn-primary"
          disabled={isSubmitting}
        >
          {isSubmitting ? "登録中..." : "一括登録する"}
        </button>
      </form>

      {result && (
        <div className="mt-4">
          {result.succeeded.length > 0 && (
            <div className="mb-3">
              <p className="small fw-semibold text-success mb-1">
                成功 ({result.succeeded.length}件)
              </p>
              <ul className="list-unstyled small">
                {result.succeeded.map((item, i) => (
                  <li key={i} className="text-truncate text-success">
                    ✓ {item.detail}
                  </li>
                ))}
              </ul>
            </div>
          )}
          {result.skipped.length > 0 && (
            <div className="mb-3">
              <p className="small fw-semibold text-secondary mb-1">
                スキップ ({result.skipped.length}件)
              </p>
              <ul className="list-unstyled small">
                {result.skipped.map((item, i) => (
                  <li key={i} className="text-secondary text-truncate">
                    – {item.url}: {item.detail}
                  </li>
                ))}
              </ul>
            </div>
          )}
          {result.failed.length > 0 && (
            <div className="mb-3">
              <p className="small fw-semibold text-danger mb-1">
                失敗 ({result.failed.length}件)
              </p>
              <ul className="list-unstyled small">
                {result.failed.map((item, i) => (
                  <li key={i} className="text-danger text-truncate">
                    ✗ {item.url}: {item.detail}
                  </li>
                ))}
              </ul>
            </div>
          )}
        </div>
      )}
    </div>
  );
}
