
use std::env;
use web3::transports::Http;
use web3::Web3;
use dydx_v3_rust::{types::*, ClientOptions, DydxClient};

// Import constants from config.rs

// Assuming these are in another module, you can import them like this
use crate::config::*;

pub async fn connect_dydx() {
    // Extract environment variables (assuming you set them up to replace the previous `config` usage in Python)
    //let host = host().expect("HOST must be set");
    let api_key = dydx_api_key().expect("DYDX_API_KEY must be set");
    let api_secret = dydx_api_secret().expect("DYDX_API_SECRET must be set");
    let api_passphrase = dydx_api_passphrase().expect("DYDX_API_PASSPHRASE must be set");
    let stark_key = stark_private_key().expect("STARK_PRIVATE_KEY must be set");
    let eth_key = env::var("ETH_PRIVATE_KEY").expect("ETH_PRIVATE_KEY must be set");
    let eth_address = env::var("ETHEREUM_ADDRESS").expect("ETH_ADDRESS must be set");

    let options: ClientOptions = ClientOptions {
        network_id: Some(5),
        api_timeout: None,
        api_key_credentials: Some(ApiKeyCredentials {
            key: &api_key,
            secret: &api_secret,
            passphrase: &api_passphrase,
        }),
        stark_private_key: Some(&stark_key),
        eth_private_key: Some(&eth_key),
    };


    let client = DydxClient::new("https://api.dydx.exchange", options);
    let private = &client.private.unwrap();
    //let private = &client.private.unwrap();
    
    //Confirm the client connection
    let account = private.get_account(&eth_address).await.expect("Error getting account");
    let account_id = account.account.id;
    let quote_balance = account.account.quote_balance;

    println!("Connection successful!");
    println!("Account ID: {}", account_id);
    println!("Quote Balance: {}", quote_balance);
    
}

