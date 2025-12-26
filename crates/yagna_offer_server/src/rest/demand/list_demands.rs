use crate::state::{AppState, DemandObj};
use actix_web::{web, HttpResponse};

pub async fn list_demands(data: web::Data<AppState>) -> HttpResponse {
    let lock = data.demands.lock().await;
    let demands: Vec<&DemandObj> = lock.demand_map.values().collect();
    HttpResponse::Ok().json(demands)
}
