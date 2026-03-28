import { Head } from "@/components/Common/Head";
import { DefaultLayout } from "@/layouts/DefaultLayout";

export default function Home() {
  return (
    <DefaultLayout>
      <Head />
      <h1 className="h3 mb-4">VTuber Song List</h1>
      <p className="text-body-secondary">
        VTuberの歌枠セトリデータベースへようこそ。
      </p>
    </DefaultLayout>
  );
}
