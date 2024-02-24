fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().compile(
        &["proto/identity.proto", "proto/operator_lifecycle.proto"],
        &["proto"],
    )?;
    //tonic_build::compile_protos("cnpg-i/proto/identity.proto", "cnpg-i/proto/operator_lifecycle.proto")?;
    Ok(())
}
