use crate::msg::*;
use crate::state::*;

use cosmwasm_std::{to_json_binary, Binary, Deps, StdResult};

pub fn query_config(deps: Deps) -> StdResult<Binary> {
    let config: Config = CONFIG.load(deps.storage)?;
    to_json_binary(&config)
}

pub fn query_game_pool(deps: Deps, game_id: String) -> StdResult<Binary> {
    let config: Config = CONFIG.load(deps.storage)?;
    let game_address = config.games.get(&game_id).ok_or(ContractError::GameNotFound {})?;
    to_json_binary()
}

pub fn query_list_games(deps: Deps) -> StdResult<Binary> {
    let config: Config = CONFIG.load(deps.storage)?;
    let games = config.games.keys().cloned().collect();
    to_json_binary(games)
}