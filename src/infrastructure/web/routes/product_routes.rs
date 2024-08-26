use actix_web::web;
use crate::infrastructure::web::controllers::product_controller::*;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/products")
            .route("/{id}", web::get().to(get_product_by_id))
            .route("", web::post().to(create_product))
            .route("/{id}", web::put().to(update_product))
            .route("/{id}", web::delete().to(delete_product))
    );
}
