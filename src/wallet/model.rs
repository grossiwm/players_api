use std::env;
use std::str::FromStr;

use diesel::prelude::*;
use ethers::core::rand;
use ethers::providers::Http;
use ethers::providers::Middleware;
use ethers::providers::Provider;
use ethers::signers::LocalWallet;
use ethers::signers::Signer;
use ethers::types::Address;
use hex::ToHex;

use crate::database::establish_connection;
use crate::schema::wallets::dsl::*;

pub struct Wallet {
    pub wallet_id: i32,
    pub player_id: i32,
    pub public_key: String,
    pub private_key: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::wallets)]
pub struct NewWallet {
    pub player_id: i32,
    pub public_key: String,
    pub private_key: String,
}


impl Wallet {

    pub fn create(pid: i32) {
        let conn = &mut establish_connection();
        let wallet = LocalWallet::new(&mut rand::thread_rng());
        let pubk = format!("{:?}",wallet.address());
        let privk = wallet.signer().to_bytes().encode_hex();

        let new_wallet = NewWallet {
            player_id: pid,
            public_key: pubk,
            private_key: privk,
        };

        diesel::insert_into(wallets)
            .values(&new_wallet)
            .execute(conn)
            .expect("Failed to create new wallet");
    }

    pub async fn get_balance(provider: Provider<Http>, address: &str) -> String {
        let address = Address::from_str(address).unwrap();
        let balance = provider.get_balance(address, None).await.unwrap();
        format!("Saldo: {}", balance)
    }

    fn get_provider() -> Provider<Http> {
        let provider_uri = &env::var("ETHEREUM_NETWORK_RPC_URL").expect("Please set ethereum provider in .env");
        Provider::<Http>::try_from(provider_uri).expect(&format!("Could not get provider at {}", provider_uri))
    }
}
