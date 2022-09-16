// use std::error::Error;
// use solana_program::pubkey::Pubkey;
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::Signer; 
use solana_sdk::{signature::{Keypair}};
use std::str;

fn create_keypair() -> Keypair {
    Keypair::new()
}

const URL: &str = "https://api.devnet.solana.com";

fn main() {
    let _rpc_client = RpcClient::new(URL);

    // how do i save priv key and use on program restart
    let sender: Keypair = create_keypair();
    let receiver: Keypair = create_keypair();

    println!("Sender: {:?}", sender.pubkey());
    println!("Sender: {:?}", sender.secret());
    println!("Receiver: {:?}", receiver.pubkey());
}
