use std::net::Ipv6Addr;

pub mod prelude;

pub mod pb {
    pub mod hello_world {
        #![allow(clippy::all, clippy::pedantic, clippy::nursery)]
        tonic::include_proto!("helloworld");
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
