use crate::model::offer::properties::Properties;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ya_client_model::NodeId;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GolemBaseOffer {
    pub id: String,
    pub properties: Properties,
    pub constraints: String,
    #[serde(rename = "providerId")]
    pub provider_id: NodeId,
    pub expiration: DateTime<Utc>,
    pub timestamp: DateTime<Utc>,
}
