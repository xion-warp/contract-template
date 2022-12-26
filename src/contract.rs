use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};

use crate::state::Config;
use shared::cards_storage::msg::{ExecuteMsg, InstantiateMsg, OwnerResponse, QueryMsg};
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    let config = Config {
        owner: deps.api.addr_canonicalize(info.sender.as_str())?,
        message: "".to_owned(),
    };

    //deps.api
    //    .debug(format!("Contract was initialized by {}", info.sender).as_str());
    config.save_config(deps.storage)?;

    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::ChangeOwner { addr } => todo!(),
        ExecuteMsg::CreateCollection {} => todo!(),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetOwner {} => to_binary(&query_owner(deps)?),
    }
}

fn query_owner(deps: Deps) -> StdResult<OwnerResponse> {
    let cfg = Config::read_config(deps.storage)?;
    Ok(OwnerResponse { owner: cfg.owner })
}
/*
#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::*;
    use cosmwasm_std::{from_binary, Coin, StdError, Uint128};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();
        let result =
        try_parse_code(deps.as_mut(), "echo (a); echo (b); reverse(ab);".to_owned()).unwrap();
        let output = result.attributes.first().unwrap().value.clone();
        println!("{}", &output);
        assert_eq!("a\n", output);
    }
}

*/
