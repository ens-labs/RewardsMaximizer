use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, NaiveDateTime};
use diesel::sql_types::Timestamp;
use crate::web::schema;

//Figure out insertable issues
//#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[derive(Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = schema::users)]
pub struct User {
    pub user_id: i32,
    pub created: chrono::NaiveDateTime,
    pub email: String,
    pub password: String,
    pub updated: chrono::NaiveDateTime,
    pub username: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::user_cards)]
pub struct UserCard {
    pub user_card_id: i32,
    pub added: NaiveDateTime,
    pub card_id: i32,
    pub expires_on: NaiveDateTime,
    pub updated: NaiveDateTime,
    pub user_id: i32,
}

//Figure out insertable issues
//#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[derive(Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = schema::cards)]
pub struct Card {
    pub card_id: i32,
    pub company_id: i32,
    pub created: chrono::NaiveDateTime,
    pub name: String,
    pub r#type: String,
    pub updated: chrono::NaiveDateTime,
}


#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::companies)]
pub struct Company {
    pub company_id: i32,
    pub contact_email: String,
    pub created: NaiveDateTime,
    pub description: String,
    pub name: String,
    pub updated: NaiveDateTime,
    pub website: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::rewards)]
pub struct Reward {
    pub reward_id: i32,
    pub company_id: i32,
    pub created: NaiveDateTime,
    pub description: String,
    pub name: String,
    pub updated: NaiveDateTime,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::user_feedback)]
pub struct UserFeedback {
    pub feedback_id: i32,
    pub comments: String,
    pub company_id: i32,
    pub created: NaiveDateTime,
    pub rating: i32,
    pub updated: NaiveDateTime,
    pub user_id: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::vendor_deals)]
pub struct VendorDeal {
    pub deal_id: i32,
    pub company_id: i32,
    pub created: NaiveDateTime,
    pub description: String,
    pub title: String,
    pub updated: NaiveDateTime,
    pub valid_from: NaiveDate,
    pub valid_to: NaiveDate,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::comments)]
pub struct Comment {
    pub comment_id: i32,
    pub comment_info: String,
    pub created: NaiveDateTime,
    pub entity_type: String,
    pub updated: NaiveDateTime,
    pub user_id: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::notifications)]
pub struct Notification {
    pub notification_id: i32,
    pub created: NaiveDateTime,
    pub message: String,
    pub r#type: String,
    pub updated: NaiveDateTime,
    pub user_id: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::user_rewards)]
pub struct UserReward {
    pub user_reward_id: i32,
    pub added: NaiveDateTime,
    pub expires_on: NaiveDateTime,
    pub reward_id: i32,
    pub status: String,
    pub updated: NaiveDateTime,
    pub user_id: i32,
}