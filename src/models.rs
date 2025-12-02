use serde::{Deserialize, Serialize};
use time::{Date, serde::format_description};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;
use std::sync::Arc;
use crate::utils::validate_stack;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;


format_description!(date_format, Date, "[year]-[month]-[day]");

#[derive(Serialize, Clone, ToSchema, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::people)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Person {
    pub id: Uuid,
    #[serde(rename = "apelido")]
    pub nick: String,
    #[serde(rename = "nome")]
    pub name: String,
    #[serde(rename = "nascimento", with = "date_format")]
    pub birthday: Date,
    pub stack: Option<Vec<Option<String>>>
}

#[derive(Deserialize, Clone, ToSchema, Validate)]
pub struct NewPerson {
    #[serde(rename = "apelido")]
    #[validate(length(min = 1, max = 32, message = "Apelido Inválido"))]
    pub nick: String,
    #[serde(rename = "nome")]
    #[validate(length(min = 1, max = 100, message = "Nome Inválido"))]
    pub name: String,
    #[serde(rename = "nascimento", with = "date_format")]
    pub birthday: Date,
    #[validate(custom(function = "validate_stack", message = "Stack Inválida"))]
    pub stack: Option<Vec<String>>
}

pub type AppState = Arc<Pool<ConnectionManager<PgConnection>>>;
