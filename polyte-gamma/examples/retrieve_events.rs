use polyte_gamma::Gamma;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gamma = Gamma::new()?;

    println!("=== Polymarket Gamma Events Example ===\n");

    // Example 1: List all events
    println!("1. Listing all events...");
    let events = gamma.events().list().send().await?;
    println!("   Found {} events", events.len());
    println!();

    // Example 2: List active events with limit
    println!("2. List active events (limited to 10):");
    let active_events = gamma.events().list().active(true).limit(10).send().await?;

    println!("   Found {} active events:", active_events.len());
    for event in active_events.iter().take(5) {
        println!("   - {}", event.title);
        println!("     Markets: {}", event.markets.len());
        println!("     Volume: {:?}", event.volume);
    }
    println!();

    // Example 3: Filter featured events
    println!("3. List featured events:");
    let featured = gamma
        .events()
        .list()
        .featured(true)
        .active(true)
        .limit(5)
        .send()
        .await?;

    println!("   Found {} featured events:", featured.len());
    for event in &featured {
        println!("   - {} ({})", event.title, event.slug);
    }
    println!();

    // Example 4: Filter by liquidity and volume
    println!("4. Filter by liquidity and volume:");
    let high_value = gamma
        .events()
        .list()
        .liquidity_min(1000.0)
        .volume_min(5000.0)
        .active(true)
        .limit(10)
        .send()
        .await?;

    println!("   Found {} high-value events:", high_value.len());
    for event in high_value.iter().take(5) {
        println!("   - {}", event.title);
        println!(
            "     Liquidity: {:?}, Volume: {:?}",
            event.liquidity, event.volume
        );
    }
    println!();

    // Example 5: Order by volume
    println!("5. Top events by volume:");
    let top_volume = gamma
        .events()
        .list()
        .active(true)
        .order("volume")
        .ascending(false)
        .limit(5)
        .send()
        .await?;

    println!("   Top 5 events by volume:");
    for (i, event) in top_volume.iter().enumerate() {
        let volume_str = event
            .volume
            .map(|v| v.to_string())
            .unwrap_or_else(|| "N/A".to_string());
        println!("   {}. {} - Volume: {}", i + 1, event.title, volume_str);
    }
    println!();

    // Example 6: Filter by date range
    println!("6. Filter by end date range:");
    let dated_events = gamma
        .events()
        .list()
        .end_date_min("2025-01-01T00:00:00Z")
        .end_date_max("2025-12-31T23:59:59Z")
        .active(true)
        .limit(10)
        .send()
        .await?;

    println!("   Found {} events ending in 2025", dated_events.len());
    println!();

    // Example 7: Filter by tag
    println!("7. Filter by tag:");
    let tagged_events = gamma
        .events()
        .list()
        .tag_id(1)
        .related_tags(true)
        .limit(10)
        .send()
        .await?;

    println!("   Found {} events with tag", tagged_events.len());
    println!();

    // Example 8: Exclude certain tags
    println!("8. Exclude specific tags:");
    let filtered_events = gamma
        .events()
        .list()
        .exclude_tag_id(vec![1, 2, 3])
        .active(true)
        .limit(10)
        .send()
        .await?;

    println!("   Found {} events excluding tags", filtered_events.len());
    println!();

    // Example 9: Get specific event by ID
    if let Some(first_event) = events.first() {
        println!("9. Getting event by ID: {}", first_event.id);
        match gamma.events().get(&first_event.id).send().await {
            Ok(event) => {
                println!("   Title: {}", event.title);
                println!("   Slug: {}", event.slug);
                println!("   Active: {}", event.active);
                println!("   Markets: {}", event.markets.len());
                println!("   Volume: {:?}", event.volume);
                println!("   Liquidity: {:?}", event.liquidity);
            }
            Err(e) => {
                eprintln!("   Error: {}", e);
            }
        }
        println!();
    }

    // Example 10: Get event by slug
    if let Some(first_event) = events.first() {
        println!("10. Getting event by slug: {}", first_event.slug);
        match gamma.events().get_by_slug(&first_event.slug).send().await {
            Ok(event) => {
                println!("   Title: {}", event.title);
                println!("   Description: {:?}", event.description);
            }
            Err(e) => {
                eprintln!("   Error: {}", e);
            }
        }
        println!();
    }

    // Example 11: Complex filtering
    println!("11. Complex multi-parameter filtering:");
    let complex = gamma
        .events()
        .list()
        .active(true)
        .featured(false)
        .archived(false)
        .liquidity_min(500.0)
        .volume_min(1000.0)
        .order("liquidity,volume")
        .ascending(false)
        .limit(5)
        .send()
        .await?;

    println!(
        "   Found {} events matching complex criteria",
        complex.len()
    );
    for event in &complex {
        println!("   - {}", event.title);
        println!(
            "     Volume: {:?}, Liquidity: {:?}",
            event.volume, event.liquidity
        );
    }

    println!("\n=== Example Complete ===");
    Ok(())
}
