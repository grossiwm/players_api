use std::env;

use ethers::{signers::{LocalWallet, Wallet, Signer}, providers::{Provider, Http, Middleware}, types::U256, abi::Address, core::{rand, k256::{ecdsa::SigningKey, Secp256k1}}};
use hex::ToHex;


pub async fn get_balance(address: &str) -> U256 {
    let provider = get_provider();
    let address: Address = address.parse().unwrap();
    provider.get_balance(address, None).await.unwrap()
}

pub fn generate_key_pair() -> (String, String) {
    let wallet = generate_wallet();
    (format!("{:?}",wallet.address()), wallet.signer().to_bytes().encode_hex())

}

fn generate_wallet() -> Wallet<SigningKey> {
    LocalWallet::new(&mut rand::thread_rng())
}

fn get_provider() -> Provider<Http> {
    let provider_uri = &env::var("ETHEREUM_NETWORK_RPC_URL").expect("Please set ethereum provider in .env");
    Provider::<Http>::try_from(provider_uri).expect(&format!("Could not get provider at {}", provider_uri))
}