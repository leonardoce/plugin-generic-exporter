use crate::{
    cnpg::{self},
    helper::DataLoader,
};
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct OperatorImpl {}

#[tonic::async_trait]
impl cnpg::operator_server::Operator for OperatorImpl {
    /// GetCapabilities gets the capabilities of the Lifecycle service

    async fn get_capabilities(
        &self,
        _request: Request<cnpg::OperatorCapabilitiesRequest>,
    ) -> Result<Response<cnpg::OperatorCapabilitiesResult>, Status> {
        Ok(Response::new(cnpg::OperatorCapabilitiesResult {
            capabilities: vec![
                cnpg::OperatorCapability {
                    r#type: Some(cnpg::operator_capability::Type::Rpc(
                        cnpg::operator_capability::Rpc {
                            r#type: cnpg::operator_capability::rpc::Type::ValidateClusterCreate
                                .into(),
                        },
                    )),
                },
                cnpg::OperatorCapability {
                    r#type: Some(cnpg::operator_capability::Type::Rpc(
                        cnpg::operator_capability::Rpc {
                            r#type: cnpg::operator_capability::rpc::Type::ValidateClusterChange
                                .into(),
                        },
                    )),
                },
                cnpg::OperatorCapability {
                    r#type: Some(cnpg::operator_capability::Type::Rpc(
                        cnpg::operator_capability::Rpc {
                            r#type: cnpg::operator_capability::rpc::Type::MutateCluster.into(),
                        },
                    )),
                },
            ],
        }))
    }

    /// ValidateCreate improves the behaviour of the validating webhook that
    /// is called on creation of the Cluster resources
    async fn validate_cluster_create(
        &self,
        request: Request<cnpg::OperatorValidateClusterCreateRequest>,
    ) -> Result<Response<cnpg::OperatorValidateClusterCreateResult>, Status> {
        let loader = crate::helper::DataLoader::from_cluster(
            crate::consts::PLUGIN_NAME,
            &request.get_ref().definition,
        )
        .map_err(|err| {
            Status::invalid_argument(format!("Error while parsing cluster definition: {}", err))
        })?;

        Ok(Response::new(cnpg::OperatorValidateClusterCreateResult {
            validation_errors: validate(&loader),
        }))
    }

    /// ValidateClusterChange improves the behavior of the validating webhook of
    /// is called on updates of the Cluster resources
    async fn validate_cluster_change(
        &self,
        request: Request<cnpg::OperatorValidateClusterChangeRequest>,
    ) -> std::result::Result<Response<cnpg::OperatorValidateClusterChangeResult>, Status> {
        let loader = crate::helper::DataLoader::from_cluster(
            crate::consts::PLUGIN_NAME,
            &request.get_ref().new_cluster,
        )
        .map_err(|err| {
            Status::invalid_argument(format!("Error while parsing cluster definition: {}", err))
        })?;

        Ok(Response::new(cnpg::OperatorValidateClusterChangeResult {
            validation_errors: validate(&loader),
        }))
    }

    /// MutateCluster fills in the defaults inside a Cluster resource
    async fn mutate_cluster(
        &self,
        request: Request<cnpg::OperatorMutateClusterRequest>,
    ) -> Result<Response<cnpg::OperatorMutateClusterResult>, Status> {
        let loader = crate::helper::DataLoader::from_cluster(
            crate::consts::PLUGIN_NAME,
            &request.get_ref().definition,
        )
        .map_err(|err| {
            Status::invalid_argument(format!("Error while parsing cluster definition: {}", err))
        })?;

        let mut new_parameters = loader.copy_parameters();
        new_parameters
            .entry("imageName".to_string())
            .or_insert(crate::consts::IMAGE_NAME_PARAMETER_DEFAULT.to_string());

        
        let patch_value = loader
            .calculate_cluster_patch(&new_parameters)
            .map_err(|e| {
                Status::internal(format!("Error while calculating JSON patch: {}", e))
            })?;
        let serialized_patch:String = serde_json::to_string(&patch_value)
            .map_err(|e| Status::internal(format!("While serializing patch: {}", e)))?;

        Ok(Response::new(cnpg::OperatorMutateClusterResult {
            json_patch: serialized_patch.into_bytes(),
        }))
    }
}

fn validate(loader: &DataLoader) -> Vec<cnpg::ValidationError> {
    let mut res: Vec<cnpg::ValidationError> = Default::default();

    if loader
        .get_parameter(crate::consts::CONFIG_MAP_PARAMETER_NAME)
        .is_none()
    {
        res.push(loader.create_validation_error(
            crate::consts::CONFIG_MAP_PARAMETER_NAME,
            "this parameter is required",
        ))
    }

    res
}
