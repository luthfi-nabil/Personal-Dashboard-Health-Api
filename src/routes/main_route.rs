use actix_web::web;

use crate::handlers::insulin_handler::{get_all_insulin_item_api, post_insulin_item_api};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/insulin-type",web::get().to(get_all_insulin_items_api))
            .route("/insulin-type", web::post().to(post_insulin_items_api))
    );
}