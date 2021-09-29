use tippy::Tippy;

mod terminal;
mod tippy;
mod entry;
mod anilist_client;
mod secrets;
mod anilist_interface;
mod scene;

fn main() {
    Tippy::default().run()
}
