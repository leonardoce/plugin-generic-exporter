use crate::cnpg;
use anyhow::Result;
use json_patch;
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
    parameters: HashMap<String, String>,
    plug_index: usize,
}

impl DataLoader {
    /// from_cluster create a new helper given a Cluster definition as
    /// passed by CNPG
    pub fn from_cluster(name: &str, definition: &[u8]) -> Result<DataLoader> {
        let cluster: serde_json::Value = serde_json::from_slice(definition)?;

        let plugins = cluster["spec"]["plugins"]
            .as_array()
            .ok_or(DataLoaderError::UnexpectedPluginSection)?;

        let (idx, current_plugin) = plugins
            .iter()
            .enumerate()
            .filter(|(_, x)| x["name"] == name)
            .next()
            .ok_or(DataLoaderError::PluginNotFound {
                name: name.to_string(),
            })?;

        let parameters = if current_plugin["parameters"].is_null() {
            HashMap::new()
        } else {
            current_plugin["parameters"]
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
                .collect()
        };

        Ok(DataLoader {
            cluster,
            parameters,
            plug_index: idx,
        })
    }

    /// get_parameters find the value of a configuration parameter
    pub fn get_parameter(&self, name: &str) -> Option<String> {
        self.parameters.get(name).map(|x| x.to_string())
    }

    // create_validation_error creates a validatoin error for the parameter
    // with a certain name
    pub fn create_validation_error(&self, name: &str, message: &str) -> cnpg::ValidationError {
        cnpg::ValidationError {
            path_components: vec![
                "spec".to_string(),
                "plugins".to_string(),
                format!("{}", self.plug_index),
                "parameters".to_string(),
                name.to_string(),
            ],
            value: self.get_parameter(name).unwrap_or_default(),
            message: message.to_string(),
        }
    }

    /// copy_parameters returns a copy of the Plugin parameters.
    /// This is typically used to set plugin default values and them
    /// computing the JSON difference to be returned from CNPG
    pub fn copy_parameters(&self) -> HashMap<String, String> {
        self.parameters.clone()
    }

    /// calculate_cluster_patch calculates the JSON patch difference between
    /// the cluster and a new cluster definition where the passed parameters
    /// are used
    pub fn calculate_cluster_patch(
        &self,
        new_parameters: &HashMap<String, String>,
    ) -> Result<serde_json::Value> {
        let mut new_cluster = self.cluster.clone();
        let new_parameters_json: serde_json::Value = serde_json::to_value(new_parameters)?;

        new_cluster["spec"]["plugins"][self.plug_index]["parameters"] = new_parameters_json;
        Ok(serde_json::to_value(json_patch::diff(
            &self.cluster,
            &new_cluster,
        ))?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CLUSTER_JSON: &str = r#"
{
    "apiVersion": "postgresql.cnpg.io/v1",
    "kind": "Cluster",
    "metadata": {
        "name": "cluster-example",
        "namespace": "default"
    },
    "spec": {
        "plugins": [
            {
                "name": "plugin-generic-exporter.leonardoce.io",
                "parameters": {
                    "configMapName": "sql-exporter-config"
                }
            }
        ]
    }
}"#;

    const CLUSTER_JSON_NULL_PARAMETERS: &str = r#"
{
    "apiVersion": "postgresql.cnpg.io/v1",
    "kind": "Cluster",
    "metadata": {
        "name": "cluster-example",
        "namespace": "default"
    },
    "spec": {
        "plugins": [
            {
                "name": "plugin-generic-exporter.leonardoce.io",
                "parameters": null
            }
        ]
    }
}    
"#;

    const CLUSTER_JSON_EMPTY_PARAMETERS: &str = r#"
{
    "apiVersion": "postgresql.cnpg.io/v1",
    "kind": "Cluster",
    "metadata": {
        "name": "cluster-example",
        "namespace": "default"
    },
    "spec": {
        "plugins": [
            {
                "name": "plugin-generic-exporter.leonardoce.io",
                "parameters": {}
            }
        ]
    }
}    
"#;

    #[test]
    fn test_decode() {
        let helper =
            DataLoader::from_cluster(crate::consts::PLUGIN_NAME, CLUSTER_JSON.as_bytes()).unwrap();

        assert_eq!(helper.plug_index, 0);
        assert_eq!(
            helper.get_parameter("configMapName").unwrap(),
            "sql-exporter-config"
        );
    }

    #[test]
    fn test_validation_error_unexistent_parameter() {
        let helper =
            DataLoader::from_cluster(crate::consts::PLUGIN_NAME, CLUSTER_JSON.as_bytes()).unwrap();

        let error = helper.create_validation_error("unknown_parameter", "test error message");

        assert_eq!(error.message, "test error message")
    }

    #[test]
    fn test_validation_error_existing_parameter() {
        let helper =
            DataLoader::from_cluster(crate::consts::PLUGIN_NAME, CLUSTER_JSON.as_bytes()).unwrap();

        let error = helper.create_validation_error("configMapName", "this configmap is weird");

        assert_eq!(error.message, "this configmap is weird");
        assert_eq!(error.value, "sql-exporter-config");
    }

    #[test]
    fn test_cluster_patch() {
        let helper =
            DataLoader::from_cluster(crate::consts::PLUGIN_NAME, CLUSTER_JSON.as_bytes()).unwrap();

        let mut new_params = helper.copy_parameters();
        new_params
            .entry("imageName".to_string())
            .or_insert("thisImage".to_string());
        new_params
            .entry("imagePullPolicy".to_string())
            .or_insert("Always".to_string());

        let patch = helper
            .calculate_cluster_patch(&new_params)
            .expect("error while calculating patch");
        assert_eq!(patch.as_array().expect("JSON patches are arrays").len(), 2);
    }

    #[test]
    fn test_decode_null_parameters() {
        let helper = DataLoader::from_cluster(
            crate::consts::PLUGIN_NAME,
            CLUSTER_JSON_NULL_PARAMETERS.as_bytes(),
        )
        .unwrap();

        assert_eq!(helper.parameters.len(), 0);
    }

    #[test]
    fn test_decode_empty_parameters() {
        let helper = DataLoader::from_cluster(
            crate::consts::PLUGIN_NAME,
            CLUSTER_JSON_EMPTY_PARAMETERS.as_bytes(),
        )
        .unwrap();

        assert_eq!(helper.parameters.len(), 0);
    }
}
