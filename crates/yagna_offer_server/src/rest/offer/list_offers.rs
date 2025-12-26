use crate::state::{AppState, OfferObj};
use actix_web::{web, HttpResponse, Responder};

pub async fn list_offers(data: web::Data<AppState>) -> impl Responder {
    let lock = data.lock.lock().await;
    let offers: Vec<&OfferObj> = lock.offer_map.values().collect();
    HttpResponse::Ok().json(offers)
}

pub async fn list_taken_offers(data: web::Data<AppState>) -> impl Responder {
    let lock = data.lock.lock().await;
    let offers: Vec<&OfferObj> = lock
        .offer_map
        .values()
        .filter(|offer_obj| offer_obj.requestor_id.is_some())
        .collect();
    HttpResponse::Ok().json(offers)
}

pub async fn list_available_offers(data: web::Data<AppState>) -> impl Responder {
    let lock = data.lock.lock().await;
    let offers: Vec<&OfferObj> = lock
        .offer_map
        .values()
        .filter(|offer_obj| offer_obj.requestor_id.is_none())
        .collect();
    HttpResponse::Ok().json(offers)
}
