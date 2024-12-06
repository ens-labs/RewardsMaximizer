use crate::web::lib::establish_connection;
use diesel::prelude::*;
use async_trait::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use password_auth::verify_password;
use serde::{Deserialize, Serialize};
use tokio::task;
use crate::web::models::User;
use crate::web::schema::users;
use axum_login::AuthSession;
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::{SaltString, rand_core::OsRng};

#[derive(Debug, Clone)]
pub struct Backend;

impl Backend {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
    pub next: Option<String>,
}

impl AuthUser for User {
    type Id = String; // Or another type that matches your User ID type

    fn id(&self) -> Self::Id {
        self.id()
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password.as_bytes() // Or whatever you use to generate the session hash
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Database error")]
    Diesel(#[from] diesel::result::Error),

    #[error(transparent)]
    TaskJoin(#[from] task::JoinError),
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = User;
    type Credentials = Credentials;
    type Error = Error;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        use crate::web::schema::users::dsl::users;
        use crate::web::schema::users::username;
        use crate::web::schema::users::user_id;
        use crate::web::schema::users::password;

        let creds_username = creds.username.clone();
        println!("username: {}, password: {}", creds_username, creds.password);
        let user: Option<User> = task::spawn_blocking(move || {
            let mut conn = establish_connection();
            users
                .filter(username.eq(creds_username))
                .select(User::as_select())
                .first::<User>(&mut conn)
                .optional()
        })
        .await??;

        println!("User fetched from DB");

        // task::spawn_blocking(|| {
        //     Ok(user.filter(|user| verify_password(creds.password, &user.password).is_ok()))
        // })
        // .await?

        Ok(user)
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
       use crate::web::schema::users::dsl::*;

        let user_id_clone = user_id.clone();
        let user = task::spawn_blocking(move || {
            let mut conn = establish_connection(); // Direct connection usage
            users.filter(user_id.eq(user_id_clone)).first::<User>(&mut conn).optional()
        })
        .await??;

        Ok(user)
    }
}

//pub type AuthSession = axum_login::AuthSession<Backend>; 