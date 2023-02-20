use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin, Decimal, Timestamp, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: Addr,
    pub min_entry_fee: Vec<Coin>,
    pub fee_amount: Decimal,
    pub fee_address: Addr,
    pub max_players: u8,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Create a new game to start at start_time
    /// A single token must be provided, which becomes the entry fee for this game
    CreateGame {
        start_time: Timestamp,
        max_players: u8,
    },
    /// Join an unstarted game
    JoinGame { idx: Uint128 },

    /// Close out a game with a winner, distribute funds.
    /// Onlly callable by admin
    EndGame { idx: Uint128, winner: Addr },

    Configure {
        admin: Option<Addr>,
        fee_amount: Option<Decimal>,
        fee_address: Option<Addr>,
        max_players: Option<u8>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(GameResponse)]
    Game { idx: Uint128 },
    #[returns(GamesResponse)]
    Games {
        limit: Option<u8>,
        start_after: Option<Uint128>,
    },
}

#[cw_serde]
pub struct ConfigResponse {
    pub admin: Addr,
    pub min_entry_fee: Vec<Coin>,
    pub fee_amount: Decimal,
    pub fee_address: Addr,
    pub max_players: u8,
}

#[cw_serde]
pub struct GameResponse {
    pub idx: Uint128,
    pub entry_fee: Coin,
    pub start_time: Timestamp,
    pub creator: Addr,
    pub max_players: u8,
    pub players: Vec<Addr>,
    pub winner: Option<Addr>,
}

#[cw_serde]
pub struct GamesResponse {
    pub games: Vec<GameResponse>,
}
