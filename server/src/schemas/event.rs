use diesel::{
    Queryable
};
use diesel::sql_types::Uuid;

#[derive(Queryable)]
pub struct Event{
    pub id: Uuid,
    pub name: String
}
