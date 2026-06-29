fn main() -> Result<(), Box<dyn std::error::Error>> {
    prost_build::compile_protos(&["src/proto/mission_input.proto"], &["src/"])?;
    Ok(())
}
