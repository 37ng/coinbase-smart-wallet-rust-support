# coinbase-smart-wallet-rust-support

This is demo code to show how to integrate Coinbase Smart Wallets that support ERC-1271.

0. Deploy a Coinbase Smart Wallet through CoinbaseSmartWalletFactory. In this example code, we utilize the one deployed on Base Sepolia. You can choose to deploy your own. When deploying the smart wallet, you can specify at most 256 owners, each represented by either EOAs or passkeys(passkey logics is not in the code, only EOA).
1. Sign a arbitrary message with the owner key, encode it as signature. Verify the signature via ERC-1271 interface `isValidSignature`.
2. Now, we can revoke an owner of the smart wallet and verify the signature will fail at the latest block, and verify it will succeed at the block before the revokation.
