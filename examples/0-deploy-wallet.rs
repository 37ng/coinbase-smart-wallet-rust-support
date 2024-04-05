// pub use coinbase_smart_wallet_factory::*;
use ethers::prelude::*;
use std::{convert::TryFrom, sync::Arc};

use rust::bindings::coinbase_smart_wallet_factory::CoinbaseSmartWalletFactory;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = Provider::<Http>::try_from("https://sepolia.base.org")?;
    let owner0: LocalWallet = "45658215d9a309352ce6b16d3678342b3a666c7cca8117dfc4da171d5cfd7853"
        .parse::<LocalWallet>()?
        .with_chain_id(Chain::BaseSepolia);
    let owner1: LocalWallet = "4611c596cd340c03b7e7a787a8373db1a2c3cd22a8b8189477b767c1f71f2747"
        .parse::<LocalWallet>()?
        .with_chain_id(Chain::BaseSepolia);
    let client = SignerMiddleware::new(provider.clone(), owner0.clone());
    let factory_address: Address = "0xeD4EAeBDBBA52DBB37259a2b75AbB87abF3a19E8".parse()?;

    let owners = vec![
        Bytes::from(H256::from(owner0.address()).0.to_vec()),
        Bytes::from(H256::from(owner1.address()).0.to_vec()),
    ];
    let nonce = U256::from(0);

    let factory = CoinbaseSmartWalletFactory::new(factory_address, Arc::new(client));
    let smart_wallet_address = factory.get_address(owners.clone(), nonce).await.unwrap();
    let tx = factory.create_account(owners, nonce);
    let _pending_tx = tx.send().await?;

    println!(
        "smart wallet deployed at address: {:?}",
        smart_wallet_address
    );
    Ok(())
}
