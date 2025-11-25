mod screen;
mod types;

use crate::types::mcq::{Mcq, Question};
use crate::types::{App, Screen, State};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    crossterm::{
        ExecutableCommand,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
};
use std::io::{Result, stdout};

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    let prompt = args.get(1).expect("Please provide a File path");

    let mcq = Mcq::new(parse_file(prompt.to_string()));

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut model = App {
        state: State::Running,
        screen: Screen::McqScreen,
        mcq: Some(mcq),
    };

    while model.state != State::Shutdown {
        terminal.draw(|f| model.view(f))?;

        let mut current_msg = model.handle_event()?;

        while current_msg.is_some() {
            current_msg = model.update(current_msg.unwrap());
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn parse_file(prompt: String) -> Vec<Question> {
    let file = std::fs::read_to_string(prompt).expect("Unable to read file");
    let questions = file.split("|").collect::<Vec<&str>>();
    let mut db: Vec<Question> = Vec::new();
    for question in questions {
        let options = question.split("#").collect::<Vec<&str>>();
        let try_options = vec![
            options[1].to_string(),
            options[2].to_string(),
            options[3].to_string(),
            options[4].to_string(),
        ];
        db.push(Question {
            question: options[0].to_string(),
            options: try_options,
            correct: options[5].parse::<u8>().unwrap(),
            selected: None,
            checked: false,
        });
    }
    db
}
