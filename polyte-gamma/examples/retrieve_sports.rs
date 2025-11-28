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

    println!("=== Polymarket Sports Example ===\n");

    // Get all sports metadata
    println!("Fetching sports metadata...");
    match gamma.sports().list().send().await {
        Ok(sports) => {
            println!("Found {} sports\n", sports.len());

            for sport in sports {
                println!("Sport: {}", sport.sport);
                println!("  ID: {}", sport.id);
                if let Some(image) = &sport.image {
                    println!("  Image: {}", image);
                }
                if let Some(resolution) = &sport.resolution {
                    println!("  Resolution: {}", resolution);
                }
                if let Some(ordering) = &sport.ordering {
                    println!("  Ordering: {}", ordering);
                }
                if let Some(series) = &sport.series {
                    println!("  Series: {}", series);
                }
                println!();
            }
        }
        Err(e) => {
            eprintln!("Error fetching sports: {}", e);
            return Err(e.into());
        }
    }

    // Example 1: List all teams
    println!("\n=== Team Listing Examples ===\n");
    println!("1. Listing all teams (limited to 10)...");
    match gamma.sports().list_teams().limit(10).send().await {
        Ok(teams) => {
            println!("   Found {} teams\n", teams.len());
            for team in teams.iter().take(5) {
                println!("   Team: {}", team.name.as_deref().unwrap_or("N/A"));
                println!("     ID: {}", team.id);
                if let Some(league) = &team.league {
                    println!("     League: {}", league);
                }
                if let Some(abbreviation) = &team.abbreviation {
                    println!("     Abbreviation: {}", abbreviation);
                }
                if let Some(record) = &team.record {
                    println!("     Record: {}", record);
                }
                println!();
            }
        }
        Err(e) => {
            eprintln!("   Error fetching teams: {}", e);
        }
    }

    // Example 2: Filter teams by league
    println!("2. Filtering teams by league (NFL)...");
    match gamma
        .sports()
        .list_teams()
        .league(vec!["NFL"])
        .limit(10)
        .send()
        .await
    {
        Ok(teams) => {
            println!("   Found {} NFL teams\n", teams.len());
            for team in teams.iter().take(5) {
                println!(
                    "   - {} ({})",
                    team.name.as_deref().unwrap_or("N/A"),
                    team.abbreviation.as_deref().unwrap_or("N/A")
                );
            }
            println!();
        }
        Err(e) => {
            eprintln!("   Error: {}", e);
        }
    }

    // Example 3: Filter by team name
    println!("3. Searching for specific team names...");
    match gamma
        .sports()
        .list_teams()
        .name(vec!["Lakers", "Warriors"])
        .send()
        .await
    {
        Ok(teams) => {
            println!("   Found {} teams\n", teams.len());
            for team in &teams {
                println!("   - {}", team.name.as_deref().unwrap_or("N/A"));
                if let Some(league) = &team.league {
                    println!("     League: {}", league);
                }
            }
            println!();
        }
        Err(e) => {
            eprintln!("   Error: {}", e);
        }
    }

    // Example 4: Filter by abbreviation
    println!("4. Filtering by team abbreviations...");
    match gamma
        .sports()
        .list_teams()
        .abbreviation(vec!["LAL", "GSW"])
        .send()
        .await
    {
        Ok(teams) => {
            println!("   Found {} teams\n", teams.len());
            for team in &teams {
                println!(
                    "   - {} ({})",
                    team.name.as_deref().unwrap_or("N/A"),
                    team.abbreviation.as_deref().unwrap_or("N/A")
                );
            }
            println!();
        }
        Err(e) => {
            eprintln!("   Error: {}", e);
        }
    }

    // Example 5: Ordering and pagination
    println!("5. Ordering teams with pagination...");
    match gamma
        .sports()
        .list_teams()
        .order("name")
        .ascending(true)
        .limit(5)
        .offset(0)
        .send()
        .await
    {
        Ok(teams) => {
            println!("   First 5 teams (ordered by name):\n");
            for team in &teams {
                println!("   - {}", team.name.as_deref().unwrap_or("N/A"));
            }
            println!();
        }
        Err(e) => {
            eprintln!("   Error: {}", e);
        }
    }

    // Example 6: Multiple leagues
    println!("6. Filtering by multiple leagues...");
    match gamma
        .sports()
        .list_teams()
        .league(vec!["NFL", "NBA"])
        .limit(20)
        .send()
        .await
    {
        Ok(teams) => {
            println!("   Found {} teams across NFL and NBA\n", teams.len());
            for team in teams.iter().take(5) {
                println!(
                    "   - {} ({})",
                    team.name.as_deref().unwrap_or("N/A"),
                    team.league.as_deref().unwrap_or("N/A")
                );
            }
            println!();
        }
        Err(e) => {
            eprintln!("   Error: {}", e);
        }
    }

    println!("=== Example Complete ===");
    Ok(())
}
