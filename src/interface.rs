use reqwest::Response;
use crate::secrets::CLIENT_SECRET;
use std::net::{TcpListener, TcpStream};
use std::collections::HashMap;
use std::{thread, fs};
use std::io::{Read, Write, ErrorKind};
use serde::Deserialize;

pub struct Interface{
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct AuthReply{
    token_type:String,
    expires_in:usize,
    access_token:String,
    refresh_token:String,
}


impl Interface{

    pub fn fetch_code() -> Result<String, std::io::Error>{
        let listener = TcpListener::bind("127.0.0.1:25252").unwrap();

        let url =
            "https://anilist.co/api/v2/oauth/authorize?client_id=6075&redirect_uri=http://localhost:25252&response_type=code";
        if webbrowser::open(url).is_ok() {
            println!("Waiting for the code to be sent back...");
            for stream in listener.incoming() {
                let code = Interface::handle_conn(stream?);
                return Ok(code);
            }
        }
        else {
            return Err(std::io::Error::new(ErrorKind::Other, "Error Happened!"));
        }
        Err(std::io::Error::new(ErrorKind::Other, "Error Happened!"))
    }

    fn handle_conn(mut stream:TcpStream) -> String{
        let mut code = "";
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();

        let mut headers = [httparse::EMPTY_HEADER; 16];
        let mut req = httparse::Request::new(&mut headers);
        let res = req.parse(&mut buffer).unwrap();
        if res.is_partial() {
            match req.path {
                Some(ref path) => {
                    if path.contains("code"){
                        let mut path_split = path.split("code=");
                        match path_split.skip(1).next(){
                            Some(res) => { code = res }
                            None => {}
                        }
                    }
                    let contents = fs::read_to_string("res/code_success.html").unwrap();

                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                        contents.len(), contents,
                    );
                    stream.write(response.as_bytes()).unwrap();
                    stream.flush().unwrap();
                },
                None => {}
            }
        }
        code.parse().unwrap()
    }

    #[tokio::main]
    pub async fn fetch_authcode(code:&str) -> Result<AuthReply, reqwest::Error> {
        let client = reqwest::Client::new();
        let mut params = HashMap::new();
        params.insert("grant_type", "authorization_code");
        params.insert("client_id", "6075");
        params.insert("client_secret", CLIENT_SECRET);
        params.insert("redirect_uri", "http://localhost:25252");
        params.insert("code", code);

        let resp = client.post("https://anilist.co/api/v2/oauth/token")
            .json(&params).send().await?.json::<AuthReply>().await?;
        Ok(resp)
    }


    // pub async fn fetch_anilist() -> Result<Response, reqwest::Error> {
    //     let client = reqwest::Client::new();
    //     let params = [
    //
    //     ];
    // }
}