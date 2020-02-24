use crate::api_error::ApiError;
use crate::user::{User, UserMessage};
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;
use uuid::Uuid;

#[get("/users")]
async fn find_all() -> Result<HttpResponse, ApiError> {
  let users = User::find_all()?;
  Ok(HttpResponse::Ok().json(users))
}

#[get("/users/{id}")]
async fn find(id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
  let user = User::find(id.into_inner())?;
  Ok(HttpResponse::Ok().json(user))
}

#[post("/users")]
async fn create(user: web::Json<User>) -> impl Responder {
  HttpResponse::Created().json(user.into_inner())
}

#[put("/users/{id}")]
async fn update(user: web::Json<User>) -> impl Responder {
  HttpResponse::Ok().json(user.into_inner())
}

#[delete("/users/{id}")]
async fn delete() -> impl Responder {
  HttpResponse::Ok().json(json!({"message": "Deleted"}))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(find_all);
  cfg.service(find);
  cfg.service(create);
  cfg.service(update);
  cfg.service(delete);
}