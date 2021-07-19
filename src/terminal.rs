use std::io::{self, stdout, Write};
use termion::raw::{RawTerminal, IntoRawMode};
use termion::event::Key;
use termion::input::TermRead;


#[derive(Default)]
pub struct Size{
    pub width: u16,
    pub height: u16,
}

#[derive(Default)]
pub struct Position{
    pub x: usize,
    pub y: usize,
}

pub struct Terminal {
    size: Size,
    _stdout: RawTerminal<std::io::Stdout>
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1,
            },
            _stdout: stdout().into_raw_mode()?,
        })
    }
    pub fn size(&self) -> &Size { &self.size }
    pub fn clear_screen() { print!("{}", termion::clear::All);}

    pub fn flush() -> Result<(), std::io::Error>{ io::stdout().flush() }

    pub fn cursor_position(position: &Position){
        let Position{x, y} = position;
        let x = x.saturating_add(1) as u16;
        let y = y.saturating_add(1) as u16;
        print!("{}", termion::cursor::Goto(x,y));
    }

    pub fn cursor_hide() { print!("{}", termion::cursor::Hide); }
    pub fn cursor_show() { print!{"{}", termion::cursor::Show}; }

    pub fn read_key() -> Result<Key, std::io::Error>{
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }
}