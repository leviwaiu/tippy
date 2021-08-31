use crate::terminal::{Terminal, Position};
use termion::event::Key;
use termion::color;

use crate::entry::Entry;
use crate::anilist_interface::AniListInterface;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

pub struct Tippy{
    terminal: Terminal,
    anime_list: Vec<Entry>,
    quit: bool,
    interface: AniListInterface,
    selected: Position,
    offset: Position,
}

impl Tippy{
    pub fn default() -> Self {
        Self {
            terminal: Terminal::default().expect("Terminal Initialisation Failed"),
            anime_list: Vec::new(),
            quit: false,
            interface: AniListInterface::default(),
            selected: Position{ x: 0, y: 1 },
            offset: Position::default(),
        }
    }
    pub fn run(&mut self) {

        self.setup();

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
            self.draw_interface();
            Terminal::cursor_position(&Position {
                x: self.selected.x.saturating_sub(self.offset.x),
                y: self.selected.y.saturating_sub(self.offset.y),
            })
        }
        Terminal::cursor_show();
        Terminal::flush()
    }
    fn process_keypresses(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Char('q') => self.quit = true,
            Key::Up
            | Key::Down
            | Key::PageUp
            | Key::PageDown => self.move_cursor(pressed_key),
            _ => (),
        }
        self.scroll();
        Ok(())
    }
    fn draw_interface(&self){
        let height = self.terminal.size().height;
        Terminal::clear_screen();
        println!("{}{}{}\r", color::Bg(color::Blue),self.format_title(), color::Bg(color::Reset));
        for terminal_row  in 0..height - 2 {
            if self.anime_list.len() > 0 {
                let entry = self.anime_list[terminal_row as usize].clone();
                println!("{}\r", self.format_entry(entry));
            }
        }
    }
    fn format_title(&self) -> String {
        //Langauge support planning for the far future?
        let labels = ["Name","Score","Progress","Type"];
        self.format_row(labels, true)
    }
    fn format_entry(&self, entry: Entry) -> String {
        let episode_count = format!("{}/{}",
                                    &entry.watched_count.to_string(),
                                    &entry.total_count.to_string());
        let labels: [&str;4] = [&entry.title, &entry.score.to_string(),
                                &episode_count, &entry.entry_type];
        self.format_row(labels, false)
    }
    fn format_row(&self, labels:[&str;4], end_padding:bool) -> String{
        let width = self.terminal.size().width as usize;

        let mut unicode_widths:Vec<usize> = Vec::new();
        for label in labels {
            let label_width = UnicodeWidthStr::width(label);
            unicode_widths.push(label_width);
        }

        let padding_one = " ".repeat(width / 2 - unicode_widths[0]);
        let padding_two = " ".repeat(width / 8 - unicode_widths[1]);
        let padding_three = " ".repeat(width / 8 - unicode_widths[2]);

        let string = format!("{}{}{}{}{}{}{}", labels[0], padding_one, labels[1], padding_two,
                             labels[2], padding_three, labels[3]);
        if end_padding {
            let padding_four = " ".repeat(width - string.graphemes(true).count());
            format!("{}{}", string, padding_four)
        }
        else {
            string
        }
    }
    fn scroll(&mut self){
        let Position {x, y} = self.selected;
        let width = self.terminal.size().width as usize;
        let height = self.terminal.size().height as usize;
        let mut offset = &mut self.offset;

        if y < offset.y {
            offset.y = y;
        }
        else if y >= offset.y.saturating_add(height) {
            offset.y = y.saturating_sub(height).saturating_add(1);
        }
    }
    fn move_cursor(&mut self, key:Key){
        let terminal_height = self.terminal.size().height as usize;
        let Position {mut x, mut y} = self.selected;
        let list_length = self.anime_list.len();

        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down =>
                if y < list_length {
                    y = y.saturating_add(1);
                },
            _ => ()
        }

        self.selected = Position {x, y}
    }
    fn setup(&mut self){
        self.interface.authentication();
        self.interface.fetch_viewer();
        self.anime_list = self.interface.fetch_anime_list();

        //REMOVE WHEN NEEDED
        //self.terminal.debug_size_override();
    }
}

fn die(e: &std::io::Error){
    Terminal::clear_screen();
    panic!("{}",e);
}