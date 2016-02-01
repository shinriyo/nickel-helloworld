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

// モデル
struct Movie {
    id: i32,
    title: String,
    director: String,
    releaseYear: i32,
    genre: String,
}
//struct EmployeeInfo {
//    EmpNo: i32,
//    EmpName: String,
//    Salary: i32,
//    DeptName: String,
//    Designation: String,
//}

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
    // also print to stdout
//    return match conn.execute("CREATE TABLE EmployeeInfo (
//        EmpNo       SERIAL PRIMARY KEY,
//        EmpName     VARCHAR (50) NOT NULL,
//        Salary      DECIMAL (18) NOT NULL,
//        DeptName    VARCHAR (50) NOT NULL,
//        Designation VARCHAR (50) NOT NULL
//    )",
    &[]) {
        // http://www.rust-ci.org/Indiv0/paste/doc/nickel/struct.Response.html
        Ok(n) => return res.send("Movie table was created."),
        // エラー
        // Err(err) => return res.send(println!("Error running query: {:?}", err))
        Err(err) => return res.send(format!("Error running query: {:?}", err))
    };
}

// INDEX
fn index<'a>(req: &mut Request, res: Response<'a>) -> MiddlewareResult<'a> {
    let mut data = HashMap::new();
    data.insert("name", "user");
    return res.render("app/movie/views/index.tpl", &data);
}

// 基本ページ
// 追加
fn add_employee<'a>(req: &mut Request, res: Response<'a>) -> MiddlewareResult<'a> {
    let mut data = HashMap::new();
    data.insert("name", "user");
    return res.render("app/employee/views/add_employee.tpl", &data);
}

// 削除
fn delete_employee<'a>(req: &mut Request, res: Response<'a>) -> MiddlewareResult<'a> {
    let mut data = HashMap::new();
    data.insert("name", "user");
    return res.render("app/employee/views/delete_employee.tpl", &data);
}

// 編集
fn edit_employee<'a>(req: &mut Request, res: Response<'a>) -> MiddlewareResult<'a> {
    let mut data = HashMap::new();
    data.insert("name", "user");
    return res.render("app/employee/views/edit_employee.tpl", &data);
}

// 一覧
fn show_employees<'a>(req: &mut Request, res: Response<'a>) -> MiddlewareResult<'a> {
    let conn = Connection::connect("postgres://postgres@localhost", SslMode::None).unwrap();
    let mut data = HashMap::new();
    data.insert("name", "user");
    for row in &conn.query("SELECT EmpNo, EmpName, Salary, DeptName,
        Designation FROM Movie", &[]).unwrap() {
//        let movie = Movie {
//            id: row.get(0),
//`director`, `genre`, `title`, `releaseYear`
//        };

//        data.insert("EmpNo", employeeInfo.EmpNo.to_string());
//        data.insert("EmpName", employeeInfo.EmpName.to_string());
//        data.insert("Salary", employeeInfo.Salary.to_string());
//        data.insert("DeptName", employeeInfo.DeptName.to_string());
//        data.insert("Designation", employeeInfo.Designation.to_string());
    }
//    for row in &conn.query("SELECT EmpNo, EmpName, Salary, DeptName,
//        Designation FROM EmployeeInfo", &[]).unwrap() {
//        let employeeInfo = EmployeeInfo {
//            EmpNo: row.get(0),
//            EmpName: row.get(1),
//            Salary: row.get(2),
//            DeptName: row.get(3),
//            Designation: row.get(4)
//        };
//
//        data.insert("EmpNo", employeeInfo.EmpNo.to_string());
//        data.insert("EmpName", employeeInfo.EmpName.to_string());
//        data.insert("Salary", employeeInfo.Salary.to_string());
//        data.insert("DeptName", employeeInfo.DeptName.to_string());
//        data.insert("Designation", employeeInfo.Designation.to_string());
//    }
    return res.render("app/employee/views/show_employees.tpl", &data);
}

// API系



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
    router.get("/", index);

    // API
    router.get("/api/movies", middleware! { |request, response|
        format!("Hello from GET /api/movies")
    });

    router.post("/api/movies", middleware! { |request, response|
        format!("Hello from POST /api/movie")
    });

    router.get("/api/movies/:id", middleware! { |request, response|
        format!("Hello from GET /api/movie/:id")
    });

    router.put("/api/movies/:id", middleware! { |request, response|
    format!("Hello from PUT /api/movie/:id")
    });

    router.delete("/api/movies/:id", middleware! { |request, response|
        format!("Hello from DELETE /api/movie/:id")
    });

    server.utilize(router);
    server.listen("localhost:6767");
}