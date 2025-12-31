use clap::Parser;
use common::{
    Cli,
    pb::hello_world::{HelloRequest, greeter_client::GreeterClient},
    prelude::{Result, prelude},
};
use std::net::SocketAddr;
use tonic::Request;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
    prelude()?;
    let cli = Cli::parse();
    let address = SocketAddr::new(cli.host, cli.port);
    let mut client = GreeterClient::connect(format!("grpc://{address}")).await?;
    for name in ["World", "Tonic", "Rust", "Error"] {
        let request = Request::new(HelloRequest { name: name.into() });
        let response = client.say_hello(request).await;
        match response {
            Ok(response) => info!("RESPONSE = {}", response.into_inner().message),
            Err(e) => {
                error!("Error Code: {}, Message: {}", e.code(), e.message());
            }
        }
    }
    Ok(())
}
