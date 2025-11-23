use ratatui::{crossterm::event::{KeyEvent, KeyCode, Event},
wigets::Paragraph, Frame};

enum Message {
    Next(u8),
}

struct McqScreen {
    current_selected: String,
}

impl McqScreen {
    fn handle_key(key: KeyEvent) -> Option<Message> {
        match key.code {
            KeyCode::Char('1') => Some(Message(Next(1)),
            KeyCode::Char('2') => Some(Message(Next(2)),
            KeyCode::Char('3') => Some(Message(Next(3)),
            KeyCode::Char('4') => Some(Message(Next(4)),
        };
    }

fn update(&mut self, msg: Message, app: &mut App) {
match msg {
    Message::Next(n) => {
        app.mcq.next(n + 1);
    }
}
}

fn handle_event(app: &App) {
 if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(handle_key(key));
            }
        }
    }

    fn view(app: &mut App, frame: &mut Frame) {
       let layout = Layout::new(Direction::Vertical, Constraint::from_percentages([50, 50]))
           .split(frame.area());
       let paragraph = Paragraph::new(app.questions[app.index].question);
       frame.render_widget(paragraph, layout[0]);

       let horizontal = Layout::new(Direction::Vertical, Constrain::from_percentages([50, 50]))
       .split(layout[1]);

       let first_split = Layout::new(Direction::Horizontal, Contraint::from_percentages([50, 50]))
       .split(horizontal[0]);
       let second_split = Layout::new(Direction::Horizontal, Contraint::from_percentages([50, 50]))
       .split(horizontal[1]);

       let option_1 = Paragraph::new(aapp.questions[app.index].options[0]);      
       let option_2 = Paragraph::new(aapp.questions[app.index].options[1]);
       let option_3 = Paragraph::new(aapp.questions[app.index].options[2]);
       let option_4 = Paragraph::new(aapp.questions[app.index].options[3]);

       frame.render_widget(option_1,first_split[0]);
       frame.render_widget(option_2,first_split[1]);
       frame.render_widget(option_3,second_split[0]);
       frame.render_widget(option_4,second_split[1]);
       
    }
}
