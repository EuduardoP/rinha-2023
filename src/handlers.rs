use axum::{
    extract::{Path, State, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;
use validator::Validate;
use crate::models::{AppState, NewPerson, Person};
use crate::db::potsgres_repository::PostgresRepository;

#[derive(Deserialize)]
pub struct SearchParams {
    pub t: String,
}

#[utoipa::path(
    post,
    path = "/pessoas",
    tag = "Pessoas",
    request_body(
        content = NewPerson,
        description = "Payload para editar usuário",
        content_type = "application/json",
    )
)]
/// Criar pessoas
pub async fn create_people(
     State(pool): State<AppState>,
    Json(new_person): Json<NewPerson>
) -> impl IntoResponse {
    if let Err(e) = new_person.validate() {
        return (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(serde_json::json!({
                "error": "Validation failed",
                "details": e.to_string()
            }))
        );
    }

    let id = Uuid::now_v7();
    let person = Person {
        id,
        name: new_person.name,
        birthday: new_person.birthday,
        nick: new_person.nick,
        stack: new_person.stack.map(|s| s.into_iter().map(Some).collect()),
    };

    let conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "DB connection failed"}))),
    };

    let mut repo = PostgresRepository { conn };

    match repo.create_people(person.clone()) {
        Ok(created_person) => (StatusCode::CREATED, Json(serde_json::to_value(created_person).unwrap())),
        Err(_) => (StatusCode::UNPROCESSABLE_ENTITY, Json(json!({"error": "Unprocessable Entity"}))), // Usually constraint violation (duplicate nick)
    }
}

#[utoipa::path(
    get,
    path = "/pessoas/{id}",
    tag = "Pessoas",
    params(
        ("id" = Uuid, Path, description = "User ID")
    )
)]
/// Buscar pessoa por ID
pub async fn get_people_by_id(
    State(pool): State<AppState>,
    Path(person_id): Path<Uuid>
) -> impl IntoResponse {
    let conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let mut repo = PostgresRepository { conn };

    match repo.get_people_by_id(person_id) {
        Ok(person) => Ok(Json(person)),
        Err(diesel::result::Error::NotFound) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[utoipa::path(
    get,
    path = "/pessoas",
    tag = "Pessoas",
    params(
        ("t" = String, Query, description = "Search term")
    )
)]
/// Buscar pessoas
pub async fn search_people(
    State(pool): State<AppState>,
    Query(params): Query<SearchParams>,
) -> impl IntoResponse {
    let conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "DB connection failed"}))),
    };

    let mut repo = PostgresRepository { conn };

    match repo.search_people(&params.t) {
        Ok(people) => (StatusCode::OK, Json(serde_json::to_value(people).unwrap())),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Internal Server Error"}))),
    }
}

#[utoipa::path(
    get,
    path = "/contagem-pessoas",
    tag = "Pessoas",
)]
/// Contagem de pessoas
pub async fn count_people(
    State(pool): State<AppState>,
) -> impl IntoResponse {
    let conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "DB connection failed"}))),
    };

    let mut repo = PostgresRepository { conn };

    match repo.count_people() {
        Ok(count) => (StatusCode::OK, Json(count.into())),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Internal Server Error"}))),
    }
}
