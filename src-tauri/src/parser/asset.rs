use std::path::PathBuf;

use super::Parser;
use anyhow::{anyhow, Context};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Asset {
    pub name: String,
    pub description: String,
    pub guid: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub rarity: String,
    pub id: u16,
}

impl Parser<Asset> for Asset {
    fn parse<P: Into<std::path::PathBuf>>(path: P) -> anyhow::Result<Asset> {
        let path: PathBuf = path.into();
        let mut item = Asset::default();

        let mut data_path = path.clone();
        let mut language_path = path.clone();

        let stem = data_path
            .file_stem()
            .ok_or_else(|| anyhow!("Failed to get file stem"))?;
        let stem = stem
            .to_str()
            .ok_or_else(|| anyhow!("Failed to convert into a &str"))?;
        data_path.push(format!("{}.dat", stem));

        language_path.push("English.dat");

        let data_content = std::fs::read_to_string(&data_path)
            .with_context(|| format!("Failed to read contents from {}", data_path.display()))?;
        let language_content = std::fs::read_to_string(&language_path).with_context(|| {
            format!(
                "Failed to read contents for file: {}",
                language_path.display()
            )
        })?;

        let data_lines = data_content.lines();
        let language_lines = language_content.lines();

        for line in data_lines {
            let mut split = line.split_whitespace();

            let field = split.next().unwrap_or("");
            let value = split.next().unwrap_or("");

            match field {
                "GUID" => item.guid = value.into(),
                "Type" => item.item_type = value.into(),
                "Rarity" => item.rarity = value.into(),
                "ID" => item.id = value.parse().context("Failed to parse ID to a u16")?,
                _ => {}
            }
        }

        for line in language_lines {
            let mut split = line.split_whitespace();

            let field = split.next().unwrap_or("");
            let remainder = split.remainder().unwrap_or("");

            match field {
                "Name" => item.name = remainder.into(),
                "Description" => item.description = remainder.into(),
                _ => {}
            }
        }

        Ok(item)
    }
}
