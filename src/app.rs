use core::panic;

use crate::asdf::{AsdfCommands, get_asdf_metadata};
use crate::event::{AppEvent, Event, EventHandler};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{DefaultTerminal, widgets::ListState};

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// asdf logs
    pub log_message: String,
    ///ASDF version
    pub asdf_version: String,
    /// Event handler.
    pub events: EventHandler,
    /// List state for the list of asdf commands.
    pub list_state: ListState,
    /// List of asdf commands.
    pub asdf_commands: Vec<(&'static str, &'static str)>,
    /// selected asdf command
    pub selected_option: AsdfCommands,
    /// user input
    pub user_input: Vec<String>,
    /// enable pop up dialog
    pub pop_up: bool,
    ///scroll the details page
    pub detail_scroll: u16,
}

impl Default for App {
    fn default() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        let asdf_version = AsdfCommands::execute(&AsdfCommands::Version);
        if asdf_version.is_err() {
            panic!(
                "asdf is not installed or not found in PATH. Please install asdf and try again."
            );
        };

        Self {
            running: true,
            log_message: String::new(),
            asdf_version: asdf_version.unwrap(),
            events: EventHandler::new(),
            list_state,
            asdf_commands: get_asdf_metadata(),
            selected_option: AsdfCommands::Version,
            user_input: Vec::new(),
            pop_up: false,
            detail_scroll: 0,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }
    pub fn selected_description(&self) -> &'static str {
        if let Some(selected) = self.list_state.selected() {
            self.asdf_commands[selected].1
        } else {
            ""
        }
    }

    pub fn up(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.asdf_commands.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn down(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.asdf_commands.len() - 1
                } else {
                    i - 1
                }
            }
            None => self.asdf_commands.len() - 1,
        };
        self.list_state.select(Some(i));
    }

    pub fn enter(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            self.log_message = String::from("executing command...");
            let command = self.asdf_commands[selected].0;
            let parameters = AsdfCommands::parameters(command);
            if parameters.is_empty() {
                //execute the command
                let asdf_command = AsdfCommands::from_name(command, vec![]);
                if asdf_command.is_ok() {
                    self.selected_option = asdf_command.unwrap();
                    let message = AsdfCommands::execute(&self.selected_option);
                    if message.is_ok() {
                        self.log_message = message.unwrap();
                    } else {
                        self.log_message = format!("Error: {}", message.unwrap_err());
                    }
                } else {
                    self.log_message = format!("Error: {}", asdf_command.unwrap_err());
                }
            } else {
                // show pop up dialog to get user input for parameters
                self.pop_up = true;
            }
        } else {
            panic!("No command selected");
        }
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    pub fn handle_events(&mut self) -> color_eyre::Result<()> {
        match self.events.next()? {
            Event::Tick => self.tick(),
            Event::Crossterm(event) => match event {
                crossterm::event::Event::Key(key_event)
                    if key_event.kind == crossterm::event::KeyEventKind::Press =>
                {
                    self.handle_key_event(key_event)?
                }
                _ => {}
            },
            Event::App(app_event) => match app_event {
                AppEvent::Quit => self.quit(),
                AppEvent::Decrement => self.down(),
                AppEvent::Increment => self.up(),
                AppEvent::Enter => self.enter(),
            },
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Esc => self.pop_up = false,
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }
            KeyCode::Down => self.events.send(AppEvent::Increment),
            KeyCode::Up => self.events.send(AppEvent::Decrement),
            KeyCode::Enter => self.events.send(AppEvent::Enter),
            KeyCode::PageUp => self.detail_scroll = self.detail_scroll.saturating_sub(10),
            KeyCode::PageDown => self.detail_scroll = self.detail_scroll.saturating_add(10),
            // Other key events you could handle here.
            // Other handlers you could add here.
            _ => {}
        }
        Ok(())
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
