#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coins, to_binary, BankMsg, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Order, Response,
    StdResult,
};
use cw_storage_plus::Bound;
// use cw2::set_contract_version;

use crate::config::{Config, CONFIG};
use crate::error::ContractError;
use crate::game::{Game, GAMES};
use crate::msg::{
    ConfigResponse, ExecuteMsg, GameResponse, GamesResponse, InstantiateMsg, QueryMsg,
};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:kujirans-racers";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config::from(msg);
    config.validate()?;
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::default())
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
            start_time,
            max_players,
        } => {
            let config = CONFIG.load(deps.storage)?;
            let entry_fee = cw_utils::one_coin(&info)?;
            config.assert_entry_fee(&entry_fee)?;
            if max_players > config.max_players {
                return Err(ContractError::InvalidGame {});
            }
            let game = Game::new(
                deps.storage,
                entry_fee.clone(),
                start_time,
                info.sender.clone(),
                max_players,
            );
            game.save(deps.storage)?;
            Ok(Response::default()
                .add_attribute("action", "create_game")
                .add_attribute("creator", info.sender)
                .add_attribute("entry_fee", entry_fee.to_string())
                .add_attribute("max_players", max_players.to_string())
                .add_attribute("start_time", start_time.to_string()))
        }
        ExecuteMsg::JoinGame { idx } => {
            let mut game = Game::load(deps.storage, &idx)?;
            let entry_fee = cw_utils::one_coin(&info)?;
            game.join(info.sender.clone(), entry_fee.clone(), env.block.time)?;
            game.save(deps.storage)?;
            Ok(Response::default()
                .add_attribute("action", "join_game")
                .add_attribute("idx", idx)
                .add_attribute("joiner", info.sender)
                .add_attribute("entry_fee", entry_fee.to_string()))
        }

        ExecuteMsg::EndGame { idx, winner } => {
            let config = CONFIG.load(deps.storage)?;

            if info.sender != config.admin {
                return Err(ContractError::Unauthorized {});
            }
            let mut game = Game::load(deps.storage, &idx)?;
            let total = game.close(winner.clone(), env.block.time)?;
            let fee = total.amount * config.fee_amount;
            let winnings = total.amount - fee;
            Ok(Response::default()
                .add_message(CosmosMsg::Bank(BankMsg::Send {
                    to_address: winner.to_string(),
                    amount: coins(winnings.u128(), total.denom.clone()),
                }))
                .add_message(CosmosMsg::Bank(BankMsg::Send {
                    to_address: config.fee_address.to_string(),
                    amount: coins(fee.u128(), total.denom),
                })))
        }
        ExecuteMsg::Configure {
            admin,
            fee_amount,
            fee_address,
            max_players,
        } => {
            let mut config = CONFIG.load(deps.storage)?;
            config.update(admin, fee_amount, fee_address, max_players);
            config.validate()?;
            CONFIG.save(deps.storage, &config)?;
            Ok(Response::default())
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&ConfigResponse::from(CONFIG.load(deps.storage)?)),
        QueryMsg::Game { idx } => to_binary(&GameResponse::from(Game::load(deps.storage, &idx)?)),
        QueryMsg::Games { limit, start_after } => {
            let limit = limit.unwrap_or(10u8) as usize;
            let min = start_after.map(|x| Bound::exclusive(x.u128()));
            let games = GAMES
                .range(deps.storage, min, None, Order::Ascending)
                .take(limit)
                .map(|x| x.map(|y| y.1))
                .collect::<StdResult<Vec<Game>>>()?;
            to_binary(&GamesResponse {
                games: games.iter().map(|g| g.clone().into()).collect(),
            })
        }
    }
}

#[cfg(test)]
mod tests {}
