// Allows visibility of app outside
pub use app::App;

// Modules
mod app;
mod auth;
mod protected;
pub mod user;
mod index;
pub mod lib;
pub mod models;
pub mod schema;
mod recommendations;
mod companies;
mod vendor_deals;
mod crowdsourcing;
pub mod card;
pub use crate::web::lib::establish_connection;
