pub mod mcq;

use self::mcq::Mcq;
use crate::screen::mcq::{McqMessage, McqScreen};
use crate::screen::statistics::{StatisticsMessage, StatisticsScreen};
use crossterm::event::{self, Event};
use ratatui::Frame;
use std::io::Result;
use std::time::Duration;

#[derive(Default)]
pub struct App {
    pub state: State,
    pub screen: Screen,
    pub mcq: Option<Mcq>,
}

pub enum AppMessage {
    McqScreen(McqMessage),
    StatisticsScreen(StatisticsMessage),
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum State {
    #[default]
    Running,
    Shutdown,
}

#[derive(Default)]
pub enum Screen {
    #[default]
    McqScreen,
    StatisticsScreen,
}

pub trait ScreenTrait {
    fn view(model: &mut App, frame: &mut Frame) {
        todo!()
    }

    fn handle_key(key: event::KeyEvent) -> Option<AppMessage> {
        match key.code {
            _ => None,
        }
    }

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
