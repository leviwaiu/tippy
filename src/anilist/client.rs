use crate::secrets::CLIENT_SECRET;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::io::{ErrorKind, Read, Write};
use std::net::{TcpListener, TcpStream};

pub struct AniListClient {
    is_auth: bool,
    access_token: Option<String>,
}

#[derive(Deserialize)]
pub struct AuthReply {
    expires_in: usize,
    access_token: String,
    refresh_token: String,
}

impl AuthReply {
    pub fn access_token(&self) -> &str {
        &self.access_token
    }
}

impl AniListClient {
    pub fn default() -> Self {
        Self {
            is_auth: false,
            access_token: None,
        }
    }

    pub fn set_auth(&mut self, access_code: Option<String>) {
        self.access_token = access_code;
        self.is_auth = true;
    }

    pub(crate) fn fetch_code() -> Result<String, std::io::Error> {
        let listener = TcpListener::bind("127.0.0.1:25252").unwrap();

        let url =
            "https://anilist.co/api/v2/oauth/authorize?client_id=6075&redirect_uri=http://localhost:25252&response_type=code";
        if webbrowser::open(url).is_ok() {
            for stream in listener.incoming() {
                let code = AniListClient::handle_conn(stream?);
                return Ok(code);
            }
        } else {
            return Err(std::io::Error::new(ErrorKind::Other, "Error Happened!"));
        }
        Err(std::io::Error::new(ErrorKind::Other, "Error Happened!"))
    }

    fn handle_conn(mut stream: TcpStream) -> String {
        let mut code = "";
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();

        let mut headers = [httparse::EMPTY_HEADER; 16];
        let mut req = httparse::Request::new(&mut headers);
        let res = req.parse(&mut buffer).unwrap();
        if res.is_partial() {
            match req.path {
                Some(ref path) => {
                    if path.contains("code") {
                        let path_split = path.split("code=");
                        match path_split.skip(1).next() {
                            Some(res) => code = res,
                            None => {}
                        }
                    }
                    let contents = fs::read_to_string("res/code_success.html").unwrap();

                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                        contents.len(),
                        contents,
                    );
                    stream.write(response.as_bytes()).unwrap();
                    stream.flush().unwrap();
                }
                None => {}
            }
        }
        code.parse().unwrap()
    }

    #[tokio::main]
    pub(crate) async fn fetch_authcode(code: &str) -> Result<AuthReply, reqwest::Error> {
        let client = reqwest::Client::new();
        let mut params = HashMap::new();
        params.insert("grant_type", "authorization_code");
        params.insert("client_id", "6075");
        params.insert("client_secret", CLIENT_SECRET);
        params.insert("redirect_uri", "http://localhost:25252");
        params.insert("code", code);

        let resp = client
            .post("https://anilist.co/api/v2/oauth/token")
            .json(&params)
            .send()
            .await?
            .json::<AuthReply>()
            .await?;
        Ok(resp)
    }

    #[tokio::main]
    pub async fn fetch_auth_content(
        &self,
        query: serde_json::Value,
    ) -> Result<String, reqwest::Error> {
        let client = reqwest::Client::new();
        let resp = client
            .post("https://graphql.anilist.co/")
            .header(
                AUTHORIZATION,
                format!("{} {}", "Bearer", self.access_token.as_ref().unwrap()),
            )
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json")
            .body(query.to_string())
            .send()
            .await?
            .text()
            .await?;
        Ok(resp)
    }

    #[tokio::main]
    pub async fn fetch_content(query: serde_json::Value) -> Result<String, reqwest::Error> {
        let client = reqwest::Client::new();
        let resp = client
            .post("https://graphql.anilist.co/")
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json")
            .body(query.to_string())
            .send()
            .await?
            .text()
            .await?;
        Ok(resp)
    }
}
