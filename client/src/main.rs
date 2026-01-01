use clap::Parser;
use common::{
    Cli,
    pb::{
        hello_world::greeter_client::GreeterClient,
        route_guide::route_guide_client::RouteGuideClient,
    },
    prelude::{Result, prelude},
};
use std::net::SocketAddr;
use tonic::transport::Channel;
use tonic_health::pb::health_client::HealthClient;
use tracing::{info, warn};

mod hello_world;
use hello_world::run_hello_world;
mod route_guide;
use route_guide::{get_features, print_features, run_record_route, run_route_chat};
mod health;
use health::run_health_watch;

#[tokio::main]
async fn main() -> Result<()> {
    prelude()?;
    let cli = Cli::parse();
    let address = SocketAddr::new(cli.host, cli.port);
    let client_url = format!("grpc://{address}");
    info!("Connecting to server at {client_url}");

    let channel = Channel::from_shared(client_url)?.connect().await?;
    let hello_world_client = GreeterClient::new(channel.clone());
    let route_guide_client = RouteGuideClient::new(channel.clone());
    let health_client = HealthClient::new(channel);

    let results = tokio::try_join!(
        run_hello_world(hello_world_client),
        get_features(route_guide_client.clone()),
        print_features(route_guide_client.clone()),
        run_record_route(route_guide_client.clone()),
        run_route_chat(route_guide_client),
        run_health_watch(health_client.clone(), "hello_world.Greeter"),
        run_health_watch(health_client, "route_guide.RouteGuide"),
    );
    if results.is_ok() {
        info!("All client operations completed successfully!");
    } else {
        warn!("Some client operations failed: {:?}", results.err());
    }

    Ok(())
}
