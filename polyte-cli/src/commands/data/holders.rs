use clap::Args;
use color_eyre::eyre::Result;
use polyte_data::DataApi;

/// Get top holders for markets
#[derive(Args)]
pub struct HoldersCommand {
    /// Market condition IDs (comma-separated, required)
    #[arg(short, long)]
    market: String,
    /// Maximum number of holders per market between 0 and 500
    #[arg(short, long, default_value = "100")]
    limit: u32,
    /// Minimum balance filter between 0 and 999999
    #[arg(long, default_value = "1")]
    min_balance: u32,
}

impl HoldersCommand {
    pub async fn run(self, data: &DataApi) -> Result<()> {
        let ids: Vec<&str> = self.market.split(',').map(|s| s.trim()).collect();
        let request = data
            .holders()
            .list(ids)
            .limit(self.limit)
            .min_balance(self.min_balance);

        let holders = request.send().await?;
        println!("{}", serde_json::to_string_pretty(&holders)?);
        Ok(())
    }
}
