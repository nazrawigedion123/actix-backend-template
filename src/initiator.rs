// src/initiator.rs
use crate::internal;

use crate::config::AppConfig;
use crate::internal::storage::user_storage::DieselUserRepository;
use crate::internal::module::user_service::{DefaultUserService, UserService};
use diesel_async::pooled_connection::bb8::Pool;
use diesel_async::AsyncPgConnection;
use std::sync::Arc;
use actix_web::{App, HttpServer};
use tracing_actix_web::TracingLogger;


/// Layer 1: Storage Layer Container
#[derive(Clone)]
pub struct Storage {
    pub user_repo: Arc<DieselUserRepository>,
}

/// Layer 2: Business Domain Modules Layer Container
#[derive(Clone)]
pub struct Modules {
    pub user_service: Arc<dyn UserService>,
}

/// Layer 3: HTTP Handler Controllers Layer Container
#[derive(Clone)]
pub struct Handlers {
    pub user_handler: Arc<dyn UserService>, // Pass target interface reference needed for endpoint controllers
}

/// Composition Root Orchestrating all layered inversions of control dependencies
#[derive(Clone)]
pub struct AppInitiator {
    pub storage: Storage,
    pub modules: Modules,
    pub handlers: Handlers,
}

impl AppInitiator {
    pub async fn init(cfg: &AppConfig) -> Self {
        tracing::info!("Initializing asynchronous connection pool adapters...");

        let manager =
            diesel_async::pooled_connection::AsyncDieselConnectionManager::<AsyncPgConnection>::new(
                &cfg.database_url,
            );

        let pool = Pool::builder()
            .max_size(10)
            .build(manager)
            .await
            .unwrap_or_else(|err| {
                panic!("CRITICAL INFRASTRUCTURE FAILURE: Failed to create database connection pool: {err}");
            });

        tracing::info!("Database connection pool established successfully");

        // --- Step 1: Initialize Storage Tier ---
        let user_repo = Arc::new(DieselUserRepository::new(pool));
        let storage = Storage { user_repo: user_repo.clone() };

        // --- Step 2: Initialize Modules Tier (Injecting Storage) ---
        let user_service = Arc::new(DefaultUserService::new(user_repo));
        let modules = Modules { user_service: user_service.clone() };

        // --- Step 3: Initialize Handlers Tier (Injecting Modules) ---
        let handlers = Handlers {
            user_handler: user_service,
        };

        Self {
            storage,
            modules,
            handlers,
        }
    }

    pub async fn initate()->std::io::Result<()>{

         internal::platform::logger::init_logger();

    // 2. Load system configurations safely via Envy environment layers
    let cfg = AppConfig::load_from_env();
         // 3. Complete structural dependency tree orchestration pipelines (Storage -> Modules -> Handlers)
    let initiator = AppInitiator::init(cfg).await;
    
    // Cache port configuration for runtime closures
    let bind_port = cfg.server_port;

    tracing::info!("Starting actix production server cluster on port {}...", bind_port);

    HttpServer::new(move || {
        // Clone a copy of our fully wired initiator tree into every parallel worker thread context
        let app_initiator = initiator.clone();

        App::new()
            // Attach our advanced asynchronous TracingLogger middleware (automatically traces request context)
            .wrap(TracingLogger::default())
            // Pass configuration route mappings down to mount all application handler endpoints
            .configure(|web_cfg| internal::routes::configure_routes(web_cfg, &app_initiator.handlers))
    })
    .bind(("127.0.0.1", bind_port))?
    .run()
    .await

    }
}

