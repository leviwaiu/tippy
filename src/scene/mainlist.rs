use crate::entry::{Entry, EntryStatus};
use termion::color;
use crate::terminal::{Terminal, Position};
use unicode_width::UnicodeWidthStr;
use unicode_segmentation::UnicodeSegmentation;
use crate::scene::SceneTrait;
use crate::anilist_interface::AniListInterface;
use termion::event::Key;
use crate::scene::settings::Settings;


pub struct MainList {
    anime_list:Vec<Entry>,
    offset: Position,
    selected: Position,

    for_change:Option<Entry>,
}

impl SceneTrait for MainList {
    fn show_view(&self, terminal: &Terminal){
        Terminal::println_bgcolor(&*self.format_title(terminal), Box::new(color::Blue));
        self.print_list(terminal);
        Terminal::print_fgcolor(&*self.format_status_row(), Box::new(color::Blue));
    }

    fn format_status_row(&self) -> String {
        return format!("{}", "Welcome to Tippy!");
    }

    fn process_key(&mut self, key:Key, terminal: &Terminal, settings:Settings) {
        match key {
            Key::Up
            | Key::Down
            | Key::PageUp
            | Key::PageDown => self.move_cursor(key, terminal),
            Key::Char('+')
            | Key::Char('-') => self.edit_entry(key, settings),
            Key::Char('s') => (),
            _ => (),
        }
        self.scroll(terminal);
    }

    fn connect_interface(&mut self, interface: &AniListInterface) {
        match self.for_change.clone() {
            Some(item) => {
                interface.edit_anime_watchcount(item);
                self.for_change = None;
            }
            None => (),
        };
    }
}

impl MainList {
    pub fn default() -> Self{
        Self{
            anime_list: Vec::new(),
            offset: Position::default(),
            selected: Position::default(),

            for_change:None,
        }
    }

    fn print_list(&self, terminal: &Terminal) {
        let height = terminal.size().height;

        for terminal_row in 0..height - 2 {
            if self.anime_list.len() > 0 && self.anime_list.len() > terminal_row as usize {
                let index = self.offset.y.saturating_add(terminal_row as usize);
                let entry = self.anime_list[index].clone();
                if terminal_row as usize == self.selected.y.saturating_sub(self.offset.y) {
                    Terminal::println_color(&*self.format_entry(entry, terminal),
                                            Box::new(color::Black), Box::new(color::White));
                } else {
                    println!("{}\r", self.format_entry(entry, terminal));
                }
            } else {
                println!("\r");
            }
        }
    }

    fn format_title(&self, terminal: &Terminal) -> String {
        //Langauge support planning for the far future?
        let labels = ["Name","Score","Progress","Type"];
        self.format_row(labels, true, terminal)
    }

    fn format_entry(&self, entry: Entry, terminal: &Terminal) -> String {
        let episode_count = format!("{}/{}",
                                    &entry.watched_count().to_string(),
                                    &entry.total_count().to_string());
        let labels: [&str;4] = [&entry.title(), &entry.score().to_string(),
            &episode_count, &entry.status().to_description()];
        self.format_row(labels, false, terminal)
    }

    fn format_row(&self, labels:[&str;4], end_padding:bool, terminal: &Terminal) -> String{
        let width = terminal.size().width as usize;

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

    fn move_cursor(&mut self, key:Key, terminal: &Terminal){
        let terminal_height = terminal.size().height as usize;
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

    fn edit_entry(&mut self, key:Key, settings: Settings){
        let selected_no = self.selected.y;
        match key {
            Key::Char('+') => {
                if self.anime_list[selected_no].watched_count() == 0
                    && self.anime_list[selected_no].status() == EntryStatus::PLANNING
                    && settings.auto_change_status()
                {
                    self.anime_list[selected_no].set_status(EntryStatus::CURRENT);
                }
                self.anime_list[selected_no].add_watched()
            },
            Key::Char('-') => self.anime_list[selected_no].remove_watched(),
            _ => (),
        }

        self.for_change = Some(self.anime_list[selected_no].clone());

    }

    fn scroll(&mut self, terminal: &Terminal){
        let Position {x:_, y} = self.selected;
        let _width = terminal.size().width as usize;
        let height = terminal.size().height as usize;
        let mut offset = &mut self.offset;

        if y <= offset.y {
            offset.y = y;
        }
        else if y >= offset.y.saturating_add(height - 2) {
            offset.y = y.saturating_sub(height - 2).saturating_add(1);
        }
    }

    pub fn set_anime_list(&mut self, list: Vec<Entry>){
        self.anime_list = list;
    }

}

