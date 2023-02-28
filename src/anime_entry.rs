//Written on a plane with no wifi so this might be very shit

#[derive(Clone)]
pub struct AnimeEntry {
    media_id:usize,
    title:String,
    episode_count:usize,
    entended_entry: Option<ExtendedInfo>,
}

#[derive(Clone)]
pub enum Format {
    TV,
    TVShort,
    Movie,
    Special,
    OVA,
    ONA,
    Music,
    Manga,
    Novel,
    OneShot
}

#[derive(Clone)]
pub struct ExtendedInfo {

    media: Format, //This can be an enum
    studio: String,
    studio_id: usize,

    episode_duration: usize,
    start_date: usize,
    end_date: usize,
    average_score: usize,
}


impl AnimeEntry {
    pub fn new(media_id: usize, name: String, episode_count: usize) -> Self {
        Self { media_id, title: name, episode_count, entended_entry: None }
    }

    pub fn media_id(&self) -> usize {
        self.media_id
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn episode_count(&self) -> usize {
        self.episode_count
    }


}

impl ExtendedInfo {
    pub fn new(media: Format, studio: String, studio_id: usize, episode_duration:usize,
    start_date:usize, end_date:usize, average_score: usize) -> Self{
        Self{
            media, studio, studio_id, episode_duration, start_date, end_date, average_score
        }
    }
}