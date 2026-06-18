/*
// src/initiator.rs
use crate::config::AppConfig;
use crate::internal::storage::user_storage::DieselUserRepository;
use crate::internal::module::user_service::{DefaultUserService, UserService};
use diesel_async::pooled_connection::bb8::Pool;
use diesel_async::AsyncPgConnection;
use std::sync::Arc;

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
    // We will place references to our actix routes / handler controllers here in the next step
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
        // user_repo is passed directly as Arc<dyn UserRepository>
        let user_service = Arc::new(DefaultUserService::new(user_repo));
        let modules = Modules { user_service };
        tracing::info!("services initialized successfully");
        // --- Step 3: Initialize Handlers Tier (Injecting Modules) ---
        let handlers = Handlers {};

        Self {
            storage,
            modules,
            handlers,
        }
    }
}*/



// src/initiator.rs
use crate::config::AppConfig;
use crate::internal::storage::user_storage::DieselUserRepository;
use crate::internal::module::user_service::{DefaultUserService, UserService};
use diesel_async::pooled_connection::bb8::Pool;
use diesel_async::AsyncPgConnection;
use std::sync::Arc;

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
}

