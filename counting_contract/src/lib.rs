use cosmwasm_std::{entry_point, to_json_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult};

mod contract;
pub mod msg;

/**
 * @name instantiate
 * Contract가 Deploy될때 실행되는 코드 | constructer()
 */
#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    Ok(Response::new())
}

#[entry_point]
pub fn query(
    _deps: Deps,
    _env: Env,
    msg: msg::QueryMsg, 
) -> StdResult<Binary> {
    use msg::QueryMsg::*;
    use contract::query;

    match msg {
        Value {} => to_json_binary(&query::value()),
    }
}
