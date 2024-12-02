use axum::{Json, Router};
use crate::web::models::{NewCard, Card};
use crate::web::schema::cards;
use diesel::prelude::*;
use axum::response::IntoResponse;
use crate::web::lib::establish_connection;

pub fn router() -> Router {
    Router::new()
        .route("/add_card", axum::routing::post(add_card)) // POST method for adding a card
}

async fn add_card(Json(new_card): Json<NewCard>) -> impl IntoResponse {
    use crate::web::schema::cards::dsl::*;

    let mut connection = establish_connection(); // Mutable reference for connection

    // Insert the new card into the database and return the inserted card
    let new_card = diesel::insert_into(cards)
        .values(&new_card)
        .returning((card_id, company_id, created, name, r#type, icon, color, updated)) // Explicitly select fields
        .get_result::<Card>(&mut connection) // Use mutable reference for connection
        .expect("Error saving new card");

    // Return the newly added card as a JSON response
    Json(new_card).into_response()  // Respond with the card data that was added
}
