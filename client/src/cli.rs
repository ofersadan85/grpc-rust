use std::net::Ipv6Addr;

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Server IP address
    #[arg(long, env, default_value_t = Ipv6Addr::LOCALHOST.into())]
    pub host: std::net::IpAddr,

    /// Port number
    #[arg(long, env, default_value_t = 50051)]
    pub port: u16,

    /// Subcommand
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(clap::Subcommand)]
pub enum Commands {
    /// Check services health status once and exit.
    /// This is the default if no subcommand is provided
    Health(HealthOptions),
    /// Run example tasks
    Examples,
    /// Send a chat message
    Chat {
        /// Message content
        message: String,
    },
}

impl Default for Commands {
    fn default() -> Self {
        Self::Health(HealthOptions::default())
    }
}

#[derive(clap::Args, Default)]
pub struct HealthOptions {
    /// Services to check health status for
    #[arg(long, env, default_values = &["", "hello_world.Greeter", "route_guide.RouteGuide"])]
    pub services: Vec<String>,

    /// Watch health status continuously
    #[arg(long, env, default_value_t = false)]
    pub watch: bool,
}
