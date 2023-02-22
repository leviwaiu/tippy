use crate::list_entry::ListStatus;

pub struct AnimeSearchEntry {
    title: String,
    mode: String,
    season: String,
    status: Option<ListStatus>,
}

impl AnimeSearchEntry {

    pub fn default(title:String, mode:String, season:String, status:Option<ListStatus>) -> Self {
        AnimeSearchEntry {
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
}
