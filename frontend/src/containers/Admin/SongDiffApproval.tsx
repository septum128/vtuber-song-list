import { useState } from "react";
import { format } from "date-fns";
import { ja } from "date-fns/locale";
import { useAdminSongDiffs, useAdminSongDiffActions } from "@/hooks/useAdminSongDiffs";
import { useAlerts } from "@/context/AlertsProvider";
import { Loading } from "@/components/Common/Loading";
import { Pagination } from "@/components/SongLists/Pagination";
import { SongDiffStatus } from "@/resources/enums";
import type { SongDiffType } from "@/resources/types";

const STATUS_LABEL: Record<number, { label: string; variant: string }> = {
  [SongDiffStatus.PENDING]: { label: "承認待ち", variant: "warning" },
  [SongDiffStatus.APPROVED]: { label: "承認済み", variant: "success" },
  [SongDiffStatus.REJECTED]: { label: "却下", variant: "danger" },
};

const PER_PAGE = 30;

type ActionState = { id: number; type: "approve" | "reject" } | null;

function DiffRow({
  diff,
  onApprove,
  onReject,
  busy,
}: {
  diff: SongDiffType;
  onApprove: () => void;
  onReject: () => void;
  busy: boolean;
}) {
  const status = STATUS_LABEL[diff.status];
  const createdAt = format(new Date(diff.created_at), "MM/dd HH:mm", { locale: ja });
  const isPending = diff.status === SongDiffStatus.PENDING;

  return (
    <tr>
      <td className="small text-body-secondary">{createdAt}</td>
      <td className="small">{diff.song_item_id}</td>
      <td className="font-monospace small">{diff.time ?? "—"}</td>
      <td className="small">{diff.title ?? "—"}</td>
      <td className="small">{diff.author ?? "—"}</td>
      <td>
        {status && (
          <span className={`badge bg-${status.variant}`}>{status.label}</span>
        )}
      </td>
      <td>
        {isPending && (
          <div className="d-flex gap-1">
            <button
              type="button"
              className="btn btn-xs btn-success btn-sm"
              onClick={onApprove}
              disabled={busy}
            >
              承認
            </button>
            <button
              type="button"
              className="btn btn-xs btn-danger btn-sm"
              onClick={onReject}
              disabled={busy}
            >
              却下
            </button>
          </div>
        )}
      </td>
    </tr>
  );
}

export function SongDiffApproval() {
  const [page, setPage] = useState(1);
  const [statusFilter, setStatusFilter] = useState<number | undefined>(
    SongDiffStatus.PENDING
  );
  const [actionState, setActionState] = useState<ActionState>(null);
  const { data: diffs, isLoading } = useAdminSongDiffs(statusFilter, page);
  const { approve, reject } = useAdminSongDiffActions();
  const { addAlert } = useAlerts();

  async function handleApprove(id: number) {
    setActionState({ id, type: "approve" });
    try {
      await approve(id);
      addAlert("success", "承認しました");
    } catch (e) {
      addAlert("danger", e instanceof Error ? e.message : "承認に失敗しました");
    } finally {
      setActionState(null);
    }
  }

  async function handleReject(id: number) {
    setActionState({ id, type: "reject" });
    try {
      await reject(id);
      addAlert("success", "却下しました");
    } catch (e) {
      addAlert("danger", e instanceof Error ? e.message : "却下に失敗しました");
    } finally {
      setActionState(null);
    }
  }

  return (
    <div>
      <div className="d-flex align-items-center gap-3 mb-3 flex-wrap">
        <h2 className="h5 mb-0">修正承認</h2>
        <div className="d-flex gap-2">
          {[
            { label: "承認待ち", value: SongDiffStatus.PENDING },
            { label: "承認済み", value: SongDiffStatus.APPROVED },
            { label: "却下", value: SongDiffStatus.REJECTED },
            { label: "すべて", value: undefined },
          ].map((opt) => (
            <button
              key={String(opt.value)}
              type="button"
              className={`btn btn-sm ${
                statusFilter === opt.value ? "btn-secondary" : "btn-outline-secondary"
              }`}
              onClick={() => {
                setStatusFilter(opt.value);
                setPage(1);
              }}
            >
              {opt.label}
            </button>
          ))}
        </div>
      </div>

      {isLoading ? (
        <Loading />
      ) : !diffs || diffs.length === 0 ? (
        <p className="text-body-secondary small">修正はありません。</p>
      ) : (
        <>
          <div className="table-responsive">
            <table className="table table-sm small">
              <thead className="table-light">
                <tr>
                  <th>投稿日</th>
                  <th>曲ID</th>
                  <th>時間</th>
                  <th>曲名</th>
                  <th>アーティスト</th>
                  <th>ステータス</th>
                  <th>操作</th>
                </tr>
              </thead>
              <tbody>
                {diffs.map((diff) => (
                  <DiffRow
                    key={diff.id}
                    diff={diff}
                    onApprove={() => void handleApprove(diff.id)}
                    onReject={() => void handleReject(diff.id)}
                    busy={actionState?.id === diff.id}
                  />
                ))}
              </tbody>
            </table>
          </div>
          <Pagination
            page={page}
            perPage={PER_PAGE}
            itemCount={diffs.length}
            onPageChange={setPage}
          />
        </>
      )}
    </div>
  );
}
