use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use diesel::sql_types::Timestamp;
use crate::web::schema;

#[derive(Clone, Serialize, Deserialize, Queryable)]
#[diesel(table_name = schema::users)]
pub struct User {
    pub user_id: i32,
    pub created: String,
    pub email: String,
    pub password: String,
    pub updated: String,
    pub username: String,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = schema::users)]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub username: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::user_cards)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserCard {
    pub user_card_id: i32,
    pub added: String,
    pub card_id: i32,
    pub expires_on: String,
    pub updated: String,
    pub user_id: i32,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = schema::cards)]
pub struct Card {
    pub card_id: Option<i32>,  // Make card_id optional to handle nullable
    pub company_id: i32,
    pub created: String,
    pub name: String,
    pub r#type: String,
    pub icon: String,
    pub color: String,
    pub updated: String,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = schema::cards)]
pub struct NewCard {
    pub company_id: i32,
    pub name: String,
    pub r#type: String,  // Rename to match your frontend field
    pub icon: String,
    pub color: String,
    pub created: String,  // Or use a Date type if necessary
    pub updated: String,  // Or use a Date type if necessary
}

#[derive(QueryableByName, Serialize, Selectable)]
#[diesel(table_name = schema::companies)]
pub struct Company {
    pub company_id: i32,
    pub name: String,
    pub description: String,
    pub website: String,
    pub contact_email: String,
    pub created: String,
    pub updated: String,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, Selectable)]
#[diesel(table_name = schema::companies)]
pub struct NewCompany {
    pub contact_email: String,
    pub description: String,
    pub name: String,
    pub website: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::rewards)]
pub struct Reward {
    pub reward_id: i32,
    pub company_id: i32,
    pub created: String,
    pub description: String,
    pub name: String,
    pub updated: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::user_feedback)]
pub struct UserFeedback {
    pub feedback_id: i32,
    pub comments: String,
    pub company_id: i32,
    pub created: String,
    pub rating: i32,
    pub updated: String,
    pub user_id: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::vendor_deals)]
pub struct VendorDeal {
    pub deal_id: i32,
    pub company_id: i32,
    pub created: String,
    pub description: String,
    pub title: String,
    pub updated: String,
    pub valid_from: String,
    pub valid_to: String,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, Selectable)]
#[diesel(table_name = schema::vendor_deals)]
pub struct NewVendorDeal {
    pub company_id: i32,
    pub description: String,
    pub title: String,
    // pub valid_from: String,
    // pub valid_to: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::comments)]
pub struct Comment {
    pub comment_id: i32,
    pub comment_info: String,
    pub created: String,
    pub entity_type: String,
    pub updated: String,
    pub user_id: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::notifications)]
pub struct Notification {
    pub notification_id: i32,
    pub created: String,
    pub message: String,
    pub r#type: String,
    pub updated: String,
    pub user_id: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::user_rewards)]
pub struct UserReward {
    pub user_reward_id: i32,
    pub added: String,
    pub expires_on: String,
    pub reward_id: i32,
    pub status: String,
    pub updated: String,
    pub user_id: i32,
}
