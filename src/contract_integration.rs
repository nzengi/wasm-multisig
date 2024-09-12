use std::env;
use web3::types::{TransactionRequest, U256, H256};
use web3::transports::Http;
use web3::Web3;

/// Submits a transaction to the Ethereum network.
/// This function handles Infura URL and private key management through environment variables.
pub async fn submit_transaction(
    to_address: &str,        // The recipient address for the transaction
    raw_data: Vec<u8>,       // The data payload for the transaction
    gas_limit: U256,         // Gas limit for the transaction
    gas_price: U256,         // Gas price for the transaction
    nonce: U256              // Transaction nonce (to ensure uniqueness)
) -> web3::Result<H256> {
    // Fetch the Infura Project ID from environment variables
    let infura_project_id = env::var("INFURA_PROJECT_ID")
        .expect("Infura project ID must be set in environment variables");

    // Construct the Infura URL using the project ID
    let infura_url = format!("https://mainnet.infura.io/v3/{}", infura_project_id);
    
    // Set up a connection to the Ethereum network using the Infura HTTP endpoint
    let transport = Http::new(&infura_url)?;
    let web3 = Web3::new(transport);

    // Create the transaction request
    let tx = TransactionRequest {
        to: Some(to_address.parse().unwrap()),  // Parse the recipient address into the appropriate format
        gas: Some(gas_limit),                    // Set the gas limit for the transaction
        gas_price: Some(gas_price),              // Set the gas price to be paid for the transaction
        value: Some(U256::zero()),               // Set the value (amount of Ether) to be sent (in this case, 0 Ether)
        data: Some(raw_data.into()),             // Include the raw data payload for the transaction
        nonce: Some(nonce),                      // Set the transaction nonce to ensure it is unique
        ..Default::default()                     // Fill any remaining fields with default values
    };

    // Send the transaction and return the transaction hash as a result
    let result = web3.eth().send_transaction(tx).await?;
    Ok(result)
}
