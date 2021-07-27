#[derive(Queryable)]
pub struct Person {
    pub id : u64,
    pub name : String,
    pub company_id : u64,
}

#[derive(Queryable)]
pub struct Company {
    pub id : u64,
}

