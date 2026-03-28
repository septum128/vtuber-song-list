import { useRouter } from "next/router";
import { useChannel } from "@/hooks/useChannels";
import { useSongItems } from "@/hooks/useSongItems";
import { useVideos } from "@/hooks/useVideos";
import { useAuth } from "@/hooks/useAuth";
import { useFavoriteIds } from "@/hooks/useFavorites";
import { SongSearchForm, type SongSearchValues } from "@/components/SongLists/SongSearchForm";
import { SongItemList } from "@/components/SongLists/SongItemList";
import { VideoList } from "@/components/SongLists/VideoList";
import { Loading } from "@/components/Common/Loading";

type Props = {
  channelId: number;
};

type Tab = "songs" | "videos";

const PER_PAGE = 30;

function parseNum(v: string | string[] | undefined, fallback: number): number {
  const n = Number(v);
  return Number.isFinite(n) && n > 0 ? n : fallback;
}

function parseStr(v: string | string[] | undefined): string {
  return typeof v === "string" ? v : "";
}

export function ChannelDetail({ channelId }: Props) {
  const router = useRouter();
  const { query } = router;
  const { user } = useAuth();
  const { favoriteIds } = useFavoriteIds();

  const tab: Tab = query.tab === "videos" ? "videos" : "songs";
  const songPage = parseNum(query.page as string, 1);
  const videoPage = parseNum(query.vpage as string, 1);
  const searchValues: SongSearchValues = {
    query: parseStr(query.q),
    since: parseStr(query.since),
    until: parseStr(query.until),
    video_title: parseStr(query.video_title),
  };

  const { data: channel, isLoading: channelLoading } = useChannel(channelId);
  const {
    data: songs,
    isLoading: songsLoading,
  } = useSongItems({
    channel_id: channelId,
    query: searchValues.query || undefined,
    since: searchValues.since || undefined,
    until: searchValues.until || undefined,
    video_title: searchValues.video_title || undefined,
    page: songPage,
    count: PER_PAGE,
  });
  const {
    data: videos,
    isLoading: videosLoading,
  } = useVideos({
    channel_id: channelId,
    only_song_lives: 1,
    page: videoPage,
    count: PER_PAGE,
  });

  function pushQuery(updates: Record<string, string | number | undefined>) {
    const next = { ...query, ...updates };
    // undefined の値を削除
    for (const key of Object.keys(next)) {
      if (next[key] === undefined || next[key] === "" || next[key] === 1) {
        if (key !== "id") delete next[key];
      }
    }
    void router.push({ pathname: router.pathname, query: next }, undefined, {
      shallow: true,
    });
  }

  function handleTabChange(newTab: Tab) {
    pushQuery({ tab: newTab === "songs" ? undefined : newTab });
  }

  function handleSongSearch(values: SongSearchValues) {
    pushQuery({
      q: values.query || undefined,
      since: values.since || undefined,
      until: values.until || undefined,
      video_title: values.video_title || undefined,
      page: undefined,
    });
  }

  function handleSongPageChange(page: number) {
    pushQuery({ page: page === 1 ? undefined : page });
  }

  function handleVideoPageChange(vpage: number) {
    pushQuery({ vpage: vpage === 1 ? undefined : vpage });
  }

  if (channelLoading) return <Loading />;
  if (!channel) {
    return <div className="alert alert-warning">チャンネルが見つかりませんでした。</div>;
  }

  const displayName = channel.custom_name || channel.name || channel.channel_id;

  return (
    <>
      <div className="d-flex align-items-center gap-3 mb-4 flex-wrap">
        <h1 className="h3 mb-0">{displayName}</h1>
        {channel.twitter_id && (
          <a
            href={`https://twitter.com/${channel.twitter_id}`}
            target="_blank"
            rel="noopener noreferrer"
            className="text-body-secondary small text-decoration-none"
          >
            @{channel.twitter_id}
          </a>
        )}
        <a
          href={`https://www.youtube.com/channel/${channel.channel_id}`}
          target="_blank"
          rel="noopener noreferrer"
          className="btn btn-sm btn-outline-danger ms-auto"
        >
          YouTubeチャンネル
        </a>
      </div>

      <ul className="nav nav-tabs mb-3">
        <li className="nav-item">
          <button
            type="button"
            className={`nav-link ${tab === "songs" ? "active" : ""}`}
            onClick={() => handleTabChange("songs")}
          >
            曲一覧
          </button>
        </li>
        <li className="nav-item">
          <button
            type="button"
            className={`nav-link ${tab === "videos" ? "active" : ""}`}
            onClick={() => handleTabChange("videos")}
          >
            動画一覧
          </button>
        </li>
      </ul>

      {tab === "songs" && (
        <>
          <SongSearchForm
            defaultValues={searchValues}
            onSearch={handleSongSearch}
          />
          {songsLoading ? (
            <Loading />
          ) : (
            <SongItemList
              items={songs ?? []}
              page={songPage}
              perPage={PER_PAGE}
              onPageChange={handleSongPageChange}
              favoriteIds={favoriteIds}
              showFavorite={!!user}
              showEdit={!!user}
            />
          )}
        </>
      )}

      {tab === "videos" && (
        <>
          {videosLoading ? (
            <Loading />
          ) : (
            <VideoList
              videos={videos ?? []}
              page={videoPage}
              perPage={PER_PAGE}
              onPageChange={handleVideoPageChange}
            />
          )}
        </>
      )}
    </>
  );
}
