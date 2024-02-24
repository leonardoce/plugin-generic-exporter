use crate::identity::identity_server::Identity;
use std::collections::HashMap;
use tonic::{Request, Response, Status};

tonic::include_proto!("cnpgi.identity.v1");

#[derive(Debug, Default)]
pub struct IdentityImpl {}

#[tonic::async_trait]
impl Identity for IdentityImpl {
    /// GetPluginMetadata gets the plugin metadata
    async fn get_plugin_metadata(
        &self,
        _request: Request<GetPluginMetadataRequest>,
    ) -> Result<Response<GetPluginMetadataResponse>, Status> {
        Ok(Response::new(GetPluginMetadataResponse {
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
        _request: Request<GetPluginCapabilitiesRequest>,
    ) -> Result<Response<GetPluginCapabilitiesResponse>, Status> {
        Ok(Response::new(GetPluginCapabilitiesResponse {
            capabilities: vec![PluginCapability {
                r#type: Some(plugin_capability::Type::Service(
                    plugin_capability::Service {
                        r#type: plugin_capability::service::Type::LifecycleService.into(),
                    },
                )),
            }],
        }))
    }

    /// Probe is used to tell if the plugin is ready to receive requests
    async fn probe(
        &self,
        _request: Request<ProbeRequest>,
    ) -> Result<Response<ProbeResponse>, Status> {
        Ok(Response::new(ProbeResponse { ready: true }))
    }
}
