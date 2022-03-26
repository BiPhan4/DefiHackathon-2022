use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item; //import map and figure out how to use it 

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub owner: Addr,
}

pub const STATE: Item<State> = Item::new("state");

/*
add states:

1 - the bill(s) / item 
2 - the customers /map 
3 - Need to figure out a way to store money that we've taken from users. Third state is item again? 

1 - how to use map and item to store customers or any state
2 - how to display bill to user 

Hello 
*/
