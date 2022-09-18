use std::str;

use solana_client::rpc_client::RpcClient;
// use solana_program::pubkey::Pubkey;
#[allow(unused_imports)]
use solana_sdk::{system_transaction, signature::{Keypair, Signature}};

#[allow(unused_imports)]
use rust_client::{create_keypair, from_key, check_balance, request_air_drop};

use solana_sdk::signature::Signer;

const URL: &str = "https://api.devnet.solana.com";

fn main() {
    let recovered: Keypair = from_key();
    let rpc_client = RpcClient::new(URL);
    // how do i save priv key and use on program restart
    // let sender: Keypair = create_keypair();
    // let receiver: Keypair = create_keypair();

    println!("Recovered: {:?}", recovered.pubkey());
    println!("Recovered: {:?}", recovered.to_base58_string());

    println!("\n==============================\n");


    if let Ok(airdrop_signature) = request_air_drop(&rpc_client, &recovered.pubkey(), 1.0) {
        println!("Airdrop finished! Signature: {:?}",  airdrop_signature);

        if let Ok(balance) = check_balance(&rpc_client, &recovered.pubkey()) {
            println!("Sender balance: {:?}", balance);
        }
    } else {
        println!("Airdrop failed");
    }

    println!("\n==============================\n");

    let epoch_info = rpc_client.get_epoch_info();
    println!("epock_info: {:?}\n", epoch_info);

    let production = rpc_client.get_block_production();
    println!("get_block_production: {:?}\n", production);
}
