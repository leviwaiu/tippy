use tippy::Tippy;

mod terminal;
mod tippy;
mod list_entry;
mod secrets;
mod scene;
mod anilist;
mod anime_entry;

fn main() {
    Tippy::default().run()
}
