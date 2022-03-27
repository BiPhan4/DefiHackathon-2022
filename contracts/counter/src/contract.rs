#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Order, Uint128, Coin, BankMsg, CosmosMsg, Uint256, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Api, Addr};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, totalPayersResponse, QueryMsg, self};
use crate::state::{State, STATE, BALANCES};

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
        bill: msg.bill,
        TotalPayers: 0,
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
        ExecuteMsg::Createaccounts {} => create_accounts(deps, info),
    }

}

pub fn create_accounts(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
        let paid = false; 
        let address = deps.api.addr_validate(&info.sender.to_string())?;
        BALANCES.save(deps.storage, &address, &paid)?;
        Ok(Response::new())
}

pub fn try_payup(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {

    let all: StdResult<Vec<_>> = BALANCES
    .range(deps.storage, None, None, Order::Ascending)
    .collect();

    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.TotalPayers += all.unwrap_or_default().len() as i32;
        Ok(state)
    })?;

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
    
        let msg = CosmosMsg::Bank(BankMsg::Send {
            to_address: config.storeowner.to_string(),
            amount: vec![ 
                Coin {
                    denom: "uluna".to_string(),
                    amount: deposit_amount,
                },
            ],
        });
        
        return Ok(Response::new().add_message(msg));;
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryPayers {} => to_binary(&QueryPayers(deps)?),
    }
}

pub fn QueryPayers(deps: Deps) -> StdResult<totalPayersResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(totalPayersResponse { payers: state.TotalPayers})
}

/* 

        for index in 0..state.TotalPayers{
            let config = STATE.load(deps.storage)?;
            let to_be_paid: (state.bill)*(1/(state.TotalPayers));

            let Divided_Amount: Uint = BALANCES[index];
            let deposit_amount: Uint128 = info
                .funds
                .iter()
                .find(|c| c.denom == "uluna")
                .map(|c| Uint128::from(c.amount))
                .unwrap_or_else(Uint128::zero);
        
                if deposit_amount.is_zero() {
                    return Err(ContractError::ZeroDeposit {});
                } 

    }*/
