use duct::cmd;

pub fn preflight() {
    cmd!("pnpm", "--version")
        .run()
        .expect("Failed to find pnpm");
    cmd!("node", "--version")
        .run()
        .expect("Failed to find node");
}

pub fn clean() {
    cmd!("rm", "-rf", "node_modules")
        .run()
        .expect("Failed to clean dashboard");
}

pub fn install() {
    cmd!("pnpm", "i",)
        .run()
        .expect("Failed to install dashboard dependencies");
}

pub fn build() {
    cmd!("pnpm", "run", "--recursive", "build")
        .run()
        .expect("Failed to build dashboard");
    cmd!(
        "cp",
        "-r",
        "service-kit-dashboard/dist/",
        "service-kit-core/dist/"
    )
    .run()
    .expect("Failed to copy dashboard to core");
}
