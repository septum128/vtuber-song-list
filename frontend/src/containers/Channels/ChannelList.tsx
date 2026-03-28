import { useChannels } from "@/hooks/useChannels";
import { ChannelCard } from "@/components/SongLists/ChannelCard";
import { Loading } from "@/components/Common/Loading";

export function ChannelList() {
  const { data: channels, isLoading, error } = useChannels();

  if (isLoading) return <Loading />;
  if (error) {
    return (
      <div className="alert alert-danger">
        チャンネルの取得に失敗しました: {error.message}
      </div>
    );
  }
  if (!channels || channels.length === 0) {
    return <p className="text-body-secondary">チャンネルが見つかりませんでした。</p>;
  }

  return (
    <div className="row row-cols-1 row-cols-md-2 row-cols-lg-3 g-3">
      {channels.map((channel) => (
        <div key={channel.id} className="col">
          <ChannelCard channel={channel} />
        </div>
      ))}
    </div>
  );
}
