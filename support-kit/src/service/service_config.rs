use super::ServiceLabel;

#[derive(Clone, Debug, Default, serde::Deserialize, PartialEq, bon::Builder)]
#[serde(rename_all = "kebab-case")]
pub struct ServiceConfig {
    #[serde(default)]
    #[builder(default, into)]
    label: ServiceLabel,

    #[serde(default)]
    #[builder(default)]
    system: bool,
}

#[test]
fn default_label() -> Result<(), Box<dyn std::error::Error>> {
    use figment::Jail;

    let config: ServiceConfig = serde_json::from_str(r#"{}"#)?;

    assert_eq!(config, ServiceConfig::default());

    let config: ServiceConfig = serde_json::from_str(
        r#"
        {
            "label": "support-kit"
        }
        "#,
    )?;

    assert_eq!(config, ServiceConfig::default());

    let config: ServiceConfig = serde_json::from_str(
        r#"
        {
            "label": "support-kit"
        }
        "#,
    )?;

    assert_eq!(config, ServiceConfig::builder().build());

    Jail::expect_with(|jail| {
        jail.set_env("CARGO_PKG_NAME", "consumer-package");

        let config: ServiceConfig = serde_json::from_str(
            r#"
            {
                "label": "consumer-package"
            }
            "#,
        )
        .expect("failed to parse");

        assert_eq!(config, ServiceConfig::default());

        Ok(())
    });

    Ok(())
}

#[test]
fn custom_service_label() -> Result<(), Box<dyn std::error::Error>> {
    let config: ServiceConfig = serde_json::from_str(
        r#"
        {
            "label": "custom-name"
        }
        "#,
    )?;

    assert_eq!(
        config,
        ServiceConfig::builder().label("custom-name").build()
    );

    Ok(())
}

#[test]
fn default_system_service() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(
        ServiceConfig::default(),
        ServiceConfig::builder().system(false).build()
    );

    Ok(())
}

#[test]
fn system_service() -> Result<(), Box<dyn std::error::Error>> {
    let config: ServiceConfig = serde_json::from_str(
        r#"
        {
            "system": true
        }
        "#,
    )?;

    assert_eq!(config, ServiceConfig::builder().system(true).build());

    Ok(())
}
