export type ChannelType = {
  id: number;
  channel_id: string;
  name: string | null;
  custom_name: string;
  twitter_id: string | null;
  icon_url: string | null;
  kind: number;
  status: number;
};

export type VideoType = {
  id: number;
  channel_id: number;
  video_id: string;
  title: string;
  kind: number;
  status: number;
  published: boolean;
  published_at: string;
};

export type SongItemType = {
  id: number;
  video_id: number;
  latest_diff_id: number | null;
  diff: {
    title: string | null;
    author: string | null;
    time: string | null;
  };
  video: {
    id: number;
    video_id: string;
    title: string;
    channel_id: number;
    channel_custom_name: string;
    kind: number;
    published_at: string;
  };
};

export type SongDiffType = {
  id: number;
  song_item_id: number;
  made_by_id: number | null;
  time: string | null;
  title: string | null;
  author: string | null;
  status: number;
  kind: number;
  created_at: string;
};

export type UserType = {
  id: number;
  name: string;
  kind: number;
};

export type ApiErrorResponse = { message?: string; description?: string };

export type AuthResponse = {
  message: string;
  user: UserType;
  token: string;
};

export type VideosQuery = {
  channel_id?: number;
  query?: string;
  since?: string;
  until?: string;
  only_song_lives?: number;
  page?: number;
  count?: number;
};

export type SongItemsQuery = {
  channel_id?: number;
  video_id?: number;
  query?: string;
  since?: string;
  until?: string;
  video_title?: string;
  page?: number;
  count?: number;
};
