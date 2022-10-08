use crate::anilist::client::AniListClient;
use crate::list_entry::{ListEntry, ListStatus};

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
        let query = "
        query{
            Viewer{
                id
            }
        }";
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
        let query = "
        query($userId: Int, $page: Int, $perPage: Int){
            Page(page:$page, perPage: $perPage){
                pageInfo {
                    total
                    currentPage
                    lastPage
                    hasNextPage
                    perPage
                }
                mediaList(userId:$userId, type:ANIME){
                    id
                    media {
                        title {
                            romaji
                            native
                        }
                        episodes
                    }
                    score
                    progress
                    status
                }
            }
        }";
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

    fn fetch_anime_list_page_filtered(
        &self,
        page: u8,
        status: ListStatus,
    ) -> serde_json::Result<serde_json::Value> {
        let query = "
        query($userId: Int, $page: Int, $perPage: Int, $status: [MediaListStatus]){
            Page(page:$page, perPage: $perPage){
                pageInfo {
                    total
                    currentPage
                    lastPage
                    hasNextPage
                    perPage
                }
                mediaList(userId:$userId, type:ANIME, status_in: $status){
                    id
                    media {
                        id
                        title {
                            romaji
                            native
                        }
                        episodes
                    }
                    score
                    progress
                    status
                }
            }
        }";
        let serde_query = serde_json::json!({"query":query, "variables": {
            "userId": self.viewer_id.as_ref().unwrap(),
            "page": page,
            "perPage":50,
            "status": [status.to_string()],
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

    pub fn fetch_anime_list(&mut self, statusfilter: ListStatus) -> Vec<ListEntry> {
        let mut anime_list = Vec::new();
        let firstpage = self
            .fetch_anime_list_page_filtered(1, statusfilter.clone())
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
                .fetch_anime_list_page_filtered(x as u8, statusfilter.clone())
                .unwrap();
            let list = nextpage["data"]["Page"]["mediaList"].as_array().unwrap();
            anime_list.extend(AniListInterface::process_anime_entry(list));
            has_next_page = nextpage["data"]["Page"]["pageInfo"]["hasNextPage"].as_bool().unwrap();
        }
        self.main_list = anime_list.clone();
        anime_list
    }

    pub fn edit_anime_watchcount(
        &self,
        edited_entry: ListEntry,
    ) -> serde_json::Result<serde_json::Value> {
        let query = "
        mutation($id: Int, $progress: Int){
            SaveMediaListEntry(id: $id, progress:$progress) {
                id
                progress
            }
        }
        ";
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

    pub fn search_anime(&self, keyword: String) -> serde_json::Result<serde_json::Value> {
        let query = "
        query($keyword: String, $page: Int, $perPage: Int){
            Page(page:$page, perPage: $perPage){
                pageInfo {
                    total
                    currentPage
                    lastPage
                    hasNextPage
                    perPage
                }
                media(type:ANIME, search:$keyword){
                    id
                    title {
                        romaji
                        native
                    }
                    format
                }
            }
        }
        ";
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
        Ok(res)
    }

    pub fn get_main_list(&self) -> Vec<ListEntry> {self.main_list.clone()}
}
