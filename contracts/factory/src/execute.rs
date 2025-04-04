use cosmwasm_std::{Deps, DepsMut, Env, MessageInfo, Response, StdResult};

pub fn execute_create_game(
    deps: DepsMut, 
    env: Env, 
    info: MessageInfo,
    game_id: String,
    white_player: String,
    black_player: String,
    status: String
) -> Result<Response, ContractError> {
    let config: Config = CONFIG.load(deps.storage)?;

    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    }

    if config.games.contains_key(&game_id) {
        return Err(ContractError::DuplicateGame {});
    }

    let msg = WasmMsg::instantiate {
        admin: None,
        code_id: state.game_pool_id,
        msg: to_json_binary(&)
    }

    Ok(Response::new()
        .add_attribute("action", "create new game")
        .add_attribute("game_id", game_id))
}

pub fn execute_wager(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    game_id: String,
    player: String
) -> Result<Response, ContractError> {
    let config: Config = CONFIG.load(deps.storage)?;

    Ok(Response::new())
}