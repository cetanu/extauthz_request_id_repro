fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().build_client(false).compile(
        &[
            "proto/envoy/service/auth/v2/external_auth.proto",
            "proto/envoy/service/auth/v3/external_auth.proto",
        ],
        &["proto/"],
    )?;
    Ok(())
}
