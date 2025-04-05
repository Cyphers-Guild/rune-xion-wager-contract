use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin};

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: Addr,
    pub white_player: PlayerInfo,
    pub black_player: PlayerInfo,
    pub admin_fee_percentage: u64,
    pub player_win_percentage: u64,
    pub player_draw_percentage: u64,
    pub winner: Option<String>,
    pub status: String,
}

#[cw_serde]
pub struct PlayerInfo {
    pub name: String,
    pub address: Addr,
}

#[cw_serde]
pub struct Wager {
    pub amount: Coin,
    pub player: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    ResolveGame { winner: String },
    RefundDraw {},
    Wager { player: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(WagersResponse)]
    GetWagers {},
    #[returns(GameInfoResponse)]
    GetGameInfo {},
}

#[cw_serde]
pub struct WagersResponse {
    pub wagers: Vec<(Addr, Wager)>,
}

#[cw_serde]
pub struct GameInfoResponse {
    pub white_player: PlayerInfo,
    pub black_player: PlayerInfo,
    pub status: String,
    pub winner: Option<String>,
    pub pool: Coin,
}
