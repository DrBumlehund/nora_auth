use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub salt: String,
    pub full_name: String,
    pub created_time: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser<'a> {
    pub id: &'a String,
    pub username: &'a String,
    pub password_hash: &'a String,
    pub salt: &'a String,
    pub full_name: &'a String,
    pub created_time: &'a NaiveDateTime,
}
