use std::str;

use solana_client::rpc_client::RpcClient;
// use solana_program::pubkey::Pubkey;
#[allow(unused_imports)]
use solana_sdk::{system_transaction, signature::{Keypair, Signature}};

#[allow(unused_imports)]
use rust_client::{create_keypair, from_key, check_balance, request_air_drop};

use solana_sdk::signature::Signer;

// #[macro_use]
// extern crate serde_json;

const URL: &str = "https://api.devnet.solana.com";

// Accounts api was lacking, I couldn't find trx info etc


// Get Transactions from last block
fn main() {
    let rpc_client = RpcClient::new(URL);

    let new_key = create_keypair();
    let secret =  new_key.to_base58_string();
    println!("new key secret: {:?}", secret);

    let recovered: Keypair = from_key("key.txt");
    let pubkey = recovered.pubkey();

    println!("Recovered: {:?}", recovered.to_base58_string());
    println!("\n==============================\n");

    let ga = rpc_client.get_account(&pubkey);                                                                                               
    println!("get_account: {:?}", ga);
    println!("\n==============================\n");

    let ad = rpc_client.get_account_data(&pubkey);                                                                                               
    println!("get_account_data: {:?}", ad);
    println!("\n==============================\n");

    // if let Ok(airdrop_signature) = request_air_drop(&rpc_client, &pubkey, 0.1) {
    //     println!("Airdrop finished! Signature: {:?}",  airdrop_signature);

    //     if let Ok(balance) = check_balance(&rpc_client, &pubkey) {
    //         println!("Sender balance: {:?}", balance);
    //     }
    // } else {
    //     println!("Airdrop failed");
    // }


    let epoch_info = rpc_client.get_epoch_info();
    println!("epock_info: {:?}\n", epoch_info);

    let slot = rpc_client.get_slot().expect("failed to get slot");
    println!("get_slot: {} \n", slot); 

    let block_time = rpc_client.get_block_time(slot).expect("failed to get block time");
    println!("block_time: {:?}\n", block_time);

    // let obj = json!({"foo":1,"bar":2});
    // println!("{}", serde_json::to_string_pretty(&obj).unwrap());

    let block = rpc_client.get_block(slot).expect("failed to get block");
    // println!("{}", serde_json::to_string_pretty(&block).unwrap());

    println!("\n==============================\n");

    for trx in &block.transactions {
        println!("trx : {:?}\n", trx.transaction);
    }
}
