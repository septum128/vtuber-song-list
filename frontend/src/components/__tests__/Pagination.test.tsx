import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { Pagination } from "../SongLists/Pagination";

describe("Pagination", () => {
  it("renders nothing when there is only one page and no more items", () => {
    const { container } = render(
      <Pagination page={1} perPage={30} itemCount={10} onPageChange={jest.fn()} />
    );
    expect(container.firstChild).toBeNull();
  });

  it("shows next button when itemCount >= perPage", () => {
    render(
      <Pagination page={1} perPage={30} itemCount={30} onPageChange={jest.fn()} />
    );
    expect(screen.getByText("次へ")).not.toBeDisabled();
  });

  it("does not show next button when itemCount < perPage", () => {
    render(
      <Pagination page={1} perPage={30} itemCount={29} onPageChange={jest.fn()} />
    );
    // component returns null when no prev and no next
    expect(screen.queryByText("次へ")).toBeNull();
  });

  it("shows prev button when page > 1", () => {
    render(
      <Pagination page={2} perPage={30} itemCount={10} onPageChange={jest.fn()} />
    );
    expect(screen.getByText("前へ")).not.toBeDisabled();
  });

  it("calls onPageChange with page - 1 when prev is clicked", async () => {
    const onPageChange = jest.fn();
    render(
      <Pagination page={3} perPage={30} itemCount={30} onPageChange={onPageChange} />
    );
    await userEvent.click(screen.getByText("前へ"));
    expect(onPageChange).toHaveBeenCalledWith(2);
  });

  it("calls onPageChange with page + 1 when next is clicked", async () => {
    const onPageChange = jest.fn();
    render(
      <Pagination page={1} perPage={30} itemCount={30} onPageChange={onPageChange} />
    );
    await userEvent.click(screen.getByText("次へ"));
    expect(onPageChange).toHaveBeenCalledWith(2);
  });

  it("displays current page number", () => {
    render(
      <Pagination page={5} perPage={30} itemCount={30} onPageChange={jest.fn()} />
    );
    expect(screen.getByText("5")).toBeInTheDocument();
  });
});
