import { format } from "date-fns";
import type { VideoType } from "@/resources/types";

export const YOUTUBE_WATCH_URL = "https://www.youtube.com/watch?v=";

type VideoLike = Pick<VideoType, "title" | "video_id">;

/** タイトル内のタブ・改行を空白に潰し、1行1動画の TSV を壊さないようにする。 */
function sanitizeTitle(title: string): string {
  return title.replace(/[\t\r\n]+/g, " ").trim();
}

export function videoWatchUrl(videoId: string): string {
  return `${YOUTUBE_WATCH_URL}${videoId}`;
}

/** 選択動画を "タイトル<TAB>URL" の行に整形して連結する（末尾改行なし）。 */
export function buildVideoListText(videos: VideoLike[]): string {
  return videos
    .map((v) => `${sanitizeTitle(v.title)}\t${videoWatchUrl(v.video_id)}`)
    .join("\n");
}

export function exportFilename(now: Date = new Date()): string {
  return `videos_${format(now, "yyyyMMdd_HHmmss")}.txt`;
}

/** テキストを Blob 化して一時リンク経由でダウンロードさせる。 */
export function downloadTextFile(filename: string, content: string): void {
  const blob = new Blob([content], { type: "text/plain;charset=utf-8" });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = filename;
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
  URL.revokeObjectURL(url);
}

/** クリップボードへコピー。Clipboard API 未対応環境では execCommand にフォールバック。 */
export async function copyText(text: string): Promise<void> {
  if (navigator.clipboard?.writeText) {
    await navigator.clipboard.writeText(text);
    return;
  }

  const textarea = document.createElement("textarea");
  textarea.value = text;
  textarea.setAttribute("readonly", "");
  textarea.style.position = "absolute";
  textarea.style.left = "-9999px";
  document.body.appendChild(textarea);
  textarea.select();
  const ok = document.execCommand("copy");
  document.body.removeChild(textarea);
  if (!ok) {
    throw new Error("クリップボードへのコピーに失敗しました");
  }
}
