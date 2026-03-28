export const UserKind = {
  MEMBER: 0,
  BANNED: 5,
  ADMIN: 10,
} as const;

export const ChannelKind = {
  HIDDEN: 0,
  PUBLISHED: 100,
} as const;

export const VideoKind = {
  VIDEO: 0,
  LIVE: 10,
  SHORT: 20,
} as const;

export const VideoStatus = {
  READY: 0,
  FETCHED: 10,
  SONG_ITEMS_CREATED: 20,
  FETCHED_HISTORY: 25,
  SPOTIFY_FETCHED: 30,
  SPOTIFY_COMPLETED: 35,
  COMPLETED: 40,
} as const;

export const SongDiffStatus = {
  PENDING: 0,
  APPROVED: 10,
  REJECTED: 20,
} as const;

export const SongDiffKind = {
  MANUAL: 0,
  AUTO: 10,
} as const;
