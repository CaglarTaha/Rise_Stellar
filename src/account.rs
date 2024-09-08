use stellar_sdk::KeyPair;
use reqwest::Client;
use serde_json::Value;

pub fn create_wallet() {
    let pair = KeyPair::random();
    println!("Public Key: {}", pair.public().to_string());
    println!("Secret Seed: {}", pair.secret().to_string());
}

pub async fn check_balance(account_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = format!("https://horizon-testnet.stellar.org/accounts/{}", account_id);
    let response = client.get(&url).send().await?;

    let account_data: Value = response.json().await?;
    println!("Account data: {:?}", account_data);

    Ok(())
}
