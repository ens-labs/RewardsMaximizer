use tokio::runtime::Builder;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use std::error::Error;

// Absolute path of mod
use crate::web::App;

// Modules
mod users;
mod web;

// The main async logic is moved to a separate function
async fn async_main() -> Result<(), Box<dyn Error>> {
    // Initialize tracing for logging
    tracing_subscriber::registry()
        .with(EnvFilter::new(std::env::var("RUST_LOG").unwrap_or_else(
            |_| "axum_login=debug,tower_sessions=debug,sqlx=warn,tower_http=debug".into(),
        )))
        .with(tracing_subscriber::fmt::layer())
        .try_init()?;

    // Start the application
    App::new().await?.serve().await
}

fn main() -> Result<(), Box<dyn Error>> {
    // Build a custom runtime
    let runtime = Builder::new_multi_thread()
        .worker_threads(8) // Set the number of worker threads (optional, adjust as needed)
        .thread_stack_size(1024 * 8000) // Set the stack size (8 MB in this example)
        .enable_all() // Enable I/O and time features
        .build()?;

    // Use the runtime to run the async main function
    runtime.block_on(async_main())
}
