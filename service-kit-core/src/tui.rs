mod clock;
mod display;
mod input;
mod window;

use window::Window;

pub async fn init() -> crate::Result<()> {
    service_kit_support::tui::App::init(vec![Box::new(Window::new())]).await?;

    Ok(())
}
