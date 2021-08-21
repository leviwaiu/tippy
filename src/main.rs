use tippy::Tippy;

mod terminal;
mod tippy;
mod entry;
mod anilist_connection;
mod interface;
mod secrets;

fn main() {
    Tippy::default().run()
}
