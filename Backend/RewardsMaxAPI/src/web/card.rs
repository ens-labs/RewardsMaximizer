use axum::{
    routing::{get, post, delete},
    Json, Router,
    extract::{Extension, Path},
    response::IntoResponse,
    http::StatusCode,
};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use crate::web::models::{NewCard, Card};
use crate::web::schema::cards;

type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub fn router() -> Router {
    Router::new()
        .route("/add_card", post(add_card))
        .route("/cards", get(get_cards))
        .route("/card/:cardId", get(get_card_by_id))
        .route("/delete_card/:id", delete(delete_card))
}

#[axum::debug_handler]
async fn add_card(
    Extension(pool): Extension<DbPool>,
    Json(new_card): Json<NewCard>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut conn = pool.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match diesel::insert_into(cards::table)
        .values(&new_card)
        .returning(Card::as_select())
        .get_result::<Card>(&mut conn)
    {
        Ok(card) => Ok((StatusCode::CREATED, Json(card)).into_response()),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[axum::debug_handler]
async fn get_cards(
    Extension(pool): Extension<DbPool>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut conn = pool.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match cards::table
        .select(Card::as_select())
        .load::<Card>(&mut conn)
    {
        Ok(results) => Ok(Json(results).into_response()),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[axum::debug_handler]
async fn get_card_by_id(
    Path(card_id): Path<i32>,
    Extension(pool): Extension<DbPool>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut conn = pool.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match cards::table
        .filter(cards::card_id.eq(card_id))
        .select(Card::as_select())
        .first::<Card>(&mut conn)
    {
        Ok(card) => Ok(Json(card).into_response()),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

#[axum::debug_handler]
async fn delete_card(
    Path(card_id_val): Path<i32>,
    Extension(pool): Extension<DbPool>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut conn = pool.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match diesel::delete(cards::table.filter(cards::card_id.eq(card_id_val)))
        .execute(&mut conn)
    {
        Ok(affected_rows) => {
            if affected_rows > 0 {
                Ok(StatusCode::NO_CONTENT.into_response())
            } else {
                Err(StatusCode::NOT_FOUND)
            }
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
