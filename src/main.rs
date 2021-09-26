use tippy::Tippy;

mod terminal;
mod tippy;
mod entry;
mod anilist_client;
mod secrets;
mod anilist_interface;
mod settings;
mod mainlist;

fn main() {
    Tippy::default().run()
}
