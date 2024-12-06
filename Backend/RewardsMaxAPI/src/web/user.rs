use axum::{routing::get, routing::post, Router, Json, response::IntoResponse};
use serde::Deserialize;
use diesel::{prelude::*, sqlite::SqliteConnection};
use crate::web::models::{NewUser, NewCard, UserCard};
use crate::web::lib::establish_connection;
use password_auth::generate_hash;
//use crate::users::AuthSession;

pub fn router() -> Router<()> {
    Router::new()
        .route("/signup", post(self::post::signup_user)) // Udpate and include auth session
        .route("/user/dashboard", get(self::get::dashboard)) //update
        .route("/user/cards", get(self::get::cards)) //update for logged-in user: /:user_id/cards
        .route("/user/addCard", get(self::get::add_card)) // update
        .route("/user/addCard", post(self::post::add_card)) // update
        .route("/user/deleteCard", get(self::get::delete_card)) // update
        .route("/user/deleteCard", post(self::post::delete_card)) // update
}

mod get {
    use super::*;

    pub async fn dashboard() -> &'static str {
        "Dashboard endpoint"
    }

    pub async fn cards() {
        use crate::web::schema::user_cards::dsl::*;

        let mut connection = establish_connection();

        // Need to update to specify user_id based on AuthSession
        let results = user_cards.filter(user_id.eq(1)).limit(5).select(UserCard::as_select()).load(&mut connection).expect("Error loading cards");
    }

    pub async fn add_card() -> &'static str {
        "DISPLAY TEMPLATE FOR ADDING CARD"
    }

    pub async fn delete_card() -> &'static str {
        "DISPLAY TEMPLATE FOR DELETING CARD"
    }
 }

mod post {
    use super::*;

    pub async fn signup_user(Json(mut new_user): Json<NewUser>) -> impl IntoResponse {
        use crate::web::schema::users::dsl::*;
        let mut connection = establish_connection();

        let hashed_password = generate_hash(&new_user.password);
        new_user.password = hashed_password;

        match diesel::insert_into(users)
            .values(&new_user)
            .execute(&mut connection)
        {
            Ok(_) => (axum::http::StatusCode::OK, "data inserted into the database").into_response(),
            Err(e) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to insert data: {}", e),
            )
            .into_response(),
        }
    }

    pub async fn add_card(Json(new_card): Json<NewCard>) -> impl IntoResponse {
        use crate::web::schema::cards::dsl::*;

        let mut connection = establish_connection();

        match diesel::insert_into(cards)
            .values(&new_card)
            .execute(&mut connection)
        {
            Ok(_) => (axum::http::StatusCode::OK, "data inserted into the database").into_response(),
            Err(e) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to insert data: {}", e),
            )
            .into_response(),
        }
    }

    // TODO:
    // Need to update based on AuthSession and given user_card_id
    pub async fn  delete_card() -> impl IntoResponse {
        use crate::web::schema::user_cards::dsl::*;

        let mut connection = establish_connection();

        match diesel::delete(
            user_cards.filter(user_id.eq(1).and(user_card_id.eq(1)))
        )
            .execute(&mut connection)
        {
            Ok(_) => (axum::http::StatusCode::OK, "data removed into the database").into_response(),
            Err(e) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to remove data: {}", e),
            )
            .into_response(),
        }
    }

}
