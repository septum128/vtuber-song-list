import { render, screen } from "@testing-library/react";
import { SongDiffList } from "../SongLists/SongDiffList";
import type { SongDiffType } from "@/resources/types";
import { SongDiffStatus, SongDiffKind } from "@/resources/enums";

const makeDiff = (overrides: Partial<SongDiffType> = {}): SongDiffType => ({
  id: 1,
  song_item_id: 10,
  made_by_id: null,
  time: "01:23:45",
  title: "Test Song",
  author: "Test Artist",
  status: SongDiffStatus.PENDING,
  kind: SongDiffKind.MANUAL,
  created_at: "2026-01-01T00:00:00Z",
  ...overrides,
});

describe("SongDiffList", () => {
  it("shows empty message when diffs is empty", () => {
    render(<SongDiffList diffs={[]} />);
    expect(screen.getByText("修正履歴はありません。")).toBeInTheDocument();
  });

  it("renders a row for each diff", () => {
    const diffs = [makeDiff({ id: 1 }), makeDiff({ id: 2, title: "Second Song" })];
    render(<SongDiffList diffs={diffs} />);
    expect(screen.getByText("Test Song")).toBeInTheDocument();
    expect(screen.getByText("Second Song")).toBeInTheDocument();
  });

  it("shows '承認待ち' badge for pending status", () => {
    render(<SongDiffList diffs={[makeDiff({ status: SongDiffStatus.PENDING })]} />);
    expect(screen.getByText("承認待ち")).toBeInTheDocument();
  });

  it("shows '承認済み' badge for approved status", () => {
    render(<SongDiffList diffs={[makeDiff({ status: SongDiffStatus.APPROVED })]} />);
    expect(screen.getByText("承認済み")).toBeInTheDocument();
  });

  it("shows '却下' badge for rejected status", () => {
    render(<SongDiffList diffs={[makeDiff({ status: SongDiffStatus.REJECTED })]} />);
    expect(screen.getByText("却下")).toBeInTheDocument();
  });

  it("shows '手動' badge for manual kind", () => {
    render(<SongDiffList diffs={[makeDiff({ kind: SongDiffKind.MANUAL })]} />);
    expect(screen.getByText("手動")).toBeInTheDocument();
  });

  it("shows 'AI自動' badge for auto kind", () => {
    render(<SongDiffList diffs={[makeDiff({ kind: SongDiffKind.AUTO })]} />);
    expect(screen.getByText("AI自動")).toBeInTheDocument();
  });

  it("shows — when time is null", () => {
    render(<SongDiffList diffs={[makeDiff({ time: null })]} />);
    // The time cell should show em dash
    const cells = screen.getAllByRole("cell");
    expect(cells[0].textContent).toBe("—");
  });
});
