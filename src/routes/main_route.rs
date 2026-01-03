use actix_web::{web};
use crate::route_middleware::get_user::CreatedByMiddleware;
use crate::handlers::insulin_handler::{
    get_all_insulin_items_api, 
    post_insulin_items_api, 
    get_all_insulin_assign_usage_api, 
    post_insulin_assign_api, 
    delete_insulin_assign_api,
    post_insulin_usage_api,
    delete_insulin_usage_api};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/user/{created_by}")
            .wrap(CreatedByMiddleware)
            .route("/insulin-item",web::get().to(get_all_insulin_items_api))
            .route("/insulin-item", web::post().to(post_insulin_items_api))
            .route("/insulin-assign-usage", web::get().to(get_all_insulin_assign_usage_api))
            .route("/insulin-assign", web::post().to(post_insulin_assign_api))
            .route("/insulin-assign/{insulin_assign_id}", web::delete().to(delete_insulin_assign_api))
            .route("/insulin-usage", web::post().to(post_insulin_usage_api))
            .route("/insulin-usage", web::delete().to(delete_insulin_usage_api))
    );
}