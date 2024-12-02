// Backend/RewardsMaxAPI/src/web/card.rs

use axum::{Json, Router};
use crate::web::models::{NewCard, Card};
use crate::web::schema::cards;
use diesel::prelude::*;
use axum::response::IntoResponse;

pub fn router() -> Router {
    Router::new()
        .route("/add_card", axum::routing::post(add_card)) // POST method for adding a card
}

async fn add_card(Json(new_card): Json<NewCard>) -> impl IntoResponse {
    use crate::web::schema::cards::dsl::*;

    // Establish DB connection (replace with your DB logic)
    let connection = crate::establish_connection(); // Implement a function to establish DB connection

    // Insert the new card into the database
    let new_card = diesel::insert_into(cards)
        .values(&new_card)
        .get_result::<Card>(&connection)
        .expect("Error saving new card");

    // Return a success response (adjust as needed)
    Json(new_card).into_response()  // Respond with the card data that was added
}
