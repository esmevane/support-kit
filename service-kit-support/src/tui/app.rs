use crate::tui::{action::Action, components::ActionContext};

use super::{event::TerminalEvent, terminal::Terminal};

pub type Components = Vec<Box<dyn crate::tui::components::Component>>;

/// Application state
#[derive(Debug)]
pub struct App {
    /// Should the application quit?
    pub should_quit: bool,
    /// A sender channel to send async actions to the application.
    pub action_tx: tokio::sync::mpsc::UnboundedSender<Action>,
    /// Components
    pub components: Vec<Box<dyn crate::tui::components::Component>>,
}

impl App {
    /// Create a new application
    pub fn new(
        action_tx: tokio::sync::mpsc::UnboundedSender<Action>,
        components: Components,
    ) -> Self {
        App {
            should_quit: false,
            action_tx,
            components,
        }
    }

    pub async fn init(components: Components) -> crate::Result<()> {
        // Create an application.
        let (action_tx, mut action_rx) = tokio::sync::mpsc::unbounded_channel();
        let mut app = Self::new(action_tx.clone(), components);

        let mut terminal = Terminal::create()?;

        terminal.enter()?;

        // Start the main loop.
        while !app.should_quit {
            let event = terminal.next().await?;

            // Render if requested.
            if let TerminalEvent::Render = event {
                terminal.draw(|frame| app.render(frame))?;
            }

            if let Ok(action) = event.try_into() {
                action_tx.send(action).unwrap();
            }

            // Update the application state.
            if let Ok(action) = action_rx.try_recv() {
                app.update(action);
            }
        }

        // Exit the user interface.
        terminal.exit()?;

        Ok(())
    }

    /// Update a running application
    pub fn update(&mut self, action: Action) -> Option<Action> {
        if action == Action::Quit {
            self.quit()
        }

        // loop through all components and update them with the action context
        for component in self.components.iter_mut() {
            let next_action = component.update(ActionContext {
                action_tx: self.action_tx.clone(),
                config: crate::tui::config::Config::default(),
                action,
            });

            if let Ok(Some(action)) = next_action {
                return Some(action);
            }
        }

        None
    }

    /// Render the application, given a frame, by looping through all components
    /// and rendering them.
    pub fn render(&self, frame: &mut ratatui::Frame) {
        for component in &self.components {
            component.view(frame, frame.size());
        }
    }

    /// Quits the application
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
