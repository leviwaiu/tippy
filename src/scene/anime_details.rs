use std::io::Stdout;
use crossterm::event::KeyCode;
use tui::backend::CrosstermBackend;
use tui::Frame;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::widgets::List;
use crate::anilist::interface::AniListInterface;
use crate::list_entry::ListEntry;
use crate::scene::Displayable;

pub struct AnimeDetails {
    anime: ListEntry,
    episode_duration: usize,
    start_date: usize,
    end_date: usize,
    average_score: usize,
    mean_score: usize,
}

impl Displayable for AnimeDetails {
    fn widget(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>) {

        let mut fsize_mod = f.size();
        fsize_mod.height -= 1;

        let mut layout = Layout::default().direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(100),
            ]).split(fsize_mod);

        layout.push(Rect {
            x:0,
            y:fsize_mod.height,
            width: fsize_mod.width,
            height:1,
        });



    }

    fn process_key(&mut self, key: KeyCode) {
        todo!()
    }

    fn connect_interface(&mut self, interface: &AniListInterface) {
        todo!()
    }
}

impl AnimeDetails {
    fn default(entry: ListEntry) -> Self {
        Self {
            anime:entry,

            episode_duration: 0,
            start_date: 0,
            end_date: 0,
            average_score: 0,
            mean_score: 0,
        }
    }
}