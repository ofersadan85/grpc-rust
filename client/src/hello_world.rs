use common::{
    pb::hello_world::{HelloRequest, greeter_client::GreeterClient},
    prelude::Result,
};
use tonic::{Request, transport::Channel};
use tracing::{error, info};

pub async fn run_hello_world(mut client: GreeterClient<Channel>) -> Result<()> {
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
