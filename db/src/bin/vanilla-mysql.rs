use mysql::*;
use mysql::prelude::Queryable;
use db::{Company,Person};

fn main() {
    let company_a = Company{id : 1};
    let url = "mysql://mayowa:abc123@localhost:3306/rust";
    let opts = Opts::from_url(url).unwrap();
    let pool = Pool::new(opts).unwrap();

    let mut conn = pool.get_conn().unwrap();

    let persons = vec![
        createPerson(1, String::from("Bob"), 1),
        createPerson(2, String::from("Judy"), 1),
        createPerson(3, String::from("Mike"), 1),
    ];

    dropTables(&mut conn);
    createTables(&mut conn);

    conn.exec_drop(
        r"INSERT INTO company VALUES (:id)", params!{
            "id" => company_a.id
        }
    );

    conn.exec_batch(
        r"INSERT INTO person VALUES (:id, :name, :company_id)",
        persons.iter().map(|person| params! {
            "id" => person.id,
            "name" => &person.name,
            "company_id" => person.company_id
        })
    );
}

fn createPerson(id : u64, name : String, company_id : u64) -> Person {
    return Person{
        id,
        name,
        company_id
    }
}

fn createTables(conn : &mut PooledConn) {
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

fn dropTables(conn : &mut PooledConn) {
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