use crate::cnpg;
use std::collections::HashMap;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct IdentityImpl {}

#[tonic::async_trait]
impl cnpg::identity_server::Identity for IdentityImpl {
    /// GetPluginMetadata gets the plugin metadata
    async fn get_plugin_metadata(
        &self,
        _request: Request<cnpg::GetPluginMetadataRequest>,
    ) -> Result<Response<cnpg::GetPluginMetadataResponse>, Status> {
        Ok(Response::new(cnpg::GetPluginMetadataResponse {
            name: "plugin-generic-exporter.leonardoce.io".to_string(),
            version: "0.0.1".to_string(),
            display_name: "Generic SQL Exporter plugin".to_string(),
            description: "Add the generic SQL exporter sidecar to CNPG instances".to_string(),
            project_url: "https://github.com/leonardoce/plugin-generic-exporter".to_string(),
            repository_url: "https://github.com/leonardoce/plugin-generic-exporter".to_string(),
            license: "Apache 2".to_string(),
            license_url: "https://github.com/leonardoce/plugin-generic-exporter/blob/main/LICENSE"
                .to_string(),
            maturity: "alpha".to_string(),
            vendor: "Leonardo Cecchi".to_string(),
            manifest: HashMap::new(),
        }))
    }

    /// GetPluginCapabilities gets information about this plugin
    async fn get_plugin_capabilities(
        &self,
        _request: Request<cnpg::GetPluginCapabilitiesRequest>,
    ) -> Result<Response<cnpg::GetPluginCapabilitiesResponse>, Status> {
        Ok(Response::new(cnpg::GetPluginCapabilitiesResponse {
            capabilities: vec![cnpg::PluginCapability {
                r#type: Some(cnpg::plugin_capability::Type::Service(
                    cnpg::plugin_capability::Service {
                        r#type: cnpg::plugin_capability::service::Type::LifecycleService.into(),
                    },
                )),
            }],
        }))
    }

    /// Probe is used to tell if the plugin is ready to receive requests
    async fn probe(
        &self,
        _request: Request<cnpg::ProbeRequest>,
    ) -> Result<Response<cnpg::ProbeResponse>, Status> {
        Ok(Response::new(cnpg::ProbeResponse { ready: true }))
    }
}
