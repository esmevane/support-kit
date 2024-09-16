/// Fails if pnpm or node is not installed
pub fn preflight() {
    duct::cmd!("pnpm", "--version")
        .run()
        .expect("Failed to find pnpm");
    duct::cmd!("node", "--version")
        .run()
        .expect("Failed to find node");
}

/// Remove node_modules directory so that we can start fresh
pub fn clean() {
    duct::cmd!("rm", "-rf", "node_modules")
        .run()
        .expect("Failed to clean dashboard");
}

/// Start the dashboard in development mode
pub fn dev() {
    duct::cmd!("pnpm", "run", "--recursive", "dev")
        .run()
        .expect("Failed to start dashboard in dev mode");
}

/// Install dashboard & general workspace dependencies
pub fn install() {
    duct::cmd!("pnpm", "i",)
        .run()
        .expect("Failed to install dashboard dependencies");
}

/// Build the static dashboard files
pub fn build() {
    duct::cmd!("pnpm", "run", "--recursive", "build")
        .run()
        .expect("Failed to build dashboard");
}

/// Copy dashboard dist to the core (executable) dist
pub fn copy() {
    duct::cmd!(
        "cp",
        "-r",
        "service-kit-dashboard/dist/",
        "service-kit-core/dist/"
    )
    .run()
    .expect("Failed to copy dashboard to core");
}
