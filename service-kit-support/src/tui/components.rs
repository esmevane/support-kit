use crate::tui::{action::Action, config::Config};
use ratatui::layout::Rect;

#[derive(Clone, Debug)]
pub struct ActionContext {
    pub action_tx: tokio::sync::mpsc::UnboundedSender<Action>,
    pub config: Config,
    pub action: Action,
}

pub trait Component: std::fmt::Debug {
    fn view(&self, frame: &mut ratatui::Frame, area: Rect);
    fn update(&mut self, action: ActionContext) -> crate::Result<Option<Action>>;
}
