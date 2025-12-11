use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Properties {
    pub golem: GolemProperties,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GolemProperties {
    pub com: Com,
    pub inf: Inf,
    pub node: Node,
    pub runtime: Runtime,
    pub srv: Srv,
}

// --- Communication (com) ---

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Com {
    pub payment: Payment,
    pub pricing: Pricing,
    pub scheme: Scheme,
    pub usage: Usage,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Payment {
    #[serde(rename = "debit-notes")]
    pub debit_notes: DebitNotes,
    pub platform: Platform,
    pub protocol: Protocol,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DebitNotes {
    // The key in JSON literally contains the question mark
    #[serde(rename = "accept-timeout?")]
    pub accept_timeout: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Erc20Platform {
    pub address: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Platform {
    #[serde(rename = "erc20-polygon-glm")]
    pub erc20_polygon_glm: Option<Erc20Platform>,
    #[serde(rename = "erc20-hoodi-tglm")]
    pub erc20_hoodi_tglm: Option<Erc20Platform>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Protocol {
    pub version: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Pricing {
    pub model: PricingModel,
}

// The JSON uses "@tag" to determine which variant this is
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "@tag")]
pub enum PricingModel {
    #[serde(rename = "linear")]
    Linear { linear: LinearPricing },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinearPricing {
    pub coeffs: Vec<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "@tag")]
pub enum Scheme {
    #[serde(rename = "payu")]
    Payu { payu: PayuScheme },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PayuScheme {
    #[serde(rename = "debit-note")]
    pub debit_note: PayuDebitNote,
    #[serde(rename = "payment-timeout-sec?")]
    pub payment_timeout_sec: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PayuDebitNote {
    #[serde(rename = "interval-sec?")]
    pub interval_sec: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Usage {
    pub vector: Vec<String>,
}

// --- Infrastructure (inf) ---

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Inf {
    pub cpu: Cpu,
    pub mem: Mem,
    pub storage: Storage,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cpu {
    pub architecture: String,
    pub cores: u32,
    pub threads: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Mem {
    pub gib: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Storage {
    pub gib: f64,
}

// --- Node (node) ---

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Node {
    pub debug: Option<NodeDebug>,
    pub id: NodeName,
    pub net: NodeNet,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeDebug {
    pub subnet: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeName {
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeNet {
    #[serde(rename = "is-public")]
    pub is_public: bool,
}

// --- Runtime (runtime) ---

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Runtime {
    pub name: String,
    pub version: String,
}

// --- Service (srv) ---

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Srv {
    pub caps: Caps,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Caps {
    #[serde(rename = "multi-activity")]
    pub multi_activity: bool,
    #[serde(rename = "payload-manifest")]
    pub payload_manifest: bool,
}
