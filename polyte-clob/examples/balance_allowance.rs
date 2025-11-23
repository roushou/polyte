use polyte_clob::{Chain, Clob, Credentials};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing subscriber to see debug logs
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("debug")),
        )
        .init();

    // Get private key and credentials from environment
    let private_key =
        std::env::var("PRIVATE_KEY").expect("PRIVATE_KEY environment variable must be set");
    let credentials = Credentials {
        key: std::env::var("API_KEY").expect("API_KEY environment variable must be set"),
        secret: std::env::var("API_SECRET").expect("API_SECRET environment variable must be set"),
        passphrase: std::env::var("API_PASSPHRASE")
            .expect("API_PASSPHRASE environment variable must be set"),
    };

    // Create CLOB client with authentication
    let clob = Clob::builder(private_key, credentials)?
        .chain(Chain::PolygonMainnet)
        .build()?;

    println!("=== Balance and Allowance Example ===\n");

    let token_id = "73470541315377973562501025254719659796416871135081220986683321361000395461644";

    // Get balance and allowance
    println!("Fetching balance and allowance for asset: {}", token_id);
    match clob.account().balance_allowance(token_id).send().await {
        Ok(response) => {
            println!("Balance: {}", response.balance);
            println!("Allowance: {}", response.allowance);
        }
        Err(e) => {
            eprintln!("Error fetching balance: {}", e);
            return Err(e.into());
        }
    }

    println!("\n=== Example Complete ===");
    Ok(())
}
