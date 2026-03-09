use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ICPStatResult {
    pub label: String,
    pub save_pcd_path: String,
    pub rmse: f32,
    pub transform: Vec<Vec<f32>>,
}

pub fn save_all_icp_stats(icp_stat_results: &[ICPStatResult], save_path: &str) -> Result<()> {
    let contents = serde_json::to_string_pretty(&icp_stat_results)
        .context("Failed to serialize icp stats to json")?;
    std::fs::write(save_path, contents).context("Failed to write icp stats to json file")?;

    Ok(())
}
