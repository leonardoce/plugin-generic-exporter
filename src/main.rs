mod identity;

use std::path::Path;
use tokio::net::UnixListener;
use tokio_stream::wrappers::UnixListenerStream;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "/tmp/plugin-generic-exporter.leonardoce.io";

    std::fs::create_dir_all(Path::new(path).parent().unwrap())?;

    let uds = UnixListener::bind(path)?;
    let uds_stream = UnixListenerStream::new(uds);

    let identity_implementation = identity::IdentityImpl::default();

    Server::builder()
        .add_service(identity::identity_server::IdentityServer::new(
            identity_implementation,
        ))
        .serve_with_incoming(uds_stream)
        .await?;

    Ok(())
}
