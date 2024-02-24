use crate::identity::operator_lifecycle_server::OperatorLifecycle;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct OperatorLifecycleImpl {}

#[tonic::async_trait]
impl OperatorLifecycle for OperatorLifecycleImpl {
    /// GetCapabilities gets the capabilities of the Lifecycle service
    async fn get_capabilities(
        &self,
        _request: Request<crate::identity::OperatorLifecycleCapabilitiesRequest>,
    ) -> std::result::Result<
        Response<crate::identity::OperatorLifecycleCapabilitiesResponse>,
        Status,
    > {
        return Ok(Response::new(
            crate::identity::OperatorLifecycleCapabilitiesResponse {
                lifecycle_capabilities: vec![
                    crate::identity::OperatorLifecycleCapabilities {
                        group: "".to_string(),
                        kind: "Pod".to_string(),
                        operation_types: vec![
                            crate::identity::OperatorOperationType {
                                r#type: crate::identity::operator_operation_type::Type::Create.into(),
                            }
                        ],
                    }
                ]
            }
        ))
    }
    
    /// LifecycleHook allows the plugin to manipulate the Kubernetes resources
    /// before the CNPG operator applies them to the Kubernetes cluster.
    async fn lifecycle_hook(
        &self,
        _request: Request<crate::identity::OperatorLifecycleRequest>,
    ) -> std::result::Result<Response<crate::identity::OperatorLifecycleResponse>, Status> {
        return Ok(
            Response::new(
                crate::identity::OperatorLifecycleResponse{
                    json_patch: vec![],
                }
            )
        );
    }
}
