use common::pb::hello_world::{HelloReply, HelloRequest, greeter_server::Greeter};
use tonic::{Request, Response, Status};

use crate::{TonicResponse, middleware::add_request_log};

#[derive(Debug)]
pub struct MyGreeter;

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(&self, request: Request<HelloRequest>) -> TonicResponse<HelloReply> {
        add_request_log(&request);
        let name = request.into_inner().name;
        if name == "Error" {
            return Err(Status::unauthenticated("Unauthenticated user"));
        }
        let reply = HelloReply {
            message: format!("Hello {name}!"),
        };
        Ok(Response::new(reply))
    }
}
