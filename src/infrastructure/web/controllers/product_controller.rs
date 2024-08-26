use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use crate::domain::products::product_service::ProductService;
use crate::domain::products::entities::product::Product;
use crate::infrastructure::database::mongo_product_repository::MongoProductRepository;

pub async fn create_product(
    service: web::Data<ProductService<MongoProductRepository>>, 
    product: web::Json<Product>
) -> impl Responder {
    match service.create_product(product.into_inner()).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_product_by_id(
    service: web::Data<ProductService<MongoProductRepository>>, 
    product_id: web::Path<Uuid>
) -> impl Responder {
    match service.get_product(product_id.into_inner()).await {
        Some(product) => HttpResponse::Ok().json(product),
        None => HttpResponse::NotFound().body("Producto no encontrado"),
    }
}

pub async fn update_product(
    service: web::Data<ProductService<MongoProductRepository>>, 
    product_id: web::Path<Uuid>, 
    product: web::Json<Product>
) -> impl Responder {
    let mut product = product.into_inner();
    product.id = product_id.into_inner();
    
    match service.update_product(product).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_product(
    service: web::Data<ProductService<MongoProductRepository>>, 
    product_id: web::Path<Uuid>
) -> impl Responder {
    match service.delete_product(product_id.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
