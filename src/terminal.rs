use tui::{backend::CrosstermBackend, Frame, Terminal};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

#[derive(Default)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

#[derive(Default, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone)]
pub struct BoxSelection {
    pub label: String,
    pub selected: bool,
}

impl BoxSelection {
    pub fn default(label: String, selected: bool) -> Self {
        Self { label, selected }
    }
}

pub struct TerminalInterface {
    pub _terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
}

impl TerminalInterface {
    pub fn default() -> Result<Self, std::io::Error> {

        enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen)?;

        let backend = CrosstermBackend::new(stdout);
        Ok(Self {
            _terminal: Terminal::new(backend)?,
        })
    }

    pub fn restore_terminal(&mut self) -> Result<(), std::io::Error>{
        disable_raw_mode()?;
        execute!(
            self._terminal.backend_mut(),
            LeaveAlternateScreen,
        )?;
        self._terminal.show_cursor()?;

        Ok(())
    }

    pub fn render_widget<F>(&mut self, widget:F) -> Result<(), std::io::Error> where
        F: FnOnce(&mut Frame<'_, CrosstermBackend<std::io::Stdout>>)
    {
        self._terminal.draw(widget).expect("");
        Ok(())
    }
}
