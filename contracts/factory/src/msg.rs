use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub game_code_id: u64,
    pub admin: String,
    pub admin_fee_percentage: u64,
    pub player_fee_percentage: u64,
}

#[cw_serde]
pub enum ExecuteMsg {
    CreateGame {
        game_id: String,
        white_player: String,
        black_player: String,
        status: String,
    },
    Wager {
        game_id: String,
        player: String,
    },
    ResolveMatch {
        game_id: String,
        winner: String,
    },
    RefundDraw {
        game_id: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(GetGameResponse)]
    GetMatchPool { game_id: String },
    #[returns(ListGamesResponse)]
    ListGames {},
}

#[cw_serde]
pub struct ConfigResponse {
    pub admin: Addr,
    pub player_win_percentage: u64,
    pub player_draw_percentage: u64,
    pub admin_fee_percentage: u64,
}

#[cw_serde]
pub struct GetGameResponse {
    pub game_address: Addr,
}

#[cw_serde]
pub struct ListGamesResponse {
    pub games: Vec<String>,
}
