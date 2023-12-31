use diesel::prelude::*;
use ethers::core::k256::FieldBytes;
use ethers::core::rand;
use ethers::signers::LocalWallet;
use ethers::signers::Signer;

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
        let privk = Self::encrypt(wallet.signer().to_bytes()).to_string();

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

    fn encrypt(_b: FieldBytes) -> &'static str {
        let r = "aaaaa";
        r
    }
}
