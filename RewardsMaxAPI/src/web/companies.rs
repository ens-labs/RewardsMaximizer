use axum::{routing::get, routing::post, Router, Json, response::IntoResponse,  http::StatusCode};
use serde::Deserialize;
use diesel::{prelude::*};
use crate::web::models::{NewCompany, Company};
use crate::web::lib::establish_connection;

pub fn router() -> Router<()> {
    Router::new()
        .route("/addCompany", post(self::post::add_company))
        .route("/addCompany", get(self::get::add_company))
        .route("/viewCompanies", get(self::get::view_companies))
}

mod get {
    use super::*;
    
    pub async fn add_company() -> &'static str {
        "Add company endpoint"
    }

    //Need to fix getting from DB
    pub async fn view_companies() -> impl IntoResponse {
        // use crate::web::schema::companies::dsl::*;
        // let mut connection = establish_connection();

        // let results = companies
        // .select(crate::web::models::Company::as_select())
        //     .load::<Company>(&mut connection)
        //     .expect("Error loading companies");

        // Json(results)
    }
 }

mod post {
    use super::*;

    pub async fn add_company(Json(new_company): Json<NewCompany>) -> impl IntoResponse {
        use crate::web::schema::companies::dsl::*;

        let mut connection = establish_connection();
    
        match diesel::insert_into(companies)
            .values(&new_company)
            .execute(&mut connection)
        {
            Ok(_) => (StatusCode::OK, "data inserted into the database").into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to insert data: {}", e),
            )
            .into_response(),
        }
    }
}