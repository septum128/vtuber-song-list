import { useRouter } from "next/router";
import { AdminLayout } from "@/layouts/AdminLayout";
import { ChannelForm } from "@/containers/Admin/ChannelForm";
import { Head } from "@/components/Common/Head";

export default function AdminChannelNewPage() {
  const router = useRouter();

  return (
    <AdminLayout>
      <Head title="チャンネル新規作成 - 管理" />
      <h2 className="h5 mb-3">チャンネル新規作成</h2>
      <ChannelForm mode="create" onSuccess={() => void router.push("/admin/channels")} />
    </AdminLayout>
  );
}
