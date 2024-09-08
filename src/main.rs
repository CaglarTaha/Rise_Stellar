mod account;
mod payment;
mod services {
    pub mod st_api;
    pub mod utils;
}

use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let secret_seed = env::var("SECRET_SEED").expect("SECRET_SEED not found");
    let receiver_address = env::var("RECEIVER_ADDRESS").expect("RECEIVER_ADDRESS not found");
    let account_id = env::var("ACCOUNT_ID").expect("ACCOUNT_ID not found");


    match services::stellar_api::get_account_balance(&account_id).await {
        Ok(balance) => println!("Account balance: {} XLM", balance),
        Err(e) => println!("{}", services::utils::format_error_message(&e.to_string())),
    }


    if let Err(e) = services::stellar_api::send_payment(&secret_seed, &receiver_address, "10", "Payment with message").await {
        println!("{}", services::utils::format_error_message(&e.to_string()));
    }
}
