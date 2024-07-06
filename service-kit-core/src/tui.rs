use service_kit_support::tui::Components;

mod clock;
mod display;
mod input;
mod window;

use window::Window;

pub async fn init() -> crate::Result<()> {
    let components: Components = vec![Box::new(Window::new())];

    service_kit_support::tui::App::init(components).await?;

    Ok(())
}
