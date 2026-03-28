import Image from "next/image";
import Link from "next/link";
import type { ChannelType } from "@/resources/types";

type Props = {
  channel: ChannelType;
};

function YouTubeIcon() {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 24 24"
      width="20"
      height="20"
      aria-label="YouTube"
    >
      <path
        fill="#FF0000"
        d="M23.5 6.2a3 3 0 0 0-2.1-2.1C19.5 3.5 12 3.5 12 3.5s-7.5 0-9.4.6A3 3 0 0 0 .5 6.2 31.5 31.5 0 0 0 0 12a31.5 31.5 0 0 0 .5 5.8 3 3 0 0 0 2.1 2.1c1.9.6 9.4.6 9.4.6s7.5 0 9.4-.6a3 3 0 0 0 2.1-2.1A31.5 31.5 0 0 0 24 12a31.5 31.5 0 0 0-.5-5.8z"
      />
      <path fill="#FFFFFF" d="M9.75 15.5 15.5 12 9.75 8.5v7z" />
    </svg>
  );
}

export function ChannelCard({ channel }: Props) {
  const displayName = channel.custom_name || channel.name || channel.channel_id;

  return (
    <div className="card h-100">
      <div className="card-body d-flex flex-column">
        <div className="d-flex align-items-center justify-content-between mb-2">
          <h5 className="card-title mb-0 d-flex align-items-center gap-2">
            <span>{displayName}</span>
            <a
              href={`https://www.youtube.com/channel/${channel.channel_id}`}
              target="_blank"
              rel="noopener noreferrer"
              aria-label={`${displayName}のYouTubeチャンネル`}
            >
              <YouTubeIcon />
            </a>
          </h5>
          {channel.icon_url && (
            <Image
              src={channel.icon_url}
              alt={displayName}
              width={48}
              height={48}
              className="rounded-circle"
            />
          )}
        </div>
        {channel.twitter_id && (
          <p className="card-text small text-body-secondary mb-2">
            @{channel.twitter_id}
          </p>
        )}
        <div className="mt-auto d-flex gap-2 flex-wrap">
          <Link
            href={`/channels/${channel.id}`}
            className="btn btn-sm btn-primary"
          >
            曲・動画を見る
          </Link>
        </div>
      </div>
    </div>
  );
}
