use crate::anilist::interface::AniListInterface;
use crate::list_entry::{ListEntry, ListStatus};
use crate::scene::settings::Settings;
use crate::scene::SceneTrait;
use crate::terminal::{BoxSelection, Position, OldTerminal};
use termion::color;
use termion::event::Key;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use strum::IntoEnumIterator;

pub struct MainList {
    anime_list: Vec<ListEntry>,
    offset: Position,
    selected: Position,

    for_change: Option<ListEntry>,

    current_sort: ListStatus,
    sort_change: bool,

    sort_select: Option<SortSelect>,
}

struct SortSelect {
    list: Vec<BoxSelection>,
    current_sel: usize,
}

impl SortSelect {
    pub fn default(current_sort: ListStatus) -> Self {
        let mut current_sel = 0;
        Self {
            list: {
                let mut vec = Vec::new();
                let mut counter = 0;
                for status in ListStatus::iter() {
                    if status == current_sort {
                        vec.push(BoxSelection {
                            label: status.to_description(),
                            selected: true,
                        });
                        current_sel = counter;
                    } else {
                        vec.push(BoxSelection {
                            label: status.to_description(),
                            selected: false,
                        });
                    }
                    counter += 1;
                }
                vec
            },
            current_sel: current_sel as usize,
        }
    }
}

impl SceneTrait for MainList {
    fn show_view(&self, terminal: &OldTerminal) {
        OldTerminal::println_bgcolor(&*self.format_title(terminal), Box::new(color::Blue));
        self.print_list(terminal);
        OldTerminal::print_fgcolor(&*self.format_status_row(), Box::new(color::Blue));
        if let Some(x) = &self.sort_select {
            self.show_sorting(terminal);
        }
    }

    fn format_status_row(&self) -> String {
        return format!("{}{}", "Welcome to Tippy!", self.offset.y);
    }

    fn process_key(&mut self, key: Key, terminal: &OldTerminal, settings: Settings) {
        match key {
            Key::Up | Key::Down | Key::PageUp | Key::PageDown | Key::Char('\n') => {
                self.move_cursor(key, terminal)
            }
            Key::Char('+') | Key::Char('-') => self.edit_entry(key, settings),
            Key::Char('s') => match self.sort_select {
                Some(_) => self.sort_select = None,
                None => self.sort_select = Some(SortSelect::default(self.current_sort.clone())),
            },
            _ => (),
        }
        self.scroll(terminal);
    }

    fn connect_interface(&mut self, interface: &mut AniListInterface) {
        match self.for_change.clone() {
            Some(item) => {
                interface.edit_anime_watchcount(item);
                self.for_change = None;
            }
            None => (),
        };
        if self.sort_change {
            self.set_anime_list(interface.fetch_anime_list(self.current_sort.clone()));
            self.selected = Position::default();
            self.offset = Position::default();
            self.sort_change = false;
        }
    }
}

impl MainList {
    pub fn default() -> Self {
        Self {
            anime_list: Vec::new(),
            offset: Position::default(),
            selected: Position::default(),

            for_change: None,

            current_sort: ListStatus::CURRENT,
            sort_change: false,

            sort_select: None,
        }
    }

    fn print_list(&self, terminal: &OldTerminal) {
        let height = terminal.size().height;

        for terminal_row in 0..height - 2 {
            if self.anime_list.len() > 0 && self.anime_list.len() > terminal_row as usize {
                let index = self.offset.y.saturating_add(terminal_row as usize);
                let entry = self.anime_list[index].clone();
                if terminal_row as usize == self.selected.y.saturating_sub(self.offset.y) {
                    OldTerminal::println_color(
                        &*self.format_entry(entry, terminal),
                        Box::new(color::Black),
                        Box::new(color::White),
                    );
                } else {
                    println!("{}\r", self.format_entry(entry, terminal));
                }
            } else {
                println!("\r");
            }
        }
    }

    fn format_title(&self, terminal: &OldTerminal) -> String {
        //Langauge support planning for the far future?
        let labels = ["Name", "Score", "Progress", "Type"];
        self.format_row(labels, true, terminal)
    }

    fn format_entry(&self, entry: ListEntry, terminal: &OldTerminal) -> String {
        let episode_count = format!(
            "{}/{}",
            &entry.watched_count().to_string(),
            &entry.total_count().to_string()
        );
        let labels: [&str; 4] = [
            &entry.title(),
            &entry.score().to_string(),
            &episode_count,
            &entry.status().to_description(),
        ];
        self.format_row(labels, false, terminal)
    }

