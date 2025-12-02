use utoipa::OpenApi;
use scalar_doc::Documentation;
use axum::response::{Html, IntoResponse};
use crate::models::{Person, NewPerson};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::create_people,
        crate::handlers::get_people_by_id,
        crate::handlers::search_people,
        crate::handlers::count_people,
    ),
    components(
        schemas(
            Person,
            NewPerson,
        )
    ),
    info(
        title = "Rinha 2023",
        version = "1.0.0"
    )
)]
pub struct Apidoc;

pub async fn doc() -> impl IntoResponse {
     Html(Documentation::new("Api Documentation title", "/openapi")
        .build()
        .unwrap())
}

pub async fn openapi() -> impl IntoResponse {
      Apidoc::openapi().to_pretty_json().unwrap()
}
