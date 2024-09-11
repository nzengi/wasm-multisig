use web3::types::{TransactionRequest, U256, H256};
use web3::transports::Http;
use web3::Web3;

pub async fn submit_transaction(raw_data: Vec<u8>, gas_price: U256, nonce: U256) -> web3::Result<H256> {
    let transport = Http::new("https://mainnet.infura.io/v3/YOUR_PROJECT_ID")?;
    let web3 = Web3::new(transport);

    let tx = TransactionRequest {
        to: Some("0xRecipientAddress".parse().unwrap()),
        gas: Some(21000.into()),
        gas_price: Some(gas_price),
        value: Some(U256::zero()),
        data: Some(raw_data.into()),
        nonce: Some(nonce),
        ..Default::default()
    };

    let result = web3.eth().send_transaction(tx).await?;
    Ok(result)
}
