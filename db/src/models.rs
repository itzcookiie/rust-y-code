use super::schema::person;
use super::schema::company;
use diesel::sql_types::*;

#[derive(Queryable)]
pub struct Person {
    pub id : i32,
    pub name : String,
    pub company_id : i32,
}

#[derive(Queryable)]
pub struct Company {
    pub id : i32
}

#[derive(Insertable)]
#[table_name="company"]
pub struct NewCompany {
    pub id: i32
}

#[derive(Insertable)]
#[table_name="person"]
pub struct NewPerson {
    pub id: i32,
    pub name: String,
    pub company_id: i32
}