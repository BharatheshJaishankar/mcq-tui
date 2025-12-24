use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::types::{App, AppMessage, Screen, ScreenTrait};
/// Contains Mcq Screen messages
pub enum McqMessage {
    /// Sends the input of the user, to evaluate the question
    Next(u8),
    /// Exits the app
    Quit,
}

/// Screen Trait is implemented on this struct
pub struct McqScreen {}

impl ScreenTrait for McqScreen {
    fn view(model: &mut App, frame: &mut ratatui::Frame) {
        // Split screen vertically: question (top) and options (bottom)
        let layout = Layout::new(Direction::Vertical, Constraint::from_percentages([40, 60]))
            .split(frame.area());

        // Only render if we have an MCQ loaded
        if let Some(mcq) = &model.mcq {
            if let Some(question) = mcq.questions.get(mcq.index as usize) {
                // Render the question text
                let question_widget = Paragraph::new(question.question.clone()).block(
                    Block::default()
                        .title(format!("Question {}", mcq.index + 1))
                        .borders(Borders::ALL),
                ).wrap(Wrap {trim: true});
                frame.render_widget(question_widget, layout[0]);

                // Split bottom half vertically into two rows
                let rows = Layout::new(Direction::Vertical, Constraint::from_percentages([50, 50]))
                    .split(layout[1]);

                // Each row split horizontally into two columns
                let first_row = Layout::new(
                    Direction::Horizontal,
                    Constraint::from_percentages([50, 50]),
                )
                .split(rows[0]);
                let second_row = Layout::new(
                    Direction::Horizontal,
                    Constraint::from_percentages([50, 50]),
                )
                .split(rows[1]);

                // Render options (assuming exactly 4)
                let option_widgets = question
                    .options
                    .iter()
                    .enumerate()
                    .map(|(i, text)| {
                        let style = Style::default();
                        if question.checked == false {
                            Paragraph::new(text.clone()).style(style).block(
                                Block::default()
                                    .title(format!("Option {}", i + 1))
                                    .borders(Borders::ALL),
                            ).wrap(Wrap {trim: true})
                        } else {
                            if question.correct == (i + 1) as u8 {
                                Paragraph::new(text.clone())
                                    .style(style.bg(Color::Blue))
                                    .block(
                                        Block::default()
                                            .title(format!("Option {}", i + 1))
                                            .borders(Borders::ALL),
                                    ).wrap(Wrap {trim: true})
                            } else {
                                if question.selected == Some((i + 1) as u8) {
                                    Paragraph::new(text.clone())
                                        .style(style.bg(Color::Red))
                                        .block(
                                            Block::default()
                                                .title(format!("Option {}", i + 1))
                                                .borders(Borders::ALL),
                                        ).wrap(Wrap {trim: true})
                                } else {
                                    Paragraph::new(text.clone()).style(style).block(
                                        Block::default()
                                            .title(format!("Option {}", i + 1))
                                            .borders(Borders::ALL),
                                    ).wrap(Wrap {trim: true})
                                }
                            }
                        }
                    })
                    .collect::<Vec<_>>();

                // Place options in grid
                frame.render_widget(option_widgets[0].clone(), first_row[0]);
                frame.render_widget(option_widgets[1].clone(), first_row[1]);
                frame.render_widget(option_widgets[2].clone(), second_row[0]);
                frame.render_widget(option_widgets[3].clone(), second_row[1]);
            }
        } else {
            // If no MCQ loaded, show placeholder
            let placeholder = Paragraph::new("No question loaded")
                .block(Block::default().title("Info").borders(Borders::ALL));
            frame.render_widget(placeholder, layout[0]);
        }
    }

    fn handle_key(key: crossterm::event::KeyEvent) -> Option<AppMessage> {
        match key.code {
            crossterm::event::KeyCode::Char('1') => {
                Some(AppMessage::McqScreen(McqMessage::Next(1)))
            }
            crossterm::event::KeyCode::Char('2') => {
                Some(AppMessage::McqScreen(McqMessage::Next(2)))
            }
            crossterm::event::KeyCode::Char('3') => {
                Some(AppMessage::McqScreen(McqMessage::Next(3)))
            }
            crossterm::event::KeyCode::Char('4') => {
                Some(AppMessage::McqScreen(McqMessage::Next(4)))
            }
            crossterm::event::KeyCode::Char('q') => Some(AppMessage::McqScreen(McqMessage::Quit)),
            _ => None,
        }
    }

    fn update(model: &mut App, msg: AppMessage) -> Option<AppMessage> {
        match msg {
            AppMessage::McqScreen(McqMessage::Next(index)) => {
                if let Some(mcq) = &mut model.mcq {
                    if mcq.questions[mcq.index as usize].checked == false {
                        mcq.questions[mcq.index as usize].selected = Some(index);
                        mcq.next(index);
                        mcq.questions[mcq.index as usize].checked = true;
                    } else {
                        mcq.questions[mcq.index as usize].checked = false;
                        mcq.index += 1;
                        if mcq.index == mcq.questions.len() as u8 {
                            model.screen = Screen::StatisticsScreen;
                        }
                    }
                }
            }
            AppMessage::McqScreen(McqMessage::Quit) => {
                model.state = crate::types::State::Shutdown;
            }
            _ => std::todo!(),
        };
        None
    }
}
