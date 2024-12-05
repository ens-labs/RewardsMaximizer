use axum::{Json, Router};
use crate::web::models::{NewCard, Card};  // Ensure Card and NewCard are imported
use crate::web::schema::cards;
use diesel::prelude::*;
use axum::response::IntoResponse;
use crate::web::lib::establish_connection;
use axum::extract::Path;  // Import Path for path parameters

pub fn router() -> Router {
    Router::new()
        .route("/add_card", axum::routing::post(add_card)) // POST method for adding a card
        .route("/delete_card/:card_id", axum::routing::delete(delete_card)) // DELETE method for deleting a card
}

// POST method to add a card
async fn add_card(Json(new_card): Json<NewCard>) -> impl IntoResponse {
    use crate::web::schema::cards::dsl::*;

    let mut connection = establish_connection(); // Mutable reference for connection

    // Insert the new card into the database and return the inserted card
    let inserted_card = diesel::insert_into(cards)
        .values(&new_card)
        .returning((card_id, company_id, created, name, r#type, icon, color, updated))
        .get_result::<Card>(&mut connection)
        .expect("Error saving new card");

    // Return the newly added card as a JSON response
    Json(inserted_card).into_response()  // Respond with the card data that was added
}

// DELETE method to delete a card by ID
async fn delete_card(Path(card_id): Path<i32>) -> impl IntoResponse {
    use crate::web::schema::cards::dsl::*;

    let mut connection = establish_connection();
    
    // Perform the delete operation
    let deleted_card = diesel::delete(cards.filter(card_id.eq(card_id)))
        .execute(&mut connection)
        .expect("Error deleting card");

    if deleted_card > 0 {
        Json("Card deleted successfully").into_response()
    } else {
        Json("Card not found").into_response()
    }
}
