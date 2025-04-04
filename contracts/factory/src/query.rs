use crate::msg::*;
use crate::state::*;

use cosmwasm_std::{to_json_binary, Binary, Deps, StdResult};

pub fn query_config(deps: Deps) -> StdResult<Binary> {
    let config: Config = CONFIG.load(deps.storage)?;
    to_json_binary(&config)
}
