use axum::{routing::get, routing::post, Router, Json, response::IntoResponse};
use serde::Deserialize;
use diesel::{prelude::*, sqlite::SqliteConnection};
use crate::web::models::{NewUser, NewCard};
use crate::web::lib::establish_connection;
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
    //use super::*;
    
    pub async fn dashboard() -> &'static str {
        "Dashboard endpoint"
    }

    pub async fn cards() -> &'static str {
        "Hello, credit cards!"
    }

    pub async fn add_card() -> &'static str {
        "Adding cards endpoint"
    }

    pub async fn delete_card() -> &'static str {
        "Delete cards endpoint"
    }
 }

mod post {
    use super::*;

    pub async fn signup_user(Json(new_user): Json<NewUser>) -> impl IntoResponse {
        use crate::web::schema::users::dsl::*;

        let mut connection = establish_connection();
    
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

    pub async fn  delete_card() -> &'static str {
        "Deleting cards endpoint"
    }

}
