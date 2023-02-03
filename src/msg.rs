use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub allowed: Vec<String>, //optional if exists will only transfer to these addesses
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    TransferAuthFunds {
        /// The `to_address`
        to_address: String,
        granter_address: String,
        denom: String,
        amount: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    QueryGranter { granter_address: String },
    QueryIbcDenom { ibc_denom: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct QueryStargateResponse {
    /// query response data in bytes, json-serialized, base64 encoded
    pub value: String,
}
