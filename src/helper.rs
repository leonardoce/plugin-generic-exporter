use anyhow::Result;
use serde_json;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataLoaderError {
    #[error("Plugin with specified name was not found")]
    PluginNotFound { name: String },

    #[error("Unexpected value in CNPG .spec.plugin section")]
    UnexpectedPluginSection,

    #[error("Unexpected value in .spec.plugin.[x].parameters")]
    UnexpectedPluginParameters,
}

pub struct DataLoader {
    cluster: serde_json::Value,
    name: String,
}

impl DataLoader {
    /// from_cluster create a new helper given a Cluster definition as
    /// passed by CNPG
    pub fn from_cluster(name: &str, definition: &[u8]) -> Result<DataLoader> {
        Ok(DataLoader {
            cluster: serde_json::from_slice(definition)?,
            name: name.to_string(),
        })
    }

    /// find_configuration gets the current plugin configuration
    pub fn find_configuration(&self) -> Result<HashMap<String, String>> {
        let plugins = self.cluster["spec"]["plugins"]
            .as_array()
            .ok_or(DataLoaderError::UnexpectedPluginSection)?;

        let current_plugin = plugins
            .iter()
            .filter(|x| x["name"] == self.name)
            .next()
            .ok_or(DataLoaderError::PluginNotFound {
                name: self.name.to_string(),
            })?;

        Ok(current_plugin["parameters"]
            .as_object()
            .ok_or(DataLoaderError::UnexpectedPluginParameters)?
            .iter()
            .map(|(name, value)| {
                (
                    name.to_string(),
                    value
                        .as_str()
                        .map(|x| x.to_string())
                        .unwrap_or("".to_string()),
                )
            })
            .collect())
    }
}
