import { useEffect } from "react";
import { useForm } from "react-hook-form";

export type SongSearchValues = {
  query: string;
  since: string;
  until: string;
  video_title: string;
};

type Props = {
  defaultValues: SongSearchValues;
  onSearch: (values: SongSearchValues) => void;
};

export function SongSearchForm({ defaultValues, onSearch }: Props) {
  const { register, handleSubmit, reset } = useForm<SongSearchValues>({
    defaultValues,
  });

  // URL params が変化した場合にフォームをリセット
  useEffect(() => {
    reset(defaultValues);
  }, [defaultValues, reset]);

  return (
    <form onSubmit={handleSubmit(onSearch)} className="mb-3">
      <div className="row g-2">
        <div className="col-12 col-md-6 col-lg-3">
          <input
            type="text"
            className="form-control form-control-sm"
            placeholder="曲名・アーティスト"
            {...register("query")}
          />
        </div>
        <div className="col-12 col-md-6 col-lg-3">
          <input
            type="text"
            className="form-control form-control-sm"
            placeholder="動画タイトル"
            {...register("video_title")}
          />
        </div>
        <div className="col-6 col-md-3 col-lg-2">
          <input
            type="date"
            className="form-control form-control-sm"
            placeholder="開始日"
            {...register("since")}
          />
        </div>
        <div className="col-6 col-md-3 col-lg-2">
          <input
            type="date"
            className="form-control form-control-sm"
            placeholder="終了日"
            {...register("until")}
          />
        </div>
        <div className="col-12 col-lg-2 d-flex gap-2">
          <button type="submit" className="btn btn-sm btn-primary flex-grow-1">
            検索
          </button>
          <button
            type="button"
            className="btn btn-sm btn-outline-secondary"
            onClick={() => onSearch({ query: "", since: "", until: "", video_title: "" })}
          >
            クリア
          </button>
        </div>
      </div>
    </form>
  );
}
