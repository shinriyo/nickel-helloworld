extern crate postgres;
extern crate openssl;
extern crate hyper;
use nickel::{Nickel, Request, Response, Router, HttpRouter, MiddlewareResult, MediaType,
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

use std::collections::BTreeMap;
use hyper::header::Location;

// モデル
#[derive(RustcDecodable, RustcEncodable)]
struct Movie {
    _id: Option<i32>,
    title: String,
    director: String,
    releaseYear: i16,
    genre: String,
}

pub fn url(shared_connection: Arc<Mutex<Connection>>, router: &mut Router) {
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
            let movies = conn.query("select id, title, releaseYear, director, genre from movie", &[]).unwrap();
            let mut v: Vec<Movie> = vec![];

            for row in &movies {
                let movie = Movie {
                    _id: row.get(0),
                    title: row.get(1),
                    releaseYear: row.get(2),
                    director: row.get(3),
                    genre: row.get(4),
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
        router.post("/api/movies", middleware! { |request, mut response|
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
                Ok(v) => {
                    println!("Inserting movie was Success.");
                    response.set(StatusCode::Ok);
                },
                Err(e) => println!("Inserting movie failed. => {:?}", e),
            };

            return response.send("");
        });
    }

    // select one
    {
        let conn = shared_connection.clone();
        router.get("/api/movies/:id", middleware! { |request, mut response|
            let conn = conn.lock().unwrap();
            let movie = conn.query(
                "select id, title, releaseYear, director, genre from movie where id = $1",
                // param string to int
                &[&request.param("id").unwrap().parse::<i32>().unwrap()]
            ).unwrap();

            // movie
            for row in &movie {
                let movie = Movie {
                    _id: row.get(0),
                    title: row.get(1),
                    releaseYear: row.get(2),
                    director: row.get(3),
                    genre: row.get(4),
                };

                let json_obj = json::encode(&movie).unwrap();
                // MediaType can be any valid type for reference see
                response.set(MediaType::Json);
                response.set(StatusCode::Ok);
                return response.send(json_obj);
            }
        });
    }

    // update
    {
        let conn = shared_connection.clone();
        router.put("/api/movies/:id", middleware! { |request, mut response|
            let conn = conn.lock().unwrap();
            let stmt = match conn.prepare("update movie set title=$1, releaseYear=$2,
                director=$3, genre=$4
                where id = $5") {
                Ok(stmt) => stmt,
                Err(e) => {
                    return response.send(format!("Preparing query failed: {}", e));
                }
            };

            // JSON to object
            let movie = request.json_as::<Movie>().unwrap();
            match stmt.execute(&[
                &movie.title.to_string(),
                &movie.releaseYear,
                &movie.director.to_string(),
                &movie.genre.to_string(),
                &movie._id
            ]) {
                Ok(v) => {
                    println!("Updating movie was Success.");
                    response.set(StatusCode::Ok);
                },
                Err(e) => println!("Updating movie failed. => {:?}", e),
            };

            return response.send("");
        });
    }

    // delete
    // curl http://localhost:6767/api/movies/1 -X DELETE
    {
        let conn = shared_connection.clone();
        router.delete("/api/movies/:id", middleware! { |request, mut response|
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
                Ok(v) => {
                    println!("Deleting movie was Success.");
                    response.set(StatusCode::Ok);
                },
                Err(e) => println!("Deleting movie failed. => {:?}", e),
            };

            return response.send("");
        });
    }
}
