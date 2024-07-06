use ratatui::{
    layout::Alignment,
    widgets::{Block, Borders, Paragraph},
};
use service_kit_support::tui::{Action, ActionContext, Component};

fn time() -> String {
    chrono::Local::now().format("%H:%M:%S").to_string()
}

/// Clock display for terminal UI
#[derive(Clone, Debug)]
pub struct Clock {
    pub time: String,
}

impl Clock {
    pub fn new() -> Self {
        Self { time: time() }
    }
}

impl Component for Clock {
    fn update(&mut self, _: ActionContext) -> service_kit_support::Result<Option<Action>> {
        self.time = time();

        Ok(None)
    }

    fn view(&self, frame: &mut ratatui::Frame, area: ratatui::layout::Rect) {
        frame.render_widget(
            Paragraph::new("")
                .block(
                    Block::default()
                        .title(self.time.clone())
                        .title_alignment(Alignment::Right)
                        .borders(Borders::NONE),
                )
                .alignment(Alignment::Center),
            area,
        )
    }
}
