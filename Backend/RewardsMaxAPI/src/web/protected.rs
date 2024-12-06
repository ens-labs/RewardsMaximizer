use askama::Template;
use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use axum_messages::{Message, Messages};
use axum_login::AuthSession;
use crate::users::Backend;

#[derive(Template)]
#[template(path = "protected.html")]
struct ProtectedTemplate<'a> {
    messages: Vec<Message>,
    username: &'a str,
}

pub fn router() -> Router<()> {
    Router::new().route("/protected", get(self::get::protected))
}

mod get {
    use super::*;

    pub async fn protected(auth_session: AuthSession<Backend>, messages: Messages) -> impl IntoResponse {
        match auth_session.user {
            Some(user) => ProtectedTemplate {
                messages: messages.into_iter().collect(),
                username: &user.username,
            }
            .into_response(),

            None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}
