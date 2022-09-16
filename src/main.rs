// use std::error::Error;
// use solana_program::pubkey::Pubkey;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{{clock:: TICKS_PER_DAY}};
use solana_sdk::signature::Signer; 
use solana_sdk::{signature::{Keypair}};
use std::str;

fn create_keypair() -> Keypair {
    Keypair::new()
}

const URL: &str = "https://api.devnet.solana.com";

fn main() {
    let rpc_client = RpcClient::new(URL);

    // how do i save priv key and use on program restart
    let sender: Keypair = create_keypair();
    let receiver: Keypair = create_keypair();

    println!("Sender: {:?}", sender.pubkey());
    println!("Sender: {:?}", sender.secret());
    println!("Receiver: {:?}", receiver.pubkey());

    println!("\n==============================\n");

    let epoch_info = rpc_client.get_epoch_info();
    println!("epock_info: {:?}\n", epoch_info);

    let production = rpc_client.get_block_production();
    println!("get_block_production: {:?}\n", production);
}
