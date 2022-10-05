use std::io::{self, stdout, Write};
use termion::cursor::DetectCursorPos;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;

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

pub struct OldTerminal {
    size: Size,
    _stdout: Option<RawTerminal<std::io::Stdout>>,
    _alt_screen: Option<AlternateScreen<std::io::Stdout>>,
}

impl OldTerminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1,
            },
            _stdout: None,
            _alt_screen: None,
        })
    }

    pub fn put_into_raw(&mut self) -> Result<(), std::io::Error> {
        self._stdout = Some(stdout().into_raw_mode()?);
        self._alt_screen = Some(AlternateScreen::from(stdout()));
        Ok(())
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn debug_size_override(&mut self) {
        self.size.height = 20;
        self.size.width = 160;
    }

    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }

    pub fn cursor_position(position: &Position) {
        let Position { x, y } = position;
        let x = x.saturating_add(1) as u16;
        let y = y.saturating_add(1) as u16;
        print!("{}", termion::cursor::Goto(x, y));
    }

    pub fn cursor_hide() {
        print!("{}", termion::cursor::Hide);
    }
    pub fn cursor_show() {
        print!("{}", termion::cursor::Show);
    }

    pub fn println_color(
        message: &str,
        fg: Box<dyn termion::color::Color>,
        bg: Box<dyn termion::color::Color>,
    ) {
        println!(
            "{}{}{}{}{}\r",
            termion::color::Bg(bg.as_ref()),
            termion::color::Fg(fg.as_ref()),
            message,
            termion::color::Fg(termion::color::Reset),
            termion::color::Bg(termion::color::Reset)
        );
    }

    pub fn print_color(
        message: &str,
        fg: Box<dyn termion::color::Color>,
        bg: Box<dyn termion::color::Color>,
    ) {
        print!(
            "{}{}{}{}{}",
            termion::color::Bg(bg.as_ref()),
            termion::color::Fg(fg.as_ref()),
            message,
            termion::color::Fg(termion::color::Reset),
            termion::color::Bg(termion::color::Reset)
        );
    }

    pub fn println_bgcolor(message: &str, color: Box<dyn termion::color::Color>) {
        println!(
            "{}{}{}\r",
            termion::color::Bg(color.as_ref()),
            message,
            termion::color::Bg(termion::color::Reset)
        );
    }

    pub fn print_bgcolor(message: &str, color: Box<dyn termion::color::Color>) {
        print!(
            "{}{}{}",
            termion::color::Bg(color.as_ref()),
            message,
            termion::color::Bg(termion::color::Reset)
        );
    }

    pub fn println_fgcolor(message: &str, color: Box<dyn termion::color::Color>) {
        println!(
            "{}{}{}\r",
            termion::color::Fg(color.as_ref()),
            message,
            termion::color::Fg(termion::color::Reset)
        );
    }

    pub fn print_fgcolor(message: &str, color: Box<dyn termion::color::Color>) {
        print!(
            "{}{}{}",
            termion::color::Fg(color.as_ref()),
            message,
            termion::color::Fg(termion::color::Reset)
        );
    }

    pub fn print_list_box(message: Vec<BoxSelection>, start: Position, size: (usize, usize)) {
        let mut position = start;
        let prev_position = stdout().cursor_pos().unwrap();
        OldTerminal::cursor_position(&position);
        print!("{}{}{}", "┌", "─".repeat(size.0), "┐");
        for x in 0..message.len() {
            position.y += 1;
            OldTerminal::cursor_position(&position);
            print!("│");
            if x == 0 {
                OldTerminal::print_fgcolor(&*message[x].label,
                                           Box::new(termion::color::Blue));
            } else if message[x].selected {
                OldTerminal::print_bgcolor(&*message[x].label,
                                           Box::new(termion::color::Blue));
            } else {
                print!("{}", &*message[x].label);
            }
            print!("{}│", " ".repeat(size.0 - message[x].label.len()));
        }
        position.y += 1;
        OldTerminal::cursor_position(&position);
        print!("{}{}{}", "└", "─".repeat(size.0), "┘");
        OldTerminal::cursor_position(&Position {
            x: prev_position.0 as usize,
            y: prev_position.1 as usize,
        })
    }

    pub fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }
}
