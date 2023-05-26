use schemars::JsonSchema;
use cosmwasm_schema::{cw_serde, QueryResponses};
use serde::{Deserialize,Serialize};
use cosmwasm_std::{Addr, Uint128, Binary};
use cw721_base::Extension;

/// Message type for `instantiate` entry_point
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct FactoryInstantiateMsg {
    pub owner: Addr,
    pub max_tokens: u32,
    pub unit_price: Uint128,
    pub name: String,
    pub symbol: String,
    pub token_code_id: u64,
    pub cw20_address: Addr,
    pub token_uri: String,
    pub extension: Extension,
}



/// Message type for `execute` entry_point

#[cw_serde]
pub enum FactoryExecuteMsg {
    Receive(Cw20ReceiveMsg),
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Cw20ReceiveMsg {
    pub sender: String,
    pub amount: Uint128,
    pub msg: Binary,
}

/// Message type for `migrate` entry_point
#[cw_serde]
pub enum MigrateMsg {}

/// Message type for `query` entry_point
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // This example query variant indicates that any client can query the contract
    // using `YourQuery` and it will return `YourQueryResponse`
    // This `returns` information will be included in contract's schema
    // which is used for client code generation.
    //
    #[returns(ConfigResponse)]
    GetConfig {},
    // YourQuery {},
}

// We define a custom struct for each query response
// pub struct YourQueryResponse {}
#[cw_serde]
pub struct ConfigResponse {
    pub owner: Addr,
    pub cw20_address: Addr,
    pub cw721_address: Option<Addr>,
    pub max_tokens: u32,
    pub unit_price: Uint128,
    pub name: String,
    pub symbol: String,
    pub token_uri: String,
    pub extension: Extension,
    pub unused_token_id: u32,
}
