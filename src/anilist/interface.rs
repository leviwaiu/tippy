use crate::anilist::client::AniListClient;
use crate::anilist::queries::{ANIME_DETAIL_QUERY_STRING, ANIME_LIST_PAGE, ANIME_LIST_PAGE_FILTERED_STRING, EDIT_WATCHCOUNT_STRING, SEARCH_STRING, VIEWER_QUERY_STRING};
use crate::anime_entry::ExtendedInfo;
use crate::list_entry::{ListEntry, ListStatus};
use crate::search_entry::AnimeSearchEntry;


pub struct AniListInterface {
    client: AniListClient,
    viewer_id: Option<u64>,

    main_list: Vec<ListEntry>,
}


impl AniListInterface {
    pub fn default() -> Self {
        Self {
            client: AniListClient::default(),
            viewer_id: None,

            main_list: Vec::new(),
        }
    }

    pub fn authentication(&mut self) {
        let code = match AniListClient::fetch_code() {
            Ok(c) => c,
            Err(_) => panic!("There is a problem!"),
        };
        let _authcode_clone = code.clone();
        let auth_reply = AniListClient::fetch_authcode(&_authcode_clone);
        let result = match auth_reply {
            Ok(res) => res,
            Err(_) => panic!("Error while fetching authcode"),
        };
        self.client
            .set_auth(Some(result.access_token().to_string()));
    }

    pub fn fetch_viewer(&mut self) -> serde_json::Result<u64> {
        let query = VIEWER_QUERY_STRING;
        let serde_query = serde_json::json!({ "query": query });
        let fut_resp = self.client.fetch_auth_content(serde_query);
        let result = match fut_resp {
            Ok(res) => res,
            Err(_) => panic!("Error while fetching authcode"),
        };
        let res_test: serde_json::Value = serde_json::from_str(&result)?;
        let user_id = match res_test["data"]["Viewer"]["id"].as_u64() {
            Some(num) => num,
            None => panic!("Error while parsing number"),
        };
        self.viewer_id = Some(user_id);
        Ok(user_id)
    }

    fn fetch_anime_list_page(&mut self, page: u8) -> serde_json::Result<serde_json::Value> {
        let query = ANIME_LIST_PAGE;
        let serde_query = serde_json::json!({"query":query, "variables": {
            "userId": self.viewer_id.as_ref().unwrap(),
            "page": page,
            "perPage":50,
        }});
        let fut_resp = self.client.fetch_auth_content(serde_query);
        let result = match fut_resp {
            Ok(res) => res,
            Err(_) => panic!("Error while fetching authcode"),
        };
        let res: serde_json::Value = serde_json::from_str(&result)?;
        Ok(res)
    }

    fn process_anime_entry(anime_list: &Vec<serde_json::Value>) -> Vec<ListEntry> {
        let mut output_list = Vec::new();
        for item in anime_list {
            let count = match item["media"]["episodes"].as_u64() {
                Some(n) => n,
                None => 9999,
            };
            let title = match item["media"]["title"]["native"].as_str() {
                Some(title) => title,
                None => item["media"]["title"]["romaji"].as_str().unwrap(),
            };
            let new_entry = ListEntry::new(
                item["id"].as_u64().unwrap(),
                item["media"]["id"].as_u64().unwrap(),
                String::from(title),
                item["progress"].as_u64().unwrap(),
                count,
                ListStatus::from_string(item["status"].as_str().unwrap()).unwrap(),
                item["score"].as_u64().unwrap(),
            );
            output_list.push(new_entry);
        }
        output_list
    }

    pub fn fetch_full_anime_list(&mut self) {
        let mut anime_list = Vec::new();
        let firstpage = self
            .fetch_anime_list_page(1)
            .unwrap();
        let list = firstpage["data"]["Page"]["mediaList"].as_array().unwrap();
        anime_list.extend(AniListInterface::process_anime_entry(list));

        let mut has_next_page = firstpage["data"]["Page"]["pageInfo"]["hasNextPage"]
            .as_bool()
            .unwrap();
        let mut x = 1;
        while has_next_page {
            x += 1;
            let nextpage = self
                .fetch_anime_list_page(x as u8)
                .unwrap();
            let list = nextpage["data"]["Page"]["mediaList"].as_array().unwrap();
            anime_list.extend(AniListInterface::process_anime_entry(list));
            has_next_page = nextpage["data"]["Page"]["pageInfo"]["hasNextPage"].as_bool().unwrap();
        }
        self.main_list = anime_list.clone();
    }

    pub fn edit_anime_watchcount(
        &self,
        edited_entry: ListEntry,
    ) -> serde_json::Result<serde_json::Value> {
        let query = EDIT_WATCHCOUNT_STRING;
        let serde_query = serde_json::json!({"query":query, "variables": {
            "id": edited_entry.id(),
            "progress": edited_entry.watched_count(),
        }});
        let fut_resp = self.client.fetch_auth_content(serde_query);
        let result = match fut_resp {
            Ok(res) => res,
            Err(_) => panic!("Error while fetching authcode"),
        };
        let res: serde_json::Value = serde_json::from_str(&result)?;
        Ok(res)
    }

    pub fn search_anime(&self, keyword: String) -> serde_json::Result<Vec<AnimeSearchEntry>> {
        let query = SEARCH_STRING;
        let serde_query = serde_json::json!({"query":query, "variables": {
            "keyword": keyword,
            "page": 1,
            "perPage": 50,
        }});
        let fut_resp = self.client.fetch_auth_content(serde_query);
        let result = match fut_resp {
            Ok(res) => res,
            Err(_) => panic!("Error while fetching authcode"),
        };
        let res: serde_json::Value = serde_json::from_str(&result)?;
        let content = res["data"]["Page"]["media"].as_array().unwrap();
        let mut anime_result = Vec::new();
        for x in content {
            let search_id = x["id"].as_u64().unwrap();
            let mut status = None;
            for i in &self.main_list {
                if search_id as usize == i.media_id() {
                    status = Some(i.status());
                }
            }
            let new_res = AnimeSearchEntry::default(
                x["id"].as_u64().unwrap() as usize,
                String::from(x["title"]["native"].as_str().unwrap()),
                String::from(x["format"].as_str().unwrap()),
                format!("{} {}", x["seasonYear"], x["season"]),
                status
            );
            anime_result.push(new_res);
        }
        Ok(anime_result)
    }

    pub fn get_anime_details(&self, media_id:usize) -> serde_json::Result<usize>{
        let query = ANIME_DETAIL_QUERY_STRING;
        let serde_query = serde_json::json!({"query":query, "variables": {
            "media_id": media_id,
        }});
        let fut_resp = self.client.fetch_auth_content(serde_query);
        let result = match fut_resp {
            Ok(res) => res,
            Err(_) => panic!("Error while fetching authcode"),
        };
        let res: serde_json::Value = serde_json::from_str(&result)?;
        let content = res["data"]["Media"].as_array().unwrap();

        Ok(0)
    }





    pub fn get_main_list(&self) -> Vec<ListEntry> {self.main_list.clone()}



}
