mod boilerplate_context;
mod boilerplate_control;
mod boilerplate_preset;
mod boilerplate_template;

pub use boilerplate_context::BoilerplateContext;
pub use boilerplate_control::BoilerplateControl;
pub use boilerplate_preset::BoilerplatePreset;
pub use boilerplate_template::BoilerplateTemplate;

#[test]
fn custom_template() {
    use crate::Configuration;
    use std::fs;

    let config = BoilerplateContext::from(Configuration::default());
    let path = std::env::temp_dir();
    let file_name = "test.txt";
    let source = "Hello, {{ name }}!";
    let template = BoilerplateTemplate::builder()
        .path(path.clone())
        .file_name(file_name)
        .source(source)
        .build();

    assert_eq!(template.file(), path.join(file_name));

    fs::remove_dir_all(&path).unwrap_or_default();
    template.write(&config).unwrap();
    assert_eq!(
        fs::read_to_string(template.file()).unwrap(),
        "Hello, support-kit!"
    );
    fs::remove_dir_all(&path).unwrap_or_default();
}

#[cfg(test)]
const CARGO_CONFIG: &str = r#"[alias]
xtask = "run --package xtask --bin xtask --""#;

#[test]
fn cargo_config() -> Result<(), Box<dyn std::error::Error>> {
    use crate::Configuration;

    let path = std::env::temp_dir();
    let controller = BoilerplateControl::builder()
        .context(Configuration::default())
        .base_path(&path)
        .build();

    controller.write(BoilerplatePreset::CargoConfig)?;

    let template = controller.template(BoilerplatePreset::CargoConfig);
    assert_eq!(
        template.key(),
        format!("{path}.cargo/config.toml", path = path.display())
    );

    assert_eq!(
        std::fs::read_to_string(template.file()).unwrap(),
        CARGO_CONFIG
    );
    std::fs::remove_dir_all(&path).unwrap_or_default();

    Ok(())
}

#[cfg(test)]
const DOCKERFILE: &str = r#"FROM rust:1.82.0-slim AS builder
COPY . .
RUN cargo build --release 

FROM gcr.io/distroless/cc:latest AS runtime
COPY --from=builder /target/release/support-kit support-kit

ENTRYPOINT ["./support-kit"]"#;

#[test]
fn dockerfile() -> Result<(), Box<dyn std::error::Error>> {
    use crate::Configuration;

    let path = std::env::temp_dir();
    let controller = BoilerplateControl::builder()
        .context(Configuration::default())
        .base_path(&path)
        .build();

    controller.write(BoilerplatePreset::Dockerfile)?;

    let template = controller.template(BoilerplatePreset::Dockerfile);
    assert_eq!(
        template.key(),
        format!(
            "{path}infrastructure/containers/Dockerfile",
            path = path.display()
        )
    );

    assert_eq!(
        std::fs::read_to_string(template.file()).unwrap(),
        DOCKERFILE
    );
    std::fs::remove_dir_all(&path).unwrap_or_default();

    Ok(())
}

#[cfg(test)]
const NU_BUILD_ACTION: &str = r#"name: Build & Deploy
on:
  push:
    branches:
      - main
jobs:
  build:
    runs-on: ubuntu-latest
    env:
      RUST_LOG: "debug,support-kit=trace"
    strategy:
      matrix:
        toolchain:
          - stable
    steps:
      - uses: actions/checkout@v4
      - uses: docker/login-action@v3
        with:
          registry: ghcr.io
          username: ${{ github.actor }}
          password: ${{ secrets.SUPPORT_KIT_REPO_KEY }}
      - run: rustup update ${{ matrix.toolchain }} && rustup default ${{ matrix.toolchain }}
      - uses: jsdaniell/create-json@v1.2.3
        with:
          name: "emblem.json"
          json: ${{ secrets.SUPPORT_KIT_CONFIG }}
      - uses: shimataro/ssh-key-action@v2
        with:
          key: ${{ secrets.SUPPORT_KIT_SSH_KEY }}
          name: id_rsa
          known_hosts: ${{ secrets.SUPPORT_KIT_KNOWN_HOSTS }}
      - run: cargo run deploy list
      - run: cargo run container build
      - run: cargo run container push
      - run: cargo run deploy restart"#;

#[test]
fn build_action() -> Result<(), Box<dyn std::error::Error>> {
    use crate::Configuration;

    let path = std::env::temp_dir();
    let controller = BoilerplateControl::builder()
        .context(Configuration::default())
        .base_path(&path)
        .build();

    controller.write(BoilerplatePreset::BuildAction)?;

    let template = controller.template(BoilerplatePreset::BuildAction);
    assert_eq!(
        template.key(),
        format!("{path}.github/workflows/build.yaml", path = path.display())
    );

    assert_eq!(
        std::fs::read_to_string(template.file()).unwrap(),
        NU_BUILD_ACTION
    );
    std::fs::remove_dir_all(&path).unwrap_or_default();

    Ok(())
}

#[cfg(test)]
const TEST_ACTION: &str = r#"name: Lint & Test
on:
  pull_request:
    branches:
      - main
env:
  RUSTFLAGS: "-Dwarnings"
jobs:
  test:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        toolchain:
          - stable
    steps:
      - uses: actions/checkout@v4
      - run: rustup update ${{ matrix.toolchain }} && rustup default ${{ matrix.toolchain }}
      - run: cargo check --all-targets --all-features
      - run: cargo test
      - run: cargo clippy --all-targets --all-features"#;

#[test]
fn test_action() -> Result<(), Box<dyn std::error::Error>> {
    use crate::Configuration;

    let path = std::env::temp_dir();
    let controller = BoilerplateControl::builder()
        .context(Configuration::default())
        .base_path(&path)
        .build();

    controller.write(BoilerplatePreset::TestAction)?;

    let template = controller.template(BoilerplatePreset::TestAction);
    assert_eq!(
        template.key(),
        format!("{path}.github/workflows/test.yaml", path = path.display())
    );

    assert_eq!(
        std::fs::read_to_string(template.file()).unwrap(),
        TEST_ACTION
    );
    std::fs::remove_dir_all(&path).unwrap_or_default();

    Ok(())
}
