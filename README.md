# Vtuber Song List

Vtuber が歌枠で歌った曲のデータベースです。曲名や作者名・枠名などで検索が出来ます。

## クイックスタート

### 必要なもの

- Rust（最新 stable）
- Node.js 20+
- Docker（PostgreSQL の起動に使用）
- 以下の API キー
  - Google Cloud（YouTube Data API v3）
  - OpenAI
  - Spotify

### 1. リポジトリのクローン

```sh
git clone https://github.com/your-username/vtuber-song-list.git
cd vtuber-song-list
```

### 2. PostgreSQL の起動

```sh
docker compose up -d
```

### 3. 環境変数の設定

`.env.sample` をコピーして API キーを設定します。

```sh
cp .env.sample .env
# .env を編集して各 API キーを入力
```

### 4. バックエンドの起動

```sh
# .env を読み込んで起動
export $(cat .env | xargs) && cargo loco start

# Doppler を使用する場合
doppler run -- cargo loco start
```

バックエンドは `http://localhost:5150` で起動します。起動時にマイグレーションが自動実行されます。

### 5. フロントエンドの起動

別ターミナルで実行します。

```sh
cd frontend
cp .env.local.example .env.local
npm install
npm run dev
```

フロントエンドは `http://localhost:3001` で起動します。

### 6. セトリの自動作成

YouTube RSS からチャンネルの最新動画を取得し、コメントから OpenAI でセトリを解析してDBに保存します。

```sh
# .env を使用する場合
export $(cat .env | xargs) && cargo loco task create_recent_videos_song_items

# Doppler を使用する場合
doppler run -- cargo loco task create_recent_videos_song_items
```

### 7. チャンネルアイコンの取得

```sh
# .env を使用する場合
export $(cat .env | xargs) && cargo loco task fetch_channel_icons

# Doppler を使用する場合
doppler run -- cargo loco task fetch_channel_icons
```

---

## 主なコマンド

```sh
# バックエンド起動
cargo loco start

# テスト実行
DATABASE_URL=postgres://loco:loco@localhost:5432/vtuber-song-list_development cargo test --all-features --all

# Lint
cargo clippy --all-features -- -D warnings -W clippy::pedantic -W clippy::nursery -W rust-2018-idioms

# マイグレーション生成
cargo loco generate migration <name>

# フロントエンド開発サーバー
cd frontend && npm run dev
```
