pub mod mcq;

use self::mcq::Mcq;
use crate::screen::mcq::{McqMessage, McqScreen};
use crate::screen::statistics::{StatisticsMessage, StatisticsScreen};
use crossterm::event::{self, Event};
use ratatui::Frame;
use std::io::Result;
use std::time::Duration;


/// App Struct which is the foundation of the TUI
#[derive(Default)]
pub struct App {
    /// There can be state which denotes how the application should run
    pub state: State,
    /// Changes the screen
    pub screen: Screen,
    /// Contains the Mcq struct
    pub mcq: Option<Mcq>,
}

/// Contains the Screen's respective messages
pub enum AppMessage {
    McqScreen(McqMessage),
    StatisticsScreen(StatisticsMessage),
}

/// The state of the app
#[derive(Debug, Default, PartialEq, Eq)]
pub enum State {
    /// Denotes app is running
    #[default]
    Running,
    /// TUI is quit
    Shutdown,
}

#[derive(Default)]
pub enum Screen {
    /// Switches to Mcq Screen
    #[default]
    McqScreen,
    /// Switches to Statistics Screen
    StatisticsScreen,
}

pub trait ScreenTrait {
    /// This renderes the TUI, it takes a frame and the App struct
    fn view(model: &mut App, frame: &mut Frame) {
        todo!()
    }
    /// It takes from KeyEvent which is coming from handle_events which is translated to the corresponding Message
    fn handle_key(key: event::KeyEvent) -> Option<AppMessage> {
        match key.code {
            _ => None,
        }
    }
    /// Takes a nessage and does work with it
    fn update(model: &mut App, msg: AppMessage) -> Option<AppMessage> {
        match msg {
            _ => todo!(),
        };
        None
    }
}

impl App {
    pub fn view(&mut self, frame: &mut Frame) {
        match self.screen {
            Screen::StatisticsScreen => StatisticsScreen::view(self, frame),
            Screen::McqScreen => McqScreen::view(self, frame),
        }
    }

    pub fn handle_event(&self) -> Result<Option<AppMessage>> {
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    return Ok(match self.screen {
                        Screen::StatisticsScreen => StatisticsScreen::handle_key(key),
                        Screen::McqScreen => McqScreen::handle_key(key),
                    });
                }
            }
        }
        Ok(None)
    }

    pub fn update(&mut self, msg: AppMessage) -> Option<AppMessage> {
        match self.screen {
            Screen::StatisticsScreen => StatisticsScreen::update(self, msg),
            Screen::McqScreen => McqScreen::update(self, msg),
        }
    }
}
