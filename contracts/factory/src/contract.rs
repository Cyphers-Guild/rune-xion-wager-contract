#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    Binary, Deps, DepsMut, Env, MessageInfo, MsgResponse, Reply, Response, StdError, StdResult, SubMsgResponse,
};
use cw2::set_contract_version;

use cw0::parse_reply_instantiate_data;

use crate::query::{query_config, query_game_pool, query_list_games};
use crate::state::{Config, CONFIG};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

use crate::execute::{
    execute_create_game, execute_refund_draw, execute_resolve_game, execute_wager,
};

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
    let config: Config = Config {
        game_code_id: msg.game_code_id,
        admin: info.sender.clone(),
        games: std::collections::HashMap::new(),
        admin_fee_percentage: msg.admin_fee_percentage,
        player_win_percentage: msg.player_win_percentage,
        player_draw_percentage: msg.player_draw_percentage,
    };

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
            status,
        } => execute_create_game(deps, env, info, game_id, white_player, black_player, status),
        ExecuteMsg::Wager { game_id, player } => execute_wager(deps, env, info, game_id, player),
        ExecuteMsg::ResolveGame { game_id, winner } => {
            execute_resolve_game(deps, env, info, game_id, winner)
        }
        ExecuteMsg::RefundDraw { game_id } => execute_refund_draw(deps, env, info, game_id),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => query_config(deps),
        QueryMsg::GetGamePool { game_id } => query_game_pool(deps, game_id),
        QueryMsg::ListGames {} => query_list_games(deps),
    }
}

// #[cfg_attr(not(feature = "library"), entry_point)]
// pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
//     match msg.id {
//         1u64 => reply_created_game_wager(deps, env, msg.result.into_result()),
//     }
// }

// pub fn reply_created_game_wager(
//     deps: DepsMut,
//     _env: Env,
//     reply: Result<SubMsgResponse, String>,
// ) -> Result<Response, ContractError> {
//     let response = reply.map_err(StdError::generic_err)?;
//     let contract_address_str: String = match response.msg_responses.first() {
//         Some(MsgResponse { msg_responses, .. }) => {
//             if let Some(cosmos_msg_response) = msg_responses.first() {
//                 // Assuming the relevant response is the first one.
//                 // You might need more logic to find the correct response if multiple exist.
//                 match cosmos_msg_response.type_url.as_str() {
//                     "/cosmwasm.wasm.v1.MsgInstantiateContractResponse"
//                     | "/cosmwasm.wasm.v1.MsgInstantiateContract2Response" => {
//                         #[derive(Deserialize)]
//                         struct InstantiateContractResponse {
//                             pub contract_address: String,
//                         }
//                         let parsed: InstantiateContractResponse = serde_json::from_slice(
//                             cosmos_msg_response.value.as_slice(),
//                         )
//                         .map_err(|e| ContractError::ReplyParseError {
//                             msg: format!("Failed to parse instantiate response: {}", e),
//                         })?;
//                         parsed.contract_address
//                     }
//                     // Handle other potential MsgResponse types if needed
//                     _ => {
//                         return Err(ContractError::ReplyParseError {
//                             msg: format!(
//                                 "Unexpected MsgResponse type: {}",
//                                 cosmos_msg_response.type_url
//                             ),
//                         });
//                     }
//                 }
//             } else {
//                 return Err(ContractError::ReplyParseError {
//                     msg: "Empty msg_responses".to_string(),
//                 });
//             }
//         }
//         _ => {
//             // Fallback to the deprecated data field
//             response
//                 .data
//                 .ok_or_else(|| ContractError::ReplyParseError {
//                     msg: "Deprecated reply data field is missing".to_string(),
//                 })
//                 .and_then(|binary| {
//                     String::from_utf8(binary.to_vec()).map_err(|e| ContractError::ReplyParseError {
//                         msg: format!("Deprecated reply data is not valid UTF-8: {}", e),
//                     })
//                 })?
//         }
//     };
// }

// #[cfg_attr(not(feature = "library"), entry_point)]
// pub fn reply1(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
//     let reply_result = msg
//         .result
//         .into_result()
//         .map_err(|e| ContractError::ReplyParseError { msg: e.to_string() })?;

//     let contract_address_str: String = match reply_result.msg_responses.first() {
//         Some(MsgResponse { data: Some(b), .. }) => {
//             let parsed: InstantiateReply = serde_json::from_slice(b.as_slice())
//                 .map_err(|e| ContractError::ReplyParseError { msg: e.to_string() })?;
//             parsed.address
//         }
//         _ => {
//             // Fallback to the deprecated data field
//             reply_result
//                 .data
//                 .ok_or_else(|| ContractError::ReplyParseError {
//                     msg: "Deprecated reply data field is missing".to_string(),
//                 })
//                 .and_then(|binary| {
//                     String::from_utf8(binary.to_vec()).map_err(|e| ContractError::ReplyParseError {
//                         msg: format!("Deprecated reply data is not valid UTF-8: {}", e),
//                     })
//                 })?
//         }
//     };
//     let reply_data = parse_reply_instantiate_data(msg)
//         .map_err(|e| ContractError::ReplyParseError { msg: e.to_string() })?;

//     let game_address = deps.api.addr_validate(&reply_data.contract_address)?;
//     let game_id = msg.id.to_string();

//     CONFIG.update(deps.storage, |mut cfg| -> Result<_, ContractError> {
//         cfg.games.insert(game_id, game_address);
//         Ok(cfg)
//     })?;

//     Ok(Response::new()
//         .add_attribute("game_id", game_id)
//         .add_attribute("game_address", reply_data.contract_address))

//     match msg.id {
//         1u64 => {
//             let res = parse_reply_instantiate_data(msg);

//             let contract_address = deps.api.addr_validate(&res.contract_address)?;

//             Ok(Response::new()
//                 .add_attribute("game_id", game_id)
//                 .add_attribute("game_address", reply_data.contract_address))
//         }
//     }
// }

#[cfg(test)]
mod tests {}
