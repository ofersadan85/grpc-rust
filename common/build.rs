use tonic_prost_build::{compile_protos, configure};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    compile_protos("proto/hello_world.proto")?;
    configure()
        .type_attribute("Feature", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("Point", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_protos(&["proto/route_guide.proto"], &["proto"])?;
    compile_protos("proto/chat.proto")?;
    Ok(())
}
