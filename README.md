# Reactチャットアプリ（バックエンド）

## 前提
- rustのインストール
- PostgreSQLのインストール
    - bin/libへのPATHを通しておく
- dieselのセットアップ ([dieselのtutorial](https://diesel.rs/guides/getting-started))

## 実行
`cargo run`

## フロントエンド
- [test chat client](https://github.com/march101348/test-chat-client)

## ブランチ運用
- `master-develop-feature`
- developからfeatureブランチを切って作業する
- 命名：feature/{作業名} 
    - ex. feature/add_user_api
- developへのプルリク作成後、march101348までレビュー依頼