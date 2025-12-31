use clap::Parser;
use common::{
    Cli,
    pb::hello_world::{
        greeter_server::{Greeter, GreeterServer},
        {HelloReply, HelloRequest},
    },
    prelude::{Result, prelude},
};
use std::net::SocketAddr;
use tonic::{Request, Response, Status, transport::Server};
use tracing::{Span, error, info, record_all, trace, trace_span};

mod middleware;
use middleware::LoggingLayer;

pub type TonicResponse = std::result::Result<Response<HelloReply>, Status>;

#[derive(Debug)]
pub struct MyGreeter;

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(&self, request: Request<HelloRequest>) -> TonicResponse {
        let current_span = Span::current();
        if let Some(client) = request.remote_addr() {
            record_all!(current_span, client = ?client);
        }
        trace!(message = ?request.get_ref());
        let name = request.into_inner().name;
        if name == "Error" {
            return Err(Status::internal("Simulated server internal error"));
        }
        let reply = HelloReply {
            message: format!("Hello {name}!"),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    prelude()?;
    let main_span = trace_span!("main");
    let _enter = main_span.enter();
    let args = Cli::parse();
    let address = SocketAddr::new(args.host, args.port);
    let middleware = tower::ServiceBuilder::new()
        .layer(LoggingLayer)
        .into_inner();
    let server = Server::builder()
        .layer(middleware)
        .add_service(GreeterServer::new(MyGreeter));
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
