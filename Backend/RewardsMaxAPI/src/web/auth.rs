use askama::Template;
use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Form, Router,
};
use axum_messages::{Message, Messages};
use serde::Deserialize;

use crate::users::{AuthSession, Backend, Credentials};

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    messages: Vec<Message>,
    next: Option<String>,
}

#[derive(Template)]
#[template(path = "signup.html")]
pub struct SignupTemplate {
    messages: Vec<Message>,
    next: Option<String>,
}



// This allows us to extract the "next" field from the query string. We use this
// to redirect after log in.
#[derive(Debug, Deserialize)]
pub struct NextUrl {
    next: Option<String>,
}

pub fn router() -> Router<()> {
    Router::new()
        .route("/login", post(self::post::login))
        .route("/login", get(self::get::login))
        .route("/logout", get(self::get::logout))
        // .route("/signup", post(self::post::signup))
        // .route("/signup", get(self::get::signup))

}

mod post {
    use super::*;

    pub async fn login(
        mut auth_session: AuthSession,
        messages: Messages,
        Form(creds): Form<Credentials>,
    ) -> impl IntoResponse {
        let user = match auth_session.authenticate(creds.clone()).await {
            Ok(Some(user)) => user,
            Ok(None) => {
                messages.error("Invalid credentials");

                let mut login_url = "/login".to_string();
                if let Some(next) = creds.next {
                    login_url = format!("{}?next={}", login_url, next);
                };

                return Redirect::to(&login_url).into_response();
            }
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };

        if auth_session.login(&user).await.is_err() {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }

        messages.success(format!("Successfully logged in as {}", user.username));

        if let Some(ref next) = creds.next {
            Redirect::to(next)
        } else {
            Redirect::to("/")
        }
        .into_response()
    }



//     pub async fn signup(
//         mut auth_session: AuthSession,
//         messages: Messages,
//         Form(creds): Form<Credentials>,
//     ) -> impl IntoResponse {
//     // Check if the username is already taken
//     // Insert diesel code to query existing usernames


//     // Check if password is same as confirm password
    


//     // Create a new user and store it in the database
//     // Insert diesel code to store in database

//     // Log the user in after successful signup
//     //if auth_session.login(&user).await.is_err() {
//     //    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
//     //}

//     messages.success(format!("Successfully signed up as" /*user.username*/));

//     if let Some(ref next) = creds.next {
//         Redirect::to(next)
//     } else {
//         Redirect::to("/")
//     }
//     .into_response()
//     }
}

mod get {
    use super::*;

    pub async fn login(
        messages: Messages,
        Query(NextUrl { next }): Query<NextUrl>,
    ) -> LoginTemplate {
        LoginTemplate {
            messages: messages.into_iter().collect(),
            next,
        }
    }

    pub async fn logout(mut auth_session: AuthSession) -> impl IntoResponse {
        match auth_session.logout().await {
            Ok(_) => Redirect::to("/login").into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    // pub async fn signup(
    //     messages: Messages,
    //     Query(NextUrl { next }): Query<NextUrl>,
    // ) -> SignupTemplate {
    //     SignupTemplate {
    //         messages: messages.into_iter().collect(),
    //         next,
    //     }
    // }
}
    

