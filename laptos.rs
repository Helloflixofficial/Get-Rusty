use laptos::{Account, Client, Transaction};
use tokio::runtime::Runtime;

#[tokio::main]
async fn main() {
    let client = Client::new("http://testnet.laptos.dev", "testnet");

    let account = Account::new();

    println!("Account Address: {}", account.address());
    println!("Account Public Key: {}", account.public_key());

    match client.get_balance(account.address()).await {
        Ok(balance) => println!("Account Balance: {} LPT", balance),
        Err(e) => println!("Error fetching balance: {}", e),
    }

    let to_address = "receiver_account_address_here"; 
    let amount = 100;

    let transaction = Transaction::new_transfer(amount, to_address);

    let signed_tx = account.sign_transaction(transaction).unwrap();

    match client.submit_transaction(signed_tx).await {
        Ok(tx_hash) => println!("Transaction submitted: {}", tx_hash),
        Err(e) => println!("Error submitting transaction: {}", e),
    }
}
