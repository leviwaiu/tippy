use futures::{executor};
use crate::anilist_client::AniListClient;

pub struct AniListInterface {
    client: AniListClient,
    user_id: Option<String>,
}

impl AniListInterface {
    pub fn default() -> Self {
        Self {
            client: AniListClient::default(),
            user_id: None,
        }
    }

    pub fn authentication(&mut self) {
        let mut code: String;
        match AniListClient::fetch_code() {
            Ok(c) => code = c,
            Err(_) => panic!("There is a problem!"),
        }
        let _authcode_clone = code.clone();
        let auth_reply = AniListClient::fetch_authcode(&_authcode_clone);
        let result = match auth_reply {
            Ok(res) => res,
            Err(_) => panic!("Error while fetching authcode"),
        };
        self.client.set_auth(Some(result.access_token));
    }

    pub fn fetch_current_user(&mut self) -> serde_json::Result<u64> {
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
        Ok(user_id)
    }

    pub fn fetch_anime_list(&mut self) {
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
                    mediaId
                    score
                    progress
                }
            }
        }";
        let serde_query = serde_json::json!({"query":query, "variables": {
            "userId": self.user_id.as_ref().unwrap(),
            "page":1,
            "perPage":50,
        }});
        let fut_resp =
            self.client.fetch_auth_content(serde_query);
        let result = match fut_resp {
            Ok(res) => res,
            Err(_) => panic!("Error while fetching authcode"),
        };
        println!("{}", result);
    }
}