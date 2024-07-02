use duct::cmd;

pub fn build() {
    cmd!(
        "wasm-pack",
        "build",
        "--target",
        "web",
        "--out-dir",
        "../service-kit-dashboard/public/wasm",
        "service-kit-web"
    )
    .run()
    .expect("Failed to build web client");
}
