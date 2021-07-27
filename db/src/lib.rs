pub mod schema;

#[macro_use]
extern crate macros;

#[macro_use]
extern crate diesel;

use mysql;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use schema::person;
use schema::company;
use diesel::sql_types::*;

#[derive(Insertable,Queryable,MysqlInsert)]
#[table_name="company"]
pub struct Company {
    pub id: i32
}

#[derive(Insertable,Queryable)]
#[table_name="person"]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub company_id: i32
}

// impl Person {
//     pub fn ll(){
//         let io = Opts::from_url("ff").unwrap();
//         let query = format!("INSERT INTO {} VALUES ({})", #id_string, #stmnt);
//         println!("{}",query);
//         let mysql_pool = crate::mysql::Pool::new(url).unwrap();
//     }
// }


pub fn establish_connection() -> MysqlConnection {
    MysqlConnection::establish("mysql://root:abc123@localhost:3306/rust ").unwrap()
}

pub fn create_company(conn: &MysqlConnection, id: i32) -> usize {
    use schema::company;
    let new_company = Company{ id };
    diesel::insert_into(company::table)
        .values(&new_company)
        .execute(conn)
        .unwrap()
}

pub fn create_person(conn: &MysqlConnection, id: i32, name: String, company_id: i32) -> usize {
    use schema::person;
    let new_person = Person {
        id,
        name,
        company_id
    };
    diesel::insert_into(person::table)
        .values(&new_person)
        .execute(conn)
        .unwrap()
}

