import Link from "next/link";
import { Head } from "@/components/Common/Head";
import { DefaultLayout } from "@/layouts/DefaultLayout";

export default function InternalError() {
  return (
    <DefaultLayout>
      <Head title="500 - サーバーエラー" />
      <div className="text-center py-5">
        <h1 className="display-1 fw-bold text-muted">500</h1>
        <p className="lead">サーバーエラーが発生しました</p>
        <Link href="/" className="btn btn-primary">
          トップに戻る
        </Link>
      </div>
    </DefaultLayout>
  );
}
