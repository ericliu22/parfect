use diesel::{
    Queryable,
};
use diesel::sql_types::Uuid;

#[derive(Queryable)]
pub struct User{
    pub id: Uuid,
    pub name: String
}
