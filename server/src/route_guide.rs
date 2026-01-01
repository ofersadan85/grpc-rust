use common::pb::route_guide::{
    Feature, Point, Rectangle, RouteNote, RouteSummary, route_guide_server::RouteGuide,
};
use std::{cmp, collections::HashMap, pin::Pin, sync::Arc};
use tokio::{sync::mpsc, time::Instant};
use tokio_stream::{Stream, StreamExt, wrappers::ReceiverStream};
use tonic::{Request, Response, Status, Streaming};
use tracing::error;

use crate::{TonicResponse, middleware::add_request_log};

#[derive(Debug, Clone)]
pub struct RouteGuideService {
    pub(crate) features: Arc<Vec<Feature>>,
}

#[tonic::async_trait]
impl RouteGuide for RouteGuideService {
    async fn get_feature(&self, request: Request<Point>) -> TonicResponse<Feature> {
        add_request_log(&request);
        for feature in &self.features[..] {
            if feature.location.as_ref() == Some(request.get_ref()) {
                return Ok(Response::new(feature.clone()));
            }
        }
        Ok(Response::new(Feature::default()))
    }

    type ListFeaturesStream = ReceiverStream<Result<Feature, Status>>;

    async fn list_features(
        &self,
        request: Request<Rectangle>,
    ) -> Result<Response<Self::ListFeaturesStream>, Status> {
        add_request_log(&request);
        let (tx, rx) = mpsc::channel(4);
        let features = self.features.clone();

        tokio::spawn(async move {
            for feature in &features[..] {
                if in_range(feature.location, request.get_ref()).is_none_or(|b| !b) {
                    continue;
                }
                if let Err(e) = tx.send(Ok(feature.clone())).await {
                    error!("Failed to send feature via stream: {e}");
                    return;
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn record_route(
        &self,
        request: Request<Streaming<Point>>,
    ) -> TonicResponse<RouteSummary> {
        add_request_log(&request);
        let mut stream = request.into_inner();
        let mut summary = RouteSummary::default();
        let mut last_point = None;
        let now = Instant::now();

        while let Some(point) = stream.next().await {
            let point = point?;
            summary.point_count += 1;

            for feature in &self.features[..] {
                if feature.location.as_ref() == Some(&point) {
                    summary.feature_count += 1;
                }
            }

            if let Some(ref last_point) = last_point {
                summary.distance += calc_distance(*last_point, point);
            }

            last_point = Some(point);
        }

        summary.elapsed_time = i32::try_from(now.elapsed().as_secs()).unwrap_or(i32::MAX);
        Ok(Response::new(summary))
    }

    type RouteChatStream = Pin<Box<dyn Stream<Item = Result<RouteNote, Status>> + Send + 'static>>;

    async fn route_chat(
        &self,
        request: Request<Streaming<RouteNote>>,
    ) -> Result<Response<Self::RouteChatStream>, Status> {
        add_request_log(&request);
        let mut notes = HashMap::new();
        let mut stream = request.into_inner();

        let output = async_stream::try_stream! {
            while let Some(note) = stream.next().await {
                let note = note?;
                let location_notes = notes.entry(note.location).or_insert(vec![]);
                location_notes.push(note);

                for note in location_notes {
                    yield note.clone();
                }
            }
        };

        Ok(Response::new(Box::pin(output) as Self::RouteChatStream))
    }
}

fn in_range(point: Option<Point>, rect: &Rectangle) -> Option<bool> {
    let point = point?;
    let lo = rect.lo.as_ref()?;
    let hi = rect.hi.as_ref()?;

    let left = cmp::min(lo.longitude, hi.longitude);
    let right = cmp::max(lo.longitude, hi.longitude);
    let top = cmp::max(lo.latitude, hi.latitude);
    let bottom = cmp::min(lo.latitude, hi.latitude);

    Some(
        point.longitude >= left
            && point.longitude <= right
            && point.latitude >= bottom
            && point.latitude <= top,
    )
}

/// Calculates the distance between two points using the "haversine" formula.
/// This code was taken from <http://www.movable-type.co.uk/scripts/latlong.html>.
#[expect(clippy::cast_possible_truncation)]
fn calc_distance(p1: Point, p2: Point) -> i32 {
    const CORD_FACTOR: f64 = 1e7;
    const R: f64 = 6_371_000.0; // meters

    let lat1 = f64::from(p1.latitude) / CORD_FACTOR;
    let lat2 = f64::from(p2.latitude) / CORD_FACTOR;
    let lng1 = f64::from(p1.longitude) / CORD_FACTOR;
    let lng2 = f64::from(p2.longitude) / CORD_FACTOR;

    let lat_rad1 = lat1.to_radians();
    let lat_rad2 = lat2.to_radians();

    let delta_lat = (lat2 - lat1).to_radians();
    let delta_lng = (lng2 - lng1).to_radians();

    let a = (delta_lat / 2f64).sin().mul_add(
        (delta_lat / 2f64).sin(),
        (lat_rad1).cos() * (lat_rad2).cos() * (delta_lng / 2f64).sin() * (delta_lng / 2f64).sin(),
    );

    let c = 2f64 * a.sqrt().atan2((1f64 - a).sqrt());

    (R * c) as i32
}
