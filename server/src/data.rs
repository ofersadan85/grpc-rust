use common::{pb::route_guide::Feature, prelude::Result};
use std::{fs::File, path::PathBuf};
use tracing::info;

pub fn load() -> Result<Vec<Feature>> {
    let path = PathBuf::from_iter([std::env!("CARGO_MANIFEST_DIR"), "route_guide_db.json"]);
    info!("Loading route guide data from {path:?}");
    let file = File::open(path)?;
    let features: Vec<Feature> = serde_json::from_reader(file)?;
    Ok(features)
}
