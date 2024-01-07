use diesel::prelude::*;
use ethers::types::U256;

use crate::database::establish_connection;
use crate::ethereum_service;
use crate::schema::wallets::dsl::*;

pub struct Wallet {
    pub wallet_id: i32,
    pub player_id: i32,
    pub address: String,
    pub private_key: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::wallets)]
pub struct NewWallet {
    pub player_id: i32,
    pub address: String,
    pub private_key: String,
}


impl Wallet {

    pub async fn get_balance(&self) -> U256 {
        ethereum_service::get_balance(&self.address).await
    }

    pub fn create(pid: i32) {
        let conn = &mut establish_connection();
        let (addr, privk) = ethereum_service::generate_key_pair();
        
        let new_wallet = NewWallet {
            player_id: pid,
            address: addr,
            private_key: privk,
        };

        diesel::insert_into(wallets)
            .values(&new_wallet)
            .execute(conn)
            .expect("Failed to create new wallet");
    }
}
