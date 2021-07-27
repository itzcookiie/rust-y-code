use db::Company;

fn main() {


    Company::mysql_insert(vec![
        Company{id : 10},
        Company{id : 20}
    ], "mysql://mayowa:abc123@localhost:3306/rust");
}

