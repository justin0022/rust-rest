use crate::api_error::ApiError;
use crate::db;
use crate::schema::user;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name = "user"]
pub struct UserMessage {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "user"]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl User {
  pub fn find_all() -> Result<Vec<Self>, ApiError> {
    let conn = db::connection()?;

    let users = user::table
      .load::<User>(&conn)?;

    Ok(users)
  }

  pub fn find(id: Uuid) -> Result<Self, ApiError> {
    let conn = db::connection()?;

    let user = user::table
      .filter(user::id.eq(id))
      .first(&conn)?;

    Ok(user)
  }

  pub fn create(user: UserMessage) -> Result<Self, ApiError> {
    let conn = db::connection()?;

    let user = User::from(user);
    let user = diesel::insert_into(user::table)
      .values(user)
      .get_result(&conn)?;

    Ok(user)
}

  // pub fn update(id: Uuid)
