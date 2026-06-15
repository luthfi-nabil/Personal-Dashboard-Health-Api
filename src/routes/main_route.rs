use crate::handlers::flutter_sync_handler::{get_sync, post_sync_push};
use crate::handlers::insulin_handler::{
    delete_insulin_assign_api, delete_insulin_usage_api, get_all_insulin_assign_usage_api,
    get_all_insulin_items_api, post_insulin_assign_api, post_insulin_items_api,
    post_insulin_usage_api,
};
use crate::handlers::swagger_handler::{get_swagger_ui, get_swagger_yaml};
use crate::route_middleware::get_user::CreatedByMiddleware;
use actix_web::web;
use std::env;

pub fn init(cfg: &mut web::ServiceConfig) {
    // ── Swagger UI (development only) ────────────────────────────────────────
    let app_env = env::var("APP_ENV").unwrap_or_else(|_| "production".to_string());
    if app_env == "development" {
        cfg.service(
            web::scope("/docs")
                .route("", web::get().to(get_swagger_ui))
                .route("/", web::get().to(get_swagger_ui))
                .route("/openapi.yaml", web::get().to(get_swagger_yaml)),
        );
    }

    // ── API Routes ───────────────────────────────────────────────────────────
    cfg.service(
        web::scope("/api/user")
            .wrap(CreatedByMiddleware)
            .route("/insulin-item", web::get().to(get_all_insulin_items_api))
            .route("/insulin-item", web::post().to(post_insulin_items_api))
            .route(
                "/insulin-assign-usage",
                web::get().to(get_all_insulin_assign_usage_api),
            )
            .route("/insulin-assign", web::post().to(post_insulin_assign_api))
            .route(
                "/insulin-assign/{insulin_assign_id}",
                web::delete().to(delete_insulin_assign_api),
            )
            .route("/insulin-usage", web::post().to(post_insulin_usage_api))
            .route("/insulin-usage", web::delete().to(delete_insulin_usage_api)),
    );

    cfg.service(
        web::scope("/api/flutter")
            .route("/health-sync", web::get().to(get_sync))
            .route("/health-sync/push", web::post().to(post_sync_push)),
    );
}
