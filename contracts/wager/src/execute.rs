use cosmwasm_std::{
    attr, Addr, BankMsg, Coin, CosmosMsg, DepsMut, Env, MessageInfo, Response, Uint128,
};

use crate::error::ContractError;
use crate::msg::Wager;
use crate::state::{Config, CONFIG};

pub fn execute_wager(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    player: String,
) -> Result<Response, ContractError> {
    let mut config: Config = CONFIG.load(deps.storage)?;

    if config.status == "finished" {
        return Err(ContractError::WagingClosed {});
    }

    if player != config.white_player.name && player != config.black_player.name {
        return Err(ContractError::InvalidPlayer {});
    }

    if info.funds.len() != 1 {
        return Err(ContractError::InsufficientAmount {});
    }

    let wager = Wager {
        amount: info.funds[0].clone(),
        player,
    };

    config.wagers.push((info.sender.clone(), wager.clone()));
    config.pool.amount += wager.amount.amount;
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attributes(vec![
        attr("action", "wager"),
        attr("sender", info.sender.to_string()),
        attr("player", wager.player),
        attr("amount", wager.amount.to_string()),
    ]))
}

pub fn execute_resolve_game(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    winner: Option<String>,
) -> Result<Response, ContractError> {
    let mut config: Config = CONFIG.load(deps.storage)?;

    if config.status != "ongoing" {
        return Err(ContractError::GameOngoing {}); // More descriptive error
    }

    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    }

    config.status = "finished".to_string(); // Update game status
    config.winner = winner.clone();
    CONFIG.save(deps.storage, &config)?;

    let total_funds = config.pool.amount;
    let admin_fee =
        total_funds * Uint128::from(config.admin_fee_percentage) / Uint128::from(100u64);

    let mut messages: Vec<CosmosMsg> = vec![];

    // Send admin fee
    messages.push(CosmosMsg::Bank(BankMsg::Send {
        to_address: config.admin.to_string(),
        amount: vec![Coin {
            denom: config.pool.denom.clone(),
            amount: admin_fee,
        }],
    }));

    let remaining_funds = total_funds - admin_fee;

    match winner.clone() {
        Some(winning_player_name) => {
            let player_win_percentage = config.player_win_percentage;
            let win_fee =
                total_funds * Uint128::from(player_win_percentage) / Uint128::from(100u64);
            let winning_player = if config.white_player.name == winning_player_name {
                &config.white_player
            } else if config.black_player.name == winning_player_name {
                &config.black_player
            } else {
                return Err(ContractError::InvalidWinner {});
            };

            // Send win fee to the winning player
            messages.push(CosmosMsg::Bank(BankMsg::Send {
                to_address: winning_player.address.to_string(),
                amount: vec![Coin {
                    denom: config.pool.denom.clone(),
                    amount: win_fee,
                }],
            }));

            let remaining_funds_after_win_fee = remaining_funds - win_fee;

            // Distribute winnings to winning wagerers (collect owned values)
            let winning_wagers: Vec<(&Addr, &Wager)> = config
                .wagers
                .iter()
                .filter(|(_, wager)| wager.player == winning_player_name)
                .map(|item| {
                    let (addr, wager) = item; // Destructure the reference
                    (addr, wager) // Yield references
                })
                .collect();

            if !winning_wagers.is_empty() {
                let total_winning_wagers_amount: Uint128 = winning_wagers
                    .iter()
                    .map(|(_, wager)| wager.amount.amount)
                    .sum();

                for (addr, wager) in winning_wagers {
                    let share = wager.amount.amount * remaining_funds_after_win_fee
                        / total_winning_wagers_amount;
                    messages.push(CosmosMsg::Bank(BankMsg::Send {
                        to_address: addr.to_string(),
                        amount: vec![Coin {
                            denom: wager.amount.denom.clone(),
                            amount: share,
                        }],
                    }));
                }
            }
        }
        None => {
            // It's a draw, refund with player_draw_percentage as fee
            let draw_fee_percentage = config.admin_fee_percentage + config.player_draw_percentage;
            let total_draw_fee_percentage = Uint128::from(draw_fee_percentage);

            for (addr, wager) in config.wagers.iter() {
                let total_wager_amount = wager.amount.amount;
                let fee_amount =
                    total_wager_amount * total_draw_fee_percentage / Uint128::from(100u64);
                let refund_amount = total_wager_amount - fee_amount;

                if !refund_amount.is_zero() {
                    messages.push(CosmosMsg::Bank(BankMsg::Send {
                        to_address: addr.to_string(),
                        amount: vec![Coin {
                            denom: wager.amount.denom.clone(),
                            amount: refund_amount,
                        }],
                    }));
                }
            }
            // Send draw fee to players proportionally
            let total_draw_fee =
                total_funds * Uint128::from(config.player_draw_percentage) / Uint128::from(100u64);
            let half_draw_fee = total_draw_fee / Uint128::from(2u64);

            messages.push(CosmosMsg::Bank(BankMsg::Send {
                to_address: config.white_player.address.to_string(),
                amount: vec![Coin {
                    denom: config.pool.denom.clone(),
                    amount: half_draw_fee,
                }],
            }));
            messages.push(CosmosMsg::Bank(BankMsg::Send {
                to_address: config.black_player.address.to_string(),
                amount: vec![Coin {
                    denom: config.pool.denom.clone(),
                    amount: half_draw_fee,
                }],
            }));
        }
    }

    Ok(Response::new().add_messages(messages).add_attributes(vec![
        attr("action", "resolve_game"),
        attr("winner", winner.unwrap_or_else(|| "draw".to_string())),
    ]))
}

pub fn execute_refund_draw(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let config: Config = CONFIG.load(deps.storage)?;

    if config.status == "ongoing" {
        return Err(ContractError::GameNotFinished {});
    }

    if config.winner.is_some() {
        return Err(ContractError::GameResolved {});
    }

    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    }

    let mut messages: Vec<cosmwasm_std::CosmosMsg> = vec![];
    let fee_percentage = config.admin_fee_percentage;

    for (addr, wager) in config.wagers.iter() {
        let fee = wager.amount.amount * Uint128::from(fee_percentage) / Uint128::from(100u64);
        let refund = wager.amount.amount - fee;

        messages.push(cosmwasm_std::CosmosMsg::Bank(BankMsg::Send {
            to_address: addr.to_string(),
            amount: vec![Coin {
                denom: wager.amount.denom.clone(),
                amount: refund,
            }],
        }));
    }

    Ok(Response::new()
        .add_messages(messages)
        .add_attributes(vec![attr("action", "refund_draw")]))
}
