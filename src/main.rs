
mod Models;

mod Controllers;
mod Managers;

use std::sync::Arc;
use axum::Router;
use axum::routing::post;
use tokio::net::TcpListener;
use crate::Controllers::gateway_controller::redirect_petition;
use crate::Models::Settings::settings_model::AppSettings;

#[tokio::main]
async fn main() {
    if cfg!(debug_assertions) {
        dotenvy::dotenv().ok();
    }
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let state = Arc::new(AppSettings::new_app_settings());
    let app: Router = Router::new()
        .route("/AVolver/Gateway", post(redirect_petition))
        .with_state(Arc::clone(&state));
    let listener = TcpListener::bind(&addr)
        .await
        .unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}
