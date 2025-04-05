use cosmwasm_schema::cw_serde;

use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::Item;

use crate::msg::{PlayerInfo, Wager};

#[cw_serde]
pub struct Config {
    pub admin: Addr,
    pub white_player: PlayerInfo,
    pub black_player: PlayerInfo,
    pub admin_fee_percentage: u64,
    pub player_win_percentage: u64,
    pub player_draw_percentage: u64,
    pub wagers: Vec<(Addr, Wager)>,
    pub winner: Option<String>,
    pub status: String,
    pub pool: Coin,
}

pub const CONFIG: Item<Config> = Item::new("config");
