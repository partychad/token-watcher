use hex;
use http::{header};
use reqwest;
use serde_json::json;
use tokio::time::{interval, Duration};

pub async fn make_post_request_with_header(
    url: &str,
    json: &serde_json::Value,
) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();

    let response = client
        .post(url)
        .header(header::CONTENT_TYPE, "application/json")
        .json(json)
        .send()
        .await?;

    let response_text = response.text().await?;
    Ok(response_text)
}

#[tokio::main]
async fn main() {

    let mut interval = interval(Duration::from_secs(1));
    let mut previous_balance: Option<f32> = None;
    loop {
        interval.tick().await;
        let balance = match calculate_wallet_balance().await {
            Ok(balance) => balance,
            Err(error) => {
                println!("Error: {}", error);
                continue;
            }
        };
        match previous_balance {
            Some(previous_balance) => {
                if balance != previous_balance {
                    println!("Balance has changed: {}", balance);
                }
            }
            None => {
                println!("Balance: {}", balance);
            }
        }
        previous_balance = Some(balance);
    }
   
}

async fn calculate_wallet_balance() -> Result<f32, Box<dyn std::error::Error>>{
    let wavax_balance = query_balance_of("0x43BF8DB4Ca35dBd9343b3f49DF1D82077b51b356", "0xB31f66AA3C1e785363F0875A1B74E27b85FD66c7");
    let avax_balance = query_eth_balance_of("0x43BF8DB4Ca35dBd9343b3f49DF1D82077b51b356");

    let (wavax_balance_result, avax_balance_result) = tokio::join!(wavax_balance, avax_balance);

    let wavax = match wavax_balance_result {
        Ok(balance) => {println!("Balance: {}", balance) ; balance} ,
        Err(error) => {println!("Error: {}", error); return Err(error);},
    };

    let avax = match avax_balance_result {
        Ok(balance) => {println!("Balance: {}", balance) ; balance},
        Err(error) => {println!("Error: {}", error); return Err(error);},
    };

   Ok(wavax + avax)
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


async fn query_balance_of(address: &str, token_address:&str) -> Result<f32, Box<dyn std::error::Error>> {
    let encoded_params = encode("balanceOf(address)", address);
    const DECIMAL: u128 = 10_u128.pow(18);
    let data = json!({
        "jsonrpc":"2.0",
        "method":"eth_call",
        "params": [
        {
            "to": token_address,
            "data": encoded_params
        },
        "latest"
        ],
        "id": 1
    });

    let response_data = match make_post_request_with_header(
        "https://api.avax.network/ext/bc/C/rpc",
        &data
    )
    .await
    {
        Ok(response) => {
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

async fn query_eth_balance_of(address: &str) -> Result<f32, Box<dyn std::error::Error>> {
    const DECIMAL: u128 = 10_u128.pow(18);
    let data = json!({
        "jsonrpc":"2.0",
        "method":"eth_getBalance",
        "params": [
        address,
        "latest"
        ],
        "id": 1
    });
    let response_data = match make_post_request_with_header(
        "https://api.avax.network/ext/bc/C/rpc",
        &data
    )
    .await
    {
        Ok(response) => {
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