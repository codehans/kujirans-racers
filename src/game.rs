use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Order, StdResult, Storage, Timestamp, Uint128};
use cw_storage_plus::Map;

use crate::{msg::GameResponse, ContractError};

pub static GAMES: Map<u128, Game> = Map::new("games");

#[cw_serde]
pub struct Game {
    pub idx: Uint128,
    pub entry_fee: Coin,
    pub start_time: Timestamp,
    pub creator: Addr,
    pub max_players: u8,
    pub players: Vec<Addr>,
    pub winner: Option<Addr>,
}

impl Game {
    fn next_idx(storage: &dyn Storage) -> Uint128 {
        match GAMES.keys(storage, None, None, Order::Descending).next() {
            Some(Ok(x)) => Uint128::from(x + 1),
            _ => Uint128::default(),
        }
    }

    pub fn new(
        storage: &dyn Storage,
        entry_fee: Coin,
        start_time: Timestamp,
        creator: Addr,
        max_players: u8,
    ) -> Self {
        Self {
            idx: Self::next_idx(storage),
            entry_fee,
            start_time,
            creator,
            max_players,
            players: vec![],
            winner: None,
        }
    }

    pub fn load(storage: &dyn Storage, idx: &Uint128) -> StdResult<Self> {
        GAMES.load(storage, idx.u128())
    }

    pub fn save(&self, storage: &mut dyn Storage) -> StdResult<()> {
        GAMES.save(storage, self.idx.u128(), self)
    }

    pub fn remove(&self, storage: &mut dyn Storage) {
        GAMES.remove(storage, self.idx.u128())
    }

    fn is_full(&self) -> bool {
        self.players.len() == self.max_players as usize
    }

    pub fn join(
        &mut self,
        joiner: Addr,
        entry_fee: Coin,
        time: Timestamp,
    ) -> Result<(), ContractError> {
        if time.seconds() > self.start_time.seconds() {
            return Err(ContractError::GameStarted {});
        }

        if self.is_full() {
            return Err(ContractError::GameFull {});
        }

        if self.entry_fee != entry_fee {
            return Err(ContractError::InvalidEntryFee {});
        }

        self.players.push(joiner);

        Ok(())
    }

    pub fn close(&mut self, winner: Addr, time: Timestamp) -> Result<Coin, ContractError> {
        if time.seconds() < self.start_time.seconds() {
            return Err(ContractError::GameNotStarted {});
        }

        self.winner = Some(winner);
        Ok(Coin {
            denom: self.entry_fee.denom.clone(),
            amount: self.entry_fee.amount * Uint128::from(self.players.len() as u32),
        })
    }
}

impl From<Game> for GameResponse {
    fn from(g: Game) -> Self {
        Self {
            idx: g.idx,
            entry_fee: g.entry_fee,
            start_time: g.start_time,
            creator: g.creator,
            max_players: g.max_players,
            players: g.players,
            winner: g.winner,
        }
    }
}
