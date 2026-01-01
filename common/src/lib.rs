use std::net::Ipv6Addr;

pub mod prelude;

pub mod pb {
    #![allow(clippy::all, clippy::pedantic, clippy::nursery)]
    pub mod hello_world {
        tonic::include_proto!("hello_world");
    }
    pub mod route_guide {
        tonic::include_proto!("route_guide");
    }
}

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Host IP address
    #[arg(long, default_value_t = Ipv6Addr::LOCALHOST.into())]
    pub host: std::net::IpAddr,

    /// Port number
    #[arg(long, default_value_t = 50051)]
    pub port: u16,
}
