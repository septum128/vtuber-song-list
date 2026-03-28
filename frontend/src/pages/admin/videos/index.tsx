import { useRouter } from "next/router";
import { AdminLayout } from "@/layouts/AdminLayout";
import { VideoList } from "@/containers/Admin/VideoList";
import { Head } from "@/components/Common/Head";

export default function AdminVideosPage() {
  const { query } = useRouter();
  const channelId = query.channel_id ? Number(query.channel_id) : undefined;

  return (
    <AdminLayout>
      <Head title="動画管理 - 管理" />
      <VideoList initialChannelId={channelId} />
    </AdminLayout>
  );
}
