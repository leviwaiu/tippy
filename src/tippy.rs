use crate::terminal::{Terminal, Position};
use termion::event::Key;
use termion::color;
use webbrowser;
use futures::executor::block_on;

use crate::entry::Entry;
use crate::secrets::CLIENT_SECRET;
use reqwest::Response;
use crate::interface::Interface;
use futures::executor;

pub struct Tippy{
    terminal: Terminal,
    anilist: Vec<Entry>,
    quit: bool,
    code: String,
}

impl Tippy{
    pub fn default() -> Self {
        Self {
            terminal: Terminal::default().expect("Terminal Initialisation Failed"),
            anilist: Vec::new(),
            quit: false,
            code: "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiIsImp0aSI6ImQyZmY5MDQ3ZWFkMTI1ZTJlNzJiOWQxMmRkYzBjNTQ2NmY3MTY1MDIxZDQwYzI4MzY3YjUxZTFjNzM0NzJlZWRhOGU5ZTM1NTlkY2IyN2FiIn0.eyJhdWQiOiI2MDc1IiwianRpIjoiZDJmZjkwNDdlYWQxMjVlMmU3MmI5ZDEyZGRjMGM1NDY2ZjcxNjUwMjFkNDBjMjgzNjdiNTFlMWM3MzQ3MmVlZGE4ZTllMzU1OWRjYjI3YWIiLCJpYXQiOjE2Mjk1NTU5MzksIm5iZiI6MTYyOTU1NTkzOSwiZXhwIjoxNjYxMDkxOTM5LCJzdWIiOiI1NDM5NzMwIiwic2NvcGVzIjpbXX0.pRGNTxqc7GcZwqbiXgJjrWbvkUAbNEY56h_vJlwHhiyJTX3bgBX328-frpnpsw0588gLn-V39s3-hAMXg3JHAeyBNsLUuqVP3fdnZykf1DwEoYyARF_QXity-gN8DHBnRbCkfxdyxHBI9awZSplEjPdbMYDaqqEr_0laAXPJU5p3xs-slJ1nV3V87zJL3OtQLBIe2ZFJNp3RM09GIGVnGlG9iAsnhflAH5qB_9W1kmeMjSJs0SyBEroTe2t9XaaJ4dce7revF_CCHeF9HcAH_LjGlkNmYgGwYCG61pXs00qaE7itUgPwORGqk_UbvYLUM5y9pOV48hg76_UT-z4z-MBI_aKqbu9TQfbKLzYPdk_C4gGTweODWhjNHN_Z1XwFE5euvBHHIGKa42i-5nOJs-XEu9au1OEqwu0tdDLj2MnmF3-s53LVlyU4Z_JuihcsCmVwK4UyQ2sltvHwFF1h2GJCrIA2wRHeTER1oZXpzQ-glS0Bf8WbpoJlnrKZCTYUy97qHJ1T_-uQz2Wg2LJBVT3qog2mB3e08z8_s8tNQJEdzU0pdtGzAsBRNkDrWjjOHQXxZX1yC3-rKZTlajWlPd-Q6-QzXh1GHL6hY1_f8JfHFuH4G2VT_lmvwWmlPXOL_sL8jnbI7ysxHTQqcUU-Fnk0piMFowHVgFZdlXxxpZ8".parse().unwrap(),
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
            _ => (),
        }
        Ok(())
    }
    fn authentication(&mut self){
        match Interface::fetch_code(){
            Ok(code) => self.code = code,
            Err(error) => panic!("There is a problem!"),
        }
        let _authcode_clone = &self.code.clone();
        let authcode_return = Interface::fetch_authcode(_authcode_clone);

        let result = match authcode_return {
            Ok(res) => res,
            Err(error) => panic!("There is a problem! {:?}", error),
        };
        if result == ""{
            let test = "test";
        }
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