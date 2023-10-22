

//constants
pub const MODE: &str = "PRODUCTION";
//Close all open positions and order
pub const ABORT_ALL_POSITIONS: bool = true;
//Find cointegrated pairs
pub const FIND_COINTEGRATED: bool = true;
//Manage exits
pub const MANAGE_EXITS: bool = true;
//Place trades
pub const PLACE_TRADES: bool = true;
//Resolution
pub const RESOLUTION: &str = "1HOUR";
//Number of bars to load
pub const WINDOW: i32 = 21;
//Thresholds - opening
pub const MAX_HALF_LIFE: i32 = 24;
pub const ZSCORE_THRESH: f64 = 1.5;
pub const USD_PER_TRADE: f64 = 100.0;
pub const USD_MIN_COLLATERAL: f64 = 1000.0;
//Thresholds - closing
pub const CLOSE_AT_ZSCORE_CROSS: bool = true;
//Ethereum address
pub const ETHEREUM_ADDRESS: &str = "0xe164984904Edf45A2920cc42C381fef8289BC8b2";
// HTTP provider
pub const HTTP_PROVIDER_MAINNET: &str = "https://eth-mainnet.g.alchemy.com/v2/4-RE_XGYtMijHNgxIzFqhURHfSihtMi_";
pub const HTTP_PROVIDER_TESTNET: &str = "https://eth-goerli.g.alchemy.com/v2/d7L05eT6Dpf8eRbySjjebzV0WzozVMlh";
//API Host
pub const API_HOST_MAINNET: &str = "https://api.dydx.exchange";
pub const API_HOST_GOERLI: &str = "https://api.stage.dydx.exchange";


//Import environment variables
use std::env;

pub fn api_host_mainnet() -> Option<String> {
    env::var("API_HOST_MAINNET").ok()
}

pub fn api_host_testnet() -> Option<String> {
    env::var("API_HOST_GOERLI").ok()
}
//Keys PRODUCTION - Mainnet DYDX
pub fn stark_private_key_mainnet() -> Option<String> {
    env::var("STARK_PRIVATE_KEY_MAINNET").ok()
}

pub fn dydx_api_key_mainnet() -> Option<String> {
    env::var("DYDX_API_KEY_MAINNET").ok()
}

pub fn dydx_api_secret_mainnet() -> Option<String> {
    env::var("DYDX_API_SECRET_MAINNET").ok()
}

pub fn dydx_api_passphrase_mainnet() -> Option<String> {
    env::var("DYDX_API_PASSPHRASE_MAINNET").ok()
}

//Keys DEVELOPMENT - Testnet DYDX
pub fn stark_private_key_testnet() -> Option<String> {
    env::var("STARK_PRIVATE_KEY_TESTNET").ok()
}

pub fn dydx_api_key_testnet() -> Option<String> {
    env::var("DYDX_API_KEY_TESTNET").ok()
}

pub fn dydx_api_secret_testnet() -> Option<String> {
    env::var("DYDX_API_SECRET_TESTNET").ok()
}

pub fn dydx_api_passphrase_testnet() -> Option<String> {
    env::var("DYDX_API_PASSPHRASE_TESTNET").ok()
}

pub fn http_provider_mainnet() -> Option<String> {
    env::var("HTTP_PROVIDER_MAINNET").ok()
}

pub fn http_provider_testnet() -> Option<String> {
    env::var("HTTP_PROVIDER_TESTNET").ok()
}


//Keys export
pub fn stark_private_key() -> Option<String> {
    if MODE == "PRODUCTION" {
        stark_private_key_mainnet()
    } else {
        stark_private_key_testnet()
    }
}

pub fn dydx_api_key() -> Option<String> {
    if MODE == "PRODUCTION" {
        dydx_api_key_mainnet()
    } else {
        dydx_api_key_testnet()
    }
}

pub fn dydx_api_secret() -> Option<String> {
    if MODE == "PRODUCTION" {
        dydx_api_secret_mainnet()
    } else {
        dydx_api_secret_testnet()
    }
}

pub fn dydx_api_passphrase() -> Option<String> {
    if MODE == "PRODUCTION" {
        dydx_api_passphrase_mainnet()
    } else {
        dydx_api_passphrase_testnet()
    }
}


//Host export

pub fn host() -> Option<String> {
    if MODE == "PRODUCTION" {
        api_host_mainnet()
    } else {
        api_host_testnet()
    }
}

//HTTP PROVIDER EXPORT
pub fn http_provider() -> Option<String> {
    if MODE == "PRODUCTION" {
        http_provider_mainnet()
    } else {
        http_provider_testnet()
    }
}