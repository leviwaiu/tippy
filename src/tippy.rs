use crate::terminal::{Terminal, Position};
use termion::event::Key;
use termion::color;

use crate::entry::{Entry, EntryStatus};
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
            selected: Position{ x: 0, y: 0 },
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
                y: self.selected.y.saturating_sub(self.offset.y) + 1,
            })
        }
        //Terminal::cursor_show();
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
            Key::Char('+')
            | Key::Char('-') => self.edit_entry(pressed_key),
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
            if self.anime_list.len() > 0 && self.anime_list.len() > terminal_row as usize {
                let index = self.offset.y.saturating_add(terminal_row as usize);
                let entry = self.anime_list[index].clone();
                if terminal_row as usize == self.selected.y.saturating_sub(self.offset.y) {
                    println!("{}{}{}{}{}\r",
                             color::Bg(color::White), color::Fg(color::Black),
                             self.format_entry(entry),
                             color::Bg(color::Reset),color::Fg(color::Reset)
                    );
                }
                else {
                    println!("{}\r", self.format_entry(entry));
                }
            }
            else {
                println!("\r");
            }
        }
        print!("{}{}{}", color::Fg(color::Blue),self.format_status_row(), color::Fg(color::Reset));
    }
    fn format_title(&self) -> String {
        //Langauge support planning for the far future?
        let labels = ["Name","Score","Progress","Type"];
        self.format_row(labels, true)
    }
    fn format_status_row(&self) -> String {
        let width = self.terminal.size().width;
        return format!("{} {} {} {}", "Welcome to Tippy!",
                       self.offset.y.to_string(), self.selected.y.to_string(), self.terminal.size().height);
    }
    fn format_entry(&self, entry: Entry) -> String {
        let episode_count = format!("{}/{}",
                                    &entry.watched_count.to_string(),
                                    &entry.total_count.to_string());
        let labels: [&str;4] = [&entry.title, &entry.score.to_string(),
                                &episode_count, &entry.status.to_description()];
        self.format_row(labels, false)
    }
    fn format_row(&self, mut labels:[&str;4], end_padding:bool) -> String{
        let width = self.terminal.size().width as usize;

        let mut unicode_widths:Vec<usize> = Vec::new();
        for label in labels {
            let label_width = UnicodeWidthStr::width(label);
            unicode_widths.push(label_width);
        }

        let mut label_one :String = labels[0].to_string();
        let mut pad: isize = width as isize * 3 / 5 - unicode_widths[0] as isize;
        if pad < 5{
            let mut label_list = labels[0].graphemes(true).collect::<Vec<&str>>();
            let mut label = label_list.clone().into_iter().map(|s| s.to_string()).collect::<String>();
            while UnicodeWidthStr::width(label.as_str()) > (width * 3 / 5 - 5) {
                label_list.pop();
                label = label_list.clone().into_iter().map(|s| s.to_string()).collect::<String>();
            }
            label_one = label.clone();
            pad = 5;
        };

        let padding_one = " ".repeat(pad as usize);
        let padding_two = " ".repeat(width / 8 - unicode_widths[1]);
        let padding_three = " ".repeat(width / 8 - unicode_widths[2]);

        let string = format!("{}{}{}{}{}{}{}", label_one, padding_one, labels[1], padding_two,
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
        let Position {x:_, y} = self.selected;
        let _width = self.terminal.size().width as usize;
        let height = self.terminal.size().height as usize;
        let mut offset = &mut self.offset;

        if y <= offset.y {
            offset.y = y;
        }
        else if y >= offset.y.saturating_add(height - 2) {
            offset.y = y.saturating_sub(height - 2).saturating_add(1);
        }
    }
    fn move_cursor(&mut self, key:Key){
        let terminal_height = self.terminal.size().height as usize;
        let Position {x, mut y} = self.selected;
        let list_length = self.anime_list.len();

        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down =>
                if y < list_length.saturating_sub(1) {
                    y = y.saturating_add(1);
                },
            Key::PageUp => {
                y = if y > terminal_height {
                    y.saturating_sub(terminal_height)
                } else {
                    0
                }
            }
            Key::PageDown => {
                y = if y.saturating_add(terminal_height) < list_length {
                    y.saturating_add(terminal_height)
                } else {
                    list_length
                }
            }
            _ => ()
        }

        self.selected = Position {x, y}
    }
    fn edit_entry(&mut self, key:Key){
        let selected_no = self.selected.y;
        match key {
            Key::Char('+') => self.anime_list[selected_no].add_watched(),
            Key::Char('-') => self.anime_list[selected_no].remove_watched(),
            _ => (),
        }
        self.interface.edit_anime_watchcount(self.anime_list[selected_no].clone());

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