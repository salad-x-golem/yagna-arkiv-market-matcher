use crate::model::offer::attributes::OfferFlatAttributes;
use crate::model::offer::base::GolemBaseOffer;
use crate::state::{AppState, OfferObj};
use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;

pub async fn push_offer(data: web::Data<AppState>, item: String) -> impl Responder {
    let decode = serde_json::from_str::<GolemBaseOffer>(&item);
    let offer = match decode {
        Ok(offer) => offer,
        Err(e) => {
            log::error!("Error decoding offer: {}", e);
            return HttpResponse::BadRequest().body("Invalid offer format");
        }
    };

    let mut lock = data.lock.lock().await;
    if lock.offer_map.contains_key(&offer.id) {
        let id = &offer.id;
        return HttpResponse::Ok().body(format!("Offer {id} already registered"));
    }
    let attributes = OfferFlatAttributes::from_gbo(&offer);
    lock.offer_map.insert(
        offer.id.clone(),
        OfferObj {
            offer,
            pushed_at: Utc::now(),
            requestor_id: None,
            attributes,
        },
    );
    HttpResponse::Ok().body("Offer added to the queue")
}
