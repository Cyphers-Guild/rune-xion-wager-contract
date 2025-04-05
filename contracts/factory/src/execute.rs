use cosmwasm_std::{
    to_json_binary, CosmosMsg, DepsMut, Env, MessageInfo, ReplyOn, Response, SubMsg, WasmMsg,
};

use crate::error::ContractError;
use crate::msg::PlayerInfo;
use crate::state::{Config, CONFIG};

pub fn generate_salt(len: usize)

pub fn execute_create_game(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    game_id: String,
    white_player: PlayerInfo,
    black_player: PlayerInfo,
    status: String,
) -> Result<Response, ContractError> {
    let config: Config = CONFIG.load(deps.storage)?;

    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    }

    if config.games.contains_key(&game_id) {
        return Err(ContractError::DuplicateGame {});
    }

    let salt = 

    let instantiate_game = CosmosMsg::Wasm(WasmMsg::Instantiate2 {
        admin: None,
        code_id: config.game_code_id,
        msg: to_json_binary(&wager::msg::InstantiateMsg {
            admin: info.sender.clone(),
            winner: None,
            status: status.clone(),
            white_player: wager::msg::PlayerInfo {
                name: white_player.name,
                address: deps.api.addr_validate(&white_player.address)?,
            },
            black_player: wager::msg::PlayerInfo {
                name: black_player.name,
                address: deps.api.addr_validate(&black_player.address)?,
            },
            admin_fee_percentage: config.admin_fee_percentage,
            player_win_percentage: config.player_win_percentage,
            player_draw_percentage: config.player_draw_percentage,
        })?,
        funds: vec![],
        label: format!("game-{}", game_id),
    });

    Ok(Response::new()
        .add_submessage(SubMsg::reply_on_success(instantiate_game, 1u64))
        .add_attribute("action", "create new game")
        .add_attribute("game_id", game_id))
}

pub fn execute_wager(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    game_id: String,
    player: String,
) -> Result<Response, ContractError> {
    let config: Config = CONFIG.load(deps.storage)?;
    let game_addr = config
        .games
        .get(&game_id)
        .ok_or(ContractError::GameNotFound {})?;

    let msg = WasmMsg::Execute {
        contract_addr: game_addr.to_string(),
        msg: to_json_binary(&wager::msg::ExecuteMsg::Wager { player })?,
        funds: info.funds,
    };

    Ok(Response::new().add_message(msg))
}

pub fn execute_resolve_game(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    game_id: String,
    winner: String,
) -> Result<Response, ContractError> {
    let config: Config = CONFIG.load(deps.storage)?;
    let game_addr = config
        .games
        .get(&game_id)
        .ok_or(ContractError::GameNotFound {})?;
    let msg = WasmMsg::Execute {
        contract_addr: game_addr.to_string(),
        msg: to_json_binary(&wager::msg::ExecuteMsg::ResolveGame { winner })?,
        funds: vec![],
    };

    Ok(Response::new().add_message(msg))
}

pub fn execute_refund_draw(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    game_id: String,
) -> Result<Response, ContractError> {
    let config: Config = CONFIG.load(deps.storage)?;
    let game_addr = config
        .games
        .get(&game_id)
        .ok_or(ContractError::GameNotFound {})?;
    let msg = WasmMsg::Execute {
        contract_addr: game_addr.to_string(),
        msg: to_json_binary(&wager::msg::ExecuteMsg::RefundDraw {})?,
        funds: vec![],
    };

    Ok(Response::new().add_message(msg))
}
