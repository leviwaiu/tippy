use crate::entry::Entry;
use termion::color;
use crate::terminal::{Terminal, Position};
use unicode_width::UnicodeWidthStr;
use unicode_segmentation::UnicodeSegmentation;
use crate::scene::SceneTrait;
use std::any::Any;
use crate::anilist_interface::AniListInterface;


pub struct MainList {
    anime_list:Vec<Entry>,
    terminal: Terminal,
    offset: Position,
    selected: Position,
}

impl SceneTrait for MainList {
    fn show_view(&self){
        Terminal::clear_screen();
        Terminal::println_bgcolor(&*self.format_title(), Box::new(color::Blue));
        self.print_list();
        Terminal::print_fgcolor(&*self.format_status_row(), Box::new(color::Blue));
    }

    fn format_status_row(&self) -> String {
        return format!("{}", "Welcome to Tippy!");
    }

}

impl MainList {
    pub fn default() -> Self{
        Self{
            anime_list: Vec::new(),
            terminal: Terminal::default().expect("Terminal Initialisation Failed"),
            offset: Position::default(),
            selected: Position::default(),
        }
    }

    pub fn print_list(&self) {
        let height = self.terminal.size().height;

        for terminal_row in 0..height - 2 {
            if self.anime_list.len() > 0 && self.anime_list.len() > terminal_row as usize {
                let index = self.offset.y.saturating_add(terminal_row as usize);
                let entry = self.anime_list[index].clone();
                if terminal_row as usize == self.selected.y.saturating_sub(self.offset.y) {
                    Terminal::println_color(&*self.format_entry(entry),
                                            Box::new(color::Black), Box::new(color::White));
                } else {
                    println!("{}\r", self.format_entry(entry));
                }
            } else {
                println!("\r");
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
                                    &entry.watched_count().to_string(),
                                    &entry.total_count().to_string());
        let labels: [&str;4] = [&entry.title(), &entry.score().to_string(),
            &episode_count, &entry.status().to_description()];
        self.format_row(labels, false)
    }

    fn format_row(&self, labels:[&str;4], end_padding:bool) -> String{
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

    pub fn set_anime_list(&mut self, list: Vec<Entry>){
        self.anime_list = list;
    }

}

