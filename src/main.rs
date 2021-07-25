use tippy::Tippy;

mod terminal;
mod tippy;
mod entry;
mod anilist_connection;
mod interface;

fn main() {
    Tippy::default().run()
}
