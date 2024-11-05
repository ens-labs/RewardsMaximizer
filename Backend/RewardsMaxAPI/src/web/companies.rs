use axum::{routing::get, routing::post, Router, Json, response::IntoResponse,  http::StatusCode};
use serde::Deserialize;
use diesel::{sql_query, prelude::*};
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

    // Able to read from DB with straight SQL, plz leave it alone
    // diesel -> NO BUENO, NO LIKE!
    pub async fn view_companies() -> impl IntoResponse {
        use crate::web::schema::companies::dsl::*;
        let mut connection = establish_connection();

        let results = sql_query("SELECT * FROM companies").load::<Company>(&mut connection); 

        match results {
            Ok(results) => (StatusCode::OK, Json(results)).into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to retrieve data: {}", e),
            )
            .into_response(),
        }
        
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