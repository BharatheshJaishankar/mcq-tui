mod types;

fn main() -> color_eyre::Result<()> {
    tui::install_panic_hook();
let mut terminal = tui::init_terminal()?;
let mut app = types::App::new();

while app.state != State::Done {
    terminal.draw(|f| app.screen.view(&mut app, f))?;
    let mut current_msq = app.screen.handle_event(&app)?;
    
    while current_msg.is_some() {
        current_msg = app.screen.update(&mut app, current_msg.unwrap());
    }
}

tui::restore_terminal()?;
Ok(())
}

mod tui {
    use ratatui::{
        backend::{Backend, CrosstermBackend},
        crossterm::{
            terminal::{
                disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
            },
            ExecutableCommand,
        },
        Terminal,
    };
    use std::{io::stdout, panic};

    pub fn init_terminal() -> color_eyre::Result<Terminal<impl Backend>> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        Ok(terminal)
    }

    pub fn restore_terminal() -> color_eyre::Result<()> {
        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn install_panic_hook() {
        let original_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic_info| {
            stdout().execute(LeaveAlternateScreen).unwrap();
            disable_raw_mode().unwrap();
            original_hook(panic_info);
        }));
    }
}    
