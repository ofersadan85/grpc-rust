use tonic_prost_build::compile_protos;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    compile_protos("proto/hello_world.proto")?;
    tonic_prost_build::configure()
        .type_attribute("Feature", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("Point", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_protos(&["proto/route_guide.proto"], &["proto"])?;
    Ok(())
}
