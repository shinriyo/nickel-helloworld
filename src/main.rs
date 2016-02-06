#[macro_use] extern crate nickel;

extern crate postgres;
extern crate openssl;
extern crate nickel_postgres;
use nickel::{Nickel, Request, Response, HttpRouter, MiddlewareResult, MediaType,
    StaticFilesHandler,JsonBody};
use nickel::status::StatusCode;
use nickel_postgres::{PostgresMiddleware, PostgresRequestExtensions};
use postgres::{Connection, SslMode};

// テンプレのハッシュに使う
use std::collections::HashMap;
use std::vec::Vec;
// json化
extern crate rustc_serialize;
use rustc_serialize::{json};


// モデル
#[derive(RustcDecodable, RustcEncodable)]
struct Movie {
    // id: i32,
    title: String,
    director: String,
    releaseYear: i16,
    genre: String,
}

// テーブルのセットアップ
fn setup_table<'a>(req: &mut Request, res: Response<'a>) -> MiddlewareResult<'a> {
    let conn = Connection::connect("postgres://postgres@localhost", SslMode::None).unwrap();
    // also print to stdout
    return match conn.execute("CREATE TABLE Movie (
        id          SERIAL PRIMARY KEY,
        title       VARCHAR (50) NOT NULL,
        releaseYear SMALLINT NOT NULL,
        director    VARCHAR (18) NOT NULL,
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
fn index<'a>(req: &mut Request, mut res: Response<'a>) -> MiddlewareResult<'a> {
    let conn = Connection::connect("postgres://postgres@localhost", SslMode::None).unwrap();
    res.set(MediaType::Html);
    res.send_file("app/movie/views/index.tpl")
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
    router.get("/setup/movie", setup_table);
    // 普通のページ
    router.get("/", index);
    //router.get("/movieApp", index);

    // API
    fn content_type<'a>(_: &mut Request, mut res: Response<'a>) -> MiddlewareResult<'a> {
        //MediaType can be any valid type for reference see
        res.set(MediaType::Json);
        res.send("{'foo':'bar'}")
//        let conn = Connection::connect("postgres://postgres@localhost", SslMode::None).unwrap();
//        let stmt = match conn.prepare("select title, releaseYear, director, genre from movie") {
//            Ok(stmt) => stmt,
//            Err(e) => {
//                return res.send(format!("Preparing query failed: {}", e));
//            }
//        };
//        let res = match stmt.execute(&[]) {
//            Ok(v) => println!("Selecting movie was Success."),
//            Err(e) => println!("Selecting movie failed. => {:?}", e),
//        };
//        res.send(&res.to_json())
    }

    router.get("/api/movies_", content_type);
    router.get("/api/movies", middleware! { |request, mut response|
        let conn = Connection::connect("postgres://postgres@localhost", SslMode::None).unwrap();
        let stmt = match conn.prepare("select title, releaseYear, director, genre from movie") {
            Ok(stmt) => stmt,
            Err(e) => {
                return response.send(format!("Preparing query failed: {}", e));
            }
        };
        let movies = match stmt.execute(&[]) {
            Ok(v) => println!("Selecting movie was Success."),
            Err(e) => println!("Selecting movie failed. => {:?}", e),
        };

        let json_obj = json::encode(&movies).unwrap();
        response.set(MediaType::Json);
        response.set(StatusCode::Ok);
        return response.send(json_obj);
    });

    router.post("/api/movies", middleware! { |request, response|
        let conn = Connection::connect("postgres://postgres@localhost", SslMode::None).unwrap();
        let stmt = match conn.prepare("insert into movie (title, releaseYear, director, genre)
            values ($1, $2, $3, $4)") {
            Ok(stmt) => stmt,
            Err(e) => {
                return response.send(format!("Preparing query failed: {}", e));
            }
        };

        let movie = request.json_as::<Movie>().unwrap();
        match stmt.execute(&[
            &movie.title.to_string(),
            &movie.releaseYear,
            &movie.director.to_string(),
            &movie.genre.to_string()
        ]) {
            Ok(v) => println!("Inserting movie was Success."),
            Err(e) => println!("Inserting movie failed. => {:?}", e),
        };
    });

    router.get("/api/movies/:id", middleware! { |request, response|
        let conn = Connection::connect("postgres://postgres@localhost", SslMode::None).unwrap();
        let stmt = match conn.prepare("select title, releaseYear, director, genre from movie
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
        let stmt = match conn.prepare("update movie set title=$1, releaseYear=$2,
            director=$3, genre=$4)
            where id = $5") {
            Ok(stmt) => stmt,
            Err(e) => {
                return response.send(format!("Preparing query failed: {}", e));
            }
        };
        stmt.execute(&[
            &request.param("title"),
            &request.param("releaseYear"),
            &request.param("director"),
            &request.param("genre"),
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