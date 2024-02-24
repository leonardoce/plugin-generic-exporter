use std::path::Path;
use tokio::net::UnixListener;
use tokio_stream::wrappers::UnixListenerStream;
use tonic::transport::Server;

mod cnpg;
mod identity;
mod operator_lifecycle;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "/plugins/plugin-generic-exporter.leonardoce.io";

    std::fs::create_dir_all(Path::new(path).parent().unwrap())?;

    let uds = UnixListener::bind(path)?;
    let uds_stream = UnixListenerStream::new(uds);

    let identity_implementation = identity::IdentityImpl::default();
    let operator_lifecycle_implementation = operator_lifecycle::OperatorLifecycleImpl::default();

    Server::builder()
        .add_service(cnpg::identity_server::IdentityServer::new(
            identity_implementation,
        ))
        .add_service(cnpg::operator_lifecycle_server::OperatorLifecycleServer::new(
            operator_lifecycle_implementation
        ))
        .serve_with_incoming(uds_stream)
        .await?;

    Ok(())
}
