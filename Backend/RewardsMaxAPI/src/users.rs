use async_trait::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use bcrypt::{hash, verify};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::task;

use crate::schema::users;

#[derive(Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    id: i64,
    pub username: String,
    password: String,
}

// Here we've implemented `Debug` manually to avoid accidentally logging the
// password hash.
impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("username", &self.username)
            .field("password", &"[redacted]")
            .finish()
    }
}

impl AuthUser for User {
    type Id = i64;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password.as_bytes() // We use the password hash as the auth
                                 // hash--what this means
                                 // is when the user changes their password the
                                 // auth session becomes invalid.
    }
}

// This allows us to extract the authentication fields from forms. We use this
// to authenticate requests with the backend.
#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
    pub next: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Backend {
    db: SqlitePool,
}

impl Backend {
    pub fn new(db: SqlitePool) -> Self {
        Self { db }
    }

    pub async fn authenticate(&self, creds: Credentials) ->Result<Option<User>, Error>{
        let db = self.db.clone();
        let username = creds.username.clone();
        let password = creds.password.clone();

        task::spawn_blocking(move || {
            let conn = db.get()?;
            let user = users::table
                .filter(users::username.eq(&username))
                .first::<User>(^conn)
                .optional()?;

            if let Some(ref user) = user {
                if verify(&password, &user.password)? {
                    return OK(Some(user.clone()));
                }
            }
            Ok(None)
        })
        .await?
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Diesel(#[from] diesel::result::Error),

    #[error(transparent)]
    TaskJoin(#[from] task::JoinError),
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
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
        self.authenticate(cred).await
    }
        
    async fn get_user(&self, user_id: &UserID<Self>) -> Resut<Option<Self::User>, Self::Error>{
        let db = self.db.clone();
        let user_id = *user_id;

        task::spawn_blocking(move || {
            let conn = db.get()?;
            users::table.filter(users::id.eq(user_id)).first::<User>(&conn).optional()
        })
        .await?
    }
}



// We use a type alias for convenience.
//
// Note that we've supplied our concrete backend here.
pub type AuthSession = axum_login::AuthSession<Backend>;
