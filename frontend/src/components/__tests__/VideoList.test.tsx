import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { VideoList } from "../SongLists/VideoList";

describe("VideoList", () => {
  it("shows empty message without pagination on an empty first page", () => {
    render(<VideoList videos={[]} page={1} perPage={30} onPageChange={jest.fn()} />);
    expect(screen.getByText("動画が見つかりませんでした。")).toBeInTheDocument();
    expect(screen.queryByText("前へ")).toBeNull();
  });

  it("keeps the prev button on an empty page past the first", async () => {
    const onPageChange = jest.fn();
    render(<VideoList videos={[]} page={2} perPage={30} onPageChange={onPageChange} />);
    expect(screen.getByText("動画が見つかりませんでした。")).toBeInTheDocument();
    await userEvent.click(screen.getByText("前へ"));
    expect(onPageChange).toHaveBeenCalledWith(1);
  });
});
