#[macro_use] extern crate nickel;

extern crate postgres;
extern crate openssl;
extern crate nickel_postgres;
use nickel::{Nickel, Request, Response, HttpRouter, MiddlewareResult, MediaType,
    StaticFilesHandler};
use nickel_postgres::{PostgresMiddleware, PostgresRequestExtensions};
use postgres::{Connection, SslMode};

// テンプレのハッシュに使う
use std::collections::HashMap;

// json
extern crate rustc_serialize;
use rustc_serialize::json::{Json, Parser};

// モデル
struct Movie {
    id: i32,
    title: String,
    director: String,
    releaseYear: i32,
    genre: String,
}

// テーブルのセットアップ
fn setup_table<'a>(req: &mut Request, res: Response<'a>) -> MiddlewareResult<'a> {
    let conn = Connection::connect("postgres://postgres@localhost", SslMode::None).unwrap();
    // also print to stdout
    return match conn.execute("CREATE TABLE Movie (
        id          SERIAL PRIMARY KEY,
        title       VARCHAR (50) NOT NULL,
        director    DECIMAL (18) NOT NULL,
        releaseYear DECIMAL (4)  NOT NULL,
        genre       VARCHAR (50) NOT NULL
    )",
    &[]) {
        // http://www.rust-ci.org/Indiv0/paste/doc/nickel/struct.Response.html
        Ok(n) => return res.send("Movie table was created."),
        // エラー
        Err(err) => return res.send(format!("Error running query: {:?}", err))
    };
}

// INDEX
fn index<'a>(req: &mut Request, res: Response<'a>) -> MiddlewareResult<'a> {
    let conn = Connection::connect("postgres://postgres@localhost", SslMode::None).unwrap();
    let mut data = HashMap::<&str, String>::new();
    return res.render("app/movie/views/index.tpl", &data);
}

fn main() {
    let mut server = Nickel::new();

    // 静的ファイル
    // http://nickel.rs/
    server.utilize(StaticFilesHandler::new("app/assets/"));
    // => 実際のアクセスは「http://localhost:6767/angular.js」

    // URLのセット
    let mut router = Nickel::router();

    // テーブル準備
    router.get("/setup/movies", setup_table);
    // 普通のページ
    router.get("/movieApp", index);

    // API
    router.get("/api/movies", middleware! { |request, response|
        let conn = Connection::connect("postgres://postgres@localhost", SslMode::None).unwrap();
        let stmt = match conn.prepare("select title, genre, director, releaseYear from movie") {
        Ok(stmt) => stmt,
        Err(e) => {
            return response.send(format!("Preparing query failed: {}", e));
        }
    };
    stmt.execute(&[]).ok().expect("Selecting movie failed");
    let mut my_son =
        r#"{
            "movies": [
                { "title": "アイアンマン"},
                { "title": "アベンジャーズ"},
                { "title": "パディントン"}
            ]
        }"#;
        let jsond = Json::from_str(my_son);
        format!("{}", jsond.unwrap())
    });

    router.post("/api/movies", middleware! { |request, response|
        let conn = Connection::connect("postgres://postgres@localhost", SslMode::None).unwrap();
        let stmt = match conn.prepare("insert into movie (title, genre, director, releaseYear)
            values ($1, $2, $3, $4)") {
            Ok(stmt) => stmt,
            Err(e) => {
                return response.send(format!("Preparing query failed: {}", e));
            }
        };
        stmt.execute(&[
            &request.param("title"),
            &request.param("genre"),
            &request.param("director"),
            &request.param("releaseYear")
        ]).ok().expect("Inserting movie failed");
    });

    router.get("/api/movies/:id", middleware! { |request, response|
        let conn = Connection::connect("postgres://postgres@localhost", SslMode::None).unwrap();
        let stmt = match conn.prepare("select title, genre, director, releaseYear from movie
            from where id = $1") {
            Ok(stmt) => stmt,
            Err(e) => {
                return response.send(format!("Preparing query failed: {}", e));
            }
        };
        stmt.execute(&[
            &request.param("id")
        ]).ok().expect("Selecting movie failed");
    });

    // update
    router.put("/api/movies/:id", middleware! { |request, response|
        let conn = Connection::connect("postgres://postgres@localhost", SslMode::None).unwrap();
        let stmt = match conn.prepare("uptede movie set title=$1, genre=$2, director=$3,
            releaseYear=$4)
            where id = $5") {
            Ok(stmt) => stmt,
            Err(e) => {
                return response.send(format!("Preparing query failed: {}", e));
            }
        };
        stmt.execute(&[
            &request.param("title"),
            &request.param("genre"),
            &request.param("director"),
            &request.param("releaseYear"),
            &request.param("id")
        ]).ok().expect("Updating movie failed");
    });

    router.delete("/api/movies/:id", middleware! { |request, response|
        let conn = Connection::connect("postgres://postgres@localhost", SslMode::None).unwrap();
        let stmt = match conn.prepare("delete movie where id = $1") {
            Ok(stmt) => stmt,
            Err(e) => {
                return response.send(format!("Preparing query failed: {}", e));
            }
        };
        stmt.execute(&[
            &request.param("id")
        ]).ok().expect("Deleting movie failed");
    });

    server.utilize(router);
    server.listen("localhost:6767");
}