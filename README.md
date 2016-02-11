# nickel-helloworld

#  自分用のrust言語+nickelフレームワーク+PostgreSQLのテンプレ　これをベースに開発できる

## PostgreSQLの起動
```
postgres -D /usr/local/var/postgres
```

## SPAの参考
http://www.sitepoint.com/creating-crud-app-minutes-angulars-resource/

# API URL
## データベースの設定アクセスすると出来る
http://localhost:6767/setup/
※最後に`/`忘れずに

## CRUD

### 一覧
GET http://localhost:6767/api/movie/

### 一件
GET http://localhost:6767/api/movie/:id

### 追加
POST http://localhost:6767/api/movie/:id

### 更新
PUT http://localhost:6767/api/movie

## 削除
DELETE http://localhost:6767/api/movie/:id

## 起動
cargo run

## これを使ってる
postgres = "0.11"
sfackler/rust-postgres
