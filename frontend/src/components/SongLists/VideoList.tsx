import type { VideoType } from "@/resources/types";
import { VideoCard } from "./VideoCard";
import { Pagination } from "./Pagination";

type Props = {
  videos: VideoType[];
  page: number;
  perPage: number;
  onPageChange: (page: number) => void;
};

export function VideoList({ videos, page, perPage, onPageChange }: Props) {
  if (videos.length === 0) {
    return <p className="text-body-secondary">動画が見つかりませんでした。</p>;
  }

  return (
    <>
      <div className="row row-cols-1 row-cols-md-2 row-cols-lg-3 g-3">
        {videos.map((video) => (
          <div key={video.id} className="col">
            <VideoCard video={video} />
          </div>
        ))}
      </div>
      <Pagination
        page={page}
        perPage={perPage}
        itemCount={videos.length}
        onPageChange={onPageChange}
      />
    </>
  );
}
