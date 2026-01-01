use common::{
    pb::route_guide::{Point, Rectangle, RouteNote, route_guide_client::RouteGuideClient},
    prelude::Result,
};
use fastrand::Rng;
use std::time::Duration;
use tokio::time;
use tonic::{Request, transport::Channel};
use tracing::{error, info};

pub async fn get_features(mut client: RouteGuideClient<Channel>) -> Result<()> {
    let response = client
        .get_feature(Request::new(Point {
            latitude: 409_146_138,
            longitude: -746_188_906,
        }))
        .await?
        .into_inner();
    info!("RESPONSE = {:?}", response);
    Ok(())
}

pub async fn print_features(mut client: RouteGuideClient<Channel>) -> Result<()> {
    let rectangle = Rectangle {
        lo: Some(Point {
            latitude: 400_000_000,
            longitude: -750_000_000,
        }),
        hi: Some(Point {
            latitude: 420_000_000,
            longitude: -730_000_000,
        }),
    };

    let mut stream = client
        .list_features(Request::new(rectangle))
        .await?
        .into_inner();

    while let Some(feature) = stream.message().await? {
        info!("FEATURE = {:?}", feature);
    }

    Ok(())
}

fn random_point(rng: &mut Rng) -> Point {
    let latitude = (rng.i32(0..180) - 90) * 10_000_000;
    let longitude = (rng.i32(0..360) - 180) * 10_000_000;
    Point {
        latitude,
        longitude,
    }
}

pub async fn run_record_route(mut client: RouteGuideClient<Channel>) -> Result<()> {
    let mut rng = Rng::new();
    let point_count: i32 = rng.i32(2..20);
    let points: Vec<Point> = (0..=point_count).map(|_| random_point(&mut rng)).collect();

    info!("Traversing {} points", points.len());
    let stream = async_stream::stream! {
        for point in points {
            info!("Visiting point {:?}", point);
            yield point;
            let sleep_duration = Duration::from_millis(rng.u64(200..400));
            tokio::time::sleep(sleep_duration).await;
        }
    };
    let request = Request::new(stream);

    match client.record_route(request).await {
        Ok(response) => info!("SUMMARY: {:?}", response.into_inner()),
        Err(e) => error!("Something went wrong: {e:?}"),
    }

    Ok(())
}

pub async fn run_route_chat(mut client: RouteGuideClient<Channel>) -> Result<()> {
    let start = time::Instant::now();

    let outbound = async_stream::stream! {
        let mut interval = time::interval(Duration::from_secs(1));

        loop {
            let time = interval.tick().await;
            let elapsed = time.duration_since(start);
            let note = RouteNote {
                location: Some(Point {
                    latitude: 409_146_138 + i32::try_from(elapsed.as_secs()).unwrap_or(0),
                    longitude: -746_188_906,
                }),
                message: format!("at {elapsed:?}"),
            };
            if elapsed.as_secs() >= 10 {
                break;
            }
            yield note;
        }
    };

    let response = client.route_chat(Request::new(outbound)).await?;
    let mut inbound = response.into_inner();

    while let Some(note) = inbound.message().await? {
        info!("NOTE = {:?}", note);
    }

    Ok(())
}
