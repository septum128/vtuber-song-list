type Props = {
  page: number;
  perPage: number;
  itemCount: number;
  onPageChange: (page: number) => void;
};

export function Pagination({ page, perPage, itemCount, onPageChange }: Props) {
  const hasPrev = page > 1;
  const hasNext = itemCount >= perPage;

  if (!hasPrev && !hasNext) return null;

  return (
    <nav aria-label="ページネーション" className="mt-3">
      <ul className="pagination justify-content-center">
        <li className={`page-item ${!hasPrev ? "disabled" : ""}`}>
          <button
            type="button"
            className="page-link"
            onClick={() => hasPrev && onPageChange(page - 1)}
            disabled={!hasPrev}
          >
            前へ
          </button>
        </li>
        <li className="page-item disabled">
          <span className="page-link">{page}</span>
        </li>
        <li className={`page-item ${!hasNext ? "disabled" : ""}`}>
          <button
            type="button"
            className="page-link"
            onClick={() => hasNext && onPageChange(page + 1)}
            disabled={!hasNext}
          >
            次へ
          </button>
        </li>
      </ul>
    </nav>
  );
}
