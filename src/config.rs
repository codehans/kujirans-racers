use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Decimal};
use cw_storage_plus::Item;

use crate::{
    msg::{ConfigResponse, InstantiateMsg},
    ContractError,
};

pub const CONFIG: Item<Config> = Item::new("config");

#[cw_serde]
pub struct Config {
    pub admin: Addr,
    pub min_entry_fee: Vec<Coin>,
    pub fee_amount: Decimal,
    pub fee_address: Addr,
    pub max_players: u8,
}

impl From<InstantiateMsg> for Config {
    fn from(msg: InstantiateMsg) -> Self {
        Self {
            admin: msg.admin,
            min_entry_fee: msg.min_entry_fee,
            fee_address: msg.fee_address,
            fee_amount: msg.fee_amount,
            max_players: msg.max_players,
        }
    }
}

impl From<Config> for ConfigResponse {
    fn from(msg: Config) -> Self {
        Self {
            admin: msg.admin,
            min_entry_fee: msg.min_entry_fee,
            fee_address: msg.fee_address,
            fee_amount: msg.fee_amount,
            max_players: msg.max_players,
        }
    }
}

impl Config {
    pub fn validate(&self) -> Result<(), ContractError> {
        if self.fee_amount > Decimal::one() {
            return Err(ContractError::InvalidConfig {});
        }
        Ok(())
    }

    pub fn assert_entry_fee(&self, fee: &Coin) -> Result<(), ContractError> {
        if let Some(_) = self
            .min_entry_fee
            .iter()
            .find(|x| x.denom == fee.denom && x.amount >= fee.amount)
        {
            return Ok(());
        }

        Err(ContractError::InvalidEntryFee {})
    }

    pub fn update(
        &mut self,
        admin: Option<Addr>,
        fee_amount: Option<Decimal>,
        fee_address: Option<Addr>,
        max_players: Option<u8>,
    ) {
        if let Some(admin) = admin {
            self.admin = admin
        }
        if let Some(fee_amount) = fee_amount {
            self.fee_amount = fee_amount
        }
        if let Some(fee_address) = fee_address {
            self.fee_address = fee_address
        }
        if let Some(max_players) = max_players {
            self.max_players = max_players
        }
    }
}
