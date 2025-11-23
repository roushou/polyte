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

    println!("=== Polymarket Gamma Tags Example ===\n");

    // Example 1: List all tags
    println!("1. Listing all tags...");
    let tags = gamma.tags().list().send().await?;
    println!("   Found {} tags", tags.len());
    println!();

    // Example 2: List with pagination and ordering
    println!("2. List tags with limit and ordering:");
    let tags_limited = gamma
        .tags()
        .list()
        .limit(10)
        .order("label")
        .ascending(true)
        .send()
        .await?;

    println!("   Found {} tags (limited to 10):", tags_limited.len());
    for tag in &tags_limited {
        println!("   - {} (slug: {}, id: {})", tag.label, tag.slug, tag.id);
    }
    println!();

    // Example 3: Filter carousel tags
    println!("3. List carousel tags:");
    let carousel_tags = gamma
        .tags()
        .list()
        .is_carousel(true)
        .limit(5)
        .send()
        .await?;

    println!("   Found {} carousel tags:", carousel_tags.len());
    for tag in &carousel_tags {
        println!("   - {}", tag.label);
    }
    println!();

    // Example 4: Get a specific tag by ID (if we have one)
    if let Some(first_tag) = tags.first() {
        println!("4. Getting tag by ID: {}", first_tag.id);
        match gamma.tags().get(&first_tag.id).send().await {
            Ok(tag) => {
                println!("   Label: {}", tag.label);
                println!("   Slug: {}", tag.slug);
                println!("   ID: {}", tag.id);
            }
            Err(e) => {
                eprintln!("   Error: {}", e);
            }
        }
        println!();
    }

    // Example 5: Get a tag by slug (if we have one)
    if let Some(first_tag) = tags.first() {
        println!("5. Getting tag by slug: {}", first_tag.slug);
        match gamma.tags().get_by_slug(&first_tag.slug).send().await {
            Ok(tag) => {
                println!("   Label: {}", tag.label);
                println!("   Slug: {}", tag.slug);
                println!("   ID: {}", tag.id);
            }
            Err(e) => {
                eprintln!("   Error: {}", e);
            }
        }
        println!();
    }

    // Example 6: Get related tags (if we have a tag)
    if let Some(first_tag) = tags.first() {
        println!("6. Getting related tags for: {}", first_tag.label);
        match gamma.tags().get_related(&first_tag.id).send().await {
            Ok(related_tags) => {
                println!("   Found {} related tags", related_tags.len());
                for related in related_tags.iter().take(5) {
                    println!("     - {}", related.label);
                }
            }
            Err(e) => {
                eprintln!("   Error: {}", e);
            }
        }
        println!();
    }

    // Example 7: Get related tags by slug (if we have a tag)
    if let Some(first_tag) = tags.first() {
        println!("7. Getting related tags by slug for: {}", first_tag.slug);
        match gamma
            .tags()
            .get_related_by_slug(&first_tag.slug)
            .send()
            .await
        {
            Ok(related_tags) => {
                println!("   Found {} related tags", related_tags.len());
                for related in related_tags.iter().take(5) {
                    println!("     - {} (slug: {})", related.label, related.slug);
                }
            }
            Err(e) => {
                eprintln!("   Error: {}", e);
            }
        }
        println!();
    }

    // Example 8: Display tag categories (grouping similar tags)
    println!("8. Tag statistics:");
    println!("   Total tags: {}", tags.len());

    // Show some interesting tags if they exist
    let politics_tags: Vec<_> = tags
        .iter()
        .filter(|t| t.label.to_lowercase().contains("politics") || t.slug.contains("politics"))
        .collect();

    let sports_tags: Vec<_> = tags
        .iter()
        .filter(|t| t.label.to_lowercase().contains("sport") || t.slug.contains("sport"))
        .collect();

    let crypto_tags: Vec<_> = tags
        .iter()
        .filter(|t| t.label.to_lowercase().contains("crypto") || t.slug.contains("crypto"))
        .collect();

    println!("   Politics-related tags: {}", politics_tags.len());
    println!("   Sports-related tags: {}", sports_tags.len());
    println!("   Crypto-related tags: {}", crypto_tags.len());

    println!("\n=== Example Complete ===");
    Ok(())
}
