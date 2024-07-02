pub mod container;
pub mod web;
pub mod server {
    pub fn dev() {
        duct::cmd!("cargo", "service-kit", "server", "full")
            .run()
            .expect("Failed to start server in dev mode");
    }
}
