use axum::{routing::get, routing::post, Router, Json, response::IntoResponse,  http::StatusCode};
use serde::Deserialize;
use diesel::{prelude::*};
use crate::web::models::{NewVendorDeal};
use crate::web::lib::establish_connection;


pub fn router() -> Router<()> {
    Router::new()
        .route("/addVendorDeals", post(self::post::add_vendor_deals))
        .route("/addVendorDeals", get(self::get::view_vendor_deals))

}

mod get {
    use super::*;

    pub async fn view_vendor_deals() -> &'static str  {
        "View Vendor deals endpoint"
    }
 }

mod post {
    use super::*;

    pub async fn add_vendor_deals(Json(new_vendor_deal): Json<NewVendorDeal>) -> impl IntoResponse {
        use crate::web::schema::vendor_deals::dsl::*;
        let mut connection = establish_connection();
    
        match diesel::insert_into(vendor_deals)
            .values(&new_vendor_deal)
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