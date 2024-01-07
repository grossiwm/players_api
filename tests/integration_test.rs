use ethers::{prelude::*, utils::{Ganache, GanacheInstance}};
use players_api::ethereum_service;
use std::{convert::TryFrom, sync::Arc, env};

#[tokio::test]
async fn should_receive_amount_for_generated_address() -> Result<(), Box<dyn std::error::Error>> {

    let ganache = set_up_ganache();
    let provider = Provider::<Http>::try_from(ganache.endpoint())?;

    let sender_wallet: LocalWallet = ganache.keys()[0].clone().into();
    let amount = U256::from(1e18 as u64);

    let (to_address, _) = ethereum_service::generate_key_pair();
    let to_address = &to_address;

    let balance = ethereum_service::get_balance(to_address).await;
    println!("Balance before operation: {} Wei", balance);
    
    send_amount_to_address(provider, to_address.parse()?, amount, sender_wallet).await;
    let balance = ethereum_service::get_balance(to_address).await;

    println!("Saldo do endereço de destino após a transação: {} Wei", balance);

    assert_eq!(balance, amount);
    Ok(())
}

fn set_up_ganache() -> GanacheInstance {
    let ganache = Ganache::new().spawn();
    let ganache_endpoint = &ganache.endpoint();
    println!("HTTP Endpoint: {}", ganache_endpoint);
    env::set_var("ETHEREUM_NETWORK_RPC_URL", ganache.endpoint());
    ganache
}

async fn send_amount_to_address(provider: Provider<Http>, to_address: Address, amount: U256, sender_wallet: LocalWallet) {


    let client = SignerMiddleware::new(provider.clone(), sender_wallet.clone());
    let client = Arc::new(client);

    let nonce = provider.get_transaction_count(sender_wallet.address(), None).await.unwrap();
    
    let tx = TransactionRequest::new()
        .to(to_address)
        .value(amount)
        .from(client.address())
        .nonce(nonce);

    let tx = client.send_transaction(tx, None).await.unwrap();
    println!("Sent! TxHash: {:?}", tx);
}
