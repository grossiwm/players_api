use ethers::{prelude::*, utils::Ganache};
use std::{convert::TryFrom, sync::Arc};

#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let ganache = Ganache::new().spawn();
    let ganache_endpoint = &ganache.endpoint();
    println!("HTTP Endpoint: {}", ganache_endpoint);

    let provider = Provider::<Http>::try_from(ganache_endpoint)?;

    let wallet: LocalWallet = ganache.keys()[0].clone().into();

    let client = SignerMiddleware::new(provider.clone(), wallet.clone());
    let client = Arc::new(client);

    let to_address: Address = "123456789abcdef123456789abcdef123456789a".parse()?;

    let amount = U256::from(1e18 as u64);

    let nonce = provider.get_transaction_count(wallet.address(), None).await?;
    
    let tx = TransactionRequest::new()
        .to(to_address)
        .value(amount)
        .from(client.address())
        .nonce(nonce);

    let balance = provider.get_balance(to_address, None).await?;
    println!("Balance before operation: {} Wei", balance);

    let tx = client.send_transaction(tx, None).await?;
    println!("Enviado! TxHash: {:?}", tx);

    let balance = provider.get_balance(to_address, None).await?;
    println!("Saldo do endereço de destino após a transação: {} Wei", balance);

    Ok(())
}
