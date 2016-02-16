#[macro_use] extern crate nickel;

extern crate postgres;
extern crate openssl;
extern crate hyper;
use nickel::{Nickel, StaticFilesHandler};
use postgres::{Connection, SslMode};
use std::sync::{Arc, Mutex};

extern crate rustc_serialize;

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
    server.utilize(router);
    server.listen("localhost:6767");
}