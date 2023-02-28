use crate::list_entry::ListStatus;

pub struct AnimeSearchEntry {
    media_id: usize,
    title: String,
    mode: String,
    season: String,
    status: Option<ListStatus>,
}

impl AnimeSearchEntry {

    pub fn default(media_id: usize, title:String, mode:String, season:String, status:Option<ListStatus>) -> Self {
        AnimeSearchEntry {
            media_id,
            title,
            mode,
            season,
            status
        }
    }

    pub fn make_vec(&self) -> Vec<String> {
        let status = match &self.status {
            Some(x) => x.to_string(),
            None => String::from(" "),
        };

        Vec::from([self.title.clone(),
            status,
            self.season.clone(),
            self.mode.clone()])
    }

    pub fn get_id(&self) -> usize {
        self.media_id
    }
}
