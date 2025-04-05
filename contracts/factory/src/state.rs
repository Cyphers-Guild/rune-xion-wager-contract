use cosmwasm_schema::cw_serde;

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[cw_serde]
pub struct Config {
    pub game_code_id: u64,
    pub admin_fee_percentage: u64,
    pub player_win_percentage: u64,
    pub player_draw_percentage: u64,
    pub games: std::collections::HashMap<String, Addr>,
    pub admin: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");
