use mysql::*;
use mysql::prelude::Queryable;
use tasks::{Car};
use clap::Clap;

fn main() {
    let url = "mysql://mayowa:abc123@localhost:3306/rust";
    let opts = Opts::from_url(url).unwrap();
    let pool = Pool::new(opts).unwrap();
    let mut conn = pool.get_conn().unwrap();

    drop_tables(&mut conn);
    create_tables(&mut conn);

    let car = Car::parse();

    conn.exec_drop(
        r"INSERT INTO car VALUES (:id, :make)", params!{
            "id" => car.id,
            "make" => car.make
        }
    );
}

fn create_tables(conn : &mut PooledConn) {
    conn.query_drop(
        r"CREATE TABLE car (
            id INT NOT NULL,
            make text not null,
            PRIMARY KEY (id)
        )"
    );
}

fn drop_tables(conn : &mut PooledConn) {
    conn.query_drop(
        r"DROP TABLE car"
    );
}