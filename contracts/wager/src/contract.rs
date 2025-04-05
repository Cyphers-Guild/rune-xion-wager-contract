#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::Coin;
use cosmwasm_std::Uint128;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// use cw2::set_contract_version;

use crate::state::{Config, CONFIG};

use cw2::set_contract_version;

use crate::error::ContractError;
use crate::execute::{execute_refund_draw, execute_resolve_game, execute_wager};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::{query_game_info, query_wagers};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:wager";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config: Config = Config {
        admin: msg.admin,
        white_player: msg.white_player,
        black_player: msg.black_player,
        wagers: vec![],
        winner: None,
        status: msg.status,
        admin_fee_percentage: msg.admin_fee_percentage,
        player_win_percentage: msg.player_win_percentage,
        player_draw_percentage: msg.player_draw_percentage,
        pool: Coin {
            denom: info.funds[0].denom.clone(),
            amount: Uint128::zero(),
        },
    };

    CONFIG.save(deps.storage, &config)?;
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::ResolveGame { winner } => execute_resolve_game(deps, env, info, Some(winner)),
        ExecuteMsg::RefundDraw {} => execute_refund_draw(deps, env, info),
        ExecuteMsg::Wager { player } => execute_wager(deps, env, info, player),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetWagers {} => query_wagers(deps),
        QueryMsg::GetGameInfo {} => query_game_info(deps),
    }
}

#[cfg(test)]
mod tests {}
