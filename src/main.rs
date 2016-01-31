#[macro_use] extern crate nickel;

extern crate postgres;
extern crate openssl;
extern crate nickel_postgres;
use nickel::{Nickel, Request, Response, HttpRouter, MiddlewareResult, MediaType};
use nickel_postgres::{PostgresMiddleware, PostgresRequestExtensions};
use postgres::{Connection, SslMode};

// テンプレのハッシュに使う
use std::collections::HashMap;

// モデル
struct EmployeeInfo {
    EmpNo: i32,
    EmpName: String,
    Salary: i32,
    DeptName: String,
    Designation: String,
}

// テーブルのセットアップ
fn setup_table<'a>(req: &mut Request, res: Response<'a>) -> MiddlewareResult<'a> {
    let conn = Connection::connect("postgres://postgres@localhost", SslMode::None).unwrap();

    // also print to stdout
    return match conn.execute("CREATE TABLE EmployeeInfo (
        EmpNo       SERIAL PRIMARY KEY,
        EmpName     VARCHAR (50) NOT NULL,
        Salary      DECIMAL (18) NOT NULL,
        DeptName    VARCHAR (50) NOT NULL,
        Designation VARCHAR (50) NOT NULL
    )",
    &[]) {
        // http://www.rust-ci.org/Indiv0/paste/doc/nickel/struct.Response.html
        Ok(n) => return res.send("EmployeeInfo table was created."),
        // エラー
        // Err(err) => return res.send(println!("Error running query: {:?}", err))
        Err(err) => return res.send(format!("Error running query: {:?}", err))
    };
}

// API系
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
    for row in &conn.query("SELECT EmpNo, EmpName, Salary, DeptName,
        Designation FROM EmployeeInfo", &[]).unwrap() {
        let employeeInfo = EmployeeInfo {
            EmpNo: row.get(0),
            EmpName: row.get(1),
            Salary: row.get(2),
            DeptName: row.get(3),
            Designation: row.get(4)
        };

        data.insert("EmpNo", employeeInfo.EmpNo.to_string());
        data.insert("EmpName", employeeInfo.EmpName.to_string());
        data.insert("Salary", employeeInfo.Salary.to_string());
        data.insert("DeptName", employeeInfo.DeptName.to_string());
        data.insert("Designation", employeeInfo.Designation.to_string());
    }
    return res.render("app/employee/views/show_employees.tpl", &data);
}

fn main() {
    let mut server = Nickel::new();
    let mut router = Nickel::router();
    router.get("/setup/", setup_table);
    router.get("/api/EmployeeInfoAPI/add/", add_employee);
    router.get("/api/EmployeeInfoAPI/delete/", delete_employee);
    router.get("/api/EmployeeInfoAPI/edit/", edit_employee);
    router.get("/api/EmployeeInfoAPI/", show_employees);

    server.utilize(router);
    server.listen("localhost:6767");
}