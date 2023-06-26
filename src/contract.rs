#[cfg(not(feature = "library"))]
mod entry {
    use cosmwasm_std::{
        entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
    };
    
    use crate::state::Config;
    use shared::<CONTRACT_NAME>::msg::{ExecuteMsg, InstantiateMsg, OwnerResponse, QueryMsg, MigrateMsg};
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

    #[entry_point]
    pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
        // No state migrations performed, just returned a Response
        Ok(Response::default())
    }

}
#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::*;
    use cosmwasm_std::{from_binary, Coin, StdError, Uint128};

    #[test]
    fn unit_test() {
        
    }
}

