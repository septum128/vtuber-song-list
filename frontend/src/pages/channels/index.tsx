import { Head } from "@/components/Common/Head";
import { DefaultLayout } from "@/layouts/DefaultLayout";
import { ChannelList } from "@/containers/Channels/ChannelList";

export default function ChannelsPage() {
  return (
    <DefaultLayout>
      <Head title="チャンネル一覧" />
      <h1 className="h3 mb-4">チャンネル一覧</h1>
      <ChannelList />
    </DefaultLayout>
  );
}
