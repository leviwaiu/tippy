use crate::terminal::{Terminal, Position};
use termion::event::Key;

pub struct Tippy{
    terminal: Terminal,
    quit: bool,
}

impl Tippy{
    pub fn default() -> Self {
        Self {
            terminal: Terminal::default().expect("Terminal Initialisation Failed"),
            quit: false,
        }
    }
    pub fn run(&mut self) {
        loop {
            if let Err(error) = self.process_screen_tick() {
                die(&error);
            }
            if self.quit{
                break;
            }
            if let Err(error) = self.process_keypresses() {
                die(&error);
            }

        }
    }
    fn process_screen_tick(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::cursor_position(&Position::default());
        if self.quit {
            Terminal::clear_screen();
            println!("Exiting...\r");
        }
        else {
            self.draw_interface()
        }
        Terminal::cursor_show();
        Terminal::flush()
    }
    fn process_keypresses(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Char('q') => self.quit = true,
            _ => (),
        }
        Ok(())
    }
    fn draw_interface(&self){
        let height = self.terminal.size().height;
        Terminal::clear_screen();
        for terminal_row in 0..height - 1 {

        }
    }
}

fn die(e: &std::io::Error){
    Terminal::clear_screen();
    panic!("{}",e);
}