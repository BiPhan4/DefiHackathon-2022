#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Uint128, Coin, BankMsg, CosmosMsg, Uint256, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{CountResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:counter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        storeowner: info.sender.clone(),
        bill: msg.bill
        /*logic to split the bill
        put contract callers into 
        store owner specifies bill amount
        and users call join. 
        Need join function: join function adds their addresses to map
        then once everyone joins have the pay up method split bill
        based on map length 
        
        */

    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Payup {} => try_payup(deps, info),
    }

}

pub fn try_payup(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let config = STATE.load(deps.storage)?;
    let deposit_amount: Uint128 = info
        .funds
        .iter()
        .find(|c| c.denom == "uluna")
        .map(|c| Uint128::from(c.amount))
        .unwrap_or_else(Uint128::zero);
        if deposit_amount.is_zero() {
            return Err(ContractError::ZeroDeposit {});
        }
        /*is depost amount 50% of config.bill 
        */
    
        let msg = CosmosMsg::Bank(BankMsg::Send {
            to_address: config.storeowner.to_string(),
            amount: vec![ 
                Coin {
                    denom: "uluna".to_string(),
                    amount: deposit_amount,
                },
            ],
        });
        
        Ok(Response::new().add_message(msg))
}

//pub createorder(deps: DepsMut, info: MessageInfo: i32) -> Result<Response, ContractError>{


//}

