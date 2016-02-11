#[macro_use] extern crate nickel;

extern crate postgres;
extern crate openssl;
extern crate hyper;
//extern crate serialize;
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

mod movie; // モジュールの読み込み

fn main() {
    let mut server = Nickel::new();

    // 静的ファイル
    // 例: http://nickel.rs/
    server.utilize(StaticFilesHandler::new("app/assets/"));
    // => 実際のアクセスは「http://localhost:6767/angular.js」

    // URLのセット
    let mut router = Nickel::router();

    let conn = Connection::connect("postgres://postgres@localhost", SslMode::None).unwrap();
    let shared_connection = Arc::new(Mutex::new(conn));
    movie::url(shared_connection.clone(), &mut router);

    server.listen("localhost:6767");
}