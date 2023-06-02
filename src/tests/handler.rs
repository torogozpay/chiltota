use crate::connection::Conn as DbConn;
use rocket_contrib::json::Json;
use crate::tests::repository::{Test, NewTest,TestData};
use serde_json::Value;


#[get("/index")]
pub fn index() {

}

#[post("/tests", format = "application/json")]
pub fn get_all(conn: DbConn) -> Json<Value> {
    let tests = Test::get_all_test(&conn);
    Json(json!({
        "status": 200,
        "result": tests,
    }))
}

#[post("/newTest", format = "application/json", data = "<new_test>")]
pub fn new_test(conn: DbConn, new_test: Json<NewTest>) -> Json<Value> {
    Json(json!({
        "status": Test::insert_test(new_test.into_inner(), &conn),
        "result": Test::get_all_test(&conn).first(),
    }))
}

#[post("/getTest", format = "application/json", data = "<test_data>")]
pub fn find_test(conn: DbConn, test_data: Json<TestData>) -> Json<Value> {
    Json(json!({
        "status": 200,
        "result": Test::get_test_by_codigo(test_data.into_inner(), &conn),
    }))
}
