pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;

use self::models::{Company, NewCompany, Person, NewPerson};

pub fn establish_connection() -> MysqlConnection {
    MysqlConnection::establish("mysql://root:abc123@localhost:3306/rust ").unwrap()
}

pub fn create_company(conn: &MysqlConnection, id: i32) -> usize {
    use schema::company;

    let new_company = NewCompany {
        id
    };

    diesel::insert_into(company::table)
        .values(&new_company)
        .execute(conn)
        .unwrap()
}

pub fn create_person(conn: &MysqlConnection, id: i32, name: String, company_id: i32) -> usize {
    use schema::person;

    let new_person = NewPerson {
        id,
        name,
        company_id
    };

    diesel::insert_into(person::table)
        .values(&new_person)
        .execute(conn)
        .unwrap()
}

