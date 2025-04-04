use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Config {
    pub admin_fee_percentage: String,
    pub player_win_percentage: u64,
    pub player_draw_percentage: u64,
    pub games: std::collections::HashMap<String, Addr>,
    pub admin: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");
