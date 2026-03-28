import { useRouter } from "next/router";
import { Head } from "@/components/Common/Head";
import { DefaultLayout } from "@/layouts/DefaultLayout";
import { ChannelDetail } from "@/containers/Channels/ChannelDetail";
import { Loading } from "@/components/Common/Loading";

export default function ChannelDetailPage() {
  const router = useRouter();
  const { id } = router.query;
  const channelId = typeof id === "string" ? Number(id) : null;

  return (
    <DefaultLayout>
      <Head title="チャンネル詳細" />
      {channelId != null && Number.isFinite(channelId) ? (
        <ChannelDetail channelId={channelId} />
      ) : (
        <Loading />
      )}
    </DefaultLayout>
  );
}
