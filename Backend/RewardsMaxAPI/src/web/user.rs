use axum::{routing::post, Router, Json, response::IntoResponse};
use diesel::{prelude::*};
use crate::web::models::{NewUser};
use crate::web::lib::establish_connection;
use password_auth::generate_hash;

pub fn router() -> Router<()> {
    Router::new()
        .route("/signup", post(self::post::signup_user))
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
            Ok(_) => (axum::http::StatusCode::OK, "user inserted into the database").into_response(),
            Err(e) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to insert user: {}", e),
            )
            .into_response(),
        }
    }
}