    fn format_row(&self, labels: [&str; 4], end_padding: bool, terminal: &OldTerminal) -> String {
        let width = terminal.size().width as usize;

        let mut unicode_widths: Vec<usize> = Vec::new();
        for label in labels {
            let label_width = UnicodeWidthStr::width(label);
            unicode_widths.push(label_width);
        }

        let mut label_one: String = labels[0].to_string();
        let mut pad: isize = width as isize * 3 / 5 - unicode_widths[0] as isize;
        if pad < 5 {
            let mut label_list = labels[0].graphemes(true).collect::<Vec<&str>>();
            let mut label = label_list
                .clone()
                .into_iter()
                .map(|s| s.to_string())
                .collect::<String>();
            while UnicodeWidthStr::width(label.as_str()) > ((width * 3 / 5) - 5) {
                label_list.pop();
                label = label_list
                    .clone()
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect::<String>();
            }
            label_one = label.clone();
            pad = 5;
        };

        let padding_one = " ".repeat(pad as usize);
        let padding_two = " ".repeat(width / 8 - unicode_widths[1]);
        let padding_three = " ".repeat(width / 8 - unicode_widths[2]);

        let string = format!(
            "{}{}{}{}{}{}{}",
            label_one, padding_one, labels[1], padding_two, labels[2], padding_three, labels[3]
        );
        if end_padding {
            let padding_four = " ".repeat(width - string.graphemes(true).count());
            format!("{}{}", string, padding_four)
        } else {
            string
        }
    }

    fn show_sorting(&self, terminal: &OldTerminal) {
        let terminal_height = terminal.size().height as usize;
        let terminal_width = terminal.size().width as usize;
        let box_start = (terminal_height / 2) - 5;
        let box_width = (terminal_width / 2) - 30;

        let mut message_vec = Vec::new();
        message_vec.push(BoxSelection {
            label: String::from("Set List Category:"),
            selected: false,
        });

        let select = self.sort_select.as_ref().unwrap();
        let boxmessages: &Vec<BoxSelection> = select.list.as_ref();
        for x in 0..select.list.len() {
            message_vec.push(boxmessages[x].clone());
        }

        OldTerminal::print_list_box(
            message_vec,
            Position {
                x: box_width,
                y: box_start,
            },
            (30, 6),
        )
    }

    fn move_cursor(&mut self, key: Key, terminal: &OldTerminal) {
        let terminal_height = terminal.size().height as usize;
        let Position { x, mut y } = self.selected;
        let list_length = self.anime_list.len();

        if let Some(x) = self.sort_select.as_mut() {
            let mut enter = false;
            match key {
                Key::Up => x.current_sel = x.current_sel.saturating_sub(1),
                Key::Down => {
                    if x.current_sel <= (x.list.len() - 2) {
                        x.current_sel = x.current_sel.saturating_add(1);
                    }
                }
                Key::Char('\n') => {
                    enter = true;
                }
                _ => (),
            }
            for no in 0..x.list.len() {
                if no == x.current_sel {
                    if enter {
                        self.current_sort =
                            ListStatus::from_description(&*x.list[no].label).unwrap();
                        self.sort_select = None;
                        self.sort_change = true;
                        return;
                    } else {
                        x.list[no].selected = true;
                    }
                } else {
                    x.list[no].selected = false;
                }
            }
        } else {
            match key {
                Key::Up => y = y.saturating_sub(1),
                Key::Down => {
                    if y < list_length.saturating_sub(1) {
                        y = y.saturating_add(1);
                    }
                }
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
                _ => (),
            }
        }

        self.selected = Position { x, y }
    }

    fn edit_entry(&mut self, key: Key, settings: Settings) {
        let selected_no = self.selected.y;
        match key {
            Key::Char('+') => {
                if self.anime_list[selected_no].watched_count() == 0
                    && self.anime_list[selected_no].status() == ListStatus::PLANNING
                    && settings.auto_change_status()
                {
                    self.anime_list[selected_no].set_status(ListStatus::CURRENT);
                }
                self.anime_list[selected_no].add_watched()
            }
            Key::Char('-') => self.anime_list[selected_no].remove_watched(),
            _ => (),
        }

        self.for_change = Some(self.anime_list[selected_no].clone());
    }

    fn scroll(&mut self, terminal: &OldTerminal) {
        let Position { x: _, y } = self.selected;
        let _width = terminal.size().width as usize;
        let height = terminal.size().height as usize;
        let mut offset = &mut self.offset;

        if y <= offset.y {
            offset.y = y;
        } else if y >= offset.y.saturating_add(height - 2) {
            offset.y = y.saturating_sub(height - 2).saturating_add(1);
        }
    }

    pub fn get_anime_list(&self) -> Vec<ListEntry> {self.anime_list.clone()}

    pub fn set_anime_list(&mut self, list: Vec<ListEntry>) {
        self.anime_list = list;
    }

    pub fn current_sort(&self) -> ListStatus {
        self.current_sort.clone()
    }

    fn temp_debug_ret_string(&self) -> String {
        match self.sort_select.as_ref() {
            None => return String::from(" "),
            Some(sel) => {
                for status in &sel.list {
                    if status.selected {
                        return status.label.clone();
                    }
                }
            }
        }
        String::from(" ")
    }
}

impl SortSelect {}
