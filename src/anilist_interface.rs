use crate::anilist_client::AniListClient;
use crate::entry::Entry;

pub struct AniListInterface {
    client: AniListClient,
    viewer_id: Option<u64>,
}

impl AniListInterface {
    pub fn default() -> Self {
        Self {
            client: AniListClient::default(),
            viewer_id: None,
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
        self.client.set_auth(Some(result.access_token));
    }

    pub fn fetch_viewer(&mut self) -> serde_json::Result<u64> {
        let query = "
        query{
            Viewer{
                id
            }
        }";
        let serde_query = serde_json::json!({"query":query});
        let fut_resp =
            self.client.fetch_auth_content(serde_query);
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

    fn fetch_anime_list_page(&mut self, page:u8) -> serde_json::Result<serde_json::Value> {
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
                        type
                    }
                    score
                    progress
                }
            }
        }";
        let serde_query = serde_json::json!({"query":query, "variables": {
            "userId": self.viewer_id.as_ref().unwrap(),
            "page":1,
            "perPage":50,
        }});
        let fut_resp =
            self.client.fetch_auth_content(serde_query);
        let result = match fut_resp {
            Ok(res) => res,
            Err(_) => panic!("Error while fetching authcode"),
        };
        let res: serde_json::Value = serde_json::from_str(&result)?;
        Ok(res)
    }


    pub fn fetch_anime_list(&mut self) -> Vec<Entry>{
        let mut anime_list = Vec::new();
        let firstpage = self.fetch_anime_list_page(1).unwrap();
        let list = firstpage["data"]["Page"]["mediaList"].as_array().unwrap();
        for item in list {
            let count = match item["media"]["episodes"].as_u64(){
                Some(n) => n,
                None => 9999
            };
            let new_entry = Entry::new(
                item["id"].as_u64().unwrap(),
                String::from(item["media"]["title"]["romaji"].as_str().unwrap()),
                item["progress"].as_u64().unwrap(),
                count,
                item["media"]["type"].to_string(),
                item["score"].as_u64().unwrap(),
            );
            anime_list.push(new_entry);
        }
        anime_list
    }
}