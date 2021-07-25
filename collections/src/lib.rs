#[derive(Copy, Clone, Debug)]
pub enum Color {
    Green,
    Blue
}

#[derive(Copy, Clone, Debug)]
pub enum Protocol {
    Http,
    Ws,
    Udp
}

#[derive(Copy, Clone, Debug)]
pub enum Country {
    UK,
    US,
    France,
    Denmark
}

#[derive(Debug)]
pub struct Obj {
    pub color: Color,
    pub protocol : Protocol,
    pub country : Country,
    pub id : i64
}

pub fn ids()->Vec<i64>{
    (0..10000000).into_iter().collect::<Vec<_>>()
}