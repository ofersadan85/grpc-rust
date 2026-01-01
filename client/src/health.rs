use common::prelude::Result;
use tonic::{Request, transport::Channel};
use tonic_health::pb::{
    HealthCheckRequest, health_check_response::ServingStatus, health_client::HealthClient,
};
use tracing::{error, info, warn};

pub async fn run_health_watch(mut client: HealthClient<Channel>, service: &str) -> Result<()> {
    let request = Request::new(HealthCheckRequest {
        service: service.to_string(),
    });
    let mut stream = client.watch(request).await?.into_inner();
    while let Some(response) = stream.message().await? {
        match response.status() {
            ServingStatus::Serving => info!("SERVICE ONLINE [{service}]"),
            ServingStatus::NotServing => error!("SERVICE OFFLINE [{service}]"),
            ServingStatus::Unknown => warn!("SERVICE STATUS UNKNOWN [{service}]"),
            ServingStatus::ServiceUnknown => error!("SERVICE UNKNOWN [{service}]"),
        }
    }
    Ok(())
}
