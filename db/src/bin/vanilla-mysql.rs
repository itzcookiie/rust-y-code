use mysql::*;
use mysql::prelude::Queryable;
use db::{Company,Person};

fn main() {
    let url = "mysql://mayowa:abc123@localhost:3306/rust";
    let opts = Opts::from_url(url).unwrap();
    let pool = Pool::new(opts).unwrap();
    let mut conn = pool.get_conn().unwrap();

    let company_a = Company{id : 1};
    let persons = vec![
        Person{
            id: 1,
            name: "".to_string(),
            company_id: 0
        },
        create_person(2, String::from("Judy"), 1),
        create_person(3, String::from("Mike"), 1),
    ];

    drop_tables(&mut conn);
    create_tables(&mut conn);

    conn.exec_drop(
        r"INSERT INTO company VALUES (:id)", params!{
            "id" => company_a.id
        }
    );

    let pp = vec!["vew","fw"];
    let stmnt = pp.iter().fold(String::new(),|a,b|{
        match a.is_empty() {
            true => format!(":{}",b),
            false => {
                format!("{}, :{}",a,b)
            }
        }
    });

    conn.exec_batch(
        r"INSERT INTO person VALUES (:id, :name, :company_id)",
        persons.iter().map(|person| params! {
            "id" => person.id,
            "name" => &person.name,
            "company_id" => person.company_id
        })
    );
}

fn create_person(id : i32, name : String, company_id : i32) -> Person {
    return Person{
        id,
        name,
        company_id
    }
}

fn create_tables(conn : &mut PooledConn) {
    conn.query_drop(
        r"CREATE TABLE company (
            id INT NOT NULL,
            PRIMARY KEY (id)
        )"
    );

    conn.query_drop(
        r"CREATE TABLE person (
            id int not null,
            name text not null,
            company_id int not null,
            PRIMARY KEY (id),
            FOREIGN KEY (company_id) REFERENCES company(id)
        )"
    );
}

fn drop_tables(conn : &mut PooledConn) {
    conn.query_drop(
        r"DROP TABLE companies"
    );
    conn.query_drop(
        r"DROP TABLE persons"
    );
    conn.query_drop(
        r"DROP TABLE company"
    );
}