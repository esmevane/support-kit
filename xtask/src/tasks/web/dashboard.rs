use duct::cmd;

/// Fails if pnpm or node is not installed
pub fn preflight() {
    cmd!("pnpm", "--version")
        .run()
        .expect("Failed to find pnpm");
    cmd!("node", "--version")
        .run()
        .expect("Failed to find node");
}

/// Remove node_modules directory so that we can start fresh
pub fn clean() {
    cmd!("rm", "-rf", "node_modules")
        .run()
        .expect("Failed to clean dashboard");
}

/// Install dashboard & general workspace dependencies
pub fn install() {
    cmd!("pnpm", "i",)
        .run()
        .expect("Failed to install dashboard dependencies");
}

/// Build the static dashboard files
pub fn build() {
    cmd!("pnpm", "run", "--recursive", "build")
        .run()
        .expect("Failed to build dashboard");
}

/// Copy dashboard dist to the core (executable) dist
pub fn copy() {
    cmd!(
        "cp",
        "-r",
        "service-kit-dashboard/dist/",
        "service-kit-core/dist/"
    )
    .run()
    .expect("Failed to copy dashboard to core");
}
