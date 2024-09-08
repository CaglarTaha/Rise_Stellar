use reqwest::Client;
use serde_json::json;

pub async fn send_payment(from_secret: &str, to_address: &str, amount: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let tx_payload = json!({
        "from": from_secret,
        "to": to_address,
        "amount": amount,
        "asset": "native",
        "memo": "Your short message here"
    });

    let response = client
        .post("https://horizon-testnet.stellar.org/transactions")
        .json(&tx_payload)
        .send()
        .await?;

    println!("Transaction response: {:?}", response);
    Ok(())
}
