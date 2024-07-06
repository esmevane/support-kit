pub use service_kit_core::Result;

#[tokio::main]
async fn main() -> service_kit::Result<()> {
    let guard = service_kit_core::run().await?;

    drop(guard);

    Ok(())
}
