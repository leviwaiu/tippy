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
        let width = self.terminal.size().width as usize;
        Terminal::clear_screen();
        println!("{}\r", self.format_title(width));
        println!("{}\r", "═".repeat(width));
        for terminal_row in 0..height - 1 {

        }
    }
    fn format_title(&self, width: usize) -> String{
        //Langauge support planning for the far future?
        let label_name = "Name";
        let label_score = "Score";
        let label_progress = "Progress";
        let label_type = "Type";

        let namecol_padding = " ".repeat(width / 2 - label_name.len());
        let scorecol_padding = " ".repeat(width / 6 - label_score.len());
        let progresscol_padding = " ".repeat(width / 6 - label_progress.len());
        format!("{}{}{}{}{}{}{}",
                label_name, namecol_padding, label_score, scorecol_padding,
                label_progress, progresscol_padding, label_type)
    }
}

fn die(e: &std::io::Error){
    Terminal::clear_screen();
    panic!("{}",e);
}