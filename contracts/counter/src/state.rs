use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Uint128, Addr};
use cw_storage_plus::{Item, Map};

use crate::msg;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub storeowner: Addr,
    pub bill: i32, 
}

pub const STATE: Item<State> = Item::new("state");

pub const BALANCES: Map<&Addr, Uint128> = Map::new("balance");

pub const PAYERS: Item<Uint128> = Item::new("payers");