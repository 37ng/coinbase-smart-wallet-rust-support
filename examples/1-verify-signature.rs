// pub use coinbase_smart_wallet_factory::*;
use ethers::{abi::Token, prelude::*};
use std::{convert::TryFrom, sync::Arc};

use rust::bindings::coinbase_smart_wallet::CoinbaseSmartWallet;

const ERC1271_MAGIC_VALUE: [u8; 4] = [0x16, 0x26, 0xba, 0x7e];

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

    // verify signatures
    let smart_wallet_address: Address = "0x2f84acc18877f2a69e864e572c810816bd4fdb64".parse()?;
    let smart_wallet = CoinbaseSmartWallet::new(smart_wallet_address, client.clone());
    let hash: [u8; 32] = H256::random().into();
    let replay_safe_hash = smart_wallet.replay_safe_hash(hash).call().await.unwrap();
    let signature0 = owner0.sign_hash(replay_safe_hash.into()).unwrap();
    // owner0 verify its own
    let res = smart_wallet
        .is_valid_signature(
            hash,
            abi::encode(&[Token::Tuple(vec![
                Token::Uint(U256::from(0)),
                Token::Bytes(signature0.to_vec()),
            ])])
            .into(),
        )
        .call()
        .await
        .unwrap();
    assert_eq!(res, ERC1271_MAGIC_VALUE);
    println!("owner0 verified its own signature");
    // owner1 verify its own
    let signature1 = owner1.sign_hash(replay_safe_hash.into()).unwrap();
    let res = smart_wallet
        .is_valid_signature(
            hash,
            abi::encode(&[Token::Tuple(vec![
                Token::Uint(U256::from(1)),
                Token::Bytes(signature1.to_vec()),
            ])])
            .into(),
        )
        .call()
        .await
        .unwrap();
    assert_eq!(res, ERC1271_MAGIC_VALUE);
    println!("owner1 verified its own signature");
    // owner0 tries to impersonate owner1
    let res = smart_wallet
        .is_valid_signature(
            hash,
            abi::encode(&[Token::Tuple(vec![
                Token::Uint(U256::from(1)),
                Token::Bytes(signature0.to_vec()),
            ])])
            .into(),
        )
        .call()
        .await
        .unwrap();
    assert_ne!(res, ERC1271_MAGIC_VALUE);
    println!("owner0 failed to impersonate owner1");

    Ok(())
}
