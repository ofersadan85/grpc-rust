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

async fn watch_service_health(mut client: HealthClient<Channel>, service: String) -> Result<()> {
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

pub async fn watch_all_services(client: HealthClient<Channel>, services: &[String]) {
    join_all(
        services
            .iter()
            .map(|service| watch_service_health(client.clone(), service.clone())),
    )
    .await;
}

pub async fn run_health_checks_once(channel: Channel, services: &[String]) -> Result<()> {
    let mut client = HealthClient::new(channel.clone());
    for service in services {
        let request = Request::new(HealthCheckRequest {
            service: service.clone(),
        });
        let response = client.check(request).await?;
        let status = response.get_ref().status();
        let service_type = if service.is_empty() {
            "SERVER"
        } else {
            "SERVICE"
        };
        match status {
            ServingStatus::Serving => info!("{service_type} HEALTHY {service}"),
            ServingStatus::NotServing => error!("{service_type} UNHEALTHY {service}"),
            ServingStatus::Unknown => warn!("{service_type} STATUS UNKNOWN {service}"),
            ServingStatus::ServiceUnknown => error!("{service_type} NOT FOUND {service}"),
        }
    }
    Ok(())
}
