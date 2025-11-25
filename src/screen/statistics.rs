use ratatui::widgets::{Block, Borders, Paragraph};

use crate::types::{App, AppMessage, ScreenTrait};

pub enum StatisticsMessage {
    Quit,
}

pub struct StatisticsScreen {}

impl ScreenTrait for StatisticsScreen {
    fn view(model: &mut App, frame: &mut ratatui::Frame) {
        let correct = model.mcq.as_ref().unwrap().correct_no;
        let total = model.mcq.as_ref().unwrap().questions.len() as u8;
        let percentage = (correct as f32 / total as f32) * 100.0;
        let question_widget = Paragraph::new(format!(
            "You got {} out of {} questions correct\nPercentage: {}%",
            correct, total, percentage as u8
        ))
        .block(Block::default().title("Statistics").borders(Borders::ALL));

        frame.render_widget(question_widget, frame.area());
    }

    fn handle_key(key: crossterm::event::KeyEvent) -> Option<AppMessage> {
        match key.code {
            crossterm::event::KeyCode::Char('q') => {
                Some(AppMessage::StatisticsScreen(StatisticsMessage::Quit))
            }
            _ => None,
        }
    }

    fn update(model: &mut App, msg: AppMessage) -> Option<AppMessage> {
        match msg {
            AppMessage::StatisticsScreen(StatisticsMessage::Quit) => {
                model.state = crate::types::State::Shutdown;
            }
            _ => std::todo!(),
        };
        None
    }
}
