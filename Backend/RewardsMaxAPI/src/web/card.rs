use axum::{Json, Router};
use crate::web::models::{NewCard, Card};  // Make sure to import NewCard
use crate::web::schema::cards;
use diesel::prelude::*;
use axum::response::IntoResponse;
use crate::web::lib::establish_connection;

pub fn router() -> Router {
    Router::new()
        .route("/add_card", axum::routing::post(add_card)) // POST method for adding a card
        .route("/cards", axum::routing::get(get_cards)) // GET method for retrieving all cards
}

// POST method to add a card
async fn add_card(Json(new_card): Json<NewCard>) -> impl IntoResponse {
    use crate::web::schema::cards::dsl::*;

    let mut connection = establish_connection();

    let new_card = diesel::insert_into(cards)
        .values(&new_card)
        .returning((card_id, company_id, created, name, r#type, icon, color, updated))
        .get_result::<Card>(&mut connection)
        .expect("Error saving new card");

    Json(new_card).into_response()
}

// GET method to retrieve all cards
async fn get_cards() -> impl IntoResponse {
    use crate::web::schema::cards::dsl::*;

    let mut connection = establish_connection();

    let results = cards
        .select((card_id, company_id, created, name, r#type, icon, color, updated))
        .load::<Card>(&mut connection)
        .expect("Error loading cards");

    Json(results).into_response()  // Returns the list of cards as a JSON response
}
