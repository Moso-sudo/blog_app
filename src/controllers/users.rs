use diesel::prelude::*
use serde::{Deserialize, Serialize}

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name  crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct User{
    pub id :i32,
    pub name :String,
    pub email:String,
    pub updated_at:chrono:NaiveDateTime,
    pub created_at:chrono:NaiveDateTime,
}