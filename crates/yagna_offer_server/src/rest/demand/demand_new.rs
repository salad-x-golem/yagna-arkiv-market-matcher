use crate::model::demand::base::DemandSubscription;
use crate::state::{AppState, DemandObj};
use actix_web::{web, HttpResponse};
use std::collections::VecDeque;

pub async fn demand_new(data: web::Data<AppState>, item: String) -> HttpResponse {
    let decode = serde_json::from_str::<DemandSubscription>(&item);

    let demand = match decode {
        Ok(filer) => filer,
        Err(e) => {
            log::error!("Error decoding demand: {}", e);
            log::error!("Received demand: {}", item);
            return HttpResponse::BadRequest().body(format!("Invalid filter format {}", e));
        }
    };
    let mut lock = data.demands.lock().await;

    if lock.demand_map.contains_key(&demand.id) {
        return HttpResponse::Conflict().body("Demand with the same id already exists");
    }

    // find existing demand from the same node
    let last_demand = lock
        .demand_map
        .values()
        .find(|v| v.demand.node_id == demand.node_id);

    let mut copy_offer_list = VecDeque::new();
    if let Some(existing_demand) = last_demand {
        log::warn!(
            "Replacing existing demand {} from node {} with new demand {}",
            existing_demand.demand.id,
            existing_demand.demand.node_id,
            demand.id
        );
        copy_offer_list = existing_demand.offer_list.clone();
    }

    // Remove existing demand from the same node, including last_demand found above.
    lock.demand_map
        .retain(|_, v| v.demand.node_id != demand.node_id);

    let _ = lock.demand_map.insert(
        demand.id.clone(),
        DemandObj {
            demand: demand.clone(),
            offer_list: copy_offer_list,
        },
    );

    HttpResponse::Ok().json(demand)
}
