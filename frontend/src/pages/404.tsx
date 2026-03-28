import Link from "next/link";
import { Head } from "@/components/Common/Head";
import { DefaultLayout } from "@/layouts/DefaultLayout";

export default function NotFound() {
  return (
    <DefaultLayout>
      <Head title="404 - ページが見つかりません" />
      <div className="text-center py-5">
        <h1 className="display-1 fw-bold text-muted">404</h1>
        <p className="lead">ページが見つかりませんでした</p>
        <Link href="/" className="btn btn-primary">
          トップに戻る
        </Link>
      </div>
    </DefaultLayout>
  );
}
