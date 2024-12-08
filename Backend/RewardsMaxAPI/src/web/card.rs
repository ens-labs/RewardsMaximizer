use axum::{Json, Router};
use crate::web::models::{NewCard, Card};
use crate::web::schema::cards;
use diesel::prelude::*;
use axum::response::{IntoResponse};
use http::StatusCode;
use crate::web::lib::establish_connection;

// This router sets up three routes for interacting with cards:
// 1. POST /add_card - Insert a new card into the database.
// 2. GET /cards - Retrieve all cards from the database.
// 3. DELETE /delete_card/:id - Delete a specific card by its ID.
pub fn router() -> Router {
    Router::new()
        .route("/add_card", axum::routing::post(add_card))    // POST method for adding a card
        .route("/cards", axum::routing::get(get_cards))        // GET method for retrieving all cards
        .route("/delete_card/:id", axum::routing::delete(delete_card)) // DELETE method for removing a card
}

// POST method to add a card.
// Since we're using SQLite, and depending on Diesel version/feature flags,
async fn add_card(Json(new_card): Json<NewCard>) -> impl IntoResponse {
    use crate::web::schema::cards::dsl::*;
    let mut connection = establish_connection();

    // First, insert the new card
    match diesel::insert_into(cards)
        .values(&new_card)
        .execute(&mut connection)
    {
        Ok(_) => {
            // If insertion succeeded, fetch the newly inserted card.
            match cards.order(card_id.desc()).first::<Card>(&mut connection) {
                Ok(inserted_card) => Json(inserted_card).into_response(),
                Err(e) => {
                    println!("Error fetching inserted card: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        },
        Err(e) => {
            println!("Error saving card: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

// GET method to retrieve all cards
async fn get_cards() -> impl IntoResponse {
    use crate::web::schema::cards::dsl::*;
    let mut connection = establish_connection();

    // Fetch all cards
    match cards
        .select((card_id, company_id, created, name, r#type, icon, color, updated))
        .load::<Card>(&mut connection) 
    {
        Ok(results) => Json(results).into_response(),
        Err(e) => {
            println!("Error loading cards: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

// DELETE method to remove a card by card_id
async fn delete_card(axum::extract::Path(card_id_val): axum::extract::Path<i32>) -> impl IntoResponse {
    use crate::web::schema::cards::dsl::*;
    
    let mut connection = establish_connection();

    // Delete the specified card
    match diesel::delete(cards.filter(card_id.eq(card_id_val)))
        .execute(&mut connection)
    {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            println!("Error deleting card: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
