use service_manager::ServiceManagerKind;

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

    /// The kind of service manager to use. Defaults to system native.
    #[serde(default)]
    pub service_manager: Option<ServiceManagerKind>,
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

#[test]
fn custom_service_manager() -> Result<(), Box<dyn std::error::Error>> {
    use service_manager::ServiceManagerKind::*;

    let expectations = [
        ("systemd", Systemd),
        ("winsw", WinSw),
        ("launchd", Launchd),
        ("openrc", OpenRc),
        ("rcd", Rcd),
        ("sc", Sc),
    ];

    for (input, expected) in expectations {
        let config: ServiceConfig = serde_json::from_str(&format!(
            r#"
            {{
                "service-manager": "{input}"
            }}
            "#
        ))?;

        assert_eq!(
            config,
            ServiceConfig::builder().service_manager(expected).build()
        );
    }

    Ok(())
}
