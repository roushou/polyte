use polyte_gamma::Gamma;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gamma = Gamma::new()?;

    println!("=== Advanced Market Filtering Example ===\n");

    // Example 1: Filter by multiple condition IDs
    println!("1. Filter by multiple condition IDs:");
    let condition_ids = vec!["0x123...".to_string(), "0x456...".to_string()];

    let markets = gamma
        .markets()
        .list()
        .condition_ids(condition_ids)
        .limit(10)
        .send()
        .await?;

    println!("   Found {} markets", markets.len());
    println!();

    // Example 2: Filter by liquidity and volume range
    println!("2. Filter by liquidity and volume:");
    let markets = gamma
        .markets()
        .list()
        .liquidity_num_min(1000.0)
        .volume_num_min(5000.0)
        .closed(false)
        .limit(20)
        .send()
        .await?;

    println!("   Found {} high-liquidity markets", markets.len());
    for market in markets.iter().take(5) {
        println!("     - {}", market.question);
        println!("       Volume: {:?}", market.volume);
        println!("       Liquidity: {:?}", market.liquidity);
    }
    println!();

    // Example 3: Filter by multiple slugs with ordering
    println!("3. Filter with ordering:");
    let markets = gamma
        .markets()
        .list()
        .order("volume")
        .ascending(false) // Descending by volume
        .closed(false)
        .limit(10)
        .send()
        .await?;

    println!("   Top 10 markets by volume:");
    for (i, market) in markets.iter().enumerate() {
        println!(
            "     {}. {} - Volume: {:?}",
            i + 1,
            market.question,
            market.volume
        );
    }
    println!();

    // Example 4: Filter by date range
    println!("4. Filter by end date range:");
    let markets = gamma
        .markets()
        .list()
        .end_date_min("2025-01-01T00:00:00Z")
        .end_date_max("2025-12-31T23:59:59Z")
        .limit(10)
        .send()
        .await?;

    println!("   Found {} markets ending in 2025", markets.len());
    println!();

    // Example 5: Filter by tag with related tags
    println!("5. Filter by tag with pagination:");
    let markets = gamma
        .markets()
        .list()
        .tag_id(1) // Example tag ID
        .include_tag(true)
        .related_tags(true)
        .offset(0)
        .limit(20)
        .send()
        .await?;

    println!("   Found {} markets with tag", markets.len());
    println!();

    // Example 6: Multiple market makers
    println!("6. Filter by multiple market maker addresses:");
    let makers = vec!["0xabc...".to_string(), "0xdef...".to_string()];

    let markets = gamma
        .markets()
        .list()
        .market_maker_address(makers)
        .limit(10)
        .send()
        .await?;

    println!("   Found {} markets from specified makers", markets.len());
    println!();

    // Example 7: Sports market filtering
    println!("7. Filter sports markets:");
    println!("Skipping: Need to identify what sports_market_types is (nba, nfl etc are invalid)");
    // let sports_types = vec!["nfl".to_string(), "nba".to_string()];
    //
    // let markets = gamma
    //     .markets()
    //     .list()
    //     .sports_market_types(sports_types)
    //     .closed(false)
    //     .limit(15)
    //     .send()
    //     .await?;
    //
    // println!("   Found {} active sports markets", markets.len());
    // println!();

    // Example 8: Complex filtering with multiple parameters
    println!("8. Complex multi-parameter filtering:");
    let markets = gamma
        .markets()
        .list()
        .liquidity_num_min(500.0)
        .volume_num_min(1000.0)
        .closed(false)
        .archived(false)
        .rewards_min_size(10.0)
        .order("volume,liquidity")
        .ascending(false)
        .limit(10)
        .send()
        .await?;

    println!(
        "   Found {} markets matching complex criteria",
        markets.len()
    );
    for market in markets.iter().take(3) {
        println!("     - {}", market.question);
        println!("       Volume: {:?}", market.volume);
        println!("       Liquidity: {:?}", market.liquidity);
    }

    println!("\n=== Example Complete ===");
    Ok(())
}
