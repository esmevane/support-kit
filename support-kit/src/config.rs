mod config_definition;
mod config_env_var;
mod config_file;
mod config_format;
mod config_manifest;
mod config_sources;
mod configuration;

use config_definition::ConfigDefinition;
use config_env_var::ConfigEnvVar;
use config_format::ConfigFormat;

pub use config_file::ConfigFile;
pub use config_manifest::ConfigManifest;
pub use config_sources::ConfigSources;
pub use configuration::Configuration;

#[test]
fn config_file_sets_sources() -> Result<(), Box<dyn std::error::Error>> {
    use clap::Parser;

    use crate::{config::ConfigFormat, Args, Environment, SupportControl};

    for format in ConfigFormat::all() {
        let envs = Environment::all();
        for env in envs {
            let expectations = [
                ("app", None, None, "support-kit"),
                (
                    &format!("app --environment {env}"),
                    Some(env),
                    None,
                    "support-kit",
                ),
                (
                    &format!("app --config-file custom.config --environment {env}"),
                    Some(env),
                    None,
                    "custom.config",
                ),
                (
                    &format!("app --name custom-app-name --environment {env}"),
                    Some(env),
                    None,
                    "custom-app-name",
                ),
                (
                    &format!("app --config-file custom.config --environment {env}"),
                    Some(env),
                    Some("custom-app-name"),
                    "custom.config",
                ),
                (
                    &format!("app --environment {env}"),
                    Some(env),
                    Some("custom-app-name"),
                    "custom-app-name",
                ),
                (
                    &format!("app --name custom-app-name --environment {env}"),
                    Some(env),
                    Some("legacy-app-name"),
                    "custom-app-name",
                ),
            ];

            for (input, env, crate_name, expected) in expectations {
                figment::Jail::expect_with(|jail| {
                    let env = env.unwrap_or_default();

                    if let Some(crate_name) = crate_name {
                        jail.set_env("CARGO_PKG_NAME", crate_name);
                    }

                    jail.create_file(format!("{expected}.{format}"), format.empty_file_contents())?;
                    jail.create_file(
                        format!("{expected}.{env}.{format}"),
                        format.empty_file_contents(),
                    )?;

                    let args = Args::try_parse_from(input.split_whitespace()).unwrap();
                    let controller = SupportControl::load_configuration(&args).unwrap();

                    assert_eq!(
                        controller.manifest().unwrap().known(),
                        ConfigManifest::builder()
                            .definitions(bon::vec![
                                (format, expected),
                                expected,
                                (format, expected, env),
                                (expected, env),
                            ])
                            .build()
                    );
                    Ok(())
                });
            }
        }
    }

    Ok(())
}
