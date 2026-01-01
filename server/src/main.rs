use clap::Parser;
use common::{
    Cli,
    pb::{
        hello_world::greeter_server::GreeterServer,
        route_guide::route_guide_server::RouteGuideServer,
    },
    prelude::{Result, prelude},
};
use std::{net::SocketAddr, sync::Arc};
use tonic::{Response, transport::Server};
use tonic_health::server::health_reporter;
use tower::ServiceBuilder;
use tracing::{error, info, trace_span};

mod middleware;
use middleware::LoggingLayer;
mod hello_world;
use hello_world::MyGreeter;
mod route_guide;
use route_guide::RouteGuideService;
mod data;

pub type TonicResponse<T> = tonic::Result<Response<T>>;

#[tokio::main]
async fn main() -> Result<()> {
    prelude()?;
    let main_span = trace_span!("main");
    let _enter = main_span.enter();
    let args = Cli::parse();
    let address = SocketAddr::new(args.host, args.port);
    let middleware = ServiceBuilder::new().layer(LoggingLayer).into_inner();
    let (health_reporter, health_service) = health_reporter();
    let route_guide = RouteGuideService {
        features: Arc::new(data::load()?),
    };

    health_reporter
        .set_serving::<GreeterServer<MyGreeter>>()
        .await;
    health_reporter
        .set_serving::<RouteGuideServer<RouteGuideService>>()
        .await;

    let server = Server::builder()
        .layer(middleware)
        .add_service(GreeterServer::new(MyGreeter))
        .add_service(RouteGuideServer::new(route_guide))
        .add_service(health_service);
    info!("Server listening on {address}");
    tokio::select! {
        ctrl_c = tokio::signal::ctrl_c() => match ctrl_c {
            Ok(()) => info!("Shutdown signal received. Goodbye!"),
            Err(ref e) => {
                error!("Could not register Ctrl-C signal handler: {e}");
                ctrl_c?;
            },
        },
        server = server.serve(address) => match server {
            Ok(()) => info!("Server shutting down. Goodbye!"),
            Err(ref e) => {
                error!("{e}");
                server?;
            },
        }
    }
    Ok(())
}
