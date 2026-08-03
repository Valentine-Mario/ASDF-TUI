use core::panic;
use std::sync::mpsc::{self, Receiver, Sender};

use crate::asdf::{AsdfCommands, Parameter, get_asdf_metadata};
use crate::event::{AppEvent, Event, EventHandler, LogEvent};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{DefaultTerminal, widgets::ListState};

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// asdf logs
    pub log_message: String,
    /// Event handler.
    pub events: EventHandler,
    /// List state for the list of asdf commands.
    pub list_state: ListState,
    /// List of asdf commands.
    pub asdf_commands: Vec<(&'static str, &'static str)>,
    /// selected asdf command
    pub selected_option: AsdfCommands,
    /// user input
    pub user_input: Vec<Parameter>,
    /// user input list state
    pub user_input_state: ListState,
    /// user selected option from the list of parameters
    pub selected_parameter: Option<usize>,
    /// enable pop up dialog
    pub pop_up: bool,
    ///scroll the details page
    pub detail_scroll: u16,
    /// send log events to the log thread
    pub tx: Sender<LogEvent>,
    ///receive log events from the log thread
    pub rx: Receiver<LogEvent>,
}

impl Default for App {
    fn default() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        let (tx, rx) = mpsc::channel();

        Self {
            running: true,
            log_message: String::new(),
            events: EventHandler::new(),
            list_state,
            asdf_commands: get_asdf_metadata(),
            selected_option: AsdfCommands::Version,
            selected_parameter: None,
            user_input: Vec::new(),
            user_input_state: ListState::default(),
            pop_up: false,
            detail_scroll: 0,
            tx,
            rx,
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
        if self.pop_up {
            let i = match self.user_input_state.selected() {
                Some(i) => {
                    if i == 0 {
                        self.user_input.len() - 1
                    } else {
                        i - 1
                    }
                }
                None => self.user_input.len() - 1,
            };
            self.user_input_state.select(Some(i));
            self.selected_parameter = Some(i)
        } else {
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
    }

    pub fn down(&mut self) {
        if self.pop_up {
            let i = match self.user_input_state.selected() {
                Some(i) => {
                    if i >= self.user_input.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };
            self.user_input_state.select(Some(i));
            self.selected_parameter = Some(i)
        } else {
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
    }

    pub fn enter(&mut self) {
        self.clear_log();
        if let Some(selected) = self.list_state.selected() {
            let command = self.asdf_commands[selected].0;
            let parameters = AsdfCommands::parameters(command);
            if parameters.is_empty() {
                //execute the command
                let asdf_command = AsdfCommands::from_name(command, vec![]);
                if asdf_command.is_ok() {
                    self.selected_option = asdf_command.unwrap();
                    AsdfCommands::execute(&self.selected_option, self.tx.clone());
                } else {
                    self.log_message = format!("Error: {}", asdf_command.unwrap_err());
                }
            } else {
                // show pop up dialog to get user input for parameters
                self.pop_up = true;
                self.user_input = parameters;
            }
        } else {
            panic!("No command selected");
        }
    }
    pub fn update_selected_parameter(&mut self, value: String) {
        if let Some(selected) = self.selected_parameter {
            self.user_input[selected]
                .value
                .get_or_insert_with(String::new)
                .push_str(&value);
        }
    }

    pub fn backspace_selected_parameter(&mut self) {
        if let Some(selected) = self.selected_parameter {
            if let Some(value) = &mut self.user_input[selected].value {
                value.pop();
            }
        }
    }

    fn clear_log(&mut self) {
        self.log_message.clear();
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
            KeyCode::Char(c) => self.update_selected_parameter(c.to_string()),
            KeyCode::Backspace => self.backspace_selected_parameter(),
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
    pub fn tick(&mut self) {
        while let Ok(event) = self.rx.try_recv() {
            match event {
                LogEvent::Log(line) => {
                    //todo: buffer the log message and display in chunks
                    self.log_message.push_str(&line);
                    self.log_message.push('\n');
                }

                LogEvent::Finished(_) => {
                    self.log_message.push_str("\n\n");
                }
            }
        }
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
