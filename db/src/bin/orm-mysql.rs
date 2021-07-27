use db::*;


fn main() {

    use db::schema::person::dsl::*;
    let connection = establish_connection();

    let company_a = Company{id: 1};
    let company_1 = create_company(&connection, 1);
    println!("Rows affected {}", company_1);
    let bob = create_person(&connection, 1, String::from("Bob"), company_a.id);
    println!("Rows affected {}", bob);
    let philip = create_person(&connection, 2, String::from("Philip"), company_a.id);
    println!("Rows affected {}", philip);
    let alice = create_person(&connection, 3, String::from("Alice"), company_a.id);
    println!("Rows affected {}", alice);


    // let results = person.filter(company_id.eq(1))
    //     .limit(3)
    //     .load::<Person>(&connection)
    //     .expect("Error loading persons");
    //
    // println!("Displaying {} persons", results.len());
    //
    // for p in results {
    //     println!("{}", p.name);
    //     println!("----------\n");
    //     println!("{}", p.company_id);
    // }
}