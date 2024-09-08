use reqwest::Client;
use serde_json::Value;

pub async fn get_account_balance(account_id: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = format!("https://horizon-testnet.stellar.org/accounts/{}", account_id);
    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        let account_data: Value = response.json().await?;
        let balances = &account_data["balances"];
        if let Some(balance) = balances.as_array().and_then(|arr| arr.get(0)) {
            let balance_amount = balance["balance"].as_str().unwrap_or("0");
            return Ok(balance_amount.to_string());
        }
    }

    Err("Unable to retrieve balance".into())
}

pub async fn send_payment(from_secret: &str, to_address: &str, amount: &str, memo: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let tx_payload = serde_json::json!({
        "from": from_secret,
        "to": to_address,
        "amount": amount,
        "asset": "native",  
        "memo": memo
    });

    let response = client
        .post("https://horizon-testnet.stellar.org/transactions")
        .json(&tx_payload)
        .send()
        .await?;

    if response.status().is_success() {
        println!("Transaction successful: {:?}", response);
        Ok(())
    } else {
        Err("Transaction failed".into())
    }
}
