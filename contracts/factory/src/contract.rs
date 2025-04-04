#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::query::query_config;
use crate::state::{Config, CONFIG};
use crate::execute::{
    execute_create_game,
    execute_wager
}

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

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
    let config = Config {
        admin: info.sender.clone(),
        games: std::collections::HashMap::new(),
        admin_fee_percentage: msg.admin_fee_percentage,
        player_win_percentage: msg.player_win_percentage,
        player_draw_percentage: msg.player_draw_percentage,
    }
    
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
            status
        } => execute_create_game(deps, env, info, game_id, white_player, black_player, status),
        ExecuteMsg::Wager { 
            game_id, 
            player 
        } => execute_wager(deps, env, info, game_id, player), 
        ExecuteMsg::ResolveGame {
            game_id
            winner
        } => execute_resolve_game(deps, env, info, game_id, winner),
        ExecuteMsg::RefundDraw {game_id} => execute_refund_draw(deps, env, info, game_id),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => query_config(deps)?,
        QueryMsg::GetGamePool {game_id} => query_game_pool(deps, game_id)?,
        QueryMsg::ListGames {} => query_list_games(deps)?,
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, reply: Reply) -> StdResult<Response> {
    let res = reply.result.into_result()?;
    let address = res.data.map(|binary| deps.api.addr_validate(binary.as_slice().unwrap()).ok_or(ContractError::NoAddress {}))?;
    let game_id - String::from(reply.id.to_string());
    let mut config: Config = CONFIG.load(deps.storage)?:

    config.games.insert(game_id, address);
    CONFIG.update(deps.storage, &to_json_binary(&config)?)?;

    Ok(Response::default())
}

#[cfg(test)]
mod tests {}
