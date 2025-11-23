use polyte_gamma::Gamma;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing subscriber to see logs
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let gamma = Gamma::builder().build()?;

    println!("=== Polymarket Gamma Series Example ===\n");

    // Example 1: List all series
    println!("1. Listing all series...");
    let series_list = gamma.series().list().send().await?;
    println!("   Found {} series", series_list.len());
    println!();

    // Example 2: Display first 5 series with details
    println!("2. First 5 series:");
    for series in series_list.iter().take(5) {
        println!("   - {} (slug: {})", series.title, series.slug);
        println!("     ID: {}", series.id);
        println!("     Active: {}", series.active);
        println!("     Closed: {}", series.closed);
        println!("     Volume: {:?}", series.volume);
        println!("     Liquidity: {:?}", series.liquidity);
        if let Some(desc) = &series.description {
            println!("     Description: {}", desc);
        }
        if !series.tags.is_empty() {
            println!("     Tags: {}", series.tags.join(", "));
        }
        println!();
    }

    // Example 3: List closed series
    println!("4. Listing closed series...");
    let closed_series = gamma.series().list().closed(true).send().await?;
    println!("   Found {} closed series", closed_series.len());
    for series in closed_series.iter().take(5) {
        println!("   - {}", series.title);
    }
    println!();

    // Example 4: Get a specific series by ID
    if let Some(first_series) = series_list.first() {
        println!("5. Getting series by ID: {}", first_series.id);
        match gamma.series().get(&first_series.id).send().await {
            Ok(series) => {
                println!("   Title: {}", series.title);
                println!("   Slug: {}", series.slug);
                println!("   Active: {}", series.active);
                println!("   Closed: {}", series.closed);
                println!("   Volume: {:?}", series.volume);
                println!("   Liquidity: {:?}", series.liquidity);

                if let Some(desc) = &series.description {
                    println!("   Description: {}", desc);
                }

                if !series.tags.is_empty() {
                    println!("   Tags: {}", series.tags.join(", "));
                }

                // Display events in this series
                if !series.events.is_empty() {
                    println!("   Events: {} total", series.events.len());
                    for event in series.events.iter().take(3) {
                        println!("     - {} ({})", event.title, event.slug);
                        println!("       Active: {}", event.active,);
                    }
                } else {
                    println!("   No events in this series");
                }
            }
            Err(e) => {
                eprintln!("   Error: {}", e);
            }
        }
        println!();
    }

    // Example 5: Series statistics
    println!("6. Series statistics:");
    let closed_count = series_list.iter().filter(|s| s.closed).count();

    let total_volume: f64 = series_list.iter().filter_map(|s| s.volume).sum();

    let total_liquidity: f64 = series_list.iter().filter_map(|s| s.liquidity).sum();

    println!("   Total series: {}", series_list.len());
    println!("   Closed: {}", closed_count);
    println!("   Total volume: ${:.2}", total_volume);
    println!("   Total liquidity: ${:.2}", total_liquidity);
    println!();

    // Example 6: Find series by tags
    if !series_list.is_empty() {
        println!("8. Series by popular tags:");

        // Collect all unique tags
        let mut tag_counts = std::collections::HashMap::new();
        for series in &series_list {
            for tag in &series.tags {
                *tag_counts.entry(tag.clone()).or_insert(0) += 1;
            }
        }

        // Sort tags by frequency
        let mut tags: Vec<_> = tag_counts.into_iter().collect();
        tags.sort_by(|a, b| b.1.cmp(&a.1));

        println!("   Most common tags:");
        for (tag, count) in tags.iter().take(5) {
            println!("     - {}: {} series", tag, count);
        }
    }

    println!("\n=== Example Complete ===");
    Ok(())
}
