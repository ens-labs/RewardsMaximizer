use axum::{routing::get, Router, response::IntoResponse};
//use axum::{routing::get, routing::post, Router, Json, response::IntoResponse,  http::StatusCode};
use askama::Template;
//use crate::web::lib::establish_connection;

#[derive(Template)]
#[template(path = "crowdsourcing.html")]
struct CrowdsourcingTemplate;

pub fn router() -> Router<()> {
    Router::new()
        .route("/crowdsourcing", get(self::get::view_crowdsourcing))
}

mod get {
    use super::*;

    pub async fn view_crowdsourcing() -> impl IntoResponse {
        CrowdsourcingTemplate.into_response();
    }
 }

// mod post {
//     use super::*;
// }