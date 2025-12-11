use crate::model::offer::base::GolemBaseOffer;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct OfferFlatAttributes {
    exe_name: String,
    subnet: String,
    cpu_architecture: String,
    cpu_threads: u32,
}

impl OfferFlatAttributes {
    pub fn from_gbo(gbo: &GolemBaseOffer) -> Self {
        OfferFlatAttributes {
            exe_name: gbo.properties.golem.runtime.name.clone(),
            subnet: gbo
                .properties
                .golem
                .node
                .debug
                .as_ref()
                .map(|f| f.subnet.clone())
                .unwrap_or_else(|| "public".to_string()),
            cpu_architecture: gbo.properties.golem.inf.cpu.architecture.clone(),
            cpu_threads: gbo.properties.golem.inf.cpu.threads,
        }
    }
}
