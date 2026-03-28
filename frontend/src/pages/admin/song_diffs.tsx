import { AdminLayout } from "@/layouts/AdminLayout";
import { SongDiffApproval } from "@/containers/Admin/SongDiffApproval";
import { Head } from "@/components/Common/Head";

export default function AdminSongDiffsPage() {
  return (
    <AdminLayout>
      <Head title="修正承認 - 管理" />
      <SongDiffApproval />
    </AdminLayout>
  );
}
