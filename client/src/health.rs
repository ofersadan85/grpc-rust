use std::{collections::HashMap, sync::LazyLock};

use common::prelude::Result;
use futures::future::join_all;
use tokio::sync::Mutex;
use tonic::{Request, transport::Channel};
use tonic_health::pb::{
    HealthCheckRequest, health_check_response::ServingStatus, health_client::HealthClient,
};
use tracing::{error, info, warn};

pub static SERVICE_STATUS: LazyLock<Mutex<HashMap<String, ServingStatus>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

async fn health_watch_service(mut client: HealthClient<Channel>, service: String) -> Result<()> {
    let request = Request::new(HealthCheckRequest {
        service: service.clone(),
    });
    let mut stream = client.watch(request).await?.into_inner();
    while let Some(response) = stream.message().await? {
        let status = response.status();
        let old_status = SERVICE_STATUS.lock().await.insert(service.clone(), status);
        if old_status.is_none_or(|old| old != status) {
            match status {
                ServingStatus::Serving => info!("SERVICE ONLINE [{service}]"),
                ServingStatus::NotServing => error!("SERVICE OFFLINE [{service}]"),
                ServingStatus::Unknown => warn!("SERVICE STATUS UNKNOWN [{service}]"),
                ServingStatus::ServiceUnknown => error!("SERVICE UNKNOWN [{service}]"),
            }
        }
    }
    Ok(())
}

pub async fn all_health_checks(client: HealthClient<Channel>, services: &[&str]) {
    join_all(
        services
            .iter()
            .map(|&service| health_watch_service(client.clone(), service.to_string())),
    )
    .await;
}
