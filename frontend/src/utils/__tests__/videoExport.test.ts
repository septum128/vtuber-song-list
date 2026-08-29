import {
  buildVideoListText,
  copyText,
  downloadTextFile,
  exportFilename,
  videoWatchUrl,
} from "../videoExport";

describe("videoWatchUrl", () => {
  it("builds a YouTube watch URL from a video id", () => {
    expect(videoWatchUrl("abc123")).toBe("https://www.youtube.com/watch?v=abc123");
  });
});

describe("buildVideoListText", () => {
  it("formats each video as 'title<TAB>URL' joined by newlines", () => {
    const text = buildVideoListText([
      { title: "【歌枠】まったり弾き語り", video_id: "abc123" },
      { title: "第2回 深夜の歌枠", video_id: "def456" },
    ]);
    expect(text).toBe(
      "【歌枠】まったり弾き語り\thttps://www.youtube.com/watch?v=abc123\n" +
        "第2回 深夜の歌枠\thttps://www.youtube.com/watch?v=def456"
    );
  });

  it("returns an empty string for an empty list", () => {
    expect(buildVideoListText([])).toBe("");
  });

  it("collapses tabs and newlines inside the title", () => {
    const text = buildVideoListText([
      { title: "歌枠\tテスト\n2部", video_id: "xyz" },
    ]);
    expect(text).toBe("歌枠 テスト 2部\thttps://www.youtube.com/watch?v=xyz");
  });
});

describe("exportFilename", () => {
  it("uses a timestamped .txt name", () => {
    expect(exportFilename(new Date("2026-08-29T09:05:07"))).toBe(
      "videos_20260829_090507.txt"
    );
  });
});

describe("downloadTextFile", () => {
  const originalCreate = URL.createObjectURL;
  const originalRevoke = URL.revokeObjectURL;

  beforeEach(() => {
    URL.createObjectURL = jest.fn(() => "blob:mock");
    URL.revokeObjectURL = jest.fn();
  });

  afterEach(() => {
    URL.createObjectURL = originalCreate;
    URL.revokeObjectURL = originalRevoke;
    jest.restoreAllMocks();
  });

  it("creates a temporary anchor, clicks it, and revokes the object URL", () => {
    const click = jest
      .spyOn(HTMLAnchorElement.prototype, "click")
      .mockImplementation(() => {});

    downloadTextFile("videos.txt", "hello");

    expect(URL.createObjectURL).toHaveBeenCalledTimes(1);
    expect(click).toHaveBeenCalledTimes(1);
    expect(URL.revokeObjectURL).toHaveBeenCalledWith("blob:mock");
    expect(document.querySelector("a")).toBeNull();
  });
});

describe("copyText", () => {
  afterEach(() => {
    jest.restoreAllMocks();
  });

  it("uses the Clipboard API when available", async () => {
    const writeText = jest.fn().mockResolvedValue(undefined);
    Object.assign(navigator, { clipboard: { writeText } });

    await copyText("payload");

    expect(writeText).toHaveBeenCalledWith("payload");
  });

  it("falls back to execCommand when Clipboard API is missing", async () => {
    Object.assign(navigator, { clipboard: undefined });
    const exec = jest.fn(() => true);
    (document as unknown as { execCommand: unknown }).execCommand = exec;

    await copyText("payload");

    expect(exec).toHaveBeenCalledWith("copy");
  });

  it("throws when the execCommand fallback fails", async () => {
    Object.assign(navigator, { clipboard: undefined });
    (document as unknown as { execCommand: unknown }).execCommand = jest.fn(() => false);

    await expect(copyText("payload")).rejects.toThrow();
  });
});
