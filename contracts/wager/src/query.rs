use crate::msg::*;
use cosmwasm_std::{to_json_binary, Binary, Deps, StdResult};

use crate::state::{Config, CONFIG};

pub fn query_wagers(deps: Deps) -> StdResult<Binary> {
    let config: Config = CONFIG.load(deps.storage)?;
    to_json_binary(&config.wagers)
}

pub fn query_game_info(deps: Deps) -> StdResult<Binary> {
    let config: Config = CONFIG.load(deps.storage)?;
    to_json_binary(&GameInfoResponse {
        white_player: config.white_player,
        black_player: config.black_player,
        status: config.status,
        winner: config.winner,
        pool: config.pool,
    })
}
