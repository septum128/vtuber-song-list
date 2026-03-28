import { useRef, useState } from "react";
import { useAlerts } from "@/context/AlertsProvider";
import { useAdminSongItemActions } from "@/hooks/useAdminSongItems";

type ParsedRow = {
  time: string;
  title: string;
  author: string;
};

type Props = {
  videoId: number;
  onSuccess: () => void;
};

function parseTsv(text: string): ParsedRow[] {
  return text
    .split("\n")
    .map((line) => line.trimEnd())
    .filter((line) => line.length > 0)
    .map((line) => {
      const cols = line.split("\t");
      return {
        time: cols[0]?.trim() ?? "",
        title: cols[1]?.trim() ?? "",
        author: cols[2]?.trim() ?? "",
      };
    });
}

export function ImportSongItemForm({ videoId, onSuccess }: Props) {
  const { bulkCreate } = useAdminSongItemActions(videoId);
  const { addAlert } = useAlerts();
  const fileInputRef = useRef<HTMLInputElement>(null);
  const [rows, setRows] = useState<ParsedRow[]>([]);
  const [isSubmitting, setIsSubmitting] = useState(false);

  function handleFileChange(e: React.ChangeEvent<HTMLInputElement>) {
    const file = e.target.files?.[0];
    if (!file) return;

    const reader = new FileReader();
    reader.onload = (ev) => {
      const text = ev.target?.result;
      if (typeof text === "string") {
        setRows(parseTsv(text));
      }
    };
    reader.readAsText(file, "UTF-8");
  }

  function handleClear() {
    setRows([]);
    if (fileInputRef.current) fileInputRef.current.value = "";
  }

  async function handleImport() {
    if (rows.length === 0) return;
    setIsSubmitting(true);
    try {
      const items = rows.map((r) => ({
        time: r.time || undefined,
        title: r.title || undefined,
        author: r.author || undefined,
      }));
      const result = await bulkCreate({ video_id: videoId, items });
      addAlert("success", `${result.created}件のセトリをインポートしました`);
      handleClear();
      onSuccess();
    } catch (e) {
      addAlert("danger", e instanceof Error ? e.message : "インポートに失敗しました");
    } finally {
      setIsSubmitting(false);
    }
  }

  return (
    <div>
      <div className="mb-3">
        <label className="form-label small fw-semibold">
          TSVファイル
          <span className="text-body-secondary fw-normal ms-2">
            （列順: 時間 / 曲名 / アーティスト）
          </span>
        </label>
        <input
          ref={fileInputRef}
          type="file"
          className="form-control form-control-sm"
          accept=".tsv,.txt"
          onChange={handleFileChange}
        />
      </div>

      {rows.length > 0 && (
        <>
          <p className="small text-body-secondary mb-1">{rows.length}件を読み込みました</p>
          <div className="table-responsive mb-3" style={{ maxHeight: "300px", overflowY: "auto" }}>
            <table className="table table-sm table-bordered small mb-0">
              <thead className="table-light sticky-top">
                <tr>
                  <th>時間</th>
                  <th>曲名</th>
                  <th>アーティスト</th>
                </tr>
              </thead>
              <tbody>
                {rows.map((row, i) => (
                  <tr key={i}>
                    <td className="font-monospace">{row.time || <span className="text-body-secondary">—</span>}</td>
                    <td>{row.title || <span className="text-body-secondary">—</span>}</td>
                    <td>{row.author || <span className="text-body-secondary">—</span>}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
          <div className="d-flex gap-2">
            <button
              type="button"
              className="btn btn-sm btn-primary"
              onClick={() => void handleImport()}
              disabled={isSubmitting}
            >
              {isSubmitting ? "インポート中..." : `${rows.length}件をインポート`}
            </button>
            <button
              type="button"
              className="btn btn-sm btn-outline-secondary"
              onClick={handleClear}
              disabled={isSubmitting}
            >
              クリア
            </button>
          </div>
        </>
      )}
    </div>
  );
}
