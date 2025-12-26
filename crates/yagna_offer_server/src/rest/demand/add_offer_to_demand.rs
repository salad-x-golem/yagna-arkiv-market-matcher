use crate::state::{AppState, DemandObj};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use ya_client_model::NodeId;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddOfferToDemand {
    pub demand_id: String,
    pub offer_id: String,
}

pub async fn add_offer_to_demand(data: web::Data<AppState>, body: String) -> HttpResponse {
    let decoded = serde_json::from_str::<AddOfferToDemand>(&body);
    let add_offer = match decoded {
        Ok(filer) => filer,
        Err(e) => {
            log::error!("Error decoding add offer to demand: {}", e);
            return HttpResponse::BadRequest().body(format!("Invalid format {}", e));
        }
    };
    let demand_id = add_offer.demand_id;
    let offer_id = add_offer.offer_id;

    let mut lock = data.demands.lock().await;
    let mut offers_lock = data.lock.lock().await;

    let offer = offers_lock.offer_map.get_mut(&offer_id);

    let offer = match offer {
        Some(offer) => offer,
        None => {
            return HttpResponse::NotFound().body("Offer not found");
        }
    };

    let get_demand = match lock.demand_map.contains_key(&demand_id) {
        true => lock.demand_map.get_mut(&demand_id),
        false => {
            let node_id = match NodeId::from_str(&demand_id) {
                Ok(id) => id,
                Err(_) => {
                    return HttpResponse::BadRequest().body("Invalid offer ID format or not found");
                }
            };
            let mut get_demand: Option<&mut DemandObj> = None;
            for (_, v) in lock.demand_map.iter_mut() {
                if v.demand.node_id == node_id {
                    get_demand = Some(v);
                    break;
                }
            }
            get_demand
        }
    };

    let demand_obj = match get_demand {
        Some(demand) => demand,
        None => {
            return HttpResponse::NotFound().body("Demand not found");
        }
    };
    if offer.requestor_id.is_some() {
        return HttpResponse::Conflict().body("Offer is already taken");
    }
    offer.requestor_id = Some(demand_obj.demand.node_id);
    demand_obj.offer_list.push_back(offer.offer.id.clone());
    HttpResponse::Ok().body("Offer added to demand successfully")
}
