mod config;
mod connection;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    // Using the configurations from the config.rs module
    dotenv().ok();

    // Assuming you want to print out some of these values for debug purposes
    println!("Operating in {} mode", config::MODE);

    // Note: For actual production usage, refrain from printing sensitive details like API keys.

    // Call the connection function
    connection::connect_dydx().await;
}
