#[derive(Clone)]
pub struct Entry{
    pub id: usize,
    pub title: String,
    pub watched_count: usize,
    pub total_count: usize,
    pub entry_type: String,
    pub score: u8,
}

impl Entry{
    pub fn default() -> Self {
        Entry{
            id:10,
            title: "This is only a test".to_string(),
            watched_count:10,
            total_count:10,
            entry_type: "Test".to_string(),
            score:0,
        }
    }

    pub fn new(id:u64, title: String, watched_count:u64, total_count:u64, entry_type:String, score:u64 ) -> Self {
        Self{
            id: id as usize,
            title,
            watched_count: watched_count as usize,
            total_count: total_count as usize,
            entry_type,
            score: score as u8,
        }
    }

    fn set_watched(&mut self, count: usize){
        if count <= self.total_count{
            self.watched_count = count;
        }
    }

    fn add_watched(&mut self){
        if self.watched_count < self.total_count{
            self.watched_count +=1 ;
        }
    }
}