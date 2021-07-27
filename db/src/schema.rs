table! {
    company (id) {
        id -> Integer,
    }
}

table! {
    person (id) {
        id -> Integer,
        name -> Text,
        company_id -> Integer,
    }
}

joinable!(person -> company (company_id));

allow_tables_to_appear_in_same_query!(
    company,
    person,
);
