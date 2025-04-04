use cosmwasm_std::{Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::state::{Config, CONFIG};

pub fn execute_create_game(
    deps: DepsMut, 
    env: Env, 
    info: MessageInfo,
    game_id: String,
    white_player: String,
    black_player: String,
    status: String,
) -> Result<Response, ContractError> {
    let config: Config = CONFIG.load(deps.storage)?;

    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    }

    if config.games.contains_key(&game_id) {
        return Err(ContractError::DuplicateGame {});
    }

    let msg = WasmMsg::Instantiate {
        admin: None,
        code_id: state.game_code_id,
        msg: to_json_binary(&)?,
        funds: info.funds, // vec![]
        label: format!("game-{}", game_id),
    };

    Ok(Response::new()
        .add_submessage(SubMsg {
            msg: msg.into(),
            id: 1,
            reply_on: ReplyOn::Success,
            weight: Default::default()
        })
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
    let game_addr = config.games.get(&game_id).ok_or(CustomError::GameNotFound {})?;

    let msg = WasmMsg::Execute {
        contract_addr: game_addr.to_string(),
        msg: to_json_binary(&wager::ExecuteMsg::Wager{player})?,
        funds: info.funds,
    }
    
    Ok(Response::new().add_message(msg))
}

pub fn execute_resolve_game(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    game_id: String,
    winner: String
) -> StdResult<Response, ContractError> {
    let config: Config = CONFIG.load(deps.storage)?;
    let game_addr = config.games.get(&game_id).ok_or(ContractError::GameNotFound {})?;
    let msg = WasmMsg::Execute {
        contract_addr: game_addr.to_string(),
        msg: to_json_binary(&wager::ExecuteMsg::ResolveGame{winner})?,
        funds: vec![],
    };

    Ok(Response::new().add_message(msg))
}

pub fn eecute_refund_draw(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    game_id: String,
) -> StdResult<Response, ContractError> {
    let config: Config = CONFIG.load(deps.storage)?;
    let game_addr = config.games.get(&game_id).ok_or(ContractError::GameNotFound {})?;
    let msg = WasmMsg::Execute {
        contract_addr: game_addr.to_string(),
        msg: to_json_binary(&wager::ExecuteMsg::RefundDraw {})?,
        funds: vec![],
    };

    Ok(Response::new().add_message(msg))
}