pub fn build() {
    duct::cmd!(
        "docker",
        "build",
        "-t",
        "service-kit-container:latest",
        "-f",
        "infrastructure/containers/service-kit/Dockerfile",
        "."
    )
    .run()
    .expect("Failed to build container");
}

pub fn preflight() {
    duct::cmd!("docker", "version")
        .run()
        .expect("Failed to check docker version");
}

pub fn clean() {
    duct::cmd!("docker", "system", "prune", "-f")
        .run()
        .expect("Failed to clean docker system");
}

pub fn install() {
    duct::cmd!("docker", "pull", "service-kit-container:latest")
        .run()
        .expect("Failed to pull container");
}

pub fn run() {
    duct::cmd!(
        "docker",
        "run",
        "--rm",
        "-it",
        "-p",
        "8080:8080",
        "service-kit-container:latest",
    )
    .run()
    .expect("Failed to run container");
}
