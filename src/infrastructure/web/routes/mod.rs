pub mod product_routes;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    product_routes::configure(cfg);
}
