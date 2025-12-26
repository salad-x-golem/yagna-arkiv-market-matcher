use crate::state::AppState;
use actix_web::web;
use chrono::Utc;

pub async fn clean_old_offers(data: web::Data<AppState>) {
    let mut lock = data.lock.lock().await;
    let now = Utc::now();
    lock.offer_map.retain(|_id, offer_obj| {
        offer_obj.offer.expiration > (now - chrono::Duration::minutes(60))
    });
}
