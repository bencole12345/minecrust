use tonic_build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/world.proto")?;
    tonic_build::compile_protos("proto/block.proto")?;
    tonic_build::compile_protos("proto/chunk.proto")?;
    tonic_build::compile_protos("proto/event.proto")?;
    tonic_build::compile_protos("proto/backend.proto")?;
    Ok(())
}
