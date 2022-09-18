use std::error::Error;
use std::fs;
use solana_client::rpc_client::RpcClient;
#[allow(unused_imports)]
use solana_sdk::{signature::{Keypair, Signature}};
use solana_program::pubkey::Pubkey;

const LAMPORTS_PER_SOL: f64 = 1000000000.0;

pub fn create_keypair() -> Keypair {
    Keypair::new()
}

pub fn from_key() -> Keypair {
    let key = fs::read_to_string("key.txt")
        .expect("The file coud not be read");

    Keypair::from_base58_string(&key)
}


pub fn check_balance(rpc_client: &RpcClient, public_key: &Pubkey) -> Result<f64, Box<dyn Error>> {
    Ok(rpc_client.get_balance(&public_key)? as f64 / LAMPORTS_PER_SOL)
}

pub fn request_air_drop(rpc_client: &RpcClient, pub_key: &Pubkey, amount_sol: f64) -> Result<Signature, Box<dyn Error>> {
    let sig = rpc_client.request_airdrop(&pub_key, (amount_sol * LAMPORTS_PER_SOL) as u64)?;
    loop {
        let confirmed = rpc_client.confirm_transaction(&sig)?;
        if confirmed {
            break;
        }
    }
    Ok(sig)
}

