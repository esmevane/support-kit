use support_kit::Config;

pub fn main() {
    let config: Config = serde_json::from_str(
        r#"
        {
            "logging": ["stderr", "stdout"]
        }
        "#,
    )
    .unwrap();

    let _logging = config.init_logging();

    tracing::trace!("Hello, world!");
    tracing::debug!("Hello, world!");
    tracing::info!("Hello, world!");
    tracing::warn!("Hello, world!");
    tracing::error!("Hello, world!");
}
