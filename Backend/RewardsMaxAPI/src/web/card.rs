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

// Define a type alias for the database pool
type DbPool = Pool<ConnectionManager<SqliteConnection>>;

// Router for card-related routes
pub fn router() -> Router {
    Router::new()
        .route("/add_card", post(add_card))
        .route("/cards", get(get_cards))
        .route("/delete_card/:id", delete(delete_card))
}

// POST method to add a card
#[axum::debug_handler]
async fn add_card(
    Extension(pool): Extension<DbPool>,
    Json(new_card): Json<NewCard>,
) -> impl IntoResponse {
    let mut conn = pool.get().expect("Failed to get DB connection");

    match diesel::insert_into(cards::table)
        .values(&new_card)
        .get_result::<Card>(&mut conn)
    {
        Ok(card) => (StatusCode::CREATED, Json(card)).into_response(),
        Err(e) => {
            eprintln!("Error inserting card: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to add card").into_response()
        }
    }
}

// GET method to retrieve all cards
#[axum::debug_handler]
async fn get_cards(
    Extension(pool): Extension<DbPool>,
) -> impl IntoResponse {
    let mut conn = pool.get().expect("Failed to get DB connection");

    match cards::table.load::<Card>(&mut conn) {
        Ok(results) => Json(results).into_response(),
        Err(e) => {
            eprintln!("Error loading cards: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch cards").into_response()
        }
    }
}

// DELETE method to remove a card by ID
#[axum::debug_handler]
async fn delete_card(
    Path(card_id_val): Path<i32>,
    Extension(pool): Extension<DbPool>,
) -> impl IntoResponse {
    let mut conn = pool.get().expect("Failed to get DB connection");

    match diesel::delete(cards::table.filter(cards::card_id.eq(card_id_val)))
        .execute(&mut conn)
    {
        Ok(affected_rows) => {
            if affected_rows > 0 {
                StatusCode::NO_CONTENT.into_response()
            } else {
                (StatusCode::NOT_FOUND, "Card not found").into_response()
            }
        }
        Err(e) => {
            eprintln!("Error deleting card: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete card").into_response()
        }
    }
}
