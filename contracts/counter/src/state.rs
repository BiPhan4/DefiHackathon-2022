use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

use crate::msg;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub storeowner: Addr,
    pub bill: i32, 
}

pub const STATE: Item<State> = Item::new("state");

pub const BALANCES: Map<&Addr, bool> = Map::new("balance");