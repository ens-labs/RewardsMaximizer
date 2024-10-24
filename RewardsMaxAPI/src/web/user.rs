use axum::{routing::get, routing::post, Router};

//use crate::users::AuthSession;

// #[derive(Template)]
// #[template(path = "cards.html")]

pub fn router() -> Router<()> {
    Router::new()
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
    //use super::*;

    pub async fn add_card() -> &'static str {
        "[post] Add card endpoint"
    }

    pub async fn  delete_card() -> &'static str {
        "Deleting cards endpoint"
    }

}
