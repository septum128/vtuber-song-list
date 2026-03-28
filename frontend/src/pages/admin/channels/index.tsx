import { AdminLayout } from "@/layouts/AdminLayout";
import { ChannelList } from "@/containers/Admin/ChannelList";
import { Head } from "@/components/Common/Head";

export default function AdminChannelsPage() {
  return (
    <AdminLayout>
      <Head title="チャンネル管理 - 管理" />
      <ChannelList />
    </AdminLayout>
  );
}
