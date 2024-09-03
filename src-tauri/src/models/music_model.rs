use diesel::prelude::*;
use serde::{Serialize,Deserialize};

#[derive(Insertable)]
#[diesel(table_name = crate::schema::music)]
pub struct NewMusic<'a> {
    pub title: &'a str,
    pub artist: &'a str,
    pub path: &'a str,
}

#[derive(Debug, Queryable, AsChangeset)]
#[diesel(table_name = crate::schema::music)]
pub struct Music {
  pub id: i32,
  pub title: String,
  pub artist: String,
  pub path: String,
}

#[derive(Serialize, Deserialize)]
pub struct MusicArg{
  pub title: Option<String>,
  pub artist: Option<String>,
  pub path:Option<String>,
}