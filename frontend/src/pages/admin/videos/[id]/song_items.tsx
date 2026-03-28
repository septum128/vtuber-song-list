import { useRouter } from "next/router";
import { AdminLayout } from "@/layouts/AdminLayout";
import { SongItemList } from "@/containers/Admin/SongItemList";
import { Head } from "@/components/Common/Head";

export default function AdminSongItemsPage() {
  const { query } = useRouter();
  const videoId = query.id ? Number(query.id) : null;
  const videoTitle =
    typeof query.title === "string" ? query.title : `動画 ID: ${videoId}`;

  if (!videoId) return null;

  return (
    <AdminLayout>
      <Head title="セトリ管理 - 管理" />
      <SongItemList videoId={videoId} videoTitle={videoTitle} />
    </AdminLayout>
  );
}
