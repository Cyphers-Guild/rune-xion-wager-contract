#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult};
use cw2::set_contract_version;

use cw_utils::parse_instantiate_response_data;

use crate::query::{query_config, query_game_pool, query_list_games};
use crate::state::{Config, CONFIG};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

use crate::execute::{
    execute_create_game, execute_refund_draw, execute_resolve_game, execute_wager,
};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:factory";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config: Config = Config {
        game_code_id: msg.game_code_id,
        admin: info.sender.clone(),
        games: std::collections::HashMap::new(),
        admin_fee_percentage: msg.admin_fee_percentage,
        player_win_percentage: msg.player_win_percentage,
        player_draw_percentage: msg.player_draw_percentage,
    };

    CONFIG.save(deps.storage, &config)?;

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("sender", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateGame {
            game_id,
            white_player,
            black_player,
            status,
        } => execute_create_game(deps, env, info, game_id, white_player, black_player, status),
        ExecuteMsg::Wager { game_id, player } => execute_wager(deps, env, info, game_id, player),
        ExecuteMsg::ResolveGame { game_id, winner } => {
            execute_resolve_game(deps, env, info, game_id, winner)
        }
        ExecuteMsg::RefundDraw { game_id } => execute_refund_draw(deps, env, info, game_id),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => query_config(deps),
        QueryMsg::GetGamePool { game_id } => query_game_pool(deps, game_id),
        QueryMsg::ListGames {} => query_list_games(deps),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, reply: Reply) -> Result<Response, ContractError> {
    let reply_result = reply
        .result
        .into_result()
        .map_err(|e| ContractError::ReplyParseError { msg: e.to_string() })?;

    let reply_data = parse_instantiate_response_data(
        reply_result.msg_responses.first().unwrap().value.as_slice(),
    )
    .map_err(|e| ContractError::ReplyParseError { msg: e.to_string() })?;

    let game_address = deps.api.addr_validate(&reply_data.contract_address)?;
    let game_id = reply.id.to_string();

    let game_id_clone = game_id.clone();
    CONFIG.update(deps.storage, |mut cfg| -> Result<_, ContractError> {
        cfg.games.insert(game_id_clone, game_address);
        Ok(cfg)
    })?;

    Ok(Response::new()
        .add_attribute("game_id", &game_id)
        .add_attribute("game_address", reply_data.contract_address))
}

#[cfg(test)]
mod tests {}
