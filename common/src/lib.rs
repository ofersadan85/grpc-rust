pub mod prelude;
pub use prelude::{Error, Result};

pub mod pb {
    #![allow(clippy::all, clippy::pedantic, clippy::nursery)]
    pub mod hello_world {
        tonic::include_proto!("hello_world");
    }
    pub mod route_guide {
        tonic::include_proto!("route_guide");
    }
}
