use clap::Parser;
use common::{
    pb::{
        hello_world::greeter_client::GreeterClient,
        route_guide::route_guide_client::RouteGuideClient,
    },
    prelude::{Result, prelude},
};
use std::net::SocketAddr;
use tokio::{signal::ctrl_c, task::JoinSet};
use tonic::transport::Channel;
use tonic_health::pb::health_client::HealthClient;
use tracing::{error, info, warn};

mod cli;
use cli::{Cli, Commands};
mod hello_world;
use hello_world::run_hello_world;
mod route_guide;
use route_guide::{get_features, print_features, run_record_route, run_route_chat};
mod chat;
use chat::send_message;
mod health;
use health::{run_health_checks_once, watch_all_services};

async fn example_tasks(channel: Channel) -> Result<()> {
    let hello_world_client = GreeterClient::new(channel.clone());
    let route_guide_client = RouteGuideClient::new(channel);
    let mut join_set = JoinSet::new();
    join_set.spawn(run_hello_world(hello_world_client));
    join_set.spawn(get_features(route_guide_client.clone()));
    join_set.spawn(print_features(route_guide_client.clone()));
    join_set.spawn(run_record_route(route_guide_client.clone()));
    join_set.spawn(run_route_chat(route_guide_client));

    while let Some(res) = join_set.join_next().await {
        if let Err(e) = res? {
            error!("Task error: {e}");
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    prelude()?;
    let cli = Cli::parse();
    let address = SocketAddr::new(cli.host, cli.port);
    let client_url = format!("grpc://{address}");

    info!("Connecting to server at {client_url}");
    let channel = Channel::from_shared(client_url)?.connect().await?;
    match cli.command.unwrap_or_default() {
        Commands::Health(options) if options.watch => {
            tokio::select! {
                () = watch_all_services(HealthClient::new(channel), options.services.as_slice()) => warn!("Server disconnected"),
                ctrl_c = ctrl_c() => match ctrl_c {
                    Ok(()) => info!("Shutdown signal received. Goodbye!"),
                    Err(ref e) => {
                        error!("Could not register Ctrl-C signal handler: {e}");
                        ctrl_c?;
                    },
                },
            }
        }
        Commands::Health(options) => {
            run_health_checks_once(channel, options.services.as_slice()).await?;
        }
        Commands::Examples => example_tasks(channel).await?,
        Commands::Chat { message } => {
            send_message(channel, message).await?;
        }
    }

    Ok(())
}
