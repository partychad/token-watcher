use std::ffi::c_float;

use hex;
use http::{header};
use reqwest;
use serde_json::json;

pub async fn make_post_request_with_header(
    url: &str,
    to: &str,
    data: &str,
    id: u16,
) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();

    let response = client
        .post(url)
        .header(header::CONTENT_TYPE, "application/json")
        .json(&json!({
            "jsonrpc":"2.0",
            "method":"eth_call",
            "params": [
            {
                "to": to,
                "data": data
            },
            "latest"
            ],
            "id": id
        }))
        .send()
        .await?;

    let response_text = response.text().await?;
    Ok(response_text)
}

#[tokio::main]
async fn main() {
   match query_balance_of("0x43BF8DB4Ca35dBd9343b3f49DF1D82077b51b356", "0xB31f66AA3C1e785363F0875A1B74E27b85FD66c7").await {
        Ok(balance) => println!("Balance: {}", balance),
        Err(error) => println!("Error: {}", error),
    }
}

fn keccak256(bytes: &[u8]) -> [u8; 32] {
    web3_hash_utils::keccak256(bytes)
}
fn encode(signature: &str, address: &str) -> String {
    use ethabi::Token;
    use std::str::FromStr;
    let address_bytes: ethabi::ethereum_types::H160 =
        ethabi::Address::from_str(&address[2..]).unwrap();
    let token = Token::Address(address_bytes);

    let encoded_params = ethabi::encode(&[token]);
    let hex_encoded_address = hex::encode(encoded_params);

    let selector = &keccak256(signature.as_bytes())[0..4];
    let hex_with_prefix = format!("0x{}", hex::encode(selector));
    let concatenated = format!("{}{}", hex_with_prefix, hex_encoded_address);
    concatenated
}


async fn query_balance_of(address: &str, token_address:&str) -> Result<c_float, Box<dyn std::error::Error>> {
    let data = encode("balanceOf(address)", address);
    const DECIMAL: u128 = 10_u128.pow(18);
    let response_data = match make_post_request_with_header(
        "https://api.avax.network/ext/bc/C/rpc",
        token_address,
        &data,
        1,
    )
    .await
    {
        Ok(response) => {
            println!("Response: {}", response);
            response
        }
        Err(error) => {
            println!("Error: {}", error);
            return Err(Box::new(error));
        }
    };

    let response_json: serde_json::Value = serde_json::from_str(&response_data)?;
    let content = response_json["result"].as_str().unwrap_or_default();

    // Convert the content (hex string) to u128
    let balance = u128::from_str_radix(content.trim_start_matches("0x"), 16)?;

    Ok(balance as f32 / DECIMAL as f32 )
}

