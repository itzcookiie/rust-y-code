-- Your SQL goes here
CREATE TABLE person (
    id int not null,
    name text not null,
    company_id int not null,
    PRIMARY KEY (id),
    FOREIGN KEY (company_id) REFERENCES company(id)
)