use crate::cnpg;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct OperatorLifecycleImpl {}

#[tonic::async_trait]
impl cnpg::operator_lifecycle_server::OperatorLifecycle for OperatorLifecycleImpl {
    /// GetCapabilities gets the capabilities of the Lifecycle service
    async fn get_capabilities(
        &self,
        _request: Request<cnpg::OperatorLifecycleCapabilitiesRequest>,
    ) -> std::result::Result<
        Response<cnpg::OperatorLifecycleCapabilitiesResponse>,
        Status,
    > {
        return Ok(Response::new(
            cnpg::OperatorLifecycleCapabilitiesResponse {
                lifecycle_capabilities: vec![
                    cnpg::OperatorLifecycleCapabilities {
                        group: "".to_string(),
                        kind: "Pod".to_string(),
                        operation_types: vec![
                            cnpg::OperatorOperationType {
                                r#type: cnpg::operator_operation_type::Type::Create.into(),
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
        _request: Request<cnpg::OperatorLifecycleRequest>,
    ) -> std::result::Result<Response<cnpg::OperatorLifecycleResponse>, Status> {
        return Ok(
            Response::new(
                cnpg::OperatorLifecycleResponse{
                    json_patch: vec![],
                }
            )
        );
    }
}
