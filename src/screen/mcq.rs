use ratatui::{crossterm::event}

enum Message {
    Greet
}

struct McqScreen {
    name: String,
}

impl McqScreen {
    fn handle_key(key : event::KeyEvent) -> Option<Message> {
        match key.code {
            _ => todo!()
        };
    }

fn update(&mut self, msg: Message, app: &mut App) {
    todo!();
}

fn handle_event(app: &App) {
 if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(handle_key(key));
            }
        }
    }

    fn view(app: &mut APp, frame: &mut Frame) {
        
    }
}
