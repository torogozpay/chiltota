use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use crate::schema::ecommerce::tests;
use crate::schema::ecommerce::tests::dsl::tests as all_tests;


// this is to get tests from the database
#[derive(Serialize, Queryable)] 
pub struct Test {
    pub id: i32,
    pub codigo: String,
    pub nombre: String,
}


// decode request data
#[derive(Deserialize)] 
pub struct TestData {
    pub codigo: String,
}

// this is to insert tests to database
#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "tests"]
pub struct NewTest {
    pub codigo: String,
    pub nombre: String,
}

impl Test {

    pub fn get_all_test(conn: &PgConnection) -> Vec<Test> {
        all_tests
            .order(tests::id.desc())
            .load::<Test>(conn)
            .expect("error!")
    }

    pub fn insert_test(test: NewTest, conn: &PgConnection) -> bool {
        diesel::insert_into(tests::table)
            .values(&test)
            .execute(conn)
            .is_ok()
    }

    pub fn get_test_by_codigo(test: TestData, conn: &PgConnection) -> Vec<Test> {
        all_tests
            .filter(tests::codigo.eq(test.codigo))
            .load::<Test>(conn)
            .expect("error!")
    }    

}