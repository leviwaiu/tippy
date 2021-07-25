use serde_json::json;
use reqwest::Client;

pub struct AniClient {
    pub client: Client,
}

impl AniClient {
    fn default() -> Self {
        Self{
            client: Client::new(),
        }
    }

    async fn make_query(self, json_input: serde_json::Value) -> serde_json::Value {
        let json = json_input;
        let resp = self.client.post("https://graphql.anilist.co/")
            .header("Content-Type", "application/json")
            .header("Accept","application/json")
            .body(json.to_string())
            .send()
            .await
            .unwrap()
            .text()
            .await;
        serde_json::from_str(&resp.unwrap()).unwrap()
    }
}
