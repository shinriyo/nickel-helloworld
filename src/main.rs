#[macro_use] extern crate nickel;

extern crate postgres;
extern crate openssl;
use nickel::{Nickel, Request, Response, HttpRouter, MiddlewareResult, MediaType,
    StaticFilesHandler,JsonBody};
use nickel::status::StatusCode;
use postgres::{Connection, SslMode};
use std::sync::{Arc, Mutex};

// テンプレのハッシュに使う
use std::collections::HashMap;
use std::vec::Vec;
// json化
extern crate rustc_serialize;
use rustc_serialize::{json};
use rustc_serialize::json::{Json, Parser};
use std::collections::BTreeMap;

// モデル
#[derive(RustcDecodable, RustcEncodable)]
struct Movie {
    // id: i32,
    title: String,
    director: String,
    releaseYear: i16,
    genre: String,
}

fn main() {
    let mut server = Nickel::new();

    // 静的ファイル
    // 例: http://nickel.rs/
    server.utilize(StaticFilesHandler::new("app/assets/"));
    // => 実際のアクセスは「http://localhost:6767/angular.js」

    // URLのセット
    let mut router = Nickel::router();

    let conn = Connection::connect("postgres://postgres@localhost", SslMode::None).unwrap();

    // for what url called logging
//    router.get("**", middleware! { |request, response|
//        println!("GET: {}.", request.path_without_query().unwrap());
//    });
//
//    router.post("**", middleware! { |request, response|
//        println!("POST:{}.", request.path_without_query().unwrap());
//    });
//
//    router.delete("**", middleware! { |request, response|
//        println!("DELETE:{}.", request.path_without_query().unwrap());
//    });
//
//    router.put("**", middleware! { |request, response|
//        println!("PUT:{}.", request.path_without_query().unwrap());
//    });

    let shared_connection = Arc::new(Mutex::<Connection>::new(conn));

    // テーブル準備
    {
        let conn = shared_connection.clone();
        router.get("/setup/movie", middleware! { |_, response|
        // also print to stdout
        return match conn.lock().unwrap().execute("CREATE TABLE Movie (
                id          SERIAL PRIMARY KEY,
                title       VARCHAR (50) NOT NULL,
                releaseYear SMALLINT NOT NULL,
                director    VARCHAR (18) NOT NULL,
                genre       VARCHAR (50) NOT NULL
            )",
        &[]) {
                // http://www.rust-ci.org/Indiv0/paste/doc/nickel/struct.Response.html
                Ok(n) => return response.send("Movie table was created."),
                // エラー
                Err(err) => return response.send(format!("Error running query: {:?}", err))
            };
        });
    }


    // APIs
    {
        router.get("/", middleware! { |_, mut response|
            response.set(MediaType::Html);
            return response.send_file("app/movie/views/index.tpl")
        });
    }

    // select all
    {
        let conn = shared_connection.clone();
        router.get("/api/movies", middleware! { |_, mut response|
            let conn = conn.lock().unwrap();
            let movies = conn.query("select title, releaseYear, director, genre from movie", &[]).unwrap();
            let mut v: Vec<Movie> = vec![];

            for row in &movies {
                let movie = Movie {
                    title: row.get(0),
                    releaseYear: row.get(1),
                    director: row.get(2),
                    genre: row.get(3),
                };

                v.push(movie);
            }

            let json_obj = json::encode(&v).unwrap();
            // MediaType can be any valid type for reference see
            response.set(MediaType::Json);
            response.set(StatusCode::Ok);
            return response.send(json_obj);
        });
    }

    // insert
    {
        let conn = shared_connection.clone();
        router.post("/api/movies", middleware! { |request, response|
            let conn = conn.lock().unwrap();
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
    }

    // select one
    {
        let conn = shared_connection.clone();
        router.get("/api/movies/:id", middleware! { |request, mut response|
            let conn = conn.lock().unwrap();
            let movie = conn.query(
                "select title, releaseYear, director, genre from movie where id = $1",
                // param string to int
                &[&request.param("id").unwrap().parse::<i32>().unwrap()]
            ).unwrap();

            for row in &movie {
                let movie = Movie {
                title: row.get(0),
                releaseYear: row.get(1),
                director: row.get(2),
                genre: row.get(3),
                };

                let json_obj = json::encode(&movie).unwrap();
                // MediaType can be any valid type for reference see
                response.set(MediaType::Json);
                response.set(StatusCode::Ok);
                return response.send(json_obj);
            }
        });
    }

    // TODO: not work from Angular.js yet
    // update
    {
        let conn = shared_connection.clone();
        router.put("/api/movies/:id", middleware! { |request, response|
            // TODO: it is bad to make new connection for each request remove later
            // but the error happens...
            // core::marker::Sync is not implemented for the type core::cell::UnsafeCell<postgres::InnerConnection>
            let conn = Connection::connect("postgres://postgres@localhost", SslMode::None).unwrap();
            let stmt = match conn.prepare("update movie set title=$1, releaseYear=$2,
                director=$3, genre=$4)
                where id = $5") {
                Ok(stmt) => stmt,
                    Err(e) => {
                    return response.send(format!("Preparing query failed: {}", e));
                }
            };
            match stmt.execute(&[
                &request.param("title"),
                &request.param("releaseYear"),
                &request.param("director"),
                &request.param("genre"),
                &request.param("id").unwrap().parse::<i32>().unwrap(),
            ]) {
                Ok(v) => println!("Updating movie was Success."),
                Err(e) => println!("Updating movie failed. => {:?}", e),
            };
        });
    }

    // delete
    // curl http://localhost:6767/api/movies/1 -X DELETE
    {
        let conn = shared_connection.clone();
        router.delete("/api/movies/:id", middleware! { |request, response|
//        router.delete("/api/movies", middleware! { |request, response|
//        let movie = request.json_as::<Movie>().unwrap();
            let conn = conn.lock().unwrap();
            let stmt = match conn.prepare("delete from movie where id = $1") {
                Ok(stmt) => stmt,
                Err(e) => {
                    return response.send(format!("Preparing query failed: {}", e));
                }
            };

            match stmt.execute(&[
                // param string to int
                &request.param("id").unwrap().parse::<i32>().unwrap()
            ]) {
                Ok(v) => println!("Deleting movie was Success."),
                Err(e) => println!("Deleting movie failed. => {:?}", e),
            };
        });
    }

    server.utilize(router);
    server.listen("localhost:6767");
}