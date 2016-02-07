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
http://localhost:6767/api/EmployeeInfoAPI/

### 追加
http://localhost:6767/api/EmployeeInfoAPI/add/

## 起動
cargo run

## これを使ってる
sfackler/rust-postgres
