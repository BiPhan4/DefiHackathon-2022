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

/*
add states:

1 - the bill(s) / item 
2 - the customers /map 
3 - Need to figure out a way to store money that we've taken from users. Third state is item again? 

1 - how to use map and item to store customers or any state
2 - how to display bill to user 

Hello 
*/