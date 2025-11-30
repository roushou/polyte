use clap::Args;
use color_eyre::eyre::Result;
use polyte_data::DataApi;

#[derive(Args)]
pub struct OpenInterestCommand {
    /// Filter by market condition IDs (comma-separated, optional)
    #[arg(short, long)]
    pub market: Option<String>,
}

impl OpenInterestCommand {
    pub async fn run(self, data: &DataApi) -> Result<()> {
        let mut request = data.open_interest().get();
        if let Some(m) = self.market {
            let market_ids: Vec<&str> = m.split(',').map(|s| s.trim()).collect();
            request = request.market(market_ids);
        }
        let open_interest = request.send().await?;
        println!("{}", serde_json::to_string_pretty(&open_interest)?);
        Ok(())
    }
}
