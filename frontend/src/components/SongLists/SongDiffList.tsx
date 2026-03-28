import { format } from "date-fns";
import { ja } from "date-fns/locale";
import type { SongDiffType } from "@/resources/types";
import { SongDiffStatus, SongDiffKind } from "@/resources/enums";

type Props = {
  diffs: SongDiffType[];
};

const STATUS_CONFIG: Record<number, { label: string; variant: string }> = {
  [SongDiffStatus.PENDING]:  { label: "承認待ち", variant: "warning" },
  [SongDiffStatus.APPROVED]: { label: "承認済み", variant: "success" },
  [SongDiffStatus.REJECTED]: { label: "却下",     variant: "danger"  },
};

const KIND_CONFIG: Record<number, { label: string; variant: string }> = {
  [SongDiffKind.MANUAL]: { label: "手動",    variant: "secondary" },
  [SongDiffKind.AUTO]:   { label: "AI自動",  variant: "info"      },
};

export function SongDiffList({ diffs }: Props) {
  if (diffs.length === 0) {
    return <p className="text-body-secondary small">修正履歴はありません。</p>;
  }

  return (
    <div className="table-responsive">
      <table className="table table-sm small">
        <thead className="table-light">
          <tr>
            <th>時間</th>
            <th>曲名</th>
            <th>アーティスト</th>
            <th>ステータス</th>
            <th>種別</th>
            <th>投稿日</th>
          </tr>
        </thead>
        <tbody>
          {diffs.map((diff) => {
            const status = STATUS_CONFIG[diff.status];
            const kind = KIND_CONFIG[diff.kind];
            const createdAt = format(new Date(diff.created_at), "yyyy/MM/dd HH:mm", {
              locale: ja,
            });
            return (
              <tr key={diff.id}>
                <td className="font-monospace">{diff.time ?? "—"}</td>
                <td>{diff.title ?? "—"}</td>
                <td>{diff.author ?? "—"}</td>
                <td>
                  {status && (
                    <span className={`badge bg-${status.variant}`}>
                      {status.label}
                    </span>
                  )}
                </td>
                <td>
                  {kind && (
                    <span className={`badge bg-${kind.variant}`}>
                      {kind.label}
                    </span>
                  )}
                </td>
                <td className="text-body-secondary">{createdAt}</td>
              </tr>
            );
          })}
        </tbody>
      </table>
    </div>
  );
}
