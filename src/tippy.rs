use crate::terminal::{Terminal, Position};
use termion::event::Key;
use termion::color;
use webbrowser;
use futures::executor::block_on;

use crate::entry::Entry;
use crate::secrets::CLIENT_SECRET;
use reqwest::Response;
use crate::interface::{Interface, AuthReply};
use futures::executor;

pub struct Tippy{
    terminal: Terminal,
    anilist: Vec<Entry>,
    quit: bool,
    interface: Interface,
}

impl Tippy{
    pub fn default() -> Self {
        Self {
            terminal: Terminal::default().expect("Terminal Initialisation Failed"),
            anilist: Vec::new(),
            quit: false,
            interface: Interface::default(),
        }
    }
    pub fn run(&mut self) {

        self.authentication();

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
        Terminal::flush()
    }
    fn process_keypresses(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Char('q') => self.quit = true,
            Key::Char('d') => (),
            _ => (),
        }
        Ok(())
    }
    fn authentication(&mut self){
        let mut code:String;
        match Interface::fetch_code(){
            Ok(c) =>  code = c,
            Err(error) => panic!("There is a problem!"),
        }
        let _authcode_clone = code.clone();
        let auth_reply = Interface::fetch_authcode(&_authcode_clone);
        let result = match auth_reply {
            Ok(res) => res,
            Err(e) => panic!("Error while fetching authcode"),
        };
        self.interface.set_access_token(result.access_token);
    }

    fn draw_interface(&self){
        let height = self.terminal.size().height;

        Terminal::clear_screen();
        println!("{}{}{}\r", color::Bg(color::Blue),self.format_title(), color::Bg(color::Reset));
        for terminal_row in 0..height - 2 {
            if self.anilist.len() == 0 {
                println!("{}\r", self.format_entry(Entry::default()));
            }
        }
    }
    fn format_title(&self) -> String {
        //Langauge support planning for the far future?
        let labels = ["Name","Score","Progress","Type"];
        self.format_row(labels)
    }
    fn format_entry(&self, entry: Entry) -> String {
        let labels: [&str;4] = [&entry.title, &entry.watched_count.to_string(),
                                &entry.total_count.to_string(), &entry.entry_type];
        self.format_row(labels)
    }
    fn format_row(&self, labels:[&str;4]) -> String{
        let width = self.terminal.size().width as usize;

        let padding_one = " ".repeat(width / 2 - labels[0].len());
        let padding_two = " ".repeat(width / 8 - labels[1].len());
        let padding_three = " ".repeat(width / 8 - labels[2].len());

        let string = format!("{}{}{}{}{}{}{}", labels[0], padding_one, labels[1], padding_two,
                             labels[2], padding_three, labels[3]);
        let padding_four = " ".repeat(width - string.len());
        format!("{}{}", string, padding_four)
    }
}

fn die(e: &std::io::Error){
    Terminal::clear_screen();
    panic!("{}",e);
}