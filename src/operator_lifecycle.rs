use crate::cnpg;
use json_patch;
use k8s_openapi::api::core::v1 as api;
use log::debug;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct OperatorLifecycleImpl {}

#[tonic::async_trait]
impl cnpg::operator_lifecycle_server::OperatorLifecycle for OperatorLifecycleImpl {
    /// GetCapabilities gets the capabilities of the Lifecycle service
    async fn get_capabilities(
        &self,
        _request: Request<cnpg::OperatorLifecycleCapabilitiesRequest>,
    ) -> std::result::Result<Response<cnpg::OperatorLifecycleCapabilitiesResponse>, Status> {
        return Ok(Response::new(cnpg::OperatorLifecycleCapabilitiesResponse {
            lifecycle_capabilities: vec![cnpg::OperatorLifecycleCapabilities {
                group: "".to_string(),
                kind: "Pod".to_string(),
                operation_types: vec![cnpg::OperatorOperationType {
                    r#type: cnpg::operator_operation_type::Type::Create.into(),
                }],
            }],
        }));
    }

    /// LifecycleHook allows the plugin to manipulate the Kubernetes resources
    /// before the CNPG operator applies them to the Kubernetes cluster.
    async fn lifecycle_hook(
        &self,
        request: Request<cnpg::OperatorLifecycleRequest>,
    ) -> std::result::Result<Response<cnpg::OperatorLifecycleResponse>, Status> {
        // We get and parse the cluster definition
        let helper = crate::helper::DataLoader::from_cluster(
            crate::consts::PLUGIN_NAME,
            &request.get_ref().cluster_definition,
        )
        .map_err(|err| {
            Status::internal(format!(
                "While decoding cluster definition: {}",
                err.to_string()
            ))
        })?;

        // When this method is called, cloudnative-pg is creating a Pod.
        // Let's inject the generic exporter sidecar here.
        let original_pod: api::Pod = serde_json::from_slice(&request.get_ref().object_definition)
            .map_err(|err| Status::internal(err.to_string()))?;

        let mut pod: api::Pod = original_pod.clone();

        // Create a generic exporter Sidecar
        let mut generic_exporter_sidecar: api::Container = Default::default();
        generic_exporter_sidecar.name = "sql-exporter".to_string();
        generic_exporter_sidecar.image = Some(
            helper
                .get_parameter(crate::consts::IMAGE_NAME_PARAMETER_NAME)
                .unwrap_or(crate::consts::IMAGE_NAME_PARAMETER_DEFAULT.to_string())
                .to_string(),
        );
        generic_exporter_sidecar.env = Some(vec![
            api::EnvVar {
                name: "CONFIG".to_string(),
                value: Some("/config/config.yml".to_string()),
                value_from: None,
            },
            api::EnvVar {
                name: "LOGLEVEL".to_string(),
                value: Some("info".to_string()),
                value_from: None,
            },
        ]);
        generic_exporter_sidecar.volume_mounts = Some(vec![
            api::VolumeMount {
                mount_path: "/config".to_string(),
                mount_propagation: None,
                name: "sql-exporter-configuration".to_string(),
                read_only: Some(true),
                sub_path: None,
                sub_path_expr: None,
            },
            api::VolumeMount {
                mount_path: "/controller".to_string(),
                mount_propagation: None,
                name: "scratch-data".to_string(),
                read_only: None,
                sub_path: None,
                sub_path_expr: None,
            },
            api::VolumeMount {
                mount_path: "/run".to_string(),
                mount_propagation: None,
                name: "scratch-data".to_string(),
                read_only: None,
                sub_path: None,
                sub_path_expr: None,
            },
        ]);
        generic_exporter_sidecar.restart_policy = Some("Always".to_string());

        // Create a volume for the exporter configuration
        let mut exporter_configuration_volume: api::Volume = Default::default();
        exporter_configuration_volume.name = "sql-exporter-configuration".to_string();
        exporter_configuration_volume.config_map = Some(api::ConfigMapVolumeSource {
            default_mode: Some(0o644),
            items: Some(vec![api::KeyToPath {
                key: "config.yml".to_string(),
                mode: None,
                path: "config.yml".to_string(),
            }]),
            name: Some(
                helper
                    .get_parameter(crate::consts::CONFIG_MAP_PARAMETER_NAME)
                    .ok_or(Status::invalid_argument("Missing config map parameter"))?
                    .to_string(),
            ),
            optional: Some(false),
        });

        // Inject the sidecar and the configuration volume
        pod.spec
            .as_mut()
            .ok_or(Status::invalid_argument("CNPG Pod without spec?"))?
            .init_containers
            .as_mut()
            .ok_or(Status::invalid_argument("CNPG Pod without init containers?"))?
            .push(generic_exporter_sidecar);
        pod.spec
            .as_mut()
            .ok_or(Status::invalid_argument("CNPG Pod without spec?"))?
            .volumes
            .as_mut()
            .ok_or(Status::invalid_argument("CNPG Pod without volumes?"))?
            .push(exporter_configuration_volume);

        // Create the json patch
        let patch = json_patch::diff(
            &serde_json::to_value(original_pod).map_err(|e| {
                Status::internal(format!(
                    "Error while serializing CNPG pod [original]: {}",
                    e
                ))
            })?,
            &serde_json::to_value(pod).map_err(|e| {
                Status::internal(format!("Error while serializing CNPG pod [new]: {}", e))
            })?,
        );

        let serialized_patch = serde_json::to_string(&patch)
            .map_err(|e| Status::internal(format!("While serializing patch: {}", e)))?;

        debug!("Serialized patch: {}", serialized_patch);

        return Ok(Response::new(cnpg::OperatorLifecycleResponse {
            json_patch: serialized_patch.into_bytes(),
        }));
    }
}
