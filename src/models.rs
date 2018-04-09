use diesel;
use diesel::prelude::*;
use schema::test;

#[table_name = "test"]
#[derive(Queryable, Insertable, Serialize, Deserialize)]
pub struct Test {
    pub id: i32,
    pub name: Option<String>,
}
