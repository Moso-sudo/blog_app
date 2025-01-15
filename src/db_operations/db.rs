use std::pg::{connection, PgConnection};
use std::env;

pub fn establish_connection ->PgConnection{
    let database_url::env::var("DATABASE_URL").expect("database url not found");
    PgConnection::estabish(&database_url).unwrap_or_else(|_| panic!("error connecting to db"))
}