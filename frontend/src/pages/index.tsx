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
      <p className="text-danger">
        このサイトはファンが非公式に運営しているものです。
        <br />
        このサイトに関して配信者様本人へ問い合わせをされるのはお控えください。
      </p>
      <p>
        セトリはYouTubeのコメント欄を元に作成しています。いつもセトリを作ってくださっている方々に感謝を。ありがとうございます！
      </p>
      <p>
        歌枠データベースは裏側で動くプログラムによって自動更新されているため、誤った情報が登録されていることがあります。
      </p>
      <p>
        ユーザ登録（ID/パスワード認証）をしていただくと、セトリの修正提案が可能ですので、そちらからセトリの修正申請をお願いいたします。
      </p>
      <p>
        お問い合わせは
        <a href="https://twitter.com/interceptor128" target="_blank">
          Twitter
        </a>
        か<a href="mailto:contact@vtuber-song.com">メール</a>へ！
      </p>
      <p>
        不具合報告はGitHubの
        <a
          href="https://github.com/septum128/vtuber-song-list/issues"
          target="_blank"
        >
          Issue
        </a>
        へ投稿をお願いいたします。
      </p>
    </DefaultLayout>
  );
}
