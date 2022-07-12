use cosmwasm_std::{
    to_binary, Addr, AllBalanceResponse, BalanceResponse, BankQuery, Coin, Deps, QueryRequest,
    StdResult, Uint256, WasmQuery,
};
use cw20::{BalanceResponse as Cw20BalanceResponse, Cw20QueryMsg, TokenInfoResponse};

pub fn query_all_balances(deps: Deps, account_addr: Addr) -> StdResult<Vec<Coin>> {
    // load price form the oracle
    let all_balances: AllBalanceResponse =
        deps.querier
            .query(&QueryRequest::Bank(BankQuery::AllBalances {
                address: account_addr.to_string(),
            }))?;
    Ok(all_balances.amount)
}

pub fn query_balance(deps: Deps, account_addr: Addr, denom: String) -> StdResult<Uint256> {
    // load price form the oracle
    let balance: BalanceResponse = deps.querier.query(&QueryRequest::Bank(BankQuery::Balance {
        address: account_addr.to_string(),
        denom,
    }))?;
    Ok(balance.amount.amount.into())
}

pub fn query_token_balance(
    deps: Deps,
    contract_addr: Addr,
    account_addr: Addr,
) -> StdResult<Uint256> {
    // load balance form the token contract
    let res: Cw20BalanceResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: contract_addr.to_string(),
        msg: to_binary(&Cw20QueryMsg::Balance {
            address: account_addr.to_string(),
        })?,
    }))?;

    // load balance form the token contract
    Ok(res.balance.into())
}

pub fn query_supply(deps: Deps, contract_addr: Addr) -> StdResult<Uint256> {
    let token_info: TokenInfoResponse =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: contract_addr.to_string(),
            msg: to_binary(&Cw20QueryMsg::TokenInfo {})?,
        }))?;

    Ok(Uint256::from(token_info.total_supply.u128()))
}
