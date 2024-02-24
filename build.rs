fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().compile(
        &[
            "cnpg-i/proto/identity.proto",
            "cnpg-i/proto/operator_lifecycle.proto",
        ],
        &["cnpg-i/proto"],
    )?;
    //tonic_build::compile_protos("cnpg-i/proto/identity.proto", "cnpg-i/proto/operator_lifecycle.proto")?;
    Ok(())
}
