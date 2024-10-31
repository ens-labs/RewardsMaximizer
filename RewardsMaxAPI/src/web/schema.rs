// Manually put together, need to figure some stuff out

diesel::table! {
    cards (card_id) {
        card_id -> Nullable<Integer>,
        company_id -> Integer,
        created -> Timestamp,
        name -> Text,
        r#type -> Text,
        updated -> Timestamp,
    }
}

diesel::table! {
    companies (company_id) {
        company_id -> Nullable<Integer>,
        contact_email -> Text,
        created -> Timestamp,
        description -> Text,
        name -> Text,
        updated -> Timestamp,
        website -> Text,
    }
}

diesel::table! {
    user_cards (user_card_id) {
        user_card_id -> Nullable<Integer>,
        added -> Timestamp,
        card_id -> Integer,
        expires_on -> Timestamp,
        updated -> Timestamp,
        user_id -> Integer,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Nullable<Integer>,
        created -> Timestamp,
        email -> Text,
        password -> Text,
        updated -> Timestamp,
        username -> Text,
    }
}

diesel::table! {
    rewards (reward_id) {
        reward_id -> Nullable<Integer>,
        company_id -> Integer,
        created -> Timestamp,
        description -> Text,
        name -> Text,
        updated -> Timestamp,
    }
}

diesel::table! {
    user_feedback (feedback_id) {
        feedback_id -> Nullable<Integer>,
        comments -> Text,
        company_id -> Integer,
        created -> Timestamp,
        rating -> Integer,
        updated -> Timestamp,
        user_id -> Integer,
    }
}

diesel::table! {
    vendor_deals (deal_id) {
        deal_id -> Nullable<Integer>,
        company_id -> Integer,
        created -> Timestamp,
        description -> Text,
        title -> Text,
        updated -> Timestamp,
        valid_from -> Date,
        valid_to -> Date,
    }
}

diesel::table! {
    comments (comment_id) {
        comment_id -> Nullable<Integer>,
        comment_info -> Text,
        created -> Timestamp,
        entity_type -> Text,
        updated -> Timestamp,
        user_id -> Integer,
    }
}

diesel::table! {
    notifications (notification_id) {
        notification_id -> Nullable<Integer>,
        created -> Timestamp,
        message -> Text,
        r#type -> Text,
        updated -> Timestamp,
        user_id -> Integer,
    }
}

diesel::table! {
    user_rewards (user_reward_id) {
        user_reward_id -> Nullable<Integer>,
        added -> Timestamp,
        expires_on -> Timestamp,
        reward_id -> Integer,
        status -> Text,
        updated -> Timestamp,
        user_id -> Integer,
    }
}




diesel::joinable!(cards -> companies (company_id));
diesel::joinable!(rewards -> companies (company_id));
diesel::joinable!(user_cards -> cards (card_id));
diesel::joinable!(user_cards -> users (user_id));
diesel::joinable!(user_feedback -> companies (company_id));
diesel::joinable!(user_feedback -> users (user_id));
diesel::joinable!(vendor_deals -> companies (company_id));
diesel::joinable!(comments -> users (user_id));
diesel::joinable!(notifications -> users (user_id));
diesel::joinable!(user_rewards -> rewards (reward_id));
diesel::joinable!(user_rewards -> users (user_id));




diesel::allow_tables_to_appear_in_same_query!(
    cards,
    companies,
    user_cards,
    users,
    rewards,
    user_feedback,
    vendor_deals,
    comments,
    notifications,
    user_rewards
);
