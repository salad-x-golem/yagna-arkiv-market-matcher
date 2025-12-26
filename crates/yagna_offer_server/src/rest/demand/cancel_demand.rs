use crate::model::demand::base::DemandCancellation;
use crate::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn demand_cancel(data: web::Data<AppState>, item: String) -> HttpResponse {
    let decode = serde_json::from_str::<DemandCancellation>(&item);

    let cancellation = match decode {
        Ok(filer) => filer,
        Err(e) => {
            log::error!("Error decoding demand cancellation: {}", e);
            return HttpResponse::BadRequest().body(format!("Invalid cancellation format {}", e));
        }
    };

    let mut lock = data.demands.lock().await;
    if lock.demand_map.remove(&cancellation.demand_id).is_some() {
        HttpResponse::Ok().body("Demand cancelled successfully")
    } else {
        HttpResponse::NotFound().body("Demand not found")
    }
}
