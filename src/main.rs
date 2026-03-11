use dotenv::dotenv;
use reqwest::Error;
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
struct GasOracleResponse {
    status: String,
    message: String,
    result: GasResult,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct GasResult {
    last_block: String,
    safe_gas_price: String,
    propose_gas_price: String,
    fast_gas_price: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    
    let api_key = env::var("ETHERSCAN_API_KEY").unwrap_or_else(|_| "YourApiKeyToken".to_string());
    let url = format!("https://api.etherscan.io/api?module=gastracker&action=gasoracle&apikey={}", api_key);
    
    println!("Fetching current gas prices from Etherscan...");
    
    let response = reqwest::get(&url).await?.json::<GasOracleResponse>().await?;
    
    if response.status == "1" {
        println!("🚀 Fast Gas Price: {} gwei", response.result.fast_gas_price);
        println!("🚶 Propose Gas Price: {} gwei", response.result.propose_gas_price);
        println!("🐢 Safe Gas Price: {} gwei", response.result.safe_gas_price);
        println!("📦 Last Block: {}", response.result.last_block);
    } else {
        println!("Error fetching gas prices: {}", response.message);
    }
    
    Ok(())
}
