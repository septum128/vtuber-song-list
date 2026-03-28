import { useState } from "react";
import Link from "next/link";
import { useAdminChannels, useAdminChannelActions } from "@/hooks/useAdminChannels";
import { useAlerts } from "@/context/AlertsProvider";
import { Loading } from "@/components/Common/Loading";
import { Modal } from "@/components/Common/Modal";
import { ChannelForm } from "./ChannelForm";
import { ChannelKind } from "@/resources/enums";
import type { ChannelType } from "@/resources/types";

const KIND_CONFIG: Record<number, { label: string; variant: string }> = {
  [ChannelKind.HIDDEN]: { label: "非公開", variant: "secondary" },
  [ChannelKind.PUBLISHED]: { label: "公開中", variant: "success" },
};

export function ChannelList() {
  const { data: channels, isLoading } = useAdminChannels();
  const { remove } = useAdminChannelActions();
  const { addAlert } = useAlerts();
  const [editTarget, setEditTarget] = useState<ChannelType | null>(null);

  async function handleDelete(channel: ChannelType) {
    if (!confirm(`「${channel.custom_name}」を削除しますか？`)) return;
    try {
      await remove(channel.id);
      addAlert("success", "削除しました");
    } catch (e) {
      addAlert("danger", e instanceof Error ? e.message : "削除に失敗しました");
    }
  }

  if (isLoading) return <Loading />;

  return (
    <div>
      <div className="d-flex align-items-center gap-3 mb-3">
        <h2 className="h5 mb-0">チャンネル管理</h2>
        <Link href="/admin/channels/new" className="btn btn-sm btn-primary ms-auto">
          + 新規作成
        </Link>
      </div>

      {!channels || channels.length === 0 ? (
        <p className="text-body-secondary small">チャンネルがありません。</p>
      ) : (
        <div className="table-responsive">
          <table className="table table-sm small">
            <thead className="table-light">
              <tr>
                <th>ID</th>
                <th>Channel ID</th>
                <th>カスタム名</th>
                <th>Twitter</th>
                <th>公開設定</th>
                <th>操作</th>
              </tr>
            </thead>
            <tbody>
              {channels.map((channel) => {
                const kind = KIND_CONFIG[channel.kind];
                return (
                  <tr key={channel.id}>
                    <td>{channel.id}</td>
                    <td className="font-monospace">{channel.channel_id}</td>
                    <td>{channel.custom_name}</td>
                    <td>{channel.twitter_id ?? "—"}</td>
                    <td>
                      {kind && (
                        <span className={`badge bg-${kind.variant}`}>{kind.label}</span>
                      )}
                    </td>
                    <td>
                      <div className="d-flex gap-1">
                        <button
                          type="button"
                          className="btn btn-xs btn-sm btn-outline-secondary"
                          onClick={() => setEditTarget(channel)}
                        >
                          編集
                        </button>
                        <Link
                          href={`/admin/videos?channel_id=${channel.id}`}
                          className="btn btn-xs btn-sm btn-outline-secondary"
                        >
                          動画
                        </Link>
                        <button
                          type="button"
                          className="btn btn-xs btn-sm btn-outline-danger"
                          onClick={() => void handleDelete(channel)}
                        >
                          削除
                        </button>
                      </div>
                    </td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        </div>
      )}

      <Modal
        show={editTarget !== null}
        onClose={() => setEditTarget(null)}
        title="チャンネル編集"
      >
        {editTarget && (
          <ChannelForm
            mode="edit"
            channel={editTarget}
            onSuccess={() => setEditTarget(null)}
          />
        )}
      </Modal>
    </div>
  );
}
