use axum_login:: {
    login_required,
    tower_sessions::{ExpiredDeletion, Expiry, SessionManagerLayer},
    AuthManagerLayerBuilder,
};

use axum_messages::MessagesManagerLayer;
use sqlx::SqlitePool;
use time::Duration;
use tokio:: { signal, task::AbortHandle};
use tower_sessions::cookie::Key;
use tower_sessions_sqlx_store::SqliteStore;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
//use crate::web::recommendations; // Import the recommendations module


use crate:: {
    users::Backend,
    web::{auth, protected, user, index, companies, vendor_deals},
};

pub struct App {
    db: SqlitePool,
}

impl App {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let db = SqlitePool::connect(":memory:").await?;
        sqlx::migrate!().run(&db).await?;

        Ok(Self { db })
    }

    pub async fn serve(self) -> Result<(), Box<dyn std::error::Error>> {
        // Session layer.
        //
        // This uses `tower-sessions` to establish a layer that will provide the session
        // as a request extension.
        let session_store = SqliteStore::new(self.db.clone());
        session_store.migrate().await?;

        let deletion_task = tokio::task::spawn(
            session_store
                .clone()
                .continuously_delete_expired(tokio::time::Duration::from_secs(60)),
        );

        // Generate a cryptographic key to sign the session cookie.
        let key = Key::generate();

        let session_layer = SessionManagerLayer::new(session_store)
            .with_secure(false)
            .with_expiry(Expiry::OnInactivity(Duration::days(1)))
            .with_signed(key);

        // Auth service.
        //
        // This combines the session layer with our backend to establish the auth
        // service which will provide the auth session as a request extension.
        let backend = Backend;
        let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

        // Main router
        //
        // If you need to add a router from another mod
        // use the merge function as shown below
        let app = protected::router()
            .route_layer(login_required!(Backend, login_url = "/login"))
            .merge(auth::router())
            .merge(user::router())
            .merge(index::router())
            .merge(companies::router())
            .merge(vendor_deals::router())
            // .route("/recommendations/:user_id", axum::routing::get(recommendations::get_recommendations))
            .layer(MessagesManagerLayer)
            .layer(auth_layer);

        // If running locally change to "127.0.0.1:8080"
        let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap(); // localhost:8000

        // Ensure we use a shutdown signal to abort the deletion task.
        axum::serve(listener, app.into_make_service())
            .with_graceful_shutdown(shutdown_signal(deletion_task.abort_handle()))
            .await?;

        deletion_task.await??;

        Ok(())
    }
}

async fn shutdown_signal(deletion_task_abort_handle: AbortHandle) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => { deletion_task_abort_handle.abort() },
        _ = terminate => { deletion_task_abort_handle.abort() },
    }
}