use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Uint128, Addr};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub storeowner: Addr,
    pub bill: i32, 
    pub TotalPayers: i32,
}


pub const STATE: Item<State> = Item::new("state");

pub const BALANCES: Map<&Addr, bool> = Map::new("balance");

