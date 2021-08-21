use reqwest::Response;
use crate::secrets::CLIENT_SECRET;

pub struct Interface{
    code:Option<String>,
    authcode:Option<String>
}

impl Interface{


    fn get_code(){

    }

    pub(crate) async fn get_authcode(self) -> Result<Response, reqwest::Error> {
        let client = reqwest::Client::new();
        let params = [
            ("grant_type", "authorization_code"), ("client_id", "6075"),
            ("CLIENT_SECRET", CLIENT_SECRET),
            ("redirect_uri", "http://example.com/callback"),
            ("code", self.code)
        ];
        let resp = client.post("https://anilist.co/api/v2/oauth/token")
            .form(&params).send().await?;
        Ok(resp)
    }
}