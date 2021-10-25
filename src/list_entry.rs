use strum_macros::EnumIter;

#[derive(Clone)]
pub struct ListEntry {
    id: usize,
    title: String,
    title_length: Option<usize>,
    watched_count: usize,
    total_count: usize,
    status: ListStatus,
    score: u8,
}

#[derive(Clone, Eq, PartialEq, EnumIter)]
pub enum ListStatus {
    CURRENT,
    PLANNING,
    COMPLETED,
    DROPPED,
    PAUSED,
    REPEATING
}

impl ListStatus {
    pub fn to_description(&self) -> String {
         match self {
             Self::CURRENT => "Watching",
             Self::PLANNING => "Plan to Watch",
             Self::COMPLETED => "Completed",
             Self::DROPPED => "Dropped",
             Self::PAUSED => "Paused",
             Self::REPEATING => "Watching (R)",
        }.to_string()
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::CURRENT => "CURRENT",
            Self::PLANNING => "PLANNING",
            Self::COMPLETED => "COMPLETED",
            Self::DROPPED => "DROPPED",
            Self::PAUSED => "PAUSED",
            Self::REPEATING => "REPEATING",
        }.to_string()
    }

    pub fn from_string(str:&str) -> Option<Self> {
        match str {
            "CURRENT" => Some(Self::CURRENT),
            "PLANNING" => Some(Self::PLANNING),
            "COMPLETED" => Some(Self::COMPLETED),
            "DROPPED" => Some(Self::DROPPED),
            "PAUSED" => Some(Self::PAUSED),
            "REPEATING" => Some(Self::REPEATING),
            _ => None,
        }
    }

    pub fn from_description(str:&str)-> Option<Self> {
        match str {
            "Watching" => Some(Self::CURRENT),
            "Plan to Watch" => Some(Self::PLANNING),
            "Completed" => Some(Self::COMPLETED),
            "Dropped" => Some(Self::DROPPED),
            "Paused" => Some(Self::PAUSED),
            "Watching (R)" => Some(Self::REPEATING),
            _ => None,
        }
    }
}

impl ListEntry {

    pub fn new(id:u64, title: String, watched_count:u64, total_count:u64, entry_type: ListStatus, score:u64 ) -> Self {
        Self{
            id: id as usize,
            title,
            title_length: None,
            watched_count: watched_count as usize,
            total_count: total_count as usize,
            status: entry_type,
            score: score as u8,
        }
    }


    pub fn add_watched(&mut self){
        if self.watched_count < self.total_count{
            self.watched_count +=1 ;
        }
    }

    pub fn remove_watched(&mut self){
        self.watched_count = self.watched_count.saturating_sub(1);
    }

    pub fn id(&self) -> usize {
        self.id
    }
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn watched_count(&self) -> usize {
        self.watched_count
    }
    pub fn total_count(&self) -> usize {
        self.total_count
    }
    pub fn status(&self) -> ListStatus {
        self.status.clone()
    }
    pub fn score(&self) -> u8 {
        self.score
    }


    pub fn set_status(&mut self, status: ListStatus) {
        self.status = status;
    }

}