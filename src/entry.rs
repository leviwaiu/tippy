pub struct Entry{
    pub id: usize,
    pub title: String,
    pub watched_count: usize,
    pub total_count: usize,
}

impl Entry{
    fn default() -> Self {
        Entry{
            id:10,
            title: "This is only a test".parse().unwrap(),
            watched_count:10,
            total_count:10,
        }
    }



    fn add_watched(mut self){
        if self.watched_count < self.total_count{
            self.watched_count +=1 ;
        }
    }
}