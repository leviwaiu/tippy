#[derive(Clone)]
pub struct Entry{
    pub id: usize,
    pub title: String,
    pub watched_count: usize,
    pub total_count: usize,
    pub status: EntryStatus,
    pub score: u8,
}

#[derive(Clone)]
pub enum EntryStatus {
    CURRENT,
    PLANNING,
    COMPLETED,
    DROPPED,
    PAUSED,
    REPEATING
}

impl EntryStatus {
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
}

impl Entry{
    pub fn default() -> Self {
        Entry{
            id:10,
            title: "This is only a test".to_string(),
            watched_count:10,
            total_count:10,
            status: EntryStatus::PLANNING,
            score:0,
        }
    }

    pub fn new(id:u64, title: String, watched_count:u64, total_count:u64, entry_type:EntryStatus, score:u64 ) -> Self {
        Self{
            id: id as usize,
            title,
            watched_count: watched_count as usize,
            total_count: total_count as usize,
            status: entry_type,
            score: score as u8,
        }
    }

    fn set_watched(&mut self, count: usize){
        if count <= self.total_count{
            self.watched_count = count;
        }
    }

    pub fn add_watched(&mut self){
        if self.watched_count < self.total_count{
            self.watched_count +=1 ;
        }
    }

    pub fn remove_watched(&mut self){
        self.watched_count.saturating_sub(1);
    }
}