const TOKEN_KEY = "auth_token";
const ADMIN_VIDEO_LIST_PER_PAGE_KEY = "admin_video_list_per_page";
const ADMIN_VIDEO_LIST_ONLY_SONG_LIVES_KEY = "admin_video_list_only_song_lives";

export function getToken(): string | null {
  if (typeof window === "undefined") return null;
  return localStorage.getItem(TOKEN_KEY);
}

export function setToken(token: string): void {
  localStorage.setItem(TOKEN_KEY, token);
}

export function removeToken(): void {
  localStorage.removeItem(TOKEN_KEY);
}

export function getAdminVideoListPerPage(): number | null {
  if (typeof window === "undefined") return null;
  const value = localStorage.getItem(ADMIN_VIDEO_LIST_PER_PAGE_KEY);
  return value ? Number(value) : null;
}

export function setAdminVideoListPerPage(perPage: number): void {
  localStorage.setItem(ADMIN_VIDEO_LIST_PER_PAGE_KEY, String(perPage));
}

export function getAdminVideoListOnlySongLives(): boolean {
  if (typeof window === "undefined") return false;
  return localStorage.getItem(ADMIN_VIDEO_LIST_ONLY_SONG_LIVES_KEY) === "true";
}

export function setAdminVideoListOnlySongLives(value: boolean): void {
  localStorage.setItem(ADMIN_VIDEO_LIST_ONLY_SONG_LIVES_KEY, String(value));
}
