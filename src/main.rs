use std::str::FromStr;

use aptos_sdk::rest_client::Client;
use once_cell::sync::Lazy;
use url::Url;

static NODE_URL: Lazy<Url> = Lazy::new(|| {
    Url::from_str(
        std::env::var("APTOS_NODE_URL")
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("https://fullnode.devnet.aptoslabs.com"),
    )
    .unwrap()
});

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let rest_client = Client::new(NODE_URL.clone());
    println!("NODE_URL {}", NODE_URL.clone());

    let aptos_version = rest_client.get_aptos_version().await?.into_inner();
    println!("aptos_version {:?}", aptos_version);

    let gas_estimation = rest_client.estimate_gas_price().await?.into_inner();
    println!("gas_estimation {:?}", gas_estimation);

    Ok(())
}
