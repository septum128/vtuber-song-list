import { Head } from "@/components/Common/Head";
import { DefaultLayout } from "@/layouts/DefaultLayout";

export default function Home() {
  return (
    <DefaultLayout>
      <Head
        url="https://www.vtuber-song.com/"
        ogImage="https://www.vtuber-song.com/Vtuber-Song-List-OGP.png"
      />
      <h1 className="h3 mb-4">VTuber Song List</h1>
      {process.env.NEXT_PUBLIC_APP_ENV === "staging" && (
        <h2>Staging Environment</h2>
      )}
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
      <p>ご連絡は以下のいずれかからお願いいたします。</p>
      <ul>
        <li>
          お問い合わせ：
          <a href="https://twitter.com/interceptor128" target="_blank">
            Twitter
          </a>
          （@interceptor128 にリプライ ※DM不可）または
          <a href="mailto:contact@vtuber-song.com">メール</a>、または
          <a
            href="https://marshmallow-qa.com/interceptor128?t=OLl4Y0&utm_medium=url_text&utm_source=promotion"
            target="_blank"
          >
            マシュマロ
          </a>
        </li>
        <li>
          不具合報告：GitHubの
          <a
            href="https://github.com/septum128/vtuber-song-list/issues"
            target="_blank"
          >
            Issue
          </a>
        </li>
      </ul>
    </DefaultLayout>
  );
}
